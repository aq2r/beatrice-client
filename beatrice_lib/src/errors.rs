use std::ffi::NulError;

use crate::bindings::Beatrice_ErrorCode;

#[derive(Debug, thiserror::Error)]
pub enum BeatriceError {
    #[error("ModelNotLoaded")]
    ModelNotLoaded,

    #[error("FileOpenError")]
    FileOpenError,

    #[error("FileTooSmall")]
    FileTooSmall,

    #[error("FileTooLarge")]
    FileTooLarge,

    #[error("InvalidFileSize")]
    InvalidFileSize,

    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("toml deserialize Error: {0}")]
    TomlError(#[from] toml::de::Error),

    #[error("NulError: {0}")]
    NulError(#[from] NulError),
}

impl TryFrom<Beatrice_ErrorCode> for BeatriceError {
    type Error = ();

    fn try_from(value: Beatrice_ErrorCode) -> Result<Self, Self::Error> {
        match value {
            Beatrice_ErrorCode::Beatrice_kSuccess => Err(()),
            Beatrice_ErrorCode::Beatrice_kFileOpenError => Ok(BeatriceError::FileOpenError),
            Beatrice_ErrorCode::Beatrice_kFileTooSmall => Ok(BeatriceError::FileTooSmall),
            Beatrice_ErrorCode::Beatrice_kFileTooLarge => Ok(BeatriceError::FileTooLarge),
            Beatrice_ErrorCode::Beatrice_kInvalidFileSize => Ok(BeatriceError::InvalidFileSize),
        }
    }
}
