use thiserror::Error;

#[derive(Error, Debug)]
pub enum SplitInHalfError {
    #[error("To split a string in half it must have an even amount of characters")]
    OddNumberOfCharacters,
}

pub trait SplitInHalf {
    fn split_in_half(&self) -> Result<(&str, &str), SplitInHalfError>;
}

impl SplitInHalf for &str {
    fn split_in_half(&self) -> Result<(&str, &str), SplitInHalfError> {
        if self.len() % 2 == 0 {
            return Ok(self.split_at(self.len() / 2));
        }

        Err(SplitInHalfError::OddNumberOfCharacters)
    }
}
