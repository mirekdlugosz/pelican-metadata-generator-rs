use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use log::{debug, trace, warn};
use walkdir::WalkDir;

use crate::models::DataStore;

mod markdown;

#[derive(Debug)]
enum FileType {
    MARKDOWN,
    UNKNOWN,
}

#[derive(Debug)]
struct PelicanFileMetadata {
    category: String,
    tags: Vec<String>,
}

trait PelicanFileParser {
    fn read_metadata(file_path: &Path) -> Option<PelicanFileMetadata>;
}

impl From<OsString> for FileType {
    fn from(value: OsString) -> Self {
        if !value.is_ascii() {
            return FileType::UNKNOWN;
        }
        if markdown::EXTENSIONS.contains(&value.to_str().unwrap()) {
            return FileType::MARKDOWN;
        }
        FileType::UNKNOWN
    }
}

pub(crate) fn fill_model_with_dir(data_store: &mut DataStore, dir_path: &PathBuf) -> () {
    for entry in WalkDir::new(dir_path) {
        let existing_entry = match entry {
            Ok(e) => e,
            Err(err) => {
                let path = err.path().unwrap_or(Path::new("")).display();
                match err.io_error() {
                    Some(io_err) => {
                        warn!("Could not read {path}: {:?}", io_err.kind());
                    }
                    None => {
                        warn!("Could not read {path}: Unknown error");
                    }
                }
                continue;
            }
        };
        if existing_entry.path().is_dir() {
            continue;
        }
        trace!("Found file {}", existing_entry.path().display());
        process_file(data_store, existing_entry.path());
    }
}

fn process_file(data_store: &mut DataStore, file_path: &Path) -> Option<()> {
    let file_extension = file_path.extension().unwrap_or_else(|| OsStr::new(""));
    let file_extension = OsString::from(file_extension);
    let file_type = FileType::from(file_extension);

    let file_metadata = match file_type {
        FileType::MARKDOWN => markdown::MarkdownFileParser::read_metadata(file_path),
        FileType::UNKNOWN => None,
    };
    let file_metadata = file_metadata?;
    debug!(
        "{} parsed as {:?}; metadata: {:?}",
        file_path.display(),
        file_type,
        file_metadata
    );
    if !file_metadata.category.is_empty() {
        data_store.categories.insert(file_metadata.category);
    }
    for tag_name in file_metadata.tags.iter() {
        data_store.tags.insert(tag_name.to_owned());
    }
    Some(())
}
