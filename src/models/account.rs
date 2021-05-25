/// Represents a clients account.
pub struct Account {
    id: u16,
    available: f32,
    held: f32,
    total: f32,
    locked: bool,
}

impl Account {
    /// Initializes a new account.
    pub fn new(id: u16) -> Self {
        Self {
            id,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }
}
