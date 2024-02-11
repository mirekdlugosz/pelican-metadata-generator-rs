use std::collections::HashSet;
use std::path::PathBuf;

use chrono::{DateTime, Local};
use log::debug;
use slug;

use crate::readers::fill_model_with_dir;

#[derive(Debug)]
pub struct DataStore {
    pub categories: HashSet<String>,
    pub tags: HashSet<String>,
}

#[derive(Debug)]
pub struct PostModel {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub category: Option<String>,
    pub tags: Vec<String>,
    root_dir: PathBuf,
}

impl PostModel {
    pub fn new(root_dir: PathBuf) -> Self {
        let right_now: DateTime<Local> = Local::now();

        Self {
            title: None,
            slug: None,
            created_at: right_now.clone(),
            updated_at: right_now.clone(),
            category: None,
            tags: Vec::new(),
            root_dir,
        }
    }

    pub fn generated_slug(&self) -> Option<String> {
        self.title.as_ref().map(|t| slug::slugify(t))
    }

    pub fn file_path(&self) -> PathBuf {
        let mut target_path = self.root_dir.clone();
        let year = self.created_at.format("%Y").to_string();
        target_path = target_path.join(year);
        let filename = match &self.slug {
            Some(s) => format!("{s}.md"),
            None => String::from("new-file.md"),
        };
        target_path = target_path.join(filename);
        debug!("Computed target path {}", target_path.display());
        target_path
    }
}

impl DataStore {
    pub fn new() -> Self {
        Self {
            categories: HashSet::new(),
            tags: HashSet::new(),
        }
    }

    pub fn fill_from_dir(&mut self, dir_path: &PathBuf) -> Result<(), String> {
        fill_model_with_dir(self, dir_path);
        Ok(())
    }
}
