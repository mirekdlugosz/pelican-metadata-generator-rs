use std::env::current_dir;
use std::fs::{create_dir_all, File};
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::path::PathBuf;
use std::string::String;

use anyhow::Error;
use log::debug;
use tempfile::NamedTempFile;

use crate::models;

pub(crate) fn write_post(model: &models::PostModel, file: &PathBuf) -> Result<(), Error> {
    let mut new_content = String::from(model);
    new_content.push('\n');

    debug!("Writing to {}:\n{}", file.display(), new_content);

    let parent = match file.parent() {
        Some(p) => p.to_owned(),
        None => current_dir()?.as_path().to_owned(),
    };

    create_dir_all(&parent)?;

    let mut tfile = NamedTempFile::new_in(&parent)?;
    tfile.write_all(new_content.as_bytes())?;

    match File::open(file) {
        Ok(fh) => {
            let mut reader = BufReader::new(fh);
            tfile.write_all(reader.fill_buf()?)?;
        }
        Err(e) if e.kind() == ErrorKind::NotFound => (),
        Err(e) => return Err(Error::new(e)),
    };

    tfile.persist(file)?;

    Ok(())
}

impl From<&models::PostModel> for String {
    fn from(value: &models::PostModel) -> Self {
        let mut output = String::new();
        output.push_str("Title: ");
        output.push_str(value.title.as_ref().unwrap());
        output.push_str("\n");

        output.push_str("Slug: ");
        output.push_str(value.slug.as_ref().unwrap());
        output.push_str("\n");

        output.push_str("Date: ");
        output.push_str(&value.created_at.format("%Y-%m-%d %H:%M:%S").to_string());
        output.push_str("\n");

        output.push_str("Category: ");
        output.push_str(value.category.as_ref().unwrap());
        output.push_str("\n");

        if value.tags.len() > 0 {
            let mut tags_copy: Vec<&str> = value.tags.iter().map(String::as_str).collect();
            tags_copy.sort_unstable();
            output.push_str("Tags: ");
            output.push_str(&tags_copy.join(", "));
            output.push_str("\n");
        }

        output
    }
}
