use std::{io::Error, path::Path};

/// Deletes a file at the given path.
///
/// - `path`: The file path to delete.
///
/// - Returns: `Ok(())` if the file was deleted successfully, or an `Err` with the error details.
pub fn delete_file(path: &str) -> Result<(), Error> {
    std::fs::remove_file(path)
}

/// Deletes a directory and all its contents.
///
/// - `path`: The directory path to delete.
///
/// - Returns: `Ok(())` if the directory and its contents were deleted successfully, or an `Err` with the error details.
pub fn delete_dir(path: &str) -> Result<(), Error> {
    let dir_path: &Path = Path::new(path);
    std::fs::remove_dir_all(dir_path)?;
    Ok(())
}
