use derive_more::Error;

#[derive(Error, Debug, derive_more::Display, derive_more::From)]
pub enum BeaError {
    #[display("Parse error: {:?}", self.source())]
    ParseError,
    #[display("HTTP request error: {:?}", self.source())]
    #[from(reqwest::Error)]
    HttpError,
    #[display("Deserialization error: {:?}", self.source())]
    #[from(serde_json::Error)]
    ConversionError,
    #[display("Could not make CSV writer: {:?}", self.source())]
    #[from(csv::Error)]
    CsvError,
    #[display("Could not serialize to binary: {:?}", self.source())]
    #[from(Box<bincode::ErrorKind>)]
    BinError(#[from] Box<bincode::ErrorKind>),
    #[display("Value not provided for {value:?}.")]
    UserBuildError { value: Vec<String> },
    #[display("Input/output error from std: {:?}", self.source())]
    #[from(std::io::Error)]
    Io,
    #[display("Could not read environmental variables from .env: {:?}", self.source())]
    #[from(std::env::VarError)]
    EnvError,
    #[display("Authorization failed: {:?}", self.source())]
    AuthError,
    #[display("Bad file name {0:?}.", self.source())]
    #[from(std::ffi::OsString)]
    FileNameError,
}
