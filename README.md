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

To get the header of the CSV file, you can use the following request or just use your browser:

```
curl -X GET http://localhost:3030/api/header
```

This will return a JSON response like the following:

 ["PassengerId", "Survived", "Pclass", "Name", "Sex", "Age", "SibSp", "Parch", "Ticket", "Fare", "Cabin", "Embarked"]


To get the row with the index 3, you can use the following request or just use your browser:

```
curl -X GET http://localhost:3030/api/row/6
```

This will return a response like the following:


["7","0","1","McCarthy, Mr. Timothy J","male","54.0","0","0","17463","51.8625","E46","S"]