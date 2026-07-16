use minicbor::{Decode, Encode};

#[derive(Debug, Clone, Encode, Decode)]
pub struct TransformError {
    #[n(0)]
    pub kind: TransformErrorKind,
    #[n(1)]
    pub severity: TransformErrorSeverity,
    #[n(2)]
    pub message: String,
}

#[derive(Debug, Clone, Encode, Decode)]
pub enum TransformErrorKind {
    #[n(0)]
    Input,
    #[n(1)]
    Output,
    #[n(2)]
    Hardware,
    #[n(3)]
    Runtime,
    #[n(4)]
    Other,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq)]
pub enum TransformErrorSeverity {
    #[n(0)]
    Fatal,
    #[n(1)]
    Error,
    #[n(2)]
    Warning,
    #[n(3)]
    Info,
    #[n(4)]
    Debug,
}
