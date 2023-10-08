# Server Project REST API electiva CP3

## Description

- This project is based on the followint article: [REST API](https://codevoweb.com/rust-crud-api-example-with-axum-and-postgresql/)

## Environment

- You should have the following environment variables in your `.env` file:

```
POSTGRES_HOST=
POSTGRES_PORT=
POSTGRES_USER=
POSTGRES_PASSWORD=
POSTGRES_DB=

PGADMIN_DEFAULT_EMAIL=
PGADMIN_DEFAULT_PASSWORD=

DATABASE_URL=
```

## How to run this project

- Clone the repository
- Enter the folder and run the command `cargo install`
- Add the environment variables
- Run the command `sqlx migrate run`
- Run the command `cargo build && cargo run` or `cargo watch -q -c -w src/ -x run`