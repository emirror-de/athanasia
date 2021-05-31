use {
    crate::models::{Transaction, TransactionQueue},
    async_stream::stream,
    csv,
    futures_util::{pin_mut, stream::StreamExt},
    log::{error},
    std::{sync::Arc, path::Path, 
        io::{BufReader},
        fs::File,
    },
    tokio::{
        sync::{RwLock}
    },
};

/// Abstration of an incoming stream of transaction.
pub struct TransactionStream {
    alive: bool,
    transaction_queue: Arc<RwLock<TransactionQueue>>,
}

impl TransactionStream {
    pub fn new() -> Self {
        // TODO: limit capacity on transactionqueue
        let transaction_queue = Arc::new(RwLock::new(TransactionQueue::new()));
        Self {
            alive: true,
            transaction_queue,
        }
    }

    /// Creates a new instance that reads from the given file.
    pub async fn stream_from_file(&mut self, name: &str) -> Result<(), std::io::Error> {
        let file = File::open(Path::new(name))?;
        let mut reader = csv::ReaderBuilder::new()
            .trim(csv::Trim::All)
            .from_reader(BufReader::new(file));
        let s = stream! {
            for result in reader.deserialize() {
                let transaction: Transaction = match result {
                    Ok(t) => t,
                    Err(msg) => {
                        error!("{}", msg.to_string());
                        continue;
                    },
                };
                yield transaction;
            }
        };
        pin_mut!(s);

        while let Some(t) = s.next().await {
            let mut q = self.transaction_queue.write().await;
            q.push(t);
        }

        self.alive = false;
        Ok(())
    }

    pub async fn next(&mut self) -> Option<Transaction> {
        let nothing_available = {
            let q = self.transaction_queue.read().await;
            q.len() == 0
        };
        if nothing_available {
            return None;
        }
        let mut q = self.transaction_queue.write().await;
        Some(q.remove(0))
    }

    /// Returns an Arc to the transaction queue.
    pub fn transaction_queue(&self) -> Arc<RwLock<TransactionQueue>> {
        self.transaction_queue.clone()
    }

    /// Tells if the stream is still alive.
    pub fn alive(&self) -> bool {
        self.alive
    }

    /// Returns true if the transaction queue is empty.
    pub async fn empty(&self) -> bool {
        self.transaction_queue.read().await.len() == 0
    }
}
