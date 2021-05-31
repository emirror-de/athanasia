use {
    crate::models::{Dealer, Storage, TransactionStream},
    log::error,
    std::sync::Arc,
    tokio::sync::Mutex,
};

/// Processing engine. Spawns the given amount of threads for processing.
pub struct Engine {
    pub thread_count: u8,
}

impl Engine {
    /// Creates a new instance.
    pub fn new(thread_count: u8) -> Self {
        Self {
            thread_count
        }
    }

    /// Makes the engine process the file.
    pub fn run(&self, file_name: &str) {
        let s = Arc::new(Mutex::new(TransactionStream::new()));
        let storage = Arc::new(Storage::new());
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        let thread_count = self.thread_count + 1;
        rt.block_on(async {
            let mut handles = vec![];
            for i in 0..thread_count.into() {
                let transaction_stream = s.clone();
                let storage_clone = storage.clone();
                let h = tokio::spawn(async move {
                    let mut d = Dealer::new(
                        i,
                        storage_clone,
                        transaction_stream,
                    );
                    d.work().await;
                });
                handles.push(h);
            }
            match s.lock().await.stream_from_file(file_name).await {
                Ok(_) => (),
                Err(msg) => error!("{}", msg.to_string()),
            }
            futures::future::join_all(handles).await;

            let mut wtr = csv::Writer::from_writer(std::io::stdout());
            let accounts = &*storage.accounts.lock().await;
            for (_, val) in accounts.get_map().iter() {
                wtr.serialize(val).unwrap();
            }
            wtr.flush().unwrap();
        });
    }
}
