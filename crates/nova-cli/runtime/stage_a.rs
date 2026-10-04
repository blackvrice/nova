//! P03 private runtime, compiled with panic=abort and linked to the emitted object.
//! C symbols are internal compiler/runtime boundaries, not the public Nova FFI ABI.
use std::cell::RefCell;
use std::io::{self, Write};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NovaString {
    ptr: *const u8,
    len: u64,
}
thread_local! {static STRINGS:RefCell<Vec<Vec<u8>>>=const{RefCell::new(Vec::new())};}
fn fatal(reason: &str, file: u32, start: u32, end: u32) -> ! {
    let _ = writeln!(
        io::stderr().lock(),
        "Nova panic: {reason} at file#{file}:{start}..{end}"
    );
    std::process::abort()
}
#[no_mangle]
pub extern "C" fn nova_panic(reason: i32, file: u32, start: u32, end: u32) -> ! {
    fatal(
        match reason {
            1 => "Int32 overflow",
            2 => "division or remainder by zero",
            3 => "integer overflow",
            _ => "runtime invariant failure",
        },
        file,
        start,
        end,
    )
}
/// Safety: only generated code passes valid, live immutable UTF-8 buffers.
unsafe fn bytes<'a>(value: NovaString, file: u32, start: u32, end: u32) -> &'a [u8] {
    if value.len > isize::MAX as u64 {
        fatal("String length overflow", file, start, end)
    }
    if value.len == 0 {
        return &[];
    }
    if value.ptr.is_null() {
        fatal("invalid String pointer", file, start, end)
    }
    unsafe { std::slice::from_raw_parts(value.ptr, value.len as usize) }
}
fn allocate(length: usize, file: u32, start: u32, end: u32) -> Vec<u8> {
    let mut value = Vec::new();
    if value.try_reserve_exact(length).is_err() {
        fatal("String allocation failed", file, start, end)
    }
    value
}
fn retain(value: Vec<u8>, file: u32, start: u32, end: u32) -> NovaString {
    let string = NovaString {
        ptr: value.as_ptr(),
        len: value.len() as u64,
    };
    STRINGS.with(|arena| {
        let mut arena = arena.borrow_mut();
        if arena.try_reserve(1).is_err() {
            fatal("String arena allocation failed", file, start, end)
        }
        arena.push(value);
    });
    string
}
#[no_mangle]
pub unsafe extern "C" fn nova_print(ptr: *const u8, len: u64, file: u32, start: u32, end: u32) {
    let value = unsafe { bytes(NovaString { ptr, len }, file, start, end) };
    let mut output = io::stdout().lock();
    if output
        .write_all(value)
        .and_then(|_| output.write_all(b"\n"))
        .and_then(|_| output.flush())
        .is_err()
    {
        fatal("stdout write failed", file, start, end)
    }
}
#[no_mangle]
pub unsafe extern "C" fn nova_format_int(
    out: *mut NovaString,
    value: i32,
    file: u32,
    start: u32,
    end: u32,
) {
    let mut buffer = [0u8; 12];
    let mut position = buffer.len();
    let mut magnitude = (value as i64).unsigned_abs();
    loop {
        position -= 1;
        buffer[position] = b'0' + (magnitude % 10) as u8;
        magnitude /= 10;
        if magnitude == 0 {
            break;
        }
    }
    if value < 0 {
        position -= 1;
        buffer[position] = b'-';
    }
    let mut text = allocate(buffer.len() - position, file, start, end);
    text.extend_from_slice(&buffer[position..]);
    let result = retain(text, file, start, end);
    unsafe { out.write(result) }
}
fn format_wide(mut magnitude: u64, negative: bool, file: u32, start: u32, end: u32) -> NovaString {
    let mut buffer = [0u8; 21];
    let mut position = buffer.len();
    loop {
        position -= 1;
        buffer[position] = b'0' + (magnitude % 10) as u8;
        magnitude /= 10;
        if magnitude == 0 {
            break;
        }
    }
    if negative {
        position -= 1;
        buffer[position] = b'-';
    }
    let mut text = allocate(buffer.len() - position, file, start, end);
    text.extend_from_slice(&buffer[position..]);
    retain(text, file, start, end)
}
/// Safety: out points to writable NovaString storage owned by generated code.
#[no_mangle]
pub unsafe extern "C" fn nova_format_char(
    out: *mut NovaString,
    value: u32,
    file: u32,
    start: u32,
    end: u32,
) {
    let scalar =
        char::from_u32(value).unwrap_or_else(|| fatal("invalid char scalar", file, start, end));
    let mut buffer = [0u8; 4];
    let encoded = scalar.encode_utf8(&mut buffer).as_bytes();
    let mut text = allocate(encoded.len(), file, start, end);
    text.extend_from_slice(encoded);
    unsafe { out.write(retain(text, file, start, end)) }
}
#[no_mangle]
pub unsafe extern "C" fn nova_format_i64(
    out: *mut NovaString,
    value: i64,
    file: u32,
    start: u32,
    end: u32,
) {
    unsafe {
        out.write(format_wide(
            value.unsigned_abs(),
            value < 0,
            file,
            start,
            end,
        ))
    }
}
#[no_mangle]
pub unsafe extern "C" fn nova_format_u64(
    out: *mut NovaString,
    value: u64,
    file: u32,
    start: u32,
    end: u32,
) {
    unsafe { out.write(format_wide(value, false, file, start, end)) }
}
#[no_mangle]
pub unsafe extern "C" fn nova_format_bool(out: *mut NovaString, value: i32) {
    let text: &'static [u8] = if value == 0 { b"false" } else { b"true" };
    unsafe {
        out.write(NovaString {
            ptr: text.as_ptr(),
            len: text.len() as u64,
        })
    }
}
// Fallible formatting preserves the runtime's source-located allocation policy.
struct FloatText(Vec<u8>);
impl std::fmt::Write for FloatText {
    fn write_str(&mut self, text: &str) -> std::fmt::Result {
        self.0
            .try_reserve(text.len())
            .map_err(|_| std::fmt::Error)?;
        self.0.extend_from_slice(text.as_bytes());
        Ok(())
    }
}
fn float_text(
    arguments: std::fmt::Arguments<'_>,
    bits: u64,
    width: u32,
    file: u32,
    start: u32,
    end: u32,
) -> NovaString {
    let mut text = FloatText(Vec::new());
    if std::fmt::write(&mut text, arguments).is_err() {
        fatal("String allocation failed", file, start, end);
    }
    decimal_even_tie(&mut text.0, bits, width, file, start, end);
    retain(text.0, file, start, end)
}
// Rust's shortest Display candidate rounds decimal ties upward. P09 requires
// the even decimal coefficient. Detect exact midpoint equality with integers:
// M*2^(E+1) == (2*C +/- 1)*10^K. Powers of 2 and 5 are cancelled separately,
// avoiding host arithmetic, approximate comparisons and large integers.
fn decimal_midpoint(
    mut mantissa: u64,
    exponent: i32,
    mut twice_coefficient: u64,
    place: i32,
) -> bool {
    let left_twos = mantissa.trailing_zeros() as i32;
    let right_twos = twice_coefficient.trailing_zeros() as i32;
    mantissa >>= left_twos;
    twice_coefficient >>= right_twos;
    if exponent + 1 + left_twos != place + right_twos {
        return false;
    }
    if place >= 0 {
        for _ in 0..place {
            if mantissa % 5 != 0 {
                return false;
            }
            mantissa /= 5;
        }
    } else {
        for _ in place..0 {
            if twice_coefficient % 5 != 0 {
                return false;
            }
            twice_coefficient /= 5;
        }
    }
    mantissa == twice_coefficient
}
fn decimal_even_tie(text: &mut Vec<u8>, bits: u64, width: u32, file: u32, start: u32, end: u32) {
    let negative = text.first() == Some(&b'-');
    let start_digit = usize::from(negative);
    if !text.get(start_digit).is_some_and(u8::is_ascii_digit) {
        return;
    }
    let point = text.iter().position(|&b| b == b'.').unwrap_or(text.len());
    let last = text.iter().rposition(|&b| b.is_ascii_digit() && b != b'0');
    let Some(last) = last else {
        return;
    };
    let mut coefficient = 0u64;
    for &byte in &text[start_digit..=last] {
        if byte == b'.' {
            continue;
        }
        coefficient = coefficient * 10 + u64::from(byte - b'0');
    }
    if coefficient % 2 == 0 {
        return;
    }
    let place = point as i32 - last as i32 - if last < point { 1 } else { 0 };
    let (p, bias, fraction_mask, exponent_mask) = if width == 32 {
        (23, 127, 0x7fffff, 0xff)
    } else {
        (52, 1023, 0xfffffffffffff, 0x7ff)
    };
    let encoded_exponent = ((bits >> p) & exponent_mask) as i32;
    let mantissa = (bits & fraction_mask) | if encoded_exponent == 0 { 0 } else { 1 << p };
    if mantissa == 0 || encoded_exponent == exponent_mask as i32 {
        return;
    }
    let exponent = encoded_exponent.max(1) - bias - p;
    let adjusted = if decimal_midpoint(mantissa, exponent, coefficient * 2 - 1, place) {
        coefficient - 1
    } else if decimal_midpoint(mantissa, exponent, coefficient * 2 + 1, place) {
        coefficient + 1
    } else {
        return;
    };
    // Fixed decimal has at most 326 characters for binary64. Reserve before
    // modifying so every subsequent write is allocation-free.
    if text.try_reserve(768).is_err() {
        fatal("String allocation failed", file, start, end);
    }
    text.clear();
    if negative {
        text.push(b'-');
    }
    let mut digits = [0u8; 20];
    let mut at = digits.len();
    let mut value = adjusted;
    while value != 0 {
        at -= 1;
        digits[at] = b'0' + (value % 10) as u8;
        value /= 10;
    }
    let digits = &digits[at..];
    let decimal_point = digits.len() as i32 + place;
    if decimal_point <= 0 {
        text.extend_from_slice(b"0.");
        for _ in decimal_point..0 {
            text.push(b'0');
        }
        text.extend_from_slice(digits);
    } else {
        for (i, &byte) in digits.iter().enumerate() {
            if i as i32 == decimal_point {
                text.push(b'.');
            }
            text.push(byte);
        }
        for _ in digits.len() as i32..decimal_point {
            text.push(b'0');
        }
    }
    if text.contains(&b'.') {
        while text.last() == Some(&b'0') {
            text.pop();
        }
        if text.last() == Some(&b'.') {
            text.pop();
        }
    }
}
/// Safety: out points to writable NovaString storage owned by generated code.
#[no_mangle]
pub unsafe extern "C" fn nova_format_f32(
    out: *mut NovaString,
    bits: u32,
    file: u32,
    start: u32,
    end: u32,
) {
    let value = f32::from_bits(bits);
    unsafe {
        out.write(float_text(
            format_args!("{value}"),
            u64::from(bits),
            32,
            file,
            start,
            end,
        ));
    }
}
/// Safety: out points to writable NovaString storage owned by generated code.
#[no_mangle]
pub unsafe extern "C" fn nova_format_f64(
    out: *mut NovaString,
    bits: u64,
    file: u32,
    start: u32,
    end: u32,
) {
    let value = f64::from_bits(bits);
    unsafe {
        out.write(float_text(
            format_args!("{value}"),
            bits,
            64,
            file,
            start,
            end,
        ));
    }
}

pub fn initialize_float_environment() {
    #[cfg(target_arch = "x86_64")]
    {
        let current = 0x1f80u32; // RN, gradual underflow, all exceptions masked.
        unsafe {
            std::arch::asm!("ldmxcsr [{pointer}]", pointer = in(reg) &current, options(nostack));
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    fatal("unsupported float host", 0, 0, 0);
}
#[no_mangle]
pub unsafe extern "C" fn nova_concat(
    out: *mut NovaString,
    parts: *const NovaString,
    count: u64,
    file: u32,
    start: u32,
    end: u32,
) {
    if count > isize::MAX as u64 / std::mem::size_of::<NovaString>() as u64 {
        fatal("String component count overflow", file, start, end)
    }
    let parts = if count == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(parts, count as usize) }
    };
    let mut length = 0usize;
    for &part in parts {
        let value = unsafe { bytes(part, file, start, end) };
        length = length
            .checked_add(value.len())
            .filter(|&len| len <= isize::MAX as usize)
            .unwrap_or_else(|| fatal("String length overflow", file, start, end));
    }
    let mut text = allocate(length, file, start, end);
    for &part in parts {
        text.extend_from_slice(unsafe { bytes(part, file, start, end) });
    }
    let result = retain(text, file, start, end);
    unsafe { out.write(result) }
}
extern "C" {
    fn nova_stage_a_entry();
}
fn main() {
    initialize_float_environment();
    // Link-time bridge is generated only after checking main() -> Unit.
    unsafe { nova_stage_a_entry() };
    STRINGS.with(|arena| arena.borrow_mut().clear());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p09_actual_formatters_match_independent_shortest_rational_oracle() {
        for row in include_str!("../../../tools/tests/fixtures/float-format.tsv").lines() {
            let row: Vec<_> = row.split('\t').collect();
            let bits = u64::from_str_radix(row[1], 16).unwrap();
            let mut out = std::mem::MaybeUninit::<NovaString>::uninit();
            unsafe {
                if row[0] == "32" {
                    nova_format_f32(out.as_mut_ptr(), bits as u32, 0, 1, 2);
                } else {
                    nova_format_f64(out.as_mut_ptr(), bits, 0, 1, 2);
                }
                let value = bytes(out.assume_init(), 0, 1, 2);
                assert_eq!(value, row[2].as_bytes(), "{row:?}");
                assert!(!value.contains(&b'e') && !value.contains(&b'E'));
            }
        }
        STRINGS.with(|arena| arena.borrow_mut().clear());
    }
}
