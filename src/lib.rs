use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug)]
pub struct FileInfo {
    pub file_name: String,
    pub file_type: String,
    pub file_path: String,
    pub created_time: SystemTime,
    pub modified_time: SystemTime,
    pub size: u64,
    pub size_kb: u64,
    pub size_mb: u64,
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

    for entry in fs::read_dir(dir)? {
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
        let size_kb = size / 1024;
        let size_mb = size_kb / 1024;
        let created_time = metadata.created()?;
        let modified_time = metadata.modified()?;

        files_info.push(FileInfo {
            file_name,
            file_type,
            file_path: path.to_str().unwrap().to_string(),
            created_time,
            modified_time,
            size,
            size_kb,
            size_mb,
        });
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
    let mut total_size = 0;
    for entry in fs::read_dir(Path::new(dir))? {
        let entry = entry?;
        let metadata = fs::metadata(entry.path())?;

        total_size += if metadata.is_file() {
            metadata.len()
        } else if metadata.is_dir() {
            get_size(&entry.path().to_string_lossy())?
        } else {
            0
        };
    }
    Ok(total_size)
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
    //判断当前目录，是否达到keep大小，如果达到，删除最旧的文件，直到小于keep
    let mut dir_size = get_size(dir).unwrap();
    if dir_size < keep {
        return Ok(vec![]);
    }
    let mut files = std::fs::read_dir(dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();
    files.sort_by_key(|path| {
        std::fs::metadata(path)
            .ok()
            .and_then(|metadata| metadata.modified().ok())
            .unwrap_or_else(|| std::time::SystemTime::UNIX_EPOCH)
    });
    let mut removed_files = Vec::new();
    while dir_size > keep {
        if let Some(file) = files.pop() {
            let metadata = std::fs::metadata(&file)?;
            let size = metadata.len();
            dir_size -= size;
            removed_files.push(file.to_str().unwrap().to_string());
            std::fs::remove_file(file)?;
        } else {
            break;
        }
    }
    Ok(removed_files)
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
        let keep = 1024 * 1024 * 1;
        let removed_files = remove_old_files(dir, keep).unwrap();
        println!("Removed files: {:?}", removed_files);
    }
}
