use std::io;
use std::error::Error;
use std::fmt;
use rand::distributions::WeightedError;
use format::{
    VarError,
    ParseErr,
};

#[derive(Debug)]
pub enum GroupErr {
    EmptyFile,
    WeightError(WeightedError),
    File(io::Error),
    FormatParse(VarError),
    ItemParse(Vec<(usize, ParseErr)>),
}

impl From<WeightedError> for GroupErr {
    fn from(e: WeightedError) -> Self {
        GroupErr::WeightError(e)
    }
}

impl From<VarError> for GroupErr {
    fn from(e: VarError) -> Self {
        GroupErr::FormatParse(e)
    }
}

impl From<Vec<(usize, ParseErr)>> for GroupErr {
    fn from(e: Vec<(usize, ParseErr)>) -> Self {
        GroupErr::ItemParse(e)
    }
}

impl From<io::Error> for GroupErr {
    fn from(e: io::Error) -> Self {
        GroupErr::File(e)
    }
}

impl fmt::Display for GroupErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GroupErr::EmptyFile => write!(f, "File is empty"),
            GroupErr::WeightError(e) => write!(f, "{}", e),
            GroupErr::File(_) => write!(f, "Failed to open file"),
            GroupErr::FormatParse(_) => write!(f, "Failed to parse format"),
            GroupErr::ItemParse(ref v) => {
                writeln!(f, "Failed to parse the following items")?;
                
                for &(i, ref e) in v {
                    writeln!(f, "{}: {}", i, e)?;
                }

                Ok(())
            }
        }
    }
}

impl Error for GroupErr {
    fn description(&self) -> &str {
        match self {
            GroupErr::EmptyFile => "File is empty",
            GroupErr::WeightError(e) => e.description(),
            GroupErr::File(_) => "Failed to open file",
            GroupErr::FormatParse(_) => "Failed to parse format",
            GroupErr::ItemParse(_) => "Failed to parse items",
        }
    }
}
