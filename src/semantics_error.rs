use thiserror::Error;

use crate::TypeName;

#[derive(Error, Debug)]
pub enum SemanticsError {
    #[error("Not found type: {0}")]
    MissingType(TypeName),
}
