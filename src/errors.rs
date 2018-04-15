#[derive(Debug, Fail)]
pub enum KcatError {
    #[fail(display = "invalid path")]
    InvalidPath,
    #[fail(display = "could not read file")]
    InvalidFile,
}
