use std::path::PathBuf;

use anyhow::Error;
use chrono::offset::TimeZone;
use chrono::{Local, NaiveDate};
use inquire::{required, DateSelect, MultiSelect, Select, Text};

use super::PostEditUI;
use crate::models::{DataStore, PostModel};
use crate::writers;

pub(crate) struct CLIPostUI {}

struct Controller<'a> {
    data_store: &'a DataStore,
    post_model: &'a mut PostModel,
}

impl<'a> Controller<'a> {
    fn get_categories(&self) -> Vec<String> {
        let mut cats: Vec<String> =
            Vec::from_iter(self.data_store.categories.iter().map(String::from));
        cats.sort_unstable();
        cats
    }

    fn get_tags(&self) -> Vec<String> {
        let mut tags: Vec<String> = Vec::from_iter(self.data_store.tags.iter().map(String::from));
        tags.sort_unstable();
        tags
    }

    fn set_title(&mut self, new_title: String) {
        self.post_model.title = Some(new_title);
    }

    fn set_slug(&mut self, new_slug: String) {
        self.post_model.slug = match new_slug.is_empty() {
            false => Some(new_slug),
            true => Some(self.post_model.generated_slug().unwrap()),
        }
    }

    fn set_created_at(&mut self, new_date: NaiveDate) {
        let new_datetime = Local
            .from_local_datetime(&new_date.and_time(Local::now().time()))
            .unwrap();
        self.post_model.created_at = new_datetime;
    }

    fn set_category(&mut self, new_category: String) {
        self.post_model.category = Some(new_category);
    }

    fn set_tags(&mut self, new_tags: Vec<String>) {
        self.post_model.tags = new_tags;
    }

    fn save_to_file(&mut self, file_path: String) -> Result<(), Error> {
        let real_path = PathBuf::from(file_path);

        writers::write_post(self.post_model, &real_path)?;
        Ok(())
    }
}

fn run_cli(data_store: &DataStore, post_model: &mut PostModel) -> Result<(), Error> {
    let mut controller = Controller {
        data_store,
        post_model,
    };

    let available_categories = (&controller).get_categories();
    let available_tags = (&controller).get_tags();

    let _title = Text::new("Title:").with_validator(required!()).prompt()?;
    let _ = &controller.set_title(_title);

    let _slug = Text::new("Slug:")
        .with_help_message("Leave empty to derive from Title")
        .prompt()?;
    let _ = &controller.set_slug(_slug);

    let default_date = (&controller).post_model.created_at.date_naive();
    let _created_date = DateSelect::new("Date created")
        .with_default(default_date)
        .prompt()?;
    let _ = &controller.set_created_at(_created_date);

    match available_categories.len() {
        0 => (),
        1 => {
            let _category = available_categories.get(0).map(String::from).unwrap();
            let _ = &controller.set_category(_category);
            ()
        }
        _ => {
            let _category = Select::new("Category", available_categories).prompt()?;
            let _ = &controller.set_category(_category);
            ()
        }
    }

    let _tags = MultiSelect::new("Tags", available_tags).prompt()?;
    let _ = &controller.set_tags(_tags);

    let default_path = &controller.post_model.file_path().display().to_string();
    let file_path = Text::new("Save as")
        .with_initial_value(default_path)
        .with_help_message("If file exists, new content will be appended on top")
        .prompt()?;

    let _ = &controller.save_to_file(file_path)?;

    Ok(())
}

impl PostEditUI for CLIPostUI {
    fn run(data_store: &DataStore, post_model: &mut PostModel) -> Result<(), Error> {
        run_cli(data_store, post_model)?;
        Ok(())
    }
}
