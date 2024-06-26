use leptos::ServerFnError;

pub enum Error {
    NoPermission,
    InvalidInput,
    NotLoggedIn,
    Database
}

impl Into<ServerFnError> for Error {
    fn into(self) -> ServerFnError {
        match self {
            Self::NoPermission => ServerFnError::ServerError("No permission to perform this action".into()),
            Self::InvalidInput => ServerFnError::ServerError("Invalid input".into()),
            Self::NotLoggedIn => ServerFnError::ServerError("Not logged in".into()),
            Self::Database => ServerFnError::ServerError("Failed to get database connection".into()),
        }
    }
}

