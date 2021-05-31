use {
    crate::models::{
        Storage,
        Transaction,
        TransactionStream,
        TransactionType,
    },
    log::{error, warn, debug},
    std::sync::Arc,
    tokio::sync::{Mutex},
};

const DEALER_SLEEP_MILLISECONDS: u8 = 1;

/// The dealer is able to process transactions and is the primary decision maker
/// in corner cases.
pub struct Dealer {
    id: u32,
    storage: Arc<Storage>,
    transaction_stream: Arc<Mutex<TransactionStream>>,
}

impl Dealer {
    /// Creates a new dealer instance.
    pub fn new(
        id: u32,
        storage: Arc<Storage>,
        transaction_stream: Arc<Mutex<TransactionStream>>,
    ) -> Self {
        Self {
            id,
            storage,
            transaction_stream,
        }
    }

    /// Starts working on the given transaction stream and storage given on
    /// creation.
    pub async fn work(&mut self) {
        loop {
            let mut t = self.transaction_stream.lock().await;
            if !t.alive() && t.empty().await {
                // the stream no longer receives new values and
                // the transactions are processed
                break;
            }
            let transaction = t.next().await;
            if let None = transaction {
                // Nothing to do, give CPU time to breath
                std::thread::sleep(std::time::Duration::from_millis(DEALER_SLEEP_MILLISECONDS as u64));
                continue;
            }
            match self.process(transaction.unwrap()).await {
                Ok(_) => (),
                Err(msg) => error!("{}", msg.to_string()),
            };
            #[cfg(debug_assertions)]
            debug!("Dealer {} processed transaction!", &self.id);
        }
        #[cfg(debug_assertions)]
        debug!("Dealer {} finished work!", &self.id);
    }

    /// Executes the given transaction on the given account.
    /// If the account is locked, the transaction will be ignored.
    pub async fn process(&self, transaction: Transaction) -> Result<(), String> {
        let account_storage = &mut self.storage.accounts.lock().await;
        let acc = account_storage.get(&transaction.client());

        if acc.locked() {
            #[cfg(debug_assertions)]
            warn!("Transaction ignored for locked account: {:?}", transaction);
            return Err(format!("Account is locked!"));
        }

        let transaction_storage = &mut self.storage.transactions.lock().await;
        match transaction.transaction_type() {
            TransactionType::Deposit => acc.deposit(&transaction.amount().unwrap()),
            TransactionType::Withdrawal => {
                acc.withdrawal(&transaction.amount().unwrap())?
            },
            TransactionType::Dispute => {
                let t = transaction_storage.get(&transaction.tx());
                if let None = t {
                    // Partner did a mistake and referred to a transaction not available.
                    #[cfg(debug_assertions)]
                    debug!("Transaction ignored, reference not existing: {:?}", transaction);
                    return Ok(());
                }
                // add to dispute register
                let d = &mut self.storage.dispute_register.lock().await;
                d.dispute(&t.unwrap().tx());

                acc.dispute(&t.unwrap().amount().unwrap());
                // transaction must not be saved because it is only referencing
                // another one
                return Ok(());
            },
            TransactionType::Resolve | TransactionType::Chargeback => {
                let t = transaction_storage.get(&transaction.tx());
                if let None = t {
                    // Partner did a mistake and referred to a transaction not available.
                    #[cfg(debug_assertions)]
                    debug!("Transaction ignored, reference not existing: {:?}", transaction);
                    return Ok(());
                }
                // check if in dispute register
                let d = &mut self.storage.dispute_register.lock().await;
                if !d.is_dispute(&t.unwrap().tx()) {
                    // Partner did a mistake, tx is not under dispute.
                    #[cfg(debug_assertions)]
                    debug!("Transaction ignored, tx is not under dispute: {:?}", transaction);
                    return Ok(());
                }
                match transaction.transaction_type() {
                    TransactionType::Resolve => acc.resolve(&t.unwrap().amount().unwrap()),
                    TransactionType::Chargeback => acc.chargeback(&t.unwrap().amount().unwrap()),
                    _ => (),
                }
                // remove from dispute register
                d.resolve(&t.unwrap().tx());
                // transaction must not be saved because it is only referencing
                // another one
                return Ok(());
            },
        }

        // store processed transaction
        transaction_storage.add(transaction);

        Ok(())
    }
}
