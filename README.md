# Rust CSV API

This is a simple Rust application that provides a web API for interacting with a CSV file. The API allows you to read the header, read specific rows, delete columns, and add new columns to the CSV file.

## Requirements

- Rust (stable) - The application is written in Rust, so you need to have Rust installed on your system. 
## Installation and Setup

1. Clone the repository to your local machine
2. you can build the application using "cargo build" and run it using "cargo r -r"

## API Endpoints
The following API endpoints are available:

# 1.Health Check

Endpoint: /health

Method: GET

Description: Returns "OK" as a response for health checks. You can use this endpoint to check if the server is running.

# 2.Read CSV Header

Endpoint: /header

Method: GET

Description: Returns the column names of the CSV file as a JSON array.

# 3.Read CSV Row

Endpoint: /row/{row_index}

Method: GET

Description: Returns the data of the specified row in the CSV file as a JSON array. The row_index parameter should be an integer representing the row index (zero-based).


# 4.Delete CSV Column

Endpoint: /delete/column/{column_name}

Method: DELETE

Description: Deletes the specified column from the CSV file. The column_name parameter should be a string representing the column name.

# 5.Add CSV Column

Endpoint: /add/column/{column_name}

Method: POST

Description: Adds a new column to the CSV file. The column_name parameter should be a string representing the new column's name. The column data should be provided in the request body as a JSON array of strings.

Assuming the application is running on localhost:3030, you can use tools like curl or a web client like Postman to interact with the API.



