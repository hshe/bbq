use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub file_name: String,
    pub file_type: String,
    pub file_path: String,
    pub created_time: SystemTime,
    pub modified_time: SystemTime,
    pub size: u64,
}

/// Compresses the specified directory into a tar.gz file.
///
/// # Arguments
///
/// * `dir` - The path of the directory to be compressed. The generated tar.gz file will be created in the same directory and named after this directory.
///
/// * `name` - The name of the tar.gz file.
///
/// # Return Value
///
/// * If successful, returns `Ok(())`.
/// * If failed, returns an `Err` containing the error information.
///
/// # Example
///
/// ```
/// use your_crate::tar_dir;
///
/// let result = tar_dir("/path/to/dir", "archive");
/// assert!(result.is_ok());
/// ```
pub fn archive_dir(dir: &str, name: &str) -> std::io::Result<()> {
    let tar_gz = format!("{}.tar.gz", name);
    let output = std::process::Command::new("tar")
        .arg("czvf")
        .arg(&tar_gz)
        .arg(dir)
        .output()?;
    if !output.status.success() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "tar failed"));
    }
    Ok(())
}

/// Removes the specified directory.
///
/// # Arguments
///
/// * `dir` - A string slice that holds the name of the directory
///
/// # Examples
///
/// ```
/// use crate::remove_dir;
///
/// let dir = "some_directory";
/// remove_dir(dir);
/// ```
pub fn remove_dir(dir: &str) -> std::io::Result<()> {
    fs::remove_dir_all(dir)
}

/// Removes the specified file.
///
/// # Arguments
///
/// * `file` - A string slice that holds the name of the file
///
/// # Examples
///
/// ```
/// use crate::remove_file;
///
/// let file = "some_file";
/// remove_file(file);
/// ```
pub fn remove_file(file: &str) -> std::io::Result<()> {
    fs::remove_file(file)
}

/// Reads a file as binary data.
///
/// # Arguments
///
/// * `file` - A string slice that holds the name of the file to read.
///
/// # Returns
///
/// * `std::io::Result<Vec<u8>>` - A Result type. If the operation was successful, it will contain a vector of bytes. If it was not successful, it will contain an error.
pub fn read_file(file: &str) -> std::io::Result<Vec<u8>> {
    fs::read(file)
}

/// Writes binary data to a file.
///
/// # Arguments
///
/// * `file` - A string slice that holds the name of the file to write to.
/// * `data` - A byte slice that contains the data to write to the file.
///
/// # Returns
///
/// * `std::io::Result<()>` - A Result type. If the operation was successful, it will contain an empty tuple. If it was not successful, it will contain an error.
pub fn write_file(file: &str, data: &[u8]) -> std::io::Result<()> {
    fs::write(file, data)
}

/// Reads a file as a text string.
///
/// # Arguments
///
/// * `file` - A string slice that holds the name of the file to read.
///
/// # Returns
///
/// * `std::io::Result<String>` - A Result type. If the operation was successful, it will contain a string. If it was not successful, it will contain an error.
pub fn read_text_file(file: &str) -> std::io::Result<String> {
    fs::read_to_string(file)
}

/// Writes a text string to a file.
///
/// # Arguments
///
/// * `file` - A string slice that holds the name of the file to write to.
/// * `data` - A string slice that contains the text to write to the file.
///
/// # Returns
///
/// * `std::io::Result<()>` - A Result type. If the operation was successful, it will contain an empty tuple. If it was not successful, it will contain an error.
pub fn write_text_file(file: &str, data: &str) -> std::io::Result<()> {
    fs::write(file, data)
}

/// Moves a file from one location to another.
///
/// # Arguments
///
/// * `src` - A string slice that holds the name of the source file.
/// * `dest` - A string slice that holds the name of the destination file.
///
/// # Examples
///
/// ```
/// let src = "src.txt";
/// let dest = "dest.txt";
/// move_file(src, dest);
/// ```
pub fn move_file(src: &str, dest: &str) -> std::io::Result<()> {
    fs::rename(src, dest)
}

pub fn get_dir_info(dir: &str) -> std::io::Result<Vec<FileInfo>> {
    let mut files_info = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            let metadata = fs::metadata(&path)?;
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            let file_type = if metadata.is_file() {
                "File".to_string()
            } else if metadata.is_dir() {
                "Directory".to_string()
            } else {
                "Unknown".to_string()
            };
            let size = metadata.len();
            let created_time = metadata.created()?;
            let modified_time = metadata.modified()?;

            files_info.push(FileInfo {
                file_name,
                file_type,
                file_path: path.to_str().unwrap().to_string(),
                created_time,
                modified_time,
                size,
            });
        }
    }

    Ok(files_info)
}

/// The `get_size` function returns the total size (in bytes) of the specified directory.
///
/// # Arguments
///
/// * `dir` - A string slice that contains the path of the directory to query.
///
/// # Return
///
/// Returns a `std::io::Result<u64>`. If the operation is successful, it will contain the total size of the directory (in bytes).
pub fn get_size(dir: &str) -> std::io::Result<u64> {
    let path = Path::new(dir);
    get_size_by_path(path)
}

fn get_size_by_path(path: &Path) -> std::io::Result<u64> {
    let metadata = fs::metadata(path)?;
    if metadata.is_file() {
        Ok(metadata.len())
    } else if metadata.is_dir() {
        let mut total_size = 0;
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_symlink() {
                continue;
            }
            total_size += get_size_by_path(&entry.path())?;
        }
        Ok(total_size)
    } else {
        Ok(0)
    }
}

/// Removes old files from a directory until the total size of the directory is less than a specified size.
///
/// # Arguments
///
/// * `dir` - A string slice that holds the name of the directory.
/// * `keep` - The maximum size (in bytes) that the directory should be. If the directory is larger than this, the oldest files will be removed until it is less than this size.
///
/// # Returns
///
/// * `std::io::Result<Vec<String>>` - A Result containing a vector of the names of the files that were removed. If an error occurred, it will contain the error.
///
/// # Example
///
/// ```
/// let removed_files = remove_old_files("/path/to/directory", 10000);
/// ```
pub fn remove_old_files(dir: &str, keep: u64) -> std::io::Result<Vec<String>> {
    let mut dir_size = get_size(dir).unwrap();
    if dir_size < keep {
        return Ok(vec![]);
    }
    let path = Path::new(dir);
    let mut files = get_files(path)?;
    files.retain(|path| {
        fs::metadata(path)
            .ok()
            .map(|metadata| !metadata.file_type().is_symlink())
            .unwrap_or(false)
    });
    files.sort_by_key(|path| {
        fs::metadata(path)
            .ok()
            .and_then(|metadata| metadata.modified().ok())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
    });
    let mut removed_files = Vec::new();
    while dir_size > keep {
        if let Some(file) = files.pop() {
            if file.is_symlink() {
                continue;
            }
            let metadata = fs::metadata(&file)?;
            let size = metadata.len();
            dir_size -= size;
            removed_files.push(file.to_str().unwrap().to_string());
            let _ = fs::remove_file(file.clone());
        } else {
            break;
        }
    }
    Ok(removed_files)
}

/// Removes specified files from the system.
///
/// # Arguments
///
/// * `files` - A vector of strings that holds the names of the files to be removed.
///
/// # Returns
///
/// * `std::io::Result<()>` - A Result indicating success or failure. If an error occurred during file removal, it will contain the error.
///
/// # Example
///
/// ```
/// let files_to_remove = vec!["/path/to/file1", "/path/to/file2"];
/// let result = remove_files(files_to_remove);
/// ```
pub fn remove_files(files: Vec<String>) -> std::io::Result<()> {
    for file in files {
        let _ = fs::remove_file(file);
    }
    Ok(())
}

/// Reads multiple files and returns their content as binaries.
///
/// # Arguments
///
/// * `files` - A vector of strings that holds the names of the files to be read.
///
/// # Returns
///
/// * `std::io::Result<Vec<Vec<u8>>>` - A Result containing a vector of binary content for each file or an error.
///
/// # Example
///
/// ```
/// let files_to_read = vec!["/path/to/file1", "/path/to/file2"];
/// let file_contents = read_files(files_to_read);
/// ```
pub fn read_files(files: Vec<String>) -> std::io::Result<Vec<Vec<u8>>> {
    let mut buffers = Vec::new();
    for file in files {
        let buffer = read_file(&file)?;
        buffers.push(buffer);
    }
    Ok(buffers)
}

/// Retrieves all files from a specified directory, including subdirectories.
///
/// # Arguments
///
/// * `dir` - A reference to a Path that holds the directory from which files should be retrieved.
///
/// # Returns
///
/// * `std::io::Result<Vec<std::path::PathBuf>>` - A Result containing a vector of PathBuf, each representing a file in the directory. If an error occurred, it will contain the error.
///
/// # Example
///
/// ```
/// let dir = Path::new("/path/to/directory");
/// let files = get_files(dir);
/// ```
pub fn get_files(dir: &Path) -> std::io::Result<Vec<std::path::PathBuf>> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            let path = entry?.path();
            if path.is_file() {
                if path.is_symlink() {
                    continue;
                }
                files.push(path);
            } else if path.is_dir() {
                match get_files(&path) {
                    Ok(sub_files) => files.extend(sub_files),
                    Err(_) => continue, // Ignore directories that cannot be accessed
                }
            }
        }
    }
    Ok(files)
}
pub fn get_files_info_by_dir(dir: &str) -> std::io::Result<Vec<FileInfo>> {
    let path = Path::new(dir);
    let mut files_info = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            let metadata = fs::metadata(&path)?;
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            let file_type = if metadata.is_file() {
                "File".to_string()
            } else if metadata.is_dir() {
                "Directory".to_string()
            } else {
                "Unknown".to_string()
            };
            let size = metadata.len();
            let created_time = metadata.created()?;
            let modified_time = metadata.modified()?;

            files_info.push(FileInfo {
                file_name,
                file_type,
                file_path: path.to_str().unwrap().to_string(),
                created_time,
                modified_time,
                size,
            });
        }
    }

    Ok(files_info)
}

#[cfg(test)]
mod tests_dir_info {
    use super::*;

    /// The `test_get_dir_info` function tests the functionality of the `get_dir_info` function.
    ///
    /// It will print out the total size of the specified directory (in bytes and MB).
    #[test]
    fn test_get_size() {
        let dir = "/Users/mojih/Downloads";
        let size = get_size(dir).unwrap();
        println!("Total size of {} is {} bytes", dir, size);
        // print MB
        println!("Total size of {} is {} MB", dir, size / 1024 / 1024);
    }
    #[test]
    fn test_get_dir_info() {
        let dir = "/Users/mojih/Downloads";
        let files_info = get_dir_info(dir).unwrap();
        for file_info in files_info {
            println!("{:?}\n", file_info);
        }
    }
}

#[cfg(test)]
mod tests_remove_old_files {
    use super::*;

    #[test]
    fn test_remove_old_files() {
        let dir = "/Users/mojih/Downloads/test";
        let keep = 1024 * 1024 * 80;
        let removed_files = remove_old_files(dir, keep).unwrap();
        println!("Removed files: {:?}", removed_files);
    }

    #[test]
    fn test_get_files() {
        let dir = "/Users/mojih/Downloads/test";
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            println!("path: {:?}", path);
        }
    }

    #[test]
    fn test_get_size_by_path() {
        let path = "/Users/mojih/Downloads/test";
        let size = get_size(path);
        if size.is_err() {
            println!("1111Error: {:?}", size);
        } else {
            let size = size.unwrap();
            println!("size: {:?}", size);
            // mb
            println!("size: {:?}", size / 1024 / 1024);
        }
    }
}
