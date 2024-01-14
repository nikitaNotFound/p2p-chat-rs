pub struct AppContext {
    username: Option<String>,
    pwd: Option<String>,
    pub should_exit: bool,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            username: None,
            pwd: None,
            should_exit: false,
        }
    }
}
