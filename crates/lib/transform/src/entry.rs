use crate::context::TransformContext;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct TranformEntry {
    pub entry: extern "C" fn(ctx: *mut TransformContext) -> i32,
}
