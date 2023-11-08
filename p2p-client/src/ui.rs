use ratatui::{Frame, widgets::{Paragraph, Block, Borders, BorderType}, prelude::Alignment, style::{Style, Color}};

use crate::app::P2pChatApp;

pub fn render(app: &mut P2pChatApp, frame: &mut Frame) {
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