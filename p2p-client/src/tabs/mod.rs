pub mod login_tab;
pub mod analytics_tab;
pub mod chat_tab;
pub mod settings_tab;

use crossterm::event::KeyEvent;
use ratatui::Frame;

use crate::app_context::P2pChatAppContext;

use self::{
    login_tab::LoginTab,
    chat_tab::ChatTab,
    settings_tab::SettingsTab,
    analytics_tab::AnalyticsTab
};

pub enum P2pChatTab {
    Login(LoginTab),
    Chat(ChatTab),
    Settings(SettingsTab),
    Analytics(AnalyticsTab),
}

pub trait AppTabHandler {
    fn render(&self, app: &mut P2pChatAppContext, frame: &mut Frame);
    fn handle_key_event(&self, app: &mut P2pChatAppContext, event: KeyEvent);
}
