use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum MyError {
    #[error("InvalidChar")]
    InvalidChar,
    #[error("InvalidUTF8Char")]
    InvalidUTF8Char,
    #[error("InvalidCscValue")]
    InvalidCscValue,
    #[error("InvalidPngHearder")]
    InvalidPngHearder,
    #[error("InvalidFilePath")]
    InvalidFilePath,
    #[error("InvalidFilePathIsDir")]
    InvalidFilePathIsDir,
    #[error("NotFoundChunkType")]
    NotFoundChunkType,
}
