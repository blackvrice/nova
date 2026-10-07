use nova_codegen::{CodegenError, CodegenUnit, TargetSpec};
use nova_mir::*;
use nova_syntax::Symbol;
use nova_types::Type;
use std::collections::BTreeMap;
use std::fmt::Write;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IrArtifact {
    pub text: String,
    pub sources: Vec<SourceInfo>,
}
pub fn emit_ir(
    unit: &CodegenUnit,
    target: TargetSpec,
    executable: bool,
) -> Result<IrArtifact, CodegenError> {
    if !matches!(target, TargetSpec::WindowsX64Msvc | TargetSpec::LinuxX64Gnu) {
        return Err(CodegenError::UnsupportedTarget(target));
    }
    let mir = unit.mir();
    let entry = if executable {
        Some(unit.executable_entry()?.0)
    } else {
        None
    };
    let mut emitter=Emitter {mir,output:format!("; Nova Stage A / P03 / LLVM 21.1.8\nsource_filename = \"nova-stage-a\"\ntarget triple = \"{}\"\n%String = type {{ ptr, i64 }}\n",target.triple()),strings:BTreeMap::new(),sequence:0};
    for (id, shape) in &mir.structs {
        let fields = shape
            .fields
            .iter()
            .map(|f| ty(f.ty))
            .collect::<Vec<_>>()
            .join(", ");
        let _ = writeln!(
            emitter.output,
            "%nova_struct_{} = type {{ {fields} }}",
            id.0
        );
    }
    let expanded = mir.callees.iter().flat_map(|c| c.parameters.iter().copied().chain([c.return_type]))
        .chain(mir.bodies.iter().flat_map(|b| b.locals.iter().map(|l| l.ty)))
        .any(|ty| ty != Type::Int32 && ty.integer().is_some())
        || mir.bodies.iter().flat_map(|b| &b.blocks).flat_map(|b| &b.statements).any(|s| {
            matches!(&s.kind, StatementKind::Assign(_, Rvalue::Interpolate(parts)) if parts.iter().any(|p| matches!(p, Operand::Constant(Constant::Integer(_)))))
        });
    emitter.output.push_str("declare void @nova_panic(i32, i32, i32, i32) noreturn\ndeclare void @nova_print(ptr, i64, i32, i32, i32)\ndeclare void @nova_format_int(ptr, i32, i32, i32, i32)\n");
    if expanded {
        emitter.output.push_str("declare void @nova_format_i64(ptr, i64, i32, i32, i32)\ndeclare void @nova_format_u64(ptr, i64, i32, i32, i32)\n");
    }
    let char_used = mir.callees.iter().flat_map(|c| c.parameters.iter().copied().chain([c.return_type]))
        .chain(mir.bodies.iter().flat_map(|b| b.locals.iter().map(|l| l.ty)))
        .any(|ty| ty == Type::Char)
        || mir.bodies.iter().flat_map(|b| &b.blocks).flat_map(|b| &b.statements).any(|s| {
            matches!(&s.kind, StatementKind::Assign(_, Rvalue::Interpolate(parts)) if parts.iter().any(|p| matches!(p, Operand::Constant(Constant::Char(_)))))
        });
    if char_used {
        emitter
            .output
            .push_str("declare void @nova_format_char(ptr, i32, i32, i32, i32)\n");
    }
    emitter.output.push_str("declare void @nova_format_bool(ptr, i32)\ndeclare void @nova_concat(ptr, ptr, i64, i32, i32, i32)\n");
    let float_used = mir.callees.iter().flat_map(|c| c.parameters.iter().copied().chain([c.return_type]))
        .chain(mir.bodies.iter().flat_map(|b| b.locals.iter().map(|l| l.ty))).any(|ty| ty.float().is_some())
        || mir.bodies.iter().flat_map(|b| &b.blocks).flat_map(|b| &b.statements).any(|s| {
            matches!(&s.kind, StatementKind::Assign(_, Rvalue::Interpolate(parts)) if parts.iter().any(|p| matches!(p, Operand::Constant(Constant::Float(_)))))
        });
    if float_used {
        emitter.output.push_str("declare void @nova_format_f32(ptr, i32, i32, i32, i32)\ndeclare void @nova_format_f64(ptr, i64, i32, i32, i32)\n");
    }
    if mir
        .bodies
        .iter()
        .flat_map(|b| &b.blocks)
        .flat_map(|b| &b.statements)
        .any(|s| matches!(&s.kind, StatementKind::Assign(_, Rvalue::CheckedCast(_, _))))
    {
        emitter.output.push_str(
            "declare float @llvm.trunc.f32(float)\ndeclare double @llvm.trunc.f64(double)\n",
        );
    }
    for width in [8, 16, 32, 64] {
        for op in ["sadd", "ssub", "smul", "uadd", "usub", "umul"] {
            if !expanded && (width != 32 || op.starts_with('u')) {
                continue;
            }
            let _ = writeln!(
                emitter.output,
                "declare {{ i{width}, i1 }} @llvm.{op}.with.overflow.i{width}(i{width}, i{width})"
            );
        }
    }
    for body in &mir.bodies {
        emitter.body(body);
    }
    if let Some(entry) = entry {
        let _=writeln!(emitter.output,"define void @nova_stage_a_entry() {{\nentry:\n  call fastcc void @nova_fn_{entry}()\n  ret void\n}}");
    }
    let mut ordered = emitter
        .strings
        .iter()
        .map(|(text, &id)| (id, text))
        .collect::<Vec<_>>();
    ordered.sort_by_key(|&(id, _)| id);
    for (id, text) in ordered {
        let escaped = text
            .as_bytes()
            .iter()
            .map(|byte| format!("\\{byte:02X}"))
            .collect::<String>();
        let _ = writeln!(
            emitter.output,
            "@nova_str_{id} = private unnamed_addr constant [{} x i8] c\"{escaped}\", align 1",
            text.len()
        );
    }
    Ok(IrArtifact {
        text: emitter.output,
        sources: mir.sources.clone(),
    })
}
fn ty(ty: Type) -> String {
    match ty {
        Type::Struct(id) | Type::Tuple(id) => format!("%nova_struct_{}", id.0),
        Type::Int8 | Type::UInt8 => "i8".into(),
        Type::Int16 | Type::UInt16 => "i16".into(),
        Type::Int32 | Type::UInt32 | Type::Char => "i32".into(),
        Type::Int64 | Type::UInt64 => "i64".into(),
        Type::Float32 => "float".into(),
        Type::Float64 => "double".into(),
        Type::Bool => "i1".into(),
        Type::String => "%String".into(),
        Type::Unit => "{}".into(),
        _ => unreachable!("verified value type"),
    }
}
fn return_ty(t: Type) -> String {
    if matches!(t, Type::Unit) || t.aggregate().is_some() {
        "void".into()
    } else {
        ty(t)
    }
}
struct Emitter<'a> {
    mir: &'a Module,
    output: String,
    strings: BTreeMap<String, usize>,
    sequence: usize,
}
impl Emitter<'_> {
    fn next(&mut self) -> String {
        let id = self.sequence;
        self.sequence += 1;
        format!("v{id}")
    }
    fn instruction(&mut self, text: impl AsRef<str>) -> String {
        let id = self.next();
        let _ = writeln!(self.output, "  %{id} = {}", text.as_ref());
        format!("%{id}")
    }
    fn line(&mut self, text: impl AsRef<str>) {
        let _ = writeln!(self.output, "  {}", text.as_ref());
    }
    fn source(&mut self, source: SourceInfo) {
        let _ = writeln!(
            self.output,
            "  ; hir {} file {} bytes {}..{} origin {:?}",
            source.hir.0,
            source.span.file().as_u32(),
            source.span.start(),
            source.span.end(),
            source.origin
        );
    }
    fn location(source: SourceInfo) -> String {
        format!(
            "i32 {}, i32 {}, i32 {}",
            source.span.file().as_u32(),
            source.span.start(),
            source.span.end()
        )
    }
    fn operand(&mut self, body: &Body, operand: &Operand) -> (Type, String) {
        match operand {
            Operand::Place(Place(local)) => {
                let type_ = body.locals[local.0].ty;
                let value = self.instruction(format!("load {}, ptr %p{}", ty(type_), local.0));
                (type_, value)
            }
            Operand::Constant(Constant::Integer(value)) => {
                (value.kind().ty(), value.value().to_string())
            }
            Operand::Constant(Constant::Int32(value)) => (Type::Int32, value.to_string()),
            Operand::Constant(Constant::Float(value)) => (
                value.kind().ty(),
                format!(
                    "bitcast (i{} {} to {})",
                    value.kind().bits(),
                    value.bits(),
                    ty(value.kind().ty())
                ),
            ),
            Operand::Constant(Constant::Bool(value)) => (Type::Bool, value.to_string()),
            Operand::Constant(Constant::Char(value)) => (Type::Char, (*value as u32).to_string()),
            Operand::Constant(Constant::Struct(id, fields) | Constant::Tuple(id, fields)) => {
                let mut value = String::from("zeroinitializer");
                for (index, field) in fields.iter().enumerate() {
                    let (t, v) = self.operand(body, &Operand::Constant(field.clone()));
                    value = self.instruction(format!(
                        "insertvalue {} {value}, {} {v}, {index}",
                        ty(self.mir.aggregate_type(*id)),
                        ty(t)
                    ));
                }
                (self.mir.aggregate_type(*id), value)
            }
            Operand::Constant(Constant::Unit) => (Type::Unit, "zeroinitializer".into()),
            Operand::Constant(Constant::String(text)) => {
                let next = self.strings.len();
                let id = *self.strings.entry(text.clone()).or_insert(next);
                (
                    Type::String,
                    format!("{{ ptr @nova_str_{id}, i64 {} }}", text.len()),
                )
            }
        }
    }
    fn guard(&mut self, condition: &str, reason: i32, source: SourceInfo) {
        let label = self.next();
        self.line(format!(
            "br i1 {condition}, label %panic.{label}, label %ok.{label}"
        ));
        let _ = writeln!(self.output, "panic.{label}:");
        self.line(format!(
            "call void @nova_panic(i32 {reason}, {})",
            Self::location(source)
        ));
        self.line("unreachable");
        let _ = writeln!(self.output, "ok.{label}:");
    }
    fn checked(
        &mut self,
        op: &str,
        type_: Type,
        left: &str,
        right: &str,
        source: SourceInfo,
    ) -> String {
        let kind = type_.integer().expect("verified integer arithmetic");
        let llvm = ty(type_);
        let sign = if kind.signed() { "s" } else { "u" };
        let pair = self.instruction(format!("call {{ {llvm}, i1 }} @llvm.{sign}{op}.with.overflow.{llvm}({llvm} {left}, {llvm} {right})"));
        let value = self.instruction(format!("extractvalue {{ {llvm}, i1 }} {pair}, 0"));
        let overflow = self.instruction(format!("extractvalue {{ {llvm}, i1 }} {pair}, 1"));
        self.guard(&overflow, if type_ == Type::Int32 { 1 } else { 3 }, source);
        value
    }
    fn checked_cast(
        &mut self,
        source: Type,
        dest: Type,
        value: String,
        location: SourceInfo,
    ) -> String {
        if source == dest {
            return value;
        }
        if let (Some(s), Some(d)) = (source.integer(), dest.integer()) {
            // i128 is backend scratch space, never a Nova semantic type.
            let op = if s.signed() { "sext" } else { "zext" };
            let wide = self.instruction(format!("{op} {} {value} to i128", ty(source)));
            let low = self.instruction(format!("icmp slt i128 {wide}, {}", d.min()));
            let high = self.instruction(format!("icmp sgt i128 {wide}, {}", d.max()));
            let invalid = self.instruction(format!("or i1 {low}, {high}"));
            self.guard(&invalid, 4, location);
            return self.instruction(format!("trunc i128 {wide} to {}", ty(dest)));
        }
        if let Some(s) = source.integer() {
            let op = if s.signed() { "sitofp" } else { "uitofp" };
            let result = self.instruction(format!("{op} {} {value} to {}", ty(source), ty(dest)));
            return self.canonical_float(dest, result);
        }
        if let Some(d) = dest.integer() {
            let kind = source.float().expect("numeric cast");
            let width = kind.bits();
            let (fraction, bias) = if width == 32 { (23, 127) } else { (52, 1023) };
            let power = |exponent: u32| ((bias + exponent) as u64) << fraction;
            let upper = power(d.bits() - u32::from(d.signed()));
            let lower = if d.signed() {
                upper | (1u64 << (width - 1))
            } else {
                0
            };
            let truncated = self.instruction(format!(
                "call {} @llvm.trunc.f{width}({} {value})",
                ty(source),
                ty(source)
            ));
            let low = self.instruction(format!(
                "fcmp oge {} {truncated}, bitcast (i{width} {lower} to {})",
                ty(source),
                ty(source)
            ));
            let high = self.instruction(format!(
                "fcmp olt {} {truncated}, bitcast (i{width} {upper} to {})",
                ty(source),
                ty(source)
            ));
            let valid = self.instruction(format!("and i1 {low}, {high}"));
            let invalid = self.instruction(format!("xor i1 {valid}, true"));
            // Ordered comparisons reject NaN. fptoi only exists on the valid edge.
            self.guard(&invalid, 4, location);
            let op = if d.signed() { "fptosi" } else { "fptoui" };
            return self.instruction(format!("{op} {} {truncated} to {}", ty(source), ty(dest)));
        }
        let op = if dest == Type::Float64 {
            "fpext"
        } else {
            "fptrunc"
        };
        let result = self.instruction(format!("{op} {} {value} to {}", ty(source), ty(dest)));
        if dest == Type::Float32 {
            let bits = self.instruction(format!("bitcast double {value} to i64"));
            let exponent = self.instruction(format!("and i64 {bits}, 9218868437227405312"));
            let finite = self.instruction(format!("icmp ne i64 {exponent}, 9218868437227405312"));
            let bits = self.instruction(format!("bitcast float {result} to i32"));
            let absolute = self.instruction(format!("and i32 {bits}, 2147483647"));
            let infinite = self.instruction(format!("icmp eq i32 {absolute}, 2139095040"));
            let invalid = self.instruction(format!("and i1 {finite}, {infinite}"));
            self.guard(&invalid, 4, location);
        }
        self.canonical_float(dest, result)
    }
    fn canonical_float(&mut self, type_: Type, value: String) -> String {
        let kind = type_.float().expect("float operation");
        let nan = self.instruction(format!("fcmp uno {} {value}, {value}", ty(type_)));
        self.instruction(format!(
            "select i1 {nan}, {} bitcast (i{} {} to {}), {} {value}",
            ty(type_),
            kind.bits(),
            kind.nan_bits(),
            ty(type_),
            ty(type_)
        ))
    }
    fn float_binary(&mut self, op: Symbol, type_: Type, left: &str, right: &str) -> String {
        let arithmetic = match op {
            Symbol::Plus => Some("fadd"),
            Symbol::Minus => Some("fsub"),
            Symbol::Star => Some("fmul"),
            Symbol::Slash => Some("fdiv"),
            _ => None,
        };
        if let Some(op) = arithmetic {
            let value = self.instruction(format!("{op} {} {left}, {right}", ty(type_)));
            self.canonical_float(type_, value)
        } else {
            let predicate = match op {
                Symbol::EqualEqual => "oeq",
                Symbol::BangEqual => "une",
                Symbol::Less => "olt",
                Symbol::LessEqual => "ole",
                Symbol::Greater => "ogt",
                Symbol::GreaterEqual => "oge",
                _ => unreachable!("verified float operator"),
            };
            self.instruction(format!("fcmp {predicate} {} {left}, {right}", ty(type_)))
        }
    }
    fn body(&mut self, body: &Body) {
        let signature = &self.mir.callees[body.callee.0];
        let parameters = signature
            .parameters
            .iter()
            .enumerate()
            .map(|(id, &type_)| {
                if type_.aggregate().is_some() {
                    format!("ptr readonly %arg{id}")
                } else {
                    format!("{} %arg{id}", ty(type_))
                }
            })
            .collect::<Vec<_>>()
            .join(", ");
        let parameters = if signature.return_type.aggregate().is_some() {
            if parameters.is_empty() {
                "ptr %out".into()
            } else {
                format!("ptr %out, {parameters}")
            }
        } else {
            parameters
        };
        let _ = writeln!(
            self.output,
            "define internal fastcc {} @nova_fn_{}({parameters}) {{\nentry:",
            return_ty(signature.return_type),
            body.callee.0
        );
        // All allocas are hoisted so a future cyclic CFG does not grow the stack.
        for (id, local) in body.locals.iter().enumerate() {
            self.line(format!("%p{id} = alloca {}", ty(local.ty)));
        }
        for (bb, block) in body.blocks.iter().enumerate() {
            if let Some(Terminator {
                kind:
                    TerminatorKind::Call {
                        callee, arguments, ..
                    },
                ..
            }) = &block.terminator
            {
                for (index, _) in arguments.iter().enumerate() {
                    let t = self.mir.callees[callee.0].parameters[index];
                    if t.aggregate().is_some() {
                        self.line(format!("%call.{bb}.{index} = alloca {}", ty(t)));
                    }
                }
            }
        }
        for (bb, block) in body.blocks.iter().enumerate() {
            for (s, statement) in block.statements.iter().enumerate() {
                if let StatementKind::Assign(_, Rvalue::Interpolate(parts)) = &statement.kind {
                    self.line(format!(
                        "%parts.{bb}.{s} = alloca [{} x %String]",
                        parts.len()
                    ));
                    for part in 0..parts.len() {
                        self.line(format!("%part.{bb}.{s}.{part} = getelementptr [{} x %String], ptr %parts.{bb}.{s}, i32 0, i32 {part}",parts.len()));
                    }
                }
            }
        }
        for (id, parameter) in body.parameters.iter().enumerate() {
            if signature.parameters[id].aggregate().is_some() {
                let t = ty(signature.parameters[id]);
                let v = self.instruction(format!("load {t}, ptr %arg{id}"));
                self.line(format!("store {t} {v}, ptr %p{}", parameter.0));
                continue;
            }
            self.line(format!(
                "store {} %arg{id}, ptr %p{}",
                ty(signature.parameters[id]),
                parameter.0
            ));
        }
        self.line(format!("br label %bb{}", body.entry.0));
        for (bb, block) in body.blocks.iter().enumerate() {
            let _ = writeln!(self.output, "bb{bb}:");
            for (s, statement) in block.statements.iter().enumerate() {
                self.source(statement.source);
                let StatementKind::Assign(Place(destination), rvalue) = &statement.kind;
                let result = match rvalue {
                    Rvalue::Aggregate(id, fields) => {
                        let mut v = String::from("zeroinitializer");
                        for (index, field) in fields.iter().enumerate() {
                            let (t, f) = self.operand(body, field);
                            v = self.instruction(format!(
                                "insertvalue {} {v}, {} {f}, {index}",
                                ty(self.mir.aggregate_type(*id)),
                                ty(t)
                            ));
                        }
                        Some((self.mir.aggregate_type(*id), v))
                    }
                    Rvalue::Project(receiver, field) => {
                        let (t, v) = self.operand(body, receiver);
                        let v = self.instruction(format!(
                            "extractvalue {} {v}, {}",
                            ty(t),
                            field.index
                        ));
                        Some((self.mir.structs[&field.structure].fields[field.index].ty, v))
                    }
                    Rvalue::Update(receiver, path, value) => {
                        let (t, root) = self.operand(body, receiver);
                        let (ft, v) = self.operand(body, value);
                        let indices = path
                            .iter()
                            .map(|f| f.index.to_string())
                            .collect::<Vec<_>>()
                            .join(", ");
                        let result = self.instruction(format!(
                            "insertvalue {} {root}, {} {v}, {indices}",
                            ty(t),
                            ty(ft)
                        ));
                        Some((t, result))
                    }
                    Rvalue::CheckedCast(value, dest) => {
                        let (source, value) = self.operand(body, value);
                        Some((
                            *dest,
                            self.checked_cast(source, *dest, value, statement.source),
                        ))
                    }
                    Rvalue::NumericConvert(value, dest) => {
                        let (source, value) = self.operand(body, value);
                        let result = if source == *dest {
                            value
                        } else {
                            let op = if source.float().is_some() {
                                "fpext"
                            } else if source.integer().expect("numeric conversion").signed() {
                                "sitofp"
                            } else {
                                "uitofp"
                            };
                            let value = self.instruction(format!(
                                "{op} {} {value} to {}",
                                ty(source),
                                ty(*dest)
                            ));
                            self.canonical_float(*dest, value)
                        };
                        Some((*dest, result))
                    }
                    Rvalue::Widen(value, dest) => {
                        let (source, value) = self.operand(body, value);
                        let result = if source == *dest {
                            value
                        } else {
                            let op = if source.integer().expect("verified widening").signed() {
                                "sext"
                            } else {
                                "zext"
                            };
                            self.instruction(format!(
                                "{op} {} {value} to {}",
                                ty(source),
                                ty(*dest)
                            ))
                        };
                        Some((*dest, result))
                    }
                    Rvalue::Use(value) => Some(self.operand(body, value)),
                    Rvalue::Unary(op, value) => {
                        let (type_, value) = self.operand(body, value);
                        let result = match op {
                            Symbol::Plus => value,
                            Symbol::Minus if type_.float().is_some() => {
                                let value = self.instruction(format!("fneg {} {value}", ty(type_)));
                                self.canonical_float(type_, value)
                            }
                            Symbol::Minus => {
                                self.checked("sub", type_, "0", &value, statement.source)
                            }
                            Symbol::Bang => self.instruction(format!("xor i1 {value}, true")),
                            _ => unreachable!(),
                        };
                        Some((type_, result))
                    }
                    Rvalue::Binary(op, left, right) => {
                        let (type_, left) = self.operand(body, left);
                        let (_, right) = self.operand(body, right);
                        let result = if type_.float().is_some() {
                            self.float_binary(*op, type_, &left, &right)
                        } else {
                            match op {
                                Symbol::Plus => {
                                    self.checked("add", type_, &left, &right, statement.source)
                                }
                                Symbol::Minus => {
                                    self.checked("sub", type_, &left, &right, statement.source)
                                }
                                Symbol::Star => {
                                    self.checked("mul", type_, &left, &right, statement.source)
                                }
                                Symbol::Slash | Symbol::Percent => {
                                    let kind = type_.integer().expect("verified division");
                                    let llvm = ty(type_);
                                    let zero =
                                        self.instruction(format!("icmp eq {llvm} {right}, 0"));
                                    self.guard(&zero, 2, statement.source);
                                    if kind.signed() {
                                        let min = self.instruction(format!(
                                            "icmp eq {llvm} {left}, {}",
                                            kind.min()
                                        ));
                                        let minus =
                                            self.instruction(format!("icmp eq {llvm} {right}, -1"));
                                        let overflow =
                                            self.instruction(format!("and i1 {min}, {minus}"));
                                        self.guard(
                                            &overflow,
                                            if type_ == Type::Int32 { 1 } else { 3 },
                                            statement.source,
                                        );
                                    }
                                    let sign = if kind.signed() { "s" } else { "u" };
                                    let op = if *op == Symbol::Slash { "div" } else { "rem" };
                                    self.instruction(format!("{sign}{op} {llvm} {left}, {right}"))
                                }
                                _ => {
                                    let unsigned = type_ == Type::Char
                                        || type_.integer().is_some_and(|kind| !kind.signed());
                                    let predicate = match op {
                                        Symbol::EqualEqual => "eq",
                                        Symbol::BangEqual => "ne",
                                        Symbol::Less => {
                                            if unsigned {
                                                "ult"
                                            } else {
                                                "slt"
                                            }
                                        }
                                        Symbol::LessEqual => {
                                            if unsigned {
                                                "ule"
                                            } else {
                                                "sle"
                                            }
                                        }
                                        Symbol::Greater => {
                                            if unsigned {
                                                "ugt"
                                            } else {
                                                "sgt"
                                            }
                                        }
                                        Symbol::GreaterEqual => {
                                            if unsigned {
                                                "uge"
                                            } else {
                                                "sge"
                                            }
                                        }
                                        _ => unreachable!(),
                                    };
                                    self.instruction(format!(
                                        "icmp {predicate} {} {left}, {right}",
                                        ty(type_)
                                    ))
                                }
                            }
                        };
                        Some((body.locals[destination.0].ty, result))
                    }
                    Rvalue::Interpolate(parts) => {
                        for (part, value) in parts.iter().enumerate() {
                            let (type_, value) = self.operand(body, value);
                            let pointer = format!("%part.{bb}.{s}.{part}");
                            match type_ {
                                type_ if type_.float().is_some() => {
                                    let width = type_.float().expect("float formatting").bits();
                                    let bits = self.instruction(format!(
                                        "bitcast {} {value} to i{width}",
                                        ty(type_)
                                    ));
                                    self.line(format!("call void @nova_format_f{width}(ptr {pointer}, i{width} {bits}, {})", Self::location(statement.source)));
                                }
                                Type::String => {
                                    self.line(format!("store %String {value}, ptr {pointer}"))
                                }
                                Type::Int32 => self.line(format!(
                                    "call void @nova_format_int(ptr {pointer}, i32 {value}, {})",
                                    Self::location(statement.source)
                                )),
                                Type::Char => self.line(format!(
                                    "call void @nova_format_char(ptr {pointer}, i32 {value}, {})",
                                    Self::location(statement.source)
                                )),
                                type_ if type_.integer().is_some() => {
                                    let kind = type_.integer().expect("integer formatting");
                                    let signed = kind.signed();
                                    let value = if kind.bits() < 64 {
                                        let op = if signed { "sext" } else { "zext" };
                                        self.instruction(format!(
                                            "{op} {} {value} to i64",
                                            ty(type_)
                                        ))
                                    } else {
                                        value
                                    };
                                    let name = if signed { "i64" } else { "u64" };
                                    self.line(format!("call void @nova_format_{name}(ptr {pointer}, i64 {value}, {})", Self::location(statement.source)));
                                }
                                Type::Bool => {
                                    let extended =
                                        self.instruction(format!("zext i1 {value} to i32"));
                                    self.line(format!("call void @nova_format_bool(ptr {pointer}, i32 {extended})"));
                                }
                                _ => unreachable!(),
                            }
                        }
                        self.line(format!(
                            "call void @nova_concat(ptr %p{}, ptr %parts.{bb}.{s}, i64 {}, {})",
                            destination.0,
                            parts.len(),
                            Self::location(statement.source)
                        ));
                        None
                    }
                };
                if let Some((type_, value)) = result {
                    self.line(format!(
                        "store {} {value}, ptr %p{}",
                        ty(type_),
                        destination.0
                    ));
                }
            }
            let terminator = block.terminator.as_ref().expect("verified terminator");
            self.source(terminator.source);
            match &terminator.kind {
                TerminatorKind::Goto(target) => self.line(format!("br label %bb{}", target.0)),
                TerminatorKind::Branch {
                    condition,
                    then_block,
                    else_block,
                } => {
                    let (_, condition) = self.operand(body, condition);
                    self.line(format!(
                        "br i1 {condition}, label %bb{}, label %bb{}",
                        then_block.0, else_block.0
                    ));
                }
                TerminatorKind::Return(value) => {
                    if signature.return_type.aggregate().is_some() {
                        let (t, v) = self.operand(body, value);
                        self.line(format!("store {} {v}, ptr %out", ty(t)));
                        self.line("ret void");
                    } else if signature.return_type == Type::Unit {
                        self.line("ret void");
                    } else {
                        let (type_, value) = self.operand(body, value);
                        self.line(format!("ret {} {value}", ty(type_)));
                    }
                }
                TerminatorKind::Unreachable => self.line("unreachable"),
                TerminatorKind::Call {
                    callee,
                    arguments,
                    destination: Place(destination),
                    target,
                } => {
                    let callee_ = &self.mir.callees[callee.0];
                    let args = arguments
                        .iter()
                        .map(|arg| self.operand(body, arg))
                        .collect::<Vec<_>>();
                    if callee_.builtin_print {
                        let string = &args[0].1;
                        let ptr = self.instruction(format!("extractvalue %String {string}, 0"));
                        let len = self.instruction(format!("extractvalue %String {string}, 1"));
                        self.line(format!(
                            "call void @nova_print(ptr {ptr}, i64 {len}, {})",
                            Self::location(terminator.source)
                        ));
                        self.line(format!(
                            "store {{}} zeroinitializer, ptr %p{}",
                            destination.0
                        ));
                    } else {
                        let mut arguments = vec![];
                        if callee_.return_type.aggregate().is_some() {
                            arguments.push(format!("ptr %p{}", destination.0));
                        }
                        for (index, (type_, value)) in args.iter().enumerate() {
                            if type_.aggregate().is_some() {
                                self.line(format!(
                                    "store {} {value}, ptr %call.{bb}.{index}",
                                    ty(*type_)
                                ));
                                arguments.push(format!("ptr %call.{bb}.{index}"));
                            } else {
                                arguments.push(format!("{} {value}", ty(*type_)));
                            }
                        }
                        let arguments = arguments.join(", ");
                        let call = format!(
                            "call fastcc {} @nova_fn_{}({arguments})",
                            return_ty(callee_.return_type),
                            callee.0
                        );
                        if callee_.return_type.aggregate().is_some() {
                            self.line(call);
                        } else if callee_.return_type == Type::Unit {
                            self.line(call);
                            self.line(format!(
                                "store {{}} zeroinitializer, ptr %p{}",
                                destination.0
                            ));
                        } else {
                            let result = self.instruction(call);
                            self.line(format!(
                                "store {} {result}, ptr %p{}",
                                ty(callee_.return_type),
                                destination.0
                            ));
                        }
                    }
                    self.line(format!("br label %bb{}", target.0));
                }
            }
        }
        self.output.push_str("}\n");
    }
}
