use {log::debug, serde::Serializer};

/// Represents an account id.
pub type AccountId = u16;
/// Represents the credit amount of an account.
pub type CreditAmount = f32;

/// Represents a clients account.
#[derive(Debug, Serialize)]
pub struct Account {
    id: AccountId,
    #[serde(serialize_with = "serialize_amount")]
    available: CreditAmount,
    #[serde(serialize_with = "serialize_amount")]
    held: CreditAmount,
    #[serde(serialize_with = "serialize_amount")]
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
    pub fn deposit(&mut self, amount: &CreditAmount) {
        self.available += amount;
        self.total += amount;
        #[cfg(debug_assertions)]
        debug!("Deposit {} on account id {}", amount, self.id);
    }

    /// Executes a withdrawal on the account.
    pub fn withdrawal(&mut self, amount: &CreditAmount) -> Result<(), String> {
        if self.available < *amount {
            let msg = format!(
                "Withdrawal ignored on account id {}. Amount: {}",
                self.id, amount
            );
            #[cfg(debug_assertions)]
            debug!("{}", msg);
            return Err(msg);
        }
        self.available -= amount;
        self.total -= amount;
        #[cfg(debug_assertions)]
        debug!("Withdrawal {} on account id {}", amount, self.id);
        Ok(())
    }

    /// Executes a dispute transaction.
    pub fn dispute(&mut self, amount: &CreditAmount) {
        self.available -= amount;
        self.held += amount;
        #[cfg(debug_assertions)]
        debug!("Dispute {} on account id {}", amount, self.id);
    }

    /// Executes a resolve transaction.
    pub fn resolve(&mut self, amount: &CreditAmount) {
        self.available += amount;
        self.held -= amount;
        #[cfg(debug_assertions)]
        debug!("Resolve {} on account id {}", amount, self.id);
    }

    /// Executes a chargeback transaction.
    pub fn chargeback(&mut self, amount: &CreditAmount) {
        self.held -= amount;
        self.total -= amount;
        self.locked = true;
        #[cfg(debug_assertions)]
        debug!("Chargeback {} on account id {}", amount, self.id);
    }
}

fn serialize_amount<S>(x: &f32, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&format!("{:.4}", x))
}
