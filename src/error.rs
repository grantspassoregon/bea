use thiserror::Error;

#[derive(Error, Debug)]
pub enum BeaError {
    #[error("Parse error.")]
    ParseError,
    #[error("HTTP request error.")]
    HttpError(#[from] reqwest::Error),
    #[error("Deserialization error.")]
    ConversionError(#[from] serde_json::Error),
    #[error("Could not make CSV writer.")]
    CsvError(#[from] csv::Error),
    #[error("Could not serialize to binary.")]
    BinError(#[from] Box<bincode::ErrorKind>),
    // #[error("Deserialize error.")]
    // DeserializeError(#[from] serde::de::value::Error),
    #[error("Value not provided for {value:?}.")]
    UserBuildError { value: Vec<String> },
    #[error("Input/output error from std.")]
    Io(#[from] std::io::Error),
    #[error("Could not read environmental variables from .env.")]
    EnvError(#[from] std::env::VarError),
    #[error("Authorization failed.")]
    AuthError,
    #[error("Bad file name {0:?}.")]
    FileNameError(std::ffi::OsString),
}
