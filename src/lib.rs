//! Library to create dot directories in common system places.
//!
//! It's essentially a wrapper around the [`dirs`] crate that allows you to create dot directories
//! (e.g. /home/user/.project-name) in common places (e.g. Home dir, Config dir etc.) to store program data and configs.
//!
//! You can also create them in custom locations
//!
//! # Examples
//!
//! ```rust,no_run
//! use dotstore;
//!
//! fn main() -> std::io::Result<()> {
//!     // Create a new directory called `/home/user/.barracuda`
//!     // The `.` is automatically appended
//!     let project_dir = dotstore::home_store("barracuda")?;
//!
//!     // Create a new directory called `/home/user/.config/.editor`
//!     let editor_dir = dotstore::config_store("editor")?;
//!
//!     // Create a new directory called `/home/user/workspace/middle-earth/.eregion`
//!     let custom_dir = dotstore::custom_store("/home/user/workspace/middle-earth", "eregion")?;
//!
//!     Ok(())
//! }
//! ```

use dirs;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Debug, PartialEq, Eq)]
enum StoreType {
    Home,
    Config,
    ConfigLocal,
    Executable,
    Audio,
    Cache,
    Data,
    DataLocal,
    Desktop,
    Download,
    Document,
    Font,
    Picture,
    Preference,
    Public,
    Runtime,
    State,
    Template,
    Video,
}

impl StoreType {
    pub fn path(&self) -> fn() -> Option<PathBuf> {
        match *self {
            Self::Home => dirs::home_dir,
            Self::Config => dirs::config_dir,
            Self::ConfigLocal => dirs::config_local_dir,
            Self::Executable => dirs::executable_dir,
            Self::Audio => dirs::audio_dir,
            Self::Cache => dirs::cache_dir,
            Self::Data => dirs::data_dir,
            Self::DataLocal => dirs::data_local_dir,
            Self::Desktop => dirs::desktop_dir,
            Self::Download => dirs::download_dir,
            Self::Document => dirs::document_dir,
            Self::Font => dirs::font_dir,
            Self::Picture => dirs::picture_dir,
            Self::Preference => dirs::preference_dir,
            Self::Public => dirs::public_dir,
            Self::Runtime => dirs::runtime_dir,
            Self::State => dirs::state_dir,
            Self::Template => dirs::template_dir,
            Self::Video => dirs::video_dir,
        }
    }
}

fn create_dir(path: &PathBuf) -> io::Result<()> {
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    Ok(())
}

fn create_store(store: StoreType, path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let dir_fn = store.path();
    let dir = if let Some(root) = dir_fn() {
        let store_dir = root.join(format!(".{}", path.as_ref().display()));
        create_dir(&store_dir)?;

        store_dir
    } else {
        unreachable!(
            "should be covered by cfg directives:\nStore Type - {store:?}\nPath - {}\nOS: {}\n\nFile a bug!",
            path.as_ref().display(),
            std::env::consts::OS
        )
    };

    Ok(dir)
}

/// Creates a new directory (prefixed with a `.`) in the system's Audio directory
///
/// See [`dirs::audio_dir`] for your system's Audio directory
pub fn audio_store(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    create_store(StoreType::Audio, path)
}

/// Creates a new directory (prefixed with a `.`) in the system's Cache directory
///
/// See [`dirs::cache_dir`] for your system's Cache directory
pub fn cache_store(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    create_store(StoreType::Cache, path)
}

pub fn config_store(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    create_store(StoreType::Config, path)
}

pub fn local_config_store(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    create_store(StoreType::ConfigLocal, path)
}

pub fn data_store(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    create_store(StoreType::Data, path)
}

pub fn local_data_store(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    create_store(StoreType::DataLocal, path)
}

pub fn desktop_store(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    create_store(StoreType::Desktop, path)
}

pub fn document_store(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    create_store(StoreType::Document, path)
}

pub fn download_store(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    create_store(StoreType::Download, path)
}

#[cfg(not(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "ios",
    target_arch = "wasm32"
)))]
pub fn executable_store(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    create_store(StoreType::Executable, path)
}

#[cfg(not(target_os = "windows"))]
pub fn font_store(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    create_store(StoreType::Font, path)
}

pub fn home_store(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    create_store(StoreType::Home, path)
}

pub fn picture_store(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    create_store(StoreType::Picture, path)
}

pub fn preference_store(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    create_store(StoreType::Preference, path)
}

pub fn public_store(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    create_store(StoreType::Public, path)
}

#[cfg(not(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "ios",
    target_arch = "wasm32"
)))]
pub fn runtime_store(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    create_store(StoreType::Runtime, path)
}

#[cfg(not(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "ios",
    target_arch = "wasm32"
)))]
pub fn state_store(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    create_store(StoreType::State, path)
}

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
pub fn template_store(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    create_store(StoreType::Template, path)
}

pub fn video_store(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    create_store(StoreType::Video, path)
}

pub fn custom_store(root: impl AsRef<Path>, path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let root = PathBuf::from(root.as_ref());
    let store_dir = root.join(format!(".{}", path.as_ref().display()));
    create_dir(&store_dir)?;

    Ok(store_dir)
}
