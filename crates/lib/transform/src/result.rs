use rhex::Rhex;

use crate::error::TransformError;

pub type TransformResult = Result<Option<Vec<Rhex>>, TransformError>;
