use {
    crate::models::{Account, AccountId, Transaction, TransactionId},
    std::collections::HashMap,
    tokio::sync::Mutex,
};

/// Combines accounts as well as all transactions and dispute transaction ids.
pub struct Storage {
    /// Stores accounts by client id.
    pub accounts: Mutex<AccountStorage>,
    /// Stores transactions by tx.
    pub transactions: Mutex<TransactionStorage>,
    /// Stores transaction ids that are under dispute.
    pub dispute_register: Mutex<DisputeRegister>,
}

impl Storage {
    /// Creates a new storage.
    pub fn new() -> Self {
        Self {
            accounts: Mutex::new(AccountStorage::new()),
            transactions: Mutex::new(TransactionStorage::new()),
            dispute_register: Mutex::new(DisputeRegister::new()),
        }
    }
}

/// Contains all accounts.
#[derive(Serialize)]
pub struct AccountStorage(HashMap<AccountId, Account>);

impl AccountStorage {
    /// Creates a new account storage instance.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Updates the storage by the given account, or creates a new one if not
    /// found.
    /// # Example
    /// ```rust
    /// use athanasia::{models::AccountStorage, models::Account};
    /// let mut storage = AccountStorage::new();
    /// let a = Account::new(1);
    /// storage.set(a);
    /// assert_eq!(storage.get(&1).available(), 0.0);
    /// ```
    pub fn set(&mut self, account: Account) -> Result<(), Account> {
        let acc = self
            .0
            .entry(account.id())
            .or_insert(Account::new(account.id()));
        *acc = account;
        Ok(())
    }

    /// Returns the stored account for the given id.
    pub fn get(&mut self, id: &AccountId) -> &mut Account {
        self.0.entry(*id).or_insert(Account::new(*id))
    }

    pub fn get_map(&self) -> &HashMap<AccountId, Account> {
        &self.0
    }
}

/// Storage for all transactions.
pub struct TransactionStorage(HashMap<TransactionId, Transaction>);

impl TransactionStorage {
    /// Creates a new transaction storage instance.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Adds the given transaction to the storage.
    /// If transaction id is already present, the transaction is being ignored.
    pub fn add(&mut self, transaction: Transaction) {
        self.0.insert(transaction.tx(), transaction);
    }

    /// Gets a transaction from the storage if available.
    pub fn get(&self, id: &TransactionId) -> Option<&Transaction> {
        self.0.get(id)
    }
}

/// Contains ids of transactions currently under dispute.
pub struct DisputeRegister(Vec<TransactionId>);

impl DisputeRegister {
    /// Creates a new dispute register instance.
    pub fn new() -> Self {
        Self(vec![])
    }

    /// Adds the transaction id to the dispute list.
    pub fn dispute(&mut self, id: &TransactionId) {
        self.0.push(*id);
    }

    /// Removes the transaction id from the dispute list.
    /// # Example
    /// ```rust
    /// use athanasia::models::{DisputeRegister, TransactionId};
    /// let mut d = DisputeRegister::new();
    /// let id = 300;
    /// let id2 = 700;
    /// d.dispute(&id);
    /// d.dispute(&id2);
    /// assert_eq!(d.is_dispute(&id), true);
    /// d.resolve(&id);
    /// assert_eq!(d.is_dispute(&id), false);
    /// assert_eq!(d.is_dispute(&id2), true);
    /// ```
    pub fn resolve(&mut self, id: &TransactionId) {
        self.0.retain(|&x| x != *id);
    }

    /// Checks if the transaction id is on the dispute list.
    pub fn is_dispute(&mut self, id: &TransactionId) -> bool {
        self.0.contains(id)
    }
}
