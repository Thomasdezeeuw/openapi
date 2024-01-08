//! Rust code generation.

use std::io;

use crate::code::Language;

//const MAX_LINE_WIDTH: usize = 80;
const INDENT_SPACES: usize = 4;

const MODULE_DOC_PREFIX: &str = "//! ";
const LINE_END: &str = "\n";

pub struct Rust;

impl Language for Rust {
    fn module_docs<W: io::Write>(&self, docs: &str, mut out: W) -> io::Result<()> {
        // TODO: limit the length of the lines.
        for line in docs.lines() {
            out.write_all(MODULE_DOC_PREFIX.as_bytes())?;
            out.write_all(line.as_bytes())?;
            out.write_all(LINE_END.as_bytes())?;
        }
        Ok(())
    }
}
