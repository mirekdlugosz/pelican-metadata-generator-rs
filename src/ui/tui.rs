use std::path::PathBuf;

use anyhow::Error;
use chrono::offset::TimeZone;
use chrono::{Local, NaiveDate};
use crossterm::{
    event::{self, Event::Key, KeyCode::Char},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::{
    prelude::{CrosstermBackend, Terminal, Frame},
    widgets::{Block, Borders, Paragraph}
};
use tui_textarea::{self, Input, TextArea};

use super::PostEditUI;
use crate::models::{DataStore, PostModel};
use crate::writers;

pub(crate) struct TUIPostUI {}

struct Controller<'a> {
    data_store: &'a DataStore,
    post_model: &'a mut PostModel,
    textarea: TextArea<'a>,
    should_quit: bool,
}

impl<'a> Controller<'a> {
    pub fn new(data_store: &'a DataStore, post_model: &'a mut PostModel) -> Self {
        let mut ta = TextArea::default();
        ta.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title("Cokolwiek")
        );
        Self {
            data_store,
            post_model,
            textarea: ta,
            should_quit: false,
        }
    }

    fn startup(&mut self) -> Result<(), Error> {
        enable_raw_mode()?;
        execute!(std::io::stderr(), EnterAlternateScreen)?;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), Error> {
        execute!(std::io::stderr(), LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    fn run(&mut self) -> Result<(), Error> {
        let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

        loop {
            t.draw(|frame| self.redraw(frame))?;
            self.handle_events()?;

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }

    fn redraw(&mut self, frame: &mut Frame) {
        // dwa taby:
        // - form
        //    Title
        //    Slug - ale jak auto?
        //    Date created
        //    Date modified - czy warto?
        //    Category
        //    Tags
        //    Pole na nowe tagi?
        // - generated metadata
        //    readonly textarea z treścią
        frame.render_widget(Paragraph::new(format!("Counter: 10")), frame.size());
        frame.render_widget((&self).textarea.widget(), frame.size());
    }

    fn handle_events(&mut self) -> Result<(), Error> {
        if event::poll(std::time::Duration::from_millis(250))? {
            if let Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        Char('q') => self.should_quit = true,
                        _ => {},
                    }
                }
            }
        }
        Ok(())
    }
}

fn run_tui(data_store: &DataStore, post_model: &mut PostModel) -> Result<(), Error> {
    let mut controller = Controller::new(data_store, post_model);

    &controller.startup()?;
    let status = &controller.run();
    &controller.shutdown()?;
    &status.as_ref().unwrap(); // FIXME: needs to figure out better way
    Ok(())
}

impl PostEditUI for TUIPostUI {
    fn run(data_store: &DataStore, post_model: &mut PostModel) -> Result<(), Error> {
        run_tui(data_store, post_model)?;
        Ok(())
    }
}
