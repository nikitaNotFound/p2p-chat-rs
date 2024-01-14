use crossterm::event::KeyEvent;
use ratatui::Frame;

use crate::app_context::AppContext;

use super::AppTabHandler;

pub struct SettingsTab {}

impl AppTabHandler for SettingsTab {
    fn render(&self, app: &mut AppContext, frame: &mut Frame) {
        todo!()
    }

    fn handle_key_event(&self, app: &mut AppContext, event: KeyEvent) {
        todo!()
    }
}