

CREATE TABLE IF NOT EXISTS users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(100) NOT NULL UNIQUE,
    password VARCHAR(200) NOT NULL
);

CREATE TABLE IF NOT EXISTS counts (
	count_id INTEGER PRIMARY KEY,
	user_id INTEGER NOT NULL,
	debtor VARCHAR(100) NOT NULL,
	title VARCHAR(50) NOT NULL,
    description TEXT,
	value REAL NOT NULL,
    paid_installments INTEGER,
    installments INTEGER DEFAULT 1,
    date_in DATE NOT NULL,
    date_out DATE NOT NULL,
	status BOOLEAN NOT NULL,
	FOREIGN KEY (user_id) REFERENCES users
);

