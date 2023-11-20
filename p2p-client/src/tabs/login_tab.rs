use crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    widgets::{Paragraph, Block, Borders, BorderType},
    layout::Alignment,
    style::{Color, Style}
};

use crate::app_context::P2pChatAppContext;

use super::AppTabHandler;

#[derive(Default)]
pub struct LoginTab {}

impl AppTabHandler for LoginTab {
    fn render(
        &self,
        app: &mut P2pChatAppContext,
        frame: &mut Frame,
    ) {
        frame.render_widget(
            Paragraph::new(format!(
                "Press `Esc` to exit.",
            ))
                .block(Block::default()
                    .title("P2P CHAT")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
                )
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Center),
            frame.size(),
        )
    }

    fn handle_key_event(&self, app: &mut P2pChatAppContext, event: KeyEvent) {
        todo!()
    }
}