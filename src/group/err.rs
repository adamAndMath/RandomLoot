use std::io;
use format::FormatErr;

#[derive(Debug)]
pub enum GroupErr {
    EmptyFile,
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
