use crate::domain::repositories::error::RepositoryError;

#[derive(Debug)]
pub enum ServiceError {
    NotFound,
    Unknown,
    BadInput,
}

pub type ServiceResult<T> = Result<T, ServiceError>;

impl From<RepositoryError> for ServiceError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::NotFound => ServiceError::NotFound,
            RepositoryError::Unknown => ServiceError::Unknown,
            RepositoryError::InvalidUuid => ServiceError::BadInput,
        }
    }
}
