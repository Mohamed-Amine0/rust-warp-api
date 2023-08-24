# dataframe-recordbatch-deltatable-in-rust
dataframe ──> recordbatch ──> deltatable

# DataFrame, RecordBatch, and DeltaTable in Rust

This repository contains Rust code demonstrating the use of DataFrame, RecordBatch, and DeltaTable for processing data. It includes examples of creating and manipulating data, generating RecordBatches from DataFrames, and DeltaTable from RecordBatches.



## Usage

To run the Rust code provided in the repository:

1. Clone the repository to your local machine:

   ```bash
   git clone https://github.com/sambaclab/dataframe-recordbatch-deltatable-in-rust.git
   ```

2. Navigate to the cloned directory:

   ```bash
   cd dataframe-recordbatch-deltatable-in-rust
   ```

3. Run the Rust code:

   - To run the main Rust code:

     ```bash
     cargo run --bin main
     ```

   - To run the Parquet content printing code:

     ```bash
     cargo run --bin print-parquet-content
     ```
   - remember to change the path of the parquet file of your choice

Please ensure you have Rust and the required dependencies installed before running the code.
Make sure to install Cargo before attempting to build and run the Rust code.
Visit this [link](https://doc.rust-lang.org/book/ch01-01-installation.html)

## How the functions interact with each other

```
├── DataFrame ──> [generate_recordbatch_from_dataframe] ──> RecordBatch
│   └── DataFrame ──> [generate_schema_from_dataframe] ──> Schema
│       ├── Series ──> [create_primitive_array] ──> Arc<dyn Array>
│       │   ├── Series ──> [col_to_vec] ──> Vec<String>



├── RecordBatch ──> [create_deltatable_from_recordbatch] ──> DeltaTable
```
## Dependencies

The examples in this repository depend on various Rust libraries, including Polars, Arrow, Parquet and DeltaLake.