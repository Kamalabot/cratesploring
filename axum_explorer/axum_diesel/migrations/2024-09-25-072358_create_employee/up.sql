CREATE TABLE axumemployee (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  age INTEGER NOT NULL,
  department TEXT NOT NULL,
  working BOOLEAN NOT NULL DEFAULT TRUE
)