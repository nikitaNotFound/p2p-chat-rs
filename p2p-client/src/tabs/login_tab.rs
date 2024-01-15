use crossterm::event::{KeyEvent, KeyCode};
use ratatui::{
    Frame,
    widgets::{Paragraph, Block, Borders, BorderType},
    layout::{Alignment, Layout, Direction, Constraint},
    style::{Color, Style}, text::{Line, Span}
};

use crate::app_context::AppContext;

use super::AppTabHandler;

#[derive(Default)]
enum Mod {
    #[default]Default,
    CommandInput,
}

#[derive(Default)]
pub struct LoginTab {
    current_mod: Mod,
}

impl AppTabHandler for LoginTab {
    fn render(
        &self,
        app: &mut AppContext,
        frame: &mut Frame,
    ) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(80),
                Constraint::Percentage(20),
            ])
            .split(frame.size());

        frame.render_widget(
            Paragraph::new(format!(
"Welcome! How would you like to join the network?\n\n
Create new profile - totally new profile, all options will be listed during creation process.\n
Login with local profile - username and master password required.\n
Recover profile - 24 word sequence to access existing profile private key.\n
Temp profile - no data will be stored locally and no ability to recover after ending session.\n",
            ))
                .block(Block::default()
                    .title("FreeChat 0.1")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
                )
                .style(Style::default().fg(Color::Green))
                .alignment(Alignment::Center),
            layout[0],
        );

        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled(" N ", Style::default().fg(Color::White).bg(Color::LightYellow)),
                Span::raw(" New profile "),
                Span::styled(" L ", Style::default().fg(Color::White).bg(Color::LightYellow)),
                Span::raw(" Login "),
                Span::styled(" R ", Style::default().fg(Color::White).bg(Color::LightYellow)),
                Span::raw(" Recover "),
                Span::styled(" T ", Style::default().fg(Color::White).bg(Color::LightYellow)),
                Span::raw(" Temp mode "),
                Span::styled(" Esc ", Style::default().fg(Color::White).bg(Color::LightYellow)),
                Span::raw(" Close app "),
            ]))
                .alignment(Alignment::Center),
            layout[1]
        );
    }

    fn handle_key_event(&self, app: &mut AppContext, event: KeyEvent) {
        match self.current_mod {
            Mod::CommandInput => {

            },
            Mod::Default => {
                match event.code {
                    KeyCode::Esc => {
                        app.should_exit = true;
                    },
                    _ => {},
                }
            }
        }
    }
}