use std::path::PathBuf;

use anyhow::Error;
use clap::{arg, Args, Parser};

use crate::models;

mod cli;
mod tui;

pub(crate) use self::cli::CLIPostUI;
pub(crate) use self::tui::TUIPostUI;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Directory to read metadata from
    #[arg(short, long, value_name = "DIRECTORY")]
    pub directory: PathBuf,

    #[clap(flatten)]
    pub ui_group: UIGroup,
}

#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
pub(crate) struct UIGroup {
    /// Use interactive CLI
    #[arg(short, long, default_value_t = true)]
    pub cli: bool,

    /// Use interactive TUI
    #[arg(short, long, default_value_t = false)]
    pub tui: bool,
}

pub(crate) enum UIType {
    CLI,
    TUI,
}

impl From<UIGroup> for UIType {
    fn from(value: UIGroup) -> Self {
        if value.tui {
            return Self::TUI;
        }
        Self::CLI
    }
}

pub(crate) trait PostEditUI {
    fn run(data_store: &models::DataStore, post_model: &mut models::PostModel)
        -> Result<(), Error>;
}
