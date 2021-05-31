/// Home of all transaction related implementations.
mod transaction;

/// Definition of a transaction stream that is required by the dealer to work with.
mod transaction_stream;

/// Account related definitions and implementations.
mod account;

/// Storage model that holds accounts and transactions.
mod storage;

/// The dealer can be bound to a storage so that its able to process transactions.
mod dealer;

pub use {
    account::{Account, AccountId},
    dealer::Dealer,
    storage::{AccountStorage, DisputeRegister, Storage, TransactionStorage},
    transaction::{
        Transaction,
        TransactionId,
        TransactionQueue,
        TransactionType,
    },
    transaction_stream::TransactionStream,
};
