/// Represents an account id.
pub type AccountId = u16;

/// Represents a clients account.
#[derive(Debug)]
pub struct Account {
    id: AccountId,
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

    /// Returns the account id.
    pub fn id(&self) -> AccountId {
        self.id
    }

    /// Returns the available amount.
    pub fn available(&self) -> f32 {
        self.available
    }

    /// Returns the held amount.
    pub fn held(&self) -> f32 {
        self.held
    }

    /// Returns the total amount.
    pub fn total(&self) -> f32 {
        self.total
    }

    /// Returns true if the account is locked.
    pub fn locked(&self) -> bool {
        self.locked
    }
}
