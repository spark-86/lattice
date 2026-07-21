use rhex::Rhex;

use crate::error::TransformError;

pub type TransformResult<'a> = Result<Option<Vec<Rhex>>, TransformError>;
