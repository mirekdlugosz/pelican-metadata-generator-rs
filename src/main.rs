mod models;
mod readers;
mod ui;
mod writers;

use clap::Parser;
use log::debug;

use crate::ui::PostEditUI;
use crate::ui::UIType;

fn main() {
    env_logger::init();

    let cli = ui::Cli::parse();

    let mut data_store = models::DataStore::new();
    let _ = data_store.fill_from_dir(&cli.directory);
    debug!("Data Store content before opening UI: {:?}", data_store);

    let mut model = models::PostModel::new(cli.directory.clone());

    let ui_result = match UIType::from(cli.ui_group) {
        UIType::CLI => ui::CLIPostUI::run(&data_store, &mut model),
        UIType::TUI => ui::TUIPostUI::run(&data_store, &mut model),
    };
    match ui_result {
        Ok(_) => return (),
        Err(e) => {
            eprintln!("Something went wrong: {:?}", e);
            return ();
        }
    }
}
