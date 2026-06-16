#[derive(Debug, Clone)]
pub struct TransformDescriptor {
    pub name: String,
    pub version: String,
    pub action: DescriptorAction,
    pub access: Vec<String>,
    pub rt_hooks: Vec<String>,
    pub hash: [u8; 32],
}

#[derive(Debug, Clone)]
pub enum DescriptorAction {
    Validate,
    Appending,
}
