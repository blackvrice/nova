"""P09 independent rational IEEE/shortest-decimal oracle. Python stdlib only.

No binary floating arithmetic, host parser, repr/format, or Rust output used.
Regenerate checked-in TSVs with `python tools/tests/float_oracle.py`.
"""
from fractions import Fraction as F
from pathlib import Path
import random


def nearest(x):
    q, r = divmod(x.numerator, x.denominator)
    return q + (2*r > x.denominator or (2*r == x.denominator and q % 2))


def power(base, exponent):
    return F(base**exponent) if exponent >= 0 else F(1, base**-exponent)


def layout(width):
    return (24, 8, 127) if width == 32 else (53, 11, 1023)


def decode(width, bits):
    p, e, bias = layout(width)
    fraction = bits & ((1 << (p-1))-1)
    exponent = (bits >> (p-1)) & ((1 << e)-1)
    if exponent == (1 << e)-1:
        return None
    value = F(fraction + ((1 << (p-1)) if exponent else 0)) * power(2, (exponent or 1)-bias-(p-1))
    return -value if bits >> (width-1) else value


def encode(width, x):
    p, eb, bias = layout(width)
    sign = (1 << (width-1)) if x < 0 else 0
    x = abs(x)
    if not x:
        return sign
    exponent = x.numerator.bit_length()-x.denominator.bit_length()
    if x < power(2, exponent):
        exponent -= 1
    exponent = max(exponent, 1-bias)
    mantissa = nearest(x / power(2, exponent-(p-1)))
    if mantissa >= 1 << p:
        mantissa //= 2
        exponent += 1
    if exponent > bias:
        return sign | (((1 << eb)-1) << (p-1))
    if mantissa < 1 << (p-1):
        return sign | mantissa
    return sign | ((exponent+bias) << (p-1)) | (mantissa-(1 << (p-1)))


def fixed(coefficient, exponent):
    while coefficient % 10 == 0:
        coefficient //= 10
        exponent += 1
    digits = str(coefficient)
    if exponent >= 0:
        return digits + '0'*exponent
    point = len(digits)+exponent
    return digits[:point]+'.'+digits[point:] if point > 0 else '0.'+'0'*(-point)+digits


def shortest(width, bits):
    x = decode(width, bits)
    if x is None:
        p, _, _ = layout(width)
        return 'NaN' if bits & ((1 << (p-1))-1) else ('-' if bits >> (width-1) else '')+'inf'
    if not x:
        return '-0' if bits >> (width-1) else '0'
    negative = x < 0
    x = abs(x)
    target = bits & ((1 << (width-1))-1)
    exponent = len(str(x.numerator))-len(str(x.denominator))
    while x < power(10, exponent):
        exponent -= 1
    while x >= power(10, exponent+1):
        exponent += 1
    for digits in range(1, 10 if width == 32 else 18):
        place = exponent-digits+1
        q = nearest(x / power(10, place))
        candidates = [(abs(F(c)*power(10, place)-x), c % 2, c) for c in (q-1, q, q+1)
                      if c > 0 and encode(width, F(c)*power(10, place)) == target]
        if candidates:
            _, _, coefficient = min(candidates)
            return ('-' if negative else '')+fixed(coefficient, place)
    raise AssertionError((width, bits))


def decimal_exact(x):
    # All IEEE midpoint denominators are powers of two.
    sign = '-' if x < 0 else ''
    x = abs(x)
    places = x.denominator.bit_length()-1
    text = str(x.numerator * 5**places).rjust(places+1, '0')
    return sign + (text[:-places]+'.'+text[-places:] if places else text+'.0')


def generate():
    root = Path(__file__).resolve().parents[2]
    rng = random.Random(0x909)
    rows, literals, operations = [], [], []
    for width in (32, 64):
        p, eb, bias = layout(width)
        infinity = ((1 << eb)-1) << (p-1)
        bits_set = {0, 1, 2, 3, (1 << (p-1))-1, 1 << (p-1), infinity-1, infinity,
                    infinity+1, infinity | (1 << (p-2)), 1 << (width-1)}
        for exp in range(1, (1 << eb)-1, 7 if width == 32 else 47):
            for frac in (0, 1, (1 << (p-2)), (1 << (p-1))-1):
                bits_set.update((exp << (p-1) | frac, (exp << (p-1) | frac) | 1 << (width-1)))
        bits_set.update(rng.getrandbits(width) for _ in range(384))
        bits_set.update(0x3f800000+i if width == 32 else 0x3ff0000000000000+i for i in (-2,-1,0,1,2))
        # Exact decimal midpoint ties, odd/even decimal neighbors, both signs.
        tie_base = 0x4a000000 if width == 32 else 0x4300000000000000
        for offset in range(1, 257):
            bits_set.update((tie_base+offset, (tie_base+offset) | 1 << (width-1)))
        for value in (F(1,10),F(5,4),F(10**20),F(1,10**7)):
            bits_set.add(encode(width,value))
        for bits in sorted(bits_set):
            rows.append(f'{width}\t{bits:x}\t{shortest(width, bits)}')
        # Direct decimal midpoint ties and decimal perturbations, including values
        # too close to a binary32 midpoint for a binary64 intermediate to retain.
        for bits in ((1 << (p-1)), (bias << (p-1)), (bias << (p-1))+1, infinity-2):
            midpoint = (decode(width, bits)+decode(width, bits+1))/2
            for delta in (F(0), power(10, -400), -power(10, -400)):
                value = midpoint+delta
                literals.append(f'{width}\t{decimal_exact(value) if delta == 0 else exact_fraction_decimal(value)}\t{encode(width, value):x}')
        finite = [b for b in sorted(bits_set) if decode(width, b) not in (None, 0)]
        for _ in range(200):
            a, b = rng.choice(finite), rng.choice(finite)
            x, y = decode(width, a), decode(width, b)
            for op, value in (('+',x+y),('-',x-y),('*',x*y),('/',x/y)):
                if value:  # Signed exact-zero behavior has separate hand tests.
                    operations.append(f'{width}\t{a:x}\t{op}\t{b:x}\t{encode(width, value):x}')
    out = root/'tools/tests/fixtures'
    out.mkdir(parents=True, exist_ok=True)
    for name, data in [('float-format.tsv',rows),('float-literals.tsv',literals),('float-operations.tsv',operations)]:
        (out/name).write_text('\n'.join(data)+'\n', encoding='ascii', newline='\n')
        print(name, len(data))
    program, expected = [], []
    names = {'+':'add','-':'sub','*':'mul','/':'div'}
    for width, ty in ((32,'float'),(64,'double')):
        for op, name in names.items():
            program.append(f'func {name}{width}(a:{ty},b:{ty})->{ty}{{return a{op}b}}')
    program.append('func main(){')
    for i, row in enumerate(operations):
        if i % 7:
            continue
        width, a, op, b, result = row.split('\t')
        width = int(width)
        ty = 'float' if width == 32 else 'double'
        a, b = shortest(width,int(a,16)), shortest(width,int(b,16))
        a += '.0' if '.' not in a else ''
        b += '.0' if '.' not in b else ''
        program.append(f'const C{i}:{ty}={a}{op}{b};let R{i}={names[op]}{width}({a},{b});print("{{C{i}}} {{R{i}}}");')
        value = shortest(width,int(result,16))
        expected.append(value+' '+value)
    program.append('}')
    (out/'float-native.nova').write_text('\n'.join(program)+'\n',encoding='ascii',newline='\n')
    (out/'float-native.stdout.txt').write_text('\n'.join(expected)+'\n',encoding='ascii',newline='\n')
    print('float-native',len(expected),'operation pairs')


def exact_fraction_decimal(x):
    # denominator consists of factors 2 and 5 after rational addition.
    sign = '-' if x < 0 else ''
    x = abs(x)
    d, twos, fives = x.denominator, 0, 0
    while d % 2 == 0:
        d //= 2; twos += 1
    while d % 5 == 0:
        d //= 5; fives += 1
    assert d == 1
    places = max(twos, fives)
    text = str(x.numerator*2**(places-twos)*5**(places-fives)).rjust(places+1,'0')
    return sign+(text[:-places]+'.'+text[-places:] if places else text+'.0')


if __name__ == '__main__':
    generate()
