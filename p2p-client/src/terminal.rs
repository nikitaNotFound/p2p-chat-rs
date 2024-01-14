use std::{io::{Stdout, stdout}, panic};

use crossterm::{terminal::{EnterAlternateScreen, LeaveAlternateScreen}, event::{EnableMouseCapture, DisableMouseCapture, KeyEvent}};
use anyhow::Result;

use crate::{app_context::AppContext, tabs::{AppTabHandler, login_tab::LoginTab}};

pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<Stdout>>;

pub struct P2pChatTerminal {
    terminal: CrosstermTerminal,
    pub tab: Box<dyn AppTabHandler>,
}

impl P2pChatTerminal {
    pub fn new(terminal: CrosstermTerminal) -> Self {
        Self {
            terminal,
            tab: Box::new(LoginTab::default()),
        }
    }

    pub fn enter(&mut self) -> Result<()> {
        crossterm::execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        crossterm::terminal::enable_raw_mode()?;

        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;

        Ok(())
    }

    pub fn draw(&mut self, app_context: &mut AppContext) -> Result<()> {
        self.terminal.draw(|frame| self.tab.render(app_context, frame))?;
        Ok(())
    }

    pub fn handle_key_event(&mut self, app_context: &mut AppContext, event: KeyEvent) {
        self.tab.handle_key_event(app_context, event);
    }

    fn reset() -> Result<()> {
        crossterm::terminal::disable_raw_mode()?;
        crossterm::execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}