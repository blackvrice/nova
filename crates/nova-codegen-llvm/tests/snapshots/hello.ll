; Nova Stage A / P03 / LLVM 21.1.8
source_filename = "nova-stage-a"
target triple = "x86_64-pc-windows-msvc"
%String = type { ptr, i64 }
declare void @nova_panic(i32, i32, i32, i32) noreturn
declare void @nova_print(ptr, i64, i32, i32, i32)
declare void @nova_format_int(ptr, i32, i32, i32, i32)
declare void @nova_format_bool(ptr, i32)
declare void @nova_concat(ptr, ptr, i64, i32, i32, i32)
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32)
declare { i32, i1 } @llvm.ssub.with.overflow.i32(i32, i32)
declare { i32, i1 } @llvm.smul.with.overflow.i32(i32, i32)
define internal fastcc void @nova_fn_1() {
entry:
  %p0 = alloca {}
  br label %bb0
bb0:
  ; hir 2 file 0 bytes 12..32 origin Source(AstNodeId(2))
  %v0 = extractvalue %String { ptr @nova_str_0, i64 11 }, 0
  %v1 = extractvalue %String { ptr @nova_str_0, i64 11 }, 1
  call void @nova_print(ptr %v0, i64 %v1, i32 0, i32 12, i32 32)
  store {} zeroinitializer, ptr %p0
  br label %bb1
bb1:
  ; hir 6 file 0 bytes 0..33 origin Source(AstNodeId(5))
  ret void
}
define void @nova_stage_a_entry() {
entry:
  call fastcc void @nova_fn_1()
  ret void
}
@nova_str_0 = private unnamed_addr constant [11 x i8] c"\48\65\6C\6C\6F\2C\20\4E\6F\76\61", align 1
