use {
    crate::models::AccountId,
    serde::{Deserialize, Serializer},
};

/// Defines a transaction id.
pub type TransactionId = u32;

/// Abstraction for a list of transactions.
/// Vec used intentionally because queues crate requires clone crate.
pub type TransactionQueue = Vec<Transaction>;

/// Defines the type of a transaction.
#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionType {
    /// Adds credit to the clients account.
    #[serde(rename(serialize = "deposit", deserialize = "deposit"))]
    Deposit,
    /// Withdraws a debit to clients account.
    #[serde(rename(serialize = "withdrawal", deserialize = "withdrawal"))]
    Withdrawal,
    /// Claim for erroneous transaction.
    #[serde(rename(serialize = "dispute", deserialize = "dispute"))]
    Dispute,
    /// Resolution to a dispute.
    #[serde(rename(serialize = "resolve", deserialize = "resolve"))]
    Resolve,
    /// Final state of dispuse, representing reversing a transaction.
    #[serde(rename(serialize = "chargeback", deserialize = "chargeback"))]
    Chargeback,
}

/// Defines a transaction.
/// Must not change after its creation, so all fields need to be private.
#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    transaction_type: TransactionType,
    client: AccountId,
    tx: TransactionId,
    #[serde(serialize_with = "serialize_amount")]
    amount: Option<f32>,
}

impl Transaction {
    /// Creates a new transaction.
    pub fn new(
        transaction_type: TransactionType,
        client: AccountId,
        tx: TransactionId,
        amount: Option<f32>,
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
    pub fn client(&self) -> AccountId {
        self.client
    }

    /// Returns the transaction id.
    pub fn tx(&self) -> TransactionId {
        self.tx
    }

    /// Returns the amount.
    pub fn amount(&self) -> Option<f32> {
        self.amount
    }
}

fn serialize_amount<S>(x: &Option<f32>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match x {
        Some(v) => s.serialize_str(&format!("{:.4}", v)),
        None => s.serialize_str(""),
    }
}
