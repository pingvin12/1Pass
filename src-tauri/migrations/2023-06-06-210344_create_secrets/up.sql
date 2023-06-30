CREATE TABLE secrets (
  id SERIAL PRIMARY KEY,
  userid INTEGER,
  title VARCHAR NOT NULL,
  content TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)