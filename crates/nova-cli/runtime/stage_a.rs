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
    // Link-time bridge is generated only after checking main() -> Unit.
    unsafe { nova_stage_a_entry() };
    STRINGS.with(|arena| arena.borrow_mut().clear());
}
