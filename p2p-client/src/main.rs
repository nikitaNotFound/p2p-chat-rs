
use app_context::P2pChatAppContext;

use ratatui::prelude::{CrosstermBackend, Terminal};
use terminal::P2pChatTerminal;
use terminal_event_handler::{TerminalEventHandler, TerminalEvent};
use std::io::stdout;
use anyhow::Result;

mod app_context;
mod terminal;
mod terminal_event_handler;
mod tabs;

fn main() -> Result<()> {
    let mut app_context = P2pChatAppContext::new();

    let event_handler = TerminalEventHandler::new(60);
    let mut chat_terminal = P2pChatTerminal::new(
        Terminal::new(CrosstermBackend::new(stdout()))?,
        event_handler,
    );

    chat_terminal.enter()?;
    loop {
        chat_terminal.draw(&mut app_context)?;

        match chat_terminal.event_handler.next()? {
            TerminalEvent::Key(e) => event_binder::bind_key(&mut app_context, e),
            _ => {},
        }

        if app_context.should_exit {
            break;
        }
    }
    chat_terminal.exit()?;

    Ok(())
}
