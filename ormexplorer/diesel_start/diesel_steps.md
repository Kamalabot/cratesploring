- cargo add diesel --features="postgres"

- cargo install diesel_cli

- creating .env file and adding DATABASE_URL="postgres://usname:psttd@localhost/database"

- diesel setup

- diesel migration generate create_table1

- edit up.sql and down.sql to add the table creation scripts

- diesel migration run / redo

- diesel migration generate --diff-schema create_table1 to generate schema.rs

- writing connection code in lib.rs by creating it under src

- create bin folder under src to store CRUD applications 


