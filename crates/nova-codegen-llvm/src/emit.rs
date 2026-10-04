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
    emitter.output.push_str("declare void @nova_format_bool(ptr, i32)\ndeclare void @nova_concat(ptr, ptr, i64, i32, i32, i32)\n");
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
fn ty(ty: Type) -> &'static str {
    match ty {
        Type::Int8 | Type::UInt8 => "i8",
        Type::Int16 | Type::UInt16 => "i16",
        Type::Int32 | Type::UInt32 => "i32",
        Type::Int64 | Type::UInt64 => "i64",
        Type::Bool => "i1",
        Type::String => "%String",
        Type::Unit => "{}",
        _ => unreachable!("verified value type"),
    }
}
fn return_ty(ty_: Type) -> &'static str {
    if ty_ == Type::Unit {
        "void"
    } else {
        ty(ty_)
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
            Operand::Constant(Constant::Bool(value)) => (Type::Bool, value.to_string()),
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
    fn body(&mut self, body: &Body) {
        let signature = &self.mir.callees[body.callee.0];
        let parameters = signature
            .parameters
            .iter()
            .enumerate()
            .map(|(id, &type_)| format!("{} %arg{id}", ty(type_)))
            .collect::<Vec<_>>()
            .join(", ");
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
                        let result = match op {
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
                                let zero = self.instruction(format!("icmp eq {llvm} {right}, 0"));
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
                                let unsigned = type_.integer().is_some_and(|kind| !kind.signed());
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
                        };
                        Some((body.locals[destination.0].ty, result))
                    }
                    Rvalue::Interpolate(parts) => {
                        for (part, value) in parts.iter().enumerate() {
                            let (type_, value) = self.operand(body, value);
                            let pointer = format!("%part.{bb}.{s}.{part}");
                            match type_ {
                                Type::String => {
                                    self.line(format!("store %String {value}, ptr {pointer}"))
                                }
                                Type::Int32 => self.line(format!(
                                    "call void @nova_format_int(ptr {pointer}, i32 {value}, {})",
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
                    if signature.return_type == Type::Unit {
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
                        let arguments = args
                            .iter()
                            .map(|(type_, value)| format!("{} {value}", ty(*type_)))
                            .collect::<Vec<_>>()
                            .join(", ");
                        let call = format!(
                            "call fastcc {} @nova_fn_{}({arguments})",
                            return_ty(callee_.return_type),
                            callee.0
                        );
                        if callee_.return_type == Type::Unit {
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
