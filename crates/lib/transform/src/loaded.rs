use libloading::Library;

use crate::{descriptor::TransformDescriptor, entry::TranformEntry};

pub struct LoadedTransform {
    pub descriptor: TransformDescriptor,
    pub entry: &'static TranformEntry,
    pub library: Library,
}
