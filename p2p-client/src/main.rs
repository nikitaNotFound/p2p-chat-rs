
use app::P2pChatApp;

use ratatui::prelude::{CrosstermBackend, Terminal};
use terminal::P2pChatTerminal;
use terminal_event_handler::{TerminalEventHandler, TerminalEvent};
use std::io::stdout;
use anyhow::Result;

mod app;
mod ui;
mod terminal;
mod terminal_event_handler;
mod event_binder;

fn main() -> Result<()> {
    let mut app = P2pChatApp::new();

    let event_handler = TerminalEventHandler::new(60);
    let mut chat_terminal = P2pChatTerminal::new(
        Terminal::new(CrosstermBackend::new(stdout()))?,
        event_handler,
    );

    chat_terminal.enter()?;
    loop {
        chat_terminal.draw(&mut app)?;

        match chat_terminal.event_handler.next()? {
            TerminalEvent::Key(e) => event_binder::bind_key(&mut app, e),
            _ => {},
        }

        if app.should_exit {
            break;
        }
    }
    chat_terminal.exit()?;

    Ok(())
}
