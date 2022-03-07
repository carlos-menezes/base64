use thiserror::Error;

#[derive(Error, Debug)]
pub enum Base64Error {
    #[error("index out of bounds")]
    OutOfBounds,
    #[error("invalid character `{character:?}`")]
    Encode { character: char },
}
