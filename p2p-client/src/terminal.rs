use std::{io::{Stdout, stdout}, panic};

use crossterm::{terminal::{EnterAlternateScreen, LeaveAlternateScreen}, event::{EnableMouseCapture, DisableMouseCapture}};
use anyhow::Result;

use crate::{terminal_event_handler::TerminalEventHandler, app_context::P2pChatAppContext};

pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<Stdout>>;

pub struct P2pChatTerminal {
    terminal: CrosstermTerminal,
    pub event_handler: TerminalEventHandler,
}

impl P2pChatTerminal {
    pub fn new(terminal: CrosstermTerminal, event_handler: TerminalEventHandler) -> Self {
        Self {
            terminal,
            event_handler,
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

    pub fn draw(&mut self, f: F) -> Result<()>
        where
    {
        self.terminal.draw(|frame| ui::render(app, frame))?;
        Ok(())
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