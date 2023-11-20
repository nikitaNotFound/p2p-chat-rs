use crossterm::event::KeyEvent;
use ratatui::Frame;

use crate::app_context::P2pChatAppContext;

use super::AppTabHandler;

pub struct ChatTab {}

impl AppTabHandler for ChatTab {
    fn render(&self, app: &mut P2pChatAppContext, frame: &mut Frame) {
        todo!()
    }

    fn handle_key_event(&self, app: &mut P2pChatAppContext, event: KeyEvent) {
        todo!()
    }
}