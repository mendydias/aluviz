use std::fmt::Display;

type Result<T> = std::result::Result<T, OutOfBoundsError>;

#[derive(Debug, Clone)]
pub struct OutOfBoundsError;

impl Display for OutOfBoundsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Request memory range is out of bounds for the simulation"
        )
    }
}
