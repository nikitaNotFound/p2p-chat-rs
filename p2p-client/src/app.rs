pub enum P2pChatScreens {
    Login,
    Settings,
    ChatList,
    ChatContent,
    ChatMessage,
}

pub struct P2pChatApp {
    username: Option<String>,
    pwd: Option<String>,
    pub should_exit: bool,
}

impl P2pChatApp {
    pub fn new() -> Self {
        Self {
            username: None,
            pwd: None,
            should_exit: false,
        }
    }
}