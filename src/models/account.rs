use {
    crate::models::{Transaction, TransactionType},
    log::{info, warn},
};

/// Represents an account id.
pub type AccountId = u16;
/// Represents the credit amount of an account.
pub type CreditAmount = f32;

/// Represents a clients account.
#[derive(Debug)]
pub struct Account {
    id: AccountId,
    available: CreditAmount,
    held: CreditAmount,
    total: CreditAmount,
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
    pub fn available(&self) -> CreditAmount {
        self.available
    }

    /// Returns the held amount.
    pub fn held(&self) -> CreditAmount {
        self.held
    }

    /// Returns the total amount.
    pub fn total(&self) -> CreditAmount {
        self.total
    }

    /// Returns true if the account is locked.
    pub fn locked(&self) -> bool {
        self.locked
    }

    /// Executes a deposit on the account.
    pub fn deposit(&mut self, amount: CreditAmount) {
        self.available += amount;
        self.total += amount;
        #[cfg(debug_assertions)]
        info!("Deposit {} on account id {}", amount, self.id);
    }

    /// Executes a withdrawal on the account.
    pub fn withdrawal(&mut self, amount: CreditAmount) -> Result<(), String> {
        if self.available < amount {
            let msg = format!(
                "Withdrawal ignored on account id {}. Amount: {}",
                self.id, amount
            );
            #[cfg(debug_assertions)]
            info!("{}", msg);
            return Err(msg);
        }
        self.available -= amount;
        self.total -= amount;
        Ok(())
    }

    /// Executes a dispute transaction. Notice that the transaction put in this
    /// function is already the referenced one (so the one that includes the amount).
    pub fn dispute(&mut self, transaction: &Transaction) {
        self.available -= transaction.amount();
        self.held += transaction.amount();
    }

    /// Executes a resolve transaction. Notice that the transaction put in this
    /// function is already the referenced one (so the one that includes the amount).
    pub fn resolve(&mut self, transaction: &Transaction) {
        self.available += transaction.amount();
        self.held -= transaction.amount();
    }

    /// Executes a chargeback transaction. Notice that the transaction put in this
    /// function is already the referenced one (so the one that includes the amount).
    pub fn chargeback(&mut self, transaction: &Transaction) {
        self.held -= transaction.amount();
        self.total -= transaction.amount();
        self.locked = true;
    }
}
