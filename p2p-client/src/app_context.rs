use crate::tabs::{AppTabHandler, login_tab::LoginTab};

pub struct P2pChatAppContext {
    username: Option<String>,
    pwd: Option<String>,
    pub tab: Box<dyn AppTabHandler>,
    pub should_exit: bool,
}

impl P2pChatAppContext {
    pub fn new() -> Self {
        Self {
            username: None,
            pwd: None,
            should_exit: false,
            tab: Box::new(LoginTab{}),
        }
    }
}