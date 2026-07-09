use std::io::{Error, ErrorKind};
use std::path::Path;

/// Function to verify the specififed Forgefile exists on disk.
pub fn verify_forgefile_exists(filepath: &String) -> Result<(), Error> {
    let path: &Path = Path::new(filepath);

    match path.try_exists() {
        Ok(true) => Ok(()),
        Ok(false) => Err(Error::new(
            ErrorKind::NotFound,
            format!("No Forgefiles found at: {}", filepath),
        )),
        Err(e) => Err(e),
    }
}
