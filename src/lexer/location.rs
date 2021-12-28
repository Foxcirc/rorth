
use std::fmt::{self as format, Debug, Formatter};

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub(crate) struct Location {
    pub(crate) line: usize,
    pub(crate) column: usize,
    pub(crate) total: usize,
}

impl Location {

    #[allow(dead_code)]
    pub(crate) fn new(line: usize, column: usize, total: usize) -> Self {
        Self { line, column, total }
    }

}

impl Debug for Location {
    
    fn fmt(&self, fmt: &mut Formatter) -> format::Result {

        fmt.write_str(&self.line.to_string())?;
        fmt.write_str(":")?;
        fmt.write_str(&self.column.to_string())?;
        // fmt.write_str(&self.total.to_string())?;

        Ok(())

    }

}
