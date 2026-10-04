#[allow(dead_code)]
#[path = "../runtime/stage_a.rs"]
mod runtime;
#[no_mangle]
pub extern "C" fn nova_stage_a_entry() {}
