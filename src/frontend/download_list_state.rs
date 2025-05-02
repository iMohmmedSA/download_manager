use ratatui::widgets::TableState;

#[derive(Debug)]
pub struct ListState<T> {
    pub state: TableState,
    pub items: Vec<T>,
}

impl<T> Default for ListState<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ListState<T> {
    pub fn new() -> Self {
        Self {
            state: TableState::default(),
            items: Vec::new(),
        }
    }

    pub fn move_down(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn move_up(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
