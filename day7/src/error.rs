use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Can't parse {input:?} into Card")]
    CardFromStr { input: String },
    #[error("Failed to parse input into Hands")]
    Hands(#[from] nom::Err<nom::error::Error<&'static str>>),
}
