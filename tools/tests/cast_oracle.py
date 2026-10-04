"""P10 exact rational cast oracle; no host floating evaluation or compiler output.
Regenerate: python tools/tests/cast_oracle.py
"""
import sys
sys.dont_write_bytecode = True
from float_oracle import F, decode, encode, layout, shortest, decimal_exact
from pathlib import Path
import random

TYPES = [("int"+str(w),w,True) for w in (8,16,32,64)] + [("uint"+str(w),w,False) for w in (8,16,32,64)]
KINDS = [x[0] for x in TYPES]+["float32","float64"]
def limits(t):
    _,w,s = next(x for x in TYPES if x[0]==t)
    return (-(1<<(w-1)), (1<<(w-1))-1) if s else (0,(1<<w)-1)
def oracle(s,d,value):
    sf,df = s.startswith("float"),d.startswith("float")
    sw,dw = int(s[5:]) if sf else 0,int(d[5:]) if df else 0
    x = decode(sw,value) if sf else F(value)
    if not df:
        if x is None: return "error"
        n = abs(x.numerator)//x.denominator
        if x<0: n=-n
        lo,hi=limits(d)
        return str(n) if lo<=n<=hi else "error"
    if x is None:
        p,e,_=layout(sw)
        dp,de,_=layout(dw)
        nan = value & ((1<<(p-1))-1)
        bits = (((1<<de)-1)<<(dp-1)) | ((1<<(dp-2)) if nan else (value>>(sw-1))<<(dw-1))
    else:
        bits=encode(dw,x)
        if not x and sf: bits=(value>>(sw-1))<<(dw-1)
        if decode(dw,bits) is None: return "error"
    return format(bits,"x")

def generate():
    rng=random.Random(0x1010)
    rows=[]
    for s in KINDS:
        if not s.startswith("float"):
            lo,hi=limits(s)
            values={lo,hi,0,1,lo+1,hi-1}
            for d in TYPES:
                a,b=limits(d[0])
                values.update(v for bound in (a,b) for v in (bound-1,bound,bound+1) if lo<=v<=hi)
            # Adjacent integer binary32 midpoints where f64 intermediates double round.
            values.update(v for v in (16777217,16777219,(1<<62)+(1<<38)+1,(1<<63)+(1<<39)+1) if lo<=v<=hi)
            values.update(rng.randint(lo,hi) for _ in range(20))
        else:
            w=int(s[5:]);p,e,bias=layout(w);inf=((1<<e)-1)<<(p-1)
            values={0,1,(1<<(p-1))-1,1<<(p-1),inf-1,inf,inf+1,inf|(1<<(p-2))}
            for d,_,_ in TYPES:
                a,b=limits(d)
                for bound in (a,b):
                    for offset in (F(-1),F(-9,10),F(0),F(9,10),F(1)):
                        bits=encode(w,F(bound)+offset)
                        values.update(v for v in (bits-1,bits,bits+1) if 0<=v<1<<w)
            for x in (F(-9,10),F(9,10),F(1,2),F(3,2)):
                values.add(encode(w,x))
            if w==64:
                # Half subnormal and overflow thresholds, their immediate binary64 neighbors.
                for x in (F(1,1<<150),F(3,1<<150),F((1<<25)-1)*(1<<103)):
                    b=encode(w,x);values.update((b-1,b,b+1))
            values.update(v|(1<<(w-1)) for v in list(values))
            values.update(rng.getrandbits(w) for _ in range(60))
        for d in KINDS:
            for v in sorted(values):
                rows.append((s,d,format(v,"x") if s.startswith("float") else str(v),oracle(s,d,v)))
    out=Path(__file__).resolve().parents[2]/"tools/tests/fixtures"
    (out/"casts.tsv").write_text("".join("\t".join(r)+"\n" for r in rows),encoding="ascii",newline="\n")
    # Real parameters preserve source width and allow every cast pair to reach backend.
    program=[f"func c{i}(v:{s})->{d}{{return v as {d}}}" for i,(s,d) in enumerate((s,d) for s in KINDS for d in KINDS)]
    program.append("func main(){")
    expected=[]
    for index,(s,d,v,r) in enumerate(rows):
        if r=="error": continue
        # Full integer->float coverage; boundary samples for other pairs.
        if s.startswith("float") and index%19: continue
        if not s.startswith("float") and not d.startswith("float") and index%11: continue
        if s.startswith("float"):
            w=int(s[5:]);bits=int(v,16);x=decode(w,bits)
            if x is None:
                p,_,_=layout(w)
                literal="(0.0/0.0)" if bits&((1<<(p-1))-1) else ("(-1.0/0.0)" if bits>>(w-1) else "(1.0/0.0)")
            else:
                literal=shortest(w,bits)
                if "." not in literal: literal+=".0"
        else: literal=v
        display=shortest(int(d[5:]),int(r,16)) if d.startswith("float") else r
        f=KINDS.index(s)*10+KINDS.index(d)
        program.append(f"const S{index}:{s}={literal};const C{index}=S{index} as {d};let R{index}=c{f}(S{index});print(\"{{C{index}}} {{R{index}}}\");")
        expected.append(display+" "+display)
    program.append("}")
    (out/"cast-native.nova").write_text("\n".join(program)+"\n",encoding="ascii",newline="\n")
    (out/"cast-native.stdout.txt").write_text("\n".join(expected)+"\n",encoding="ascii",newline="\n")
    print("casts",len(rows),"native pairs",len(expected))
if __name__=="__main__": generate()
