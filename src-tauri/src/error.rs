// Source: https://github.com/awesomeapp-dev/rust-desktop-app/blob/main/src-tauri/src/error.rs

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    GameDoesNotExist(String),
    VersionDoesNotExist(String),
    ProfileFailure(String),
    VersionFailure(String),

    JsonSerde(serde_json::Error),

    Reqwest(reqwest::Error),

    IO(std::io::Error),

    XMLError(xmltree::Error),

    WindowsCore(windows::core::HSTRING),
}

// region:    --- Froms
impl From<xmltree::Error> for Error {
    fn from(val: xmltree::Error) -> Self {
        Error::XMLError(val)
    }
}
impl From<serde_json::Error> for Error {
    fn from(val: serde_json::Error) -> Self {
        Error::JsonSerde(val)
    }
}
impl From<reqwest::Error> for Error {
    fn from(val: reqwest::Error) -> Self {
        Error::Reqwest(val)
    }
}
impl From<std::io::Error> for Error {
    fn from(val: std::io::Error) -> Self {
        Error::IO(val)
    }
}
impl From<windows::core::Error> for Error {
    fn from(val: windows::core::Error) -> Self {
        Error::WindowsCore(val.message())
    }
}
// endregion: --- Froms

// region:    --- Error Boiler
impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boiler
