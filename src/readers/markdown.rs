use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use log::warn;

use super::{PelicanFileMetadata, PelicanFileParser};

type RawMetaStorage = HashMap<String, String>;

pub(super) static EXTENSIONS: [&str; 4] = ["md", "markdown", "mkd", "mdown"];

pub(super) struct MarkdownFileParser {}

impl PelicanFileParser for MarkdownFileParser {
    fn read_metadata(path: &Path) -> Option<PelicanFileMetadata> {
        let raw_meta = read_raw_meta(path)?;

        let mut category = String::new();
        let mut tags: Vec<String> = Vec::new();

        for (key, value) in raw_meta.iter() {
            match key.to_lowercase().as_str() {
                "category" => category.push_str(value.trim()),
                "tags" => {
                    for tag_name in value.split(',') {
                        tags.push(String::from(tag_name.trim()));
                    }
                }
                _ => (),
            }
        }

        if category.is_empty() && tags.is_empty() {
            return None;
        }

        let pfm = PelicanFileMetadata { category, tags };
        Some(pfm)
    }
}

fn read_raw_meta(path: &Path) -> Option<RawMetaStorage> {
    let mut raw_meta: RawMetaStorage = HashMap::with_capacity(16);

    let fh = File::open(path)
        .or_else(|e| {
            warn!("Failed to open {}: {:?}", path.display(), e.kind());
            Err(e)
        })
        .ok()?;
    let reader = BufReader::new(fh);

    for line in reader.lines() {
        let line = line
            .or_else(|e| {
                warn!(
                    "Error while reading file {}: {:?}",
                    path.display(),
                    e.kind()
                );
                Err(e)
            })
            .ok()?;

        if line.trim().is_empty() {
            break;
        }

        let mut parsed_line = line.splitn(2, ':');
        let meta_key = match parsed_line.next() {
            Some(v) => v,
            None => continue,
        };
        let meta_value = match parsed_line.next() {
            Some(v) => v,
            None => continue,
        };

        raw_meta.insert(
            String::from(meta_key.trim()),
            String::from(meta_value.trim()),
        );
    }
    Some(raw_meta)
}
