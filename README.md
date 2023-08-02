# rust_api
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
curl -X GET http://localhost:3030/header?file_path=./src/titanic.csv
```

This will return a JSON response like the following:

```
0	"PassengerId"
1	"Survived"
2	"Pclass"
3	"Name"
4	"Sex"
5	"Age"
6	"SibSp"
7	"Parch"
8	"Ticket"
9	"Fare"
10	"Cabin"
11	"Embarked"
```

To get the row with the index 3, you can use the following request or just use your browser:

```
curl -X GET http://localhost:3030/row/132?file_path=./src/titanic.csv
```

This will return a response like the following:

```
0	"PassengerId: 133"
1	"Survived: 0"
2	"Pclass: 3"
3	'Name: "Robins, Mrs. Alexander A (Grace Charity Laury)"'
4	'Sex: "female"'
5	"Age: 47.0"
6	"SibSp: 1"
7	"Parch: 0"
8	'Ticket: "A/5. 3337"'
9	"Fare: 14.5"
10	"Cabin: null"
11	'Embarked: "S"'
```