use std::io::{Error, ErrorKind};
use std::path::Path;

/// Function to verify the specififed path exists on disk.
pub fn verify_path_exists(filepath: &String, forgefile: bool) -> Result<(), Error> {
    let path: &Path = Path::new(filepath);

    match path.try_exists() {
        Ok(true) => Ok(()),
        Ok(false) => {
            let error_str = if forgefile {
                format!("No Forgefiles found at: {}", filepath)
            } else {
                format!("Path '{}' does not exist", filepath)
            };
            Err(Error::new(ErrorKind::NotFound, error_str))
        }
        Err(e) => Err(e),
    }
}
