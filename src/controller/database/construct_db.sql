CREATE TABLE IF NOT EXISTS users (
    user_id BIGSERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(100) NOT NULL UNIQUE,
    password VARCHAR(200) NOT NULL
);

CREATE TABLE IF NOT EXISTS counts (
	count_id BIGINT PRIMARY KEY,
	user_id BIGINT NOT NULL,
	debtor VARCHAR(100) NOT NULL,
	title VARCHAR(50) NOT NULL,
    description TEXT,
	value REAL NOT NULL,
    paid_installments BIGINT,
    installments BIGINT DEFAULT 1,
    date_in DATE NOT NULL,
    date_out DATE NOT NULL,
	status BOOLEAN NOT NULL,
	FOREIGN KEY (user_id) REFERENCES users
);

SELECT * FROM users;
SELECT * FROM counts;

-- DROP TABLE counts;
-- DROP TABLE users;