use std::io;
use std::error::Error;
use std::fmt;
use format::FormatErr;

#[derive(Debug)]
pub enum GroupErr {
    EmptyFile,
    NoItems,
    File(io::Error),
    FormatParse(FormatErr),
    ItemParse(Vec<(usize, FormatErr)>),
}

impl From<FormatErr> for GroupErr {
    fn from(e: FormatErr) -> Self {
        GroupErr::FormatParse(e)
    }
}

impl From<Vec<(usize, FormatErr)>> for GroupErr {
    fn from(e: Vec<(usize, FormatErr)>) -> Self {
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
            &GroupErr::EmptyFile => write!(f, "File is empty"),
            &GroupErr::NoItems => write!(f, "No items"),
            &GroupErr::File(_) => write!(f, "Failed to open file"),
            &GroupErr::FormatParse(_) => write!(f, "Failed to parse format"),
            &GroupErr::ItemParse(ref v) => {
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
            &GroupErr::EmptyFile => "File is empty",
            &GroupErr::NoItems => "No items",
            &GroupErr::File(_) => "Failed to open file",
            &GroupErr::FormatParse(_) => "Failed to parse format",
            &GroupErr::ItemParse(_) => "Failed to parse items",
        }
    }
}
