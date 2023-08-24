# Rust REST API for DataFrame Manipulation

This is a REST API written in Rust using the warp framework for handling DataFrame manipulation operations. The API provides endpoints to perform various operations on Parquet files, such as deleting columns, renaming columns, adding data to columns, adding null columns, casting columns, adding columns with values, adding rows, deleting rows, updating rows, and updating the row counter.

P.S: All of the methods support arguments for an input parquet file path and an output parquet file for future implementations but for now I kept all of the changes on a single parquet file for this implementation. 

## Getting Started

Before you begin make sure you have Rust installed on your system.

1. Clone the repository.
2. Navigate to the project directory.
3. Build and run the API using "cargo run --release <parquet_path>" (you can use ./data/test.parquet)
   Replace `<parquet_path>` with the path to your input Parquet file.
4. The API will be accessible at http://127.0.0.1:3030.

## Directory Structure

The code has been organized into separate directories to enhance modularity and maintainability:

- `src/`: Contains the main application code.
  - `handlers/`: Contains individual handler modules for different API endpoints.
    - `column_handlers.rs`: contains handlers responsible for Dataframe columns' manipulations.
    - `row_handlers.rs`: contains handlers responsible for Dataframe rows' manipulations.
  - `models/`: Contains data models and structures.
    - `mod.rs`: Contains definitions of the data models and structures.
  - `main.rs`: The entry point of the application.
  - `utils.rs`: Contains utility functions, macros, and traits used across the application.
- `data/`: Contains sample data files (e.g., test.parquet).

## API Endpoints

The following endpoints are available:

- GET /display-dataframe: Display the contents of the DataFrame.
- DELETE /delete-column/{column_name}: Delete a column from the DataFrame.
- PUT /rename-column/{old_column_name}: Rename a column in the DataFrame.
- POST /add-data-to-column/{column_name}: Add data to a column in the DataFrame.
  P.S: you need to format the data correctly to avoid errors, here's an example to help with that:
  Adding data to a column of Strings:
```json
{
"data": {
"Utf8Owned": "new data"
}
}
```
- POST /add-null-column/{column_name}/{column_type}: Add a null column of a specified type to the DataFrame.
- PUT /cast-column/{column_name}/{new_type}: Cast a column to a new data type.
- POST /add-column-with-values/{column_name}: Add a new column with specified values to the DataFrame.
P.S: you need to format the data correctly to avoid errors, here's an example to help with that:
This is a column of integers:
```json
[
{
"Int32": 898
},
{
"Int32": 5464
},
{
"Int32": 85694
},
{
"Int32": 500
}
]
```
- POST /add-row: Add a new row with specified values to the DataFrame.
P.S: you need to format the data correctly to avoid errors, here's an example to help with that:
This example should work on the provided parquet file in "./data/test.parquet"
```json
[
{
"Int32": 78
},
{
"Int32": 65
},
{
"Int32": 782
},
{
"Utf8Owned": "Hermano"
},
{
"Int32": 30
},
{
"Utf8Owned": "example"
}
]
```
- DELETE /delete-row/{row_index}: Delete a row from the DataFrame.
- PUT /update-row/{row_index}: Update a row in the DataFrame.
P.S: you need to format the data correctly to avoid errors, here's an example to help with that:
This example should work on the provided parquet file in "./data/test.parquet"
```json
[
{
"Int32": 8585
},
{
"Int32": 87456
},
{
"Int32": 879
},
{
"Utf8Owned": "test_data"
},
{
"Int32": 785
},
{
"Utf8Owned": "updated :D"
}
]
```
- PUT /update-row-counter: Update the row counter in the DataFrame and add a visible column displaying the index.

## Dependencies

The following Rust crates are used in this project:

- `lazy_static` for lazy initialization of static variables. "cargo add lazy_static"
- `polars` for DataFrame manipulation with features parquet and serde "cargo add polars --features serde,parquet"
- `polars-core` for DataFrame manipulation functions. "cargo add polars-core"
- `polars-io` for Parquet file reading and writing. "cargo add polars-io"
- `serde` and `serde_json` for serialization and deserialization of JSON. "cargo add serde" and "cargo add serde_json"
- `warp` for building the REST API. "cargo add warp"
- `tokio` with the `full` feature for asynchronous programming. "cargo add tokio --features full"

## Notes

- This API uses asynchronous programming with the `tokio` runtime.
- The API endpoints are defined using the `warp` framework.
- DataFrame manipulation operations are performed using the polars-core crate.
