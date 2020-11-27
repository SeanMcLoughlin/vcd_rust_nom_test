use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VCDParserError {
    #[error("{}", source)]
    FileOpenError {
        #[from]
        source: io::Error,
    },
}
