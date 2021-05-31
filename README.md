# Athanasia - the experimental, multi-threaded transaction processing project

(Name generated using [Behind the name](https://www.behindthename.com/random/random.php?number=1&sets=5&gender=f&surname=&all=yes))

An incoming data stream of bank transactions needs to be processed. The incoming transactions are continuous in time, so it is required to make sure that the transactions are processed in order.

**The current state of development does not have any performance increases compared to single threaded implementation. But could be easily extended to become more performant by implementing the second approach stated below.**

## Approaches

1. The transaction stream is processed one after another. The lock on the transaction queue is removed after the processing of this transaction has been finished. This approach does not give any benefits when running multi-threaded compared to a single threaded application.
2. The transaction stream is being sorted by accounts. Because of the sorting it is possible to process different accounts in parallel. This can be established by eg. giving every ```Dealer``` a range of account ids that it is responsible for.

## CSV structure

The input structure of needs to be like the following (whitespaces are ignored):

```
type,client,tx,amount
deposit,1,1,10.0000
withdrawal,1,2,10.0000
dispute,1,2,
resolve,1,2,
chargeback,1,2,
```

## Generating CSV test files

The crate contains a binary ```generate_csv``` that can be used to generate a csv file. The current state of development only supports the generation of valid ```Deposit``` and ```Withdrawal``` transactions. Run it using:

```rust
cargo run --bin generate_csv -- -a 10 -t 10
```

where ```-a``` is the number of accounts, and ```-t``` the number of transactions generated.

## Running the application

Athanasia takes a single argument as input, which is a String containing the source csv file that contains all transactions to be processed. The file needs to include the header. To run the application enter in the terminal:

```rust
cargo run -- transactions.csv
```

where ```transactions.csv``` determines the input file.

### Available options

* ```-t``` The count of threads that are spawned to process the transactions, limited to 8 threads.
* ```-l``` The log level that is used to display messages, mainly used for debugging. Possible values: ```info```, ```warn```, ```error```, ```debug```. Falls back to ```info``` if unknown value has been entered.

The application creates logfiles in the project directory.

## Tests

The crate has been tested by using manually written csv files that can be found in the ```resources``` folder.

For complicate parts, doc tests have been added.

## Further development

### Increasing performance

Implementing the second option given in the approaches should lead to an increased processing performance due to parallelization.

### Handling input streams capacity

To make this transaction engine work properly when using streams, the transaction stream capacity should be limited to make sure the memory limit is not exceeded.