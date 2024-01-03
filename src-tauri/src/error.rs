// Source: https://github.com/awesomeapp-dev/rust-desktop-app/blob/main/src-tauri/src/error.rs

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	GameDoesNotExist(String),

	JsonSerde(serde_json::Error),

	IO(std::io::Error),
}


// region:    --- Froms
impl From<serde_json::Error> for Error {
	fn from(val: serde_json::Error) -> Self {
		Error::JsonSerde(val)
	}
}
impl From<std::io::Error> for Error {
	fn from(val: std::io::Error) -> Self {
		Error::IO(val)
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