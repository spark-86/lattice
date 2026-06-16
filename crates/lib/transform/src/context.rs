// This is the entry point context
pub struct TransformContext<'a> {
    pub input: &'a [u8],
    pub output: &'a mut Option<Vec<u8>>,
    pub diag: &'a mut Option<Vec<u8>>,
}
