# RUST_API
## README

This code provides a simple API for reading and parsing CSV files. The API is implemented using the Warp framework and the Polars library.

The API has two routes:

* `/health`: This route returns a simple "OK" message to indicate that the API is healthy.
* `/api/header`: This route returns the header of the CSV file as JSON.
* `/api/row/<row_index>`: This route returns the row with the specified index as JSON.

To run the API, simply run the following command:

```
cargo run
```

The API will be available on localhost:3030.

## Example

To get the header of the CSV file, you can use the following request:

```
curl -X GET http://localhost:3030/api/header
```

This will return a JSON response like the following:

 ["PassengerId", "Survived", "Pclass", "Name", "Sex", "Age", "SibSp", "Parch", "Ticket", "Fare", "Cabin", "Embarked"]


To get the row with the index 3, you can use the following request:

```
curl -X GET http://localhost:3030/api/row/3
```

This will return a response like the following:


"[\"4\",\"1\",\"1\",\"\\\"Futrelle, Mrs. Jacques Heath (Lily May Peel)\\\"\",\"\\\"female\\\"\",\"35.0\",\"1\",\"0\",\"\\\"113803\\\"\",\"53.1\",\"\\\"C123\\\"\",\"\\\"S\\\"\"]"

## Important Branches

- **test/polars branch**: In this branch, you'll find an enhanced version of my main codebase. I use this branch for testing new features and improvements before merging them into the main code. It also provides a better display of data, enhancing user experience.

- **dataframe/recordbatch/deltatable branch**: Here, you'll discover a refined version of my code where the API seamlessly handles dataframes, record batches, and delta tables. I've implemented functionalities to parse between these data structures efficiently.

Feel free to explore these branches to see the latest developments and improvements in my project.
