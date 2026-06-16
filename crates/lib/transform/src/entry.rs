use crate::context::TransformContext;

#[repr(C)]
pub struct TranformEntry {
    pub entry: extern "C" fn(ctx: *mut TransformContext) -> i32,
}
