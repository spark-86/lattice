use transform::registry::TransformRegistry;

pub struct Processor {
    pub transforms: TransformRegistry,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            transforms: TransformRegistry::new(),
        }
    }
}
