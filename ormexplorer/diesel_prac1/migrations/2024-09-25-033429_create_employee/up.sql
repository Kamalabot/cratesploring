-- Your SQL goes here
CREATE TABLE employee (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  age INTEGER NOT NULL,
  department TEXT NOT NULL,
  working BOOLEAN NOT NULL DEFAULT TRUE
)
