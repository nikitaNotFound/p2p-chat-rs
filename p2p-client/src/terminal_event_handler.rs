use std::{sync::mpsc, time::{Duration, Instant}, thread::{self, JoinHandle}};
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use anyhow::Result;

pub enum TerminalEvent {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

pub struct TerminalEventHandler {
    sender: mpsc::Sender<TerminalEvent>,
    receiver: mpsc::Receiver<TerminalEvent>,
    handler: JoinHandle<()>,
}

impl TerminalEventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();

        let handler = Self::generate_event_handler(tick_rate, sender.clone());

        Self {sender, receiver, handler}
    }

    fn generate_event_handler(tick_rate: Duration, sender: mpsc::Sender<TerminalEvent>) -> JoinHandle<()> {
        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate.checked_sub(last_tick.elapsed()).unwrap_or(tick_rate);

                if event::poll(timeout).expect("no events available") {
                    match event::read().expect("unable to read event") {
                    CrosstermEvent::Key(e) => {
                        if e.kind == event::KeyEventKind::Press {
                        sender.send(TerminalEvent::Key(e))
                        } else {
                        Ok(()) // ignore KeyEventKind::Release on windows
                        }
                    },
                    CrosstermEvent::Mouse(e) => sender.send(TerminalEvent::Mouse(e)),
                    CrosstermEvent::Resize(w, h) => sender.send(TerminalEvent::Resize(w, h)),
                    _ => unimplemented!(),
                    }
                    .expect("failed to send terminal event")
                }

                if last_tick.elapsed() >= tick_rate {
                    sender.send(TerminalEvent::Tick).expect("failed to send tick event");
                    last_tick = Instant::now();
                }
            }
        })
    }

    pub fn next(&self) -> Result<TerminalEvent> {
        Ok(self.receiver.recv()?)
    }
}