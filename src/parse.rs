//! Module with spec parsing.

use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

use crate::Spec;

/// Read a JSON or YAML [Open API Specification].
///
/// [Open API Specification]: Spec
#[cfg(any(feature = "json", feature = "yaml"))]
pub fn read_from_file<P: AsRef<Path>>(path: P) -> io::Result<Spec> {
    _read_from_file(path.as_ref())
}

#[cfg(any(feature = "json", feature = "yaml"))]
fn _read_from_file(path: &Path) -> io::Result<Spec> {
    match path.extension().and_then(|e| e.to_str()) {
        #[cfg(feature = "json")]
        Some("json") => _read_from_json_file(path),
        #[cfg(feature = "yaml")]
        Some("yaml") => _read_from_yaml_file(path),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "unsupported file format",
        )),
    }
}

/// [`read_from_file`], but only for JSON files.
#[cfg(feature = "json")]
pub fn read_from_json_file<P: AsRef<Path>>(path: P) -> io::Result<Spec> {
    _read_from_json_file(path.as_ref())
}

#[cfg(feature = "json")]
fn _read_from_json_file(path: &Path) -> io::Result<Spec> {
    from_file(path, |file| {
        serde_json::from_reader(file).map_err(Into::into)
    })
}

/// [`read_from_file`], but only for YAML files.
#[cfg(feature = "yaml")]
pub fn read_from_yaml_file<P: AsRef<Path>>(path: P) -> io::Result<Spec> {
    _read_from_yaml_file(path.as_ref())
}

#[cfg(feature = "yaml")]
fn _read_from_yaml_file(path: &Path) -> io::Result<Spec> {
    from_file(path, |file| {
        serde_yaml::from_reader(file).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
    })
}

fn from_file<P>(path: &Path, parse: P) -> io::Result<Spec>
where
    P: FnOnce(BufReader<File>) -> io::Result<Spec>,
{
    let file = BufReader::new(File::open(path)?);
    parse(file)
}
