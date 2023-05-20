#[derive(Debug)]
pub enum RepositoryError {
    NotFound,
    InvalidUuid,
    Unknown,
}

pub type RepositoryResult<T> = Result<T, RepositoryError>;
