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
  
  - The above step was converted to functions inside the lib.rs
  
  - Each function takes the conn as parameter, and operatates the table

- create async version of the functions.
  
  - each function individually connects to postgres database, and awaits it
  
  - operates on the recieved connection
