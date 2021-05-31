use {
    athanasia::models::{Transaction, TransactionType},
    clap::{crate_authors, crate_version, Clap},
    rand::Rng,
};

/// This small generator creates transactions that can be used to test athanasia.
/// WIP. Currently it is only able to generated meaningful Deposit and Withdrawal
/// transactions.
#[derive(Clap, Debug, Clone)]
#[clap(
    version = crate_version!(),
    author = crate_authors!(", ")
)]
pub struct Config {
    #[clap(short)]
    pub transaction_count: u32,
    #[clap(short)]
    pub account_count: u16,
}

fn main() {
    let config = Config::parse();

    let mut rng = rand::thread_rng();

    let mut v = vec![];
    let mut transaction_ids: Vec<u32> = (0..config.transaction_count).collect();
    let account_ids: Vec<u16> = (0..config.account_count).collect();

    for _i in 0..config.transaction_count {
        let amount = rng.gen::<f32>();
        let transaction_type = rng.gen_range(0..5);
        let (transaction_type, amount) = match transaction_type {
            0 => (TransactionType::Deposit, Some(amount)),
            1 => (TransactionType::Withdrawal, Some(amount)),
            2 => (TransactionType::Dispute, None),
            3 => (TransactionType::Resolve, None),
            4 => (TransactionType::Chargeback, None),
            _ => panic!("No transaction type matched your selection!"),
        };

        let transaction_index = rng.gen_range(0..transaction_ids.len());
        let id = transaction_ids.remove(transaction_index);
        let account_index = rng.gen_range(0..account_ids.len());
        let acc_id = account_ids[account_index];

        v.push(Transaction::new(transaction_type, acc_id, id, amount));
    }
    let mut wtr = csv::Writer::from_writer(std::io::stdout());
    for t in v {
        wtr.serialize(t).unwrap();
    }
    wtr.flush().unwrap();
}
