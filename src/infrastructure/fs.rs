//! Filesystem abstraction for testability.
//!
//! This module provides a trait for filesystem operations, allowing tests
//! to mock the filesystem without touching real files.

use std::path::{Path, PathBuf};

use crate::error::{Error, Result};

/// Abstraction over filesystem operations for testability.
pub trait FileSystem: Send + Sync {
    /// Reads the contents of a file as a UTF-8 string.
    fn read_to_string(&self, path: &Path) -> Result<String>;

    /// Writes string contents to a file, creating parent directories as needed.
    fn write(&self, path: &Path, contents: &str) -> Result<()>;

    /// Lists all files matching a glob pattern in a directory.
    fn glob(&self, base: &Path, pattern: &str) -> Result<Vec<PathBuf>>;

    /// Checks if a path exists.
    fn exists(&self, path: &Path) -> bool;

    /// Creates a directory and all parent directories.
    fn create_dir_all(&self, path: &Path) -> Result<()>;
}

/// Production filesystem implementation using `std::fs`.
#[derive(Debug, Clone, Default)]
pub struct RealFileSystem;

impl RealFileSystem {
    /// Creates a new real filesystem instance.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl FileSystem for RealFileSystem {
    fn read_to_string(&self, path: &Path) -> Result<String> {
        std::fs::read_to_string(path).map_err(|source| Error::FileRead {
            path: path.to_path_buf(),
            source,
        })
    }

    fn write(&self, path: &Path, contents: &str) -> Result<()> {
        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent).map_err(|source| Error::FileWrite {
                    path: path.to_path_buf(),
                    source,
                })?;
            }
        }

        std::fs::write(path, contents).map_err(|source| Error::FileWrite {
            path: path.to_path_buf(),
            source,
        })
    }

    fn glob(&self, base: &Path, pattern: &str) -> Result<Vec<PathBuf>> {
        let full_pattern = base.join(pattern);
        let pattern_str = full_pattern.to_string_lossy();

        let entries: Vec<PathBuf> = glob::glob(&pattern_str)
            .map_err(|e| Error::GlobPattern(e.to_string()))?
            .filter_map(std::result::Result::ok)
            .collect();

        Ok(entries)
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn create_dir_all(&self, path: &Path) -> Result<()> {
        std::fs::create_dir_all(path).map_err(|source| Error::FileWrite {
            path: path.to_path_buf(),
            source,
        })
    }
}

/// In-memory filesystem for testing.
#[cfg(any(test, feature = "testing"))]
#[allow(clippy::expect_used)]
pub mod test_support {
    use super::*;
    use std::collections::HashMap;
    use std::sync::{Arc, RwLock};

    /// In-memory filesystem for testing without touching real files.
    #[derive(Debug, Clone, Default)]
    pub struct InMemoryFileSystem {
        files: Arc<RwLock<HashMap<PathBuf, String>>>,
    }

    impl InMemoryFileSystem {
        /// Creates a new empty in-memory filesystem.
        pub fn new() -> Self {
            Self::default()
        }

        /// Adds a file with the given content.
        pub fn add_file(&self, path: impl AsRef<Path>, content: impl Into<String>) {
            let mut files = self.files.write().expect("lock poisoned");
            files.insert(path.as_ref().to_path_buf(), content.into());
        }

        /// Returns all files in the filesystem.
        pub fn files(&self) -> HashMap<PathBuf, String> {
            self.files.read().expect("lock poisoned").clone()
        }
    }

    impl FileSystem for InMemoryFileSystem {
        fn read_to_string(&self, path: &Path) -> Result<String> {
            let files = self.files.read().expect("lock poisoned");
            files.get(path).cloned().ok_or_else(|| Error::FileRead {
                path: path.to_path_buf(),
                source: std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"),
            })
        }

        fn write(&self, path: &Path, contents: &str) -> Result<()> {
            let mut files = self.files.write().expect("lock poisoned");
            files.insert(path.to_path_buf(), contents.to_string());
            Ok(())
        }

        fn glob(&self, base: &Path, pattern: &str) -> Result<Vec<PathBuf>> {
            let files = self.files.read().expect("lock poisoned");

            // Simple pattern matching for testing
            // Supports "*.md" and "**/*.md"
            let is_recursive = pattern.starts_with("**/");
            let suffix = if is_recursive {
                &pattern[3..] // Remove "**/"
            } else {
                pattern
            };

            let paths: Vec<PathBuf> = files
                .keys()
                .filter(|path| {
                    if is_recursive {
                        // Match any file under base with the suffix
                        path.starts_with(base)
                            && path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .is_some_and(|n| matches_simple_pattern(n, suffix))
                    } else {
                        // Match files directly in base
                        path.parent() == Some(base)
                            && path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .is_some_and(|n| matches_simple_pattern(n, pattern))
                    }
                })
                .cloned()
                .collect();

            Ok(paths)
        }

        fn exists(&self, path: &Path) -> bool {
            let files = self.files.read().expect("lock poisoned");
            files.contains_key(path)
        }

        fn create_dir_all(&self, _path: &Path) -> Result<()> {
            // No-op for in-memory filesystem
            Ok(())
        }
    }

    /// Simple glob pattern matching for testing.
    fn matches_simple_pattern(name: &str, pattern: &str) -> bool {
        if pattern == "*" {
            true
        } else if let Some(suffix) = pattern.strip_prefix("*.") {
            name.ends_with(&format!(".{suffix}"))
        } else {
            name == pattern
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_in_memory_fs_read_write() {
            let fs = InMemoryFileSystem::new();
            let path = PathBuf::from("/test/file.txt");

            fs.add_file(&path, "hello world");
            let content = fs.read_to_string(&path).expect("should read");
            assert_eq!(content, "hello world");
        }

        #[test]
        fn test_in_memory_fs_glob() {
            let fs = InMemoryFileSystem::new();
            fs.add_file("/docs/adr/adr_0001.md", "content1");
            fs.add_file("/docs/adr/adr_0002.md", "content2");
            fs.add_file("/docs/adr/readme.txt", "readme");

            let matches = fs
                .glob(Path::new("/docs/adr"), "*.md")
                .expect("should glob");

            assert_eq!(matches.len(), 2);
            assert!(matches.iter().all(|p| p.extension() == Some("md".as_ref())));
        }

        #[test]
        fn test_in_memory_fs_read_nonexistent() {
            let fs = InMemoryFileSystem::new();
            let result = fs.read_to_string(Path::new("/nonexistent"));
            assert!(result.is_err());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_real_fs_read_write() {
        let temp = TempDir::new().expect("should create temp dir");
        let path = temp.path().join("test.txt");

        let fs = RealFileSystem::new();

        fs.write(&path, "hello world").expect("should write");
        let content = fs.read_to_string(&path).expect("should read");

        assert_eq!(content, "hello world");
    }

    #[test]
    fn test_real_fs_creates_parent_dirs() {
        let temp = TempDir::new().expect("should create temp dir");
        let path = temp.path().join("nested/dirs/test.txt");

        let fs = RealFileSystem::new();
        fs.write(&path, "content").expect("should write");

        assert!(path.exists());
    }

    #[test]
    fn test_real_fs_glob() {
        let temp = TempDir::new().expect("should create temp dir");
        let fs = RealFileSystem::new();

        fs.write(&temp.path().join("adr_0001.md"), "content1")
            .expect("write 1");
        fs.write(&temp.path().join("adr_0002.md"), "content2")
            .expect("write 2");
        fs.write(&temp.path().join("readme.txt"), "readme")
            .expect("write 3");

        let matches = fs.glob(temp.path(), "*.md").expect("should glob");

        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_real_fs_exists() {
        let temp = TempDir::new().expect("should create temp dir");
        let fs = RealFileSystem::new();
        let path = temp.path().join("exists.txt");

        assert!(!fs.exists(&path));

        fs.write(&path, "content").expect("should write");

        assert!(fs.exists(&path));
    }
}
