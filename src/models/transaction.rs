/// Defines the type of a transaction.
pub enum TransactionType {
    /// Adds credit to the clients account.
    Deposit,
    /// Withdraws a debit to clients account.
    Withdrawal,
    /// Claim for erroneous transaction.
    Dispute,
    /// Resolution to a dispute.
    Resolve,
    /// Final state of dispuse, representing reversing a transaction.
    Chargeback,
}

/// Defines a transaction.
/// Must not change after its creation, so all fields need to be private.
pub struct Transaction {
    transaction_type: TransactionType,
    client: u16,
    tx: u32,
    amount: f32,
}

impl Transaction {
    /// Creates a new transaction.
    pub fn new(
        transaction_type: TransactionType,
        client: u16,
        tx: u32,
        amount: f32,
    ) -> Self {
        Self {
            transaction_type,
            client,
            tx,
            amount,
        }
    }
    /// Returns the transaction type.
    pub fn transaction_type(&self) -> &TransactionType {
        &self.transaction_type
    }

    /// Returns the client id.
    pub fn client(&self) -> u16 {
        self.client
    }

    /// Returns the transaction id.
    pub fn tx(&self) -> u32 {
        self.tx
    }

    /// Returns the amount.
    pub fn amount(&self) -> f32 {
        self.amount
    }
}
