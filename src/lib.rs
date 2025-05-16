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

fn create_store(store: StoreType, path: impl AsRef<Path>) -> io::Result<Option<PathBuf>> {
    let dir_fn = store.path();
    if let Some(root) = dir_fn() {
        let store_dir = root.join(format!(".{}", path.as_ref().display()));
        create_dir(&store_dir)?;

        Ok(Some(store_dir))
    } else {
        Ok(None)
    }
}

/// Creates a new dot directory in the systems Audio path (See [`dirs::audio_dir`])
pub fn audio_store(path: impl AsRef<Path>) -> io::Result<Option<PathBuf>> {
    create_store(StoreType::Audio, path)
}

/// Creates a new dot directory in the systems Cache path (See [`dirs::cache_dir`])
pub fn cache_store(path: impl AsRef<Path>) -> io::Result<Option<PathBuf>> {
    create_store(StoreType::Cache, path)
}

/// Creates a new dot directory in the systems Config path (See [`dirs::config_dir`])
pub fn config_store(path: impl AsRef<Path>) -> io::Result<Option<PathBuf>> {
    create_store(StoreType::Config, path)
}

/// Creates a new dot directory in the systems local Config path (See [`dirs::config_local_dir`])
pub fn local_config_store(path: impl AsRef<Path>) -> io::Result<Option<PathBuf>> {
    create_store(StoreType::ConfigLocal, path)
}

/// Creates a new dot directory in the systems Data path (See [`dirs::data_dir`])
pub fn data_store(path: impl AsRef<Path>) -> io::Result<Option<PathBuf>> {
    create_store(StoreType::Data, path)
}

/// Creates a new dot directory in the systems local Data path (See [`dirs::data_local_dir`])
pub fn local_data_store(path: impl AsRef<Path>) -> io::Result<Option<PathBuf>> {
    create_store(StoreType::DataLocal, path)
}

/// Creates a new dot directory in the systems Desktop path (See [`dirs::desktop_dir`])
pub fn desktop_store(path: impl AsRef<Path>) -> io::Result<Option<PathBuf>> {
    create_store(StoreType::Desktop, path)
}

/// Creates a new dot directory in the systems Document path (See [`dirs::document_dir`])
pub fn document_store(path: impl AsRef<Path>) -> io::Result<Option<PathBuf>> {
    create_store(StoreType::Document, path)
}

/// Creates a new dot directory in the systems Download path (See [`dirs::download_dir`])
pub fn download_store(path: impl AsRef<Path>) -> io::Result<Option<PathBuf>> {
    create_store(StoreType::Download, path)
}

/// Creates a new dot directory in the systems Executable path (See [`dirs::executable_dir`])
pub fn executable_store(path: impl AsRef<Path>) -> io::Result<Option<PathBuf>> {
    create_store(StoreType::Executable, path)
}

/// Creates a new dot directory in the systems Font path (See [`dirs::font_dir`])
pub fn font_store(path: impl AsRef<Path>) -> io::Result<Option<PathBuf>> {
    create_store(StoreType::Font, path)
}

/// Creates a new dot directory in the systems Home path (See [`dirs::home_dir`])
pub fn home_store(path: impl AsRef<Path>) -> io::Result<Option<PathBuf>> {
    create_store(StoreType::Home, path)
}

/// Creates a new dot directory in the systems Picture path (See [`dirs::picture_dir`])
pub fn picture_store(path: impl AsRef<Path>) -> io::Result<Option<PathBuf>> {
    create_store(StoreType::Picture, path)
}

/// Creates a new dot directory in the systems Preference path (See [`dirs::picture_dir`])
pub fn preference_store(path: impl AsRef<Path>) -> io::Result<Option<PathBuf>> {
    create_store(StoreType::Preference, path)
}

/// Creates a new dot directory in the systems Public path (See [`dirs::public_dir`])
pub fn public_store(path: impl AsRef<Path>) -> io::Result<Option<PathBuf>> {
    create_store(StoreType::Public, path)
}

/// Creates a new dot directory in the systems Runtime path (See [`dirs::runtime_dir`])
pub fn runtime_store(path: impl AsRef<Path>) -> io::Result<Option<PathBuf>> {
    create_store(StoreType::Runtime, path)
}

/// Creates a new dot directory in the systems State path (See [`dirs::runtime_dir`])
pub fn state_store(path: impl AsRef<Path>) -> io::Result<Option<PathBuf>> {
    create_store(StoreType::State, path)
}

/// Creates a new dot directory in the systems Template path (See [`dirs::template_dir`])
pub fn template_store(path: impl AsRef<Path>) -> io::Result<Option<PathBuf>> {
    create_store(StoreType::Template, path)
}

/// Creates a new dot directory in the systems Video path (See [`dirs::video_dir`])
pub fn video_store(path: impl AsRef<Path>) -> io::Result<Option<PathBuf>> {
    create_store(StoreType::Video, path)
}

/// Create a new dot directory in a custom location of your choosing.
///
/// # Argments
///
/// * `root_path`: The root directory to create the target
/// * `target`: The name of the dot directory or path to create
///
/// # Examples
///
/// ```rust,no_run
/// use dotstore;
///
/// fn main() -> std::io::Result<()> {
///     // Create a new dot directory in a specified location
///     // Will create `/home/user/project/config/.app`
///     let path = dotstore::custom_store("/home/user/project/config", "app")?;
///
///     // Will create `/home/user/project/.settings/user/local`
///     let another_path = dotstore::custom_store("/home/user/project", "settings/user/local")?;
/// }
/// ```
pub fn custom_store(
    root_path: impl AsRef<Path>,
    target: impl AsRef<Path>,
) -> io::Result<Option<PathBuf>> {
    let root = PathBuf::from(root_path.as_ref());
    let store_dir = root.join(format!(".{}", target.as_ref().display()));
    create_dir(&store_dir)?;

    Ok(Some(store_dir))
}
