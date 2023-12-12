CREATE TABLE IF NOT EXISTS users (
    user_id BIGSERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    password VARCHAR(200) NOT NULL
);

CREATE TABLE IF NOT EXISTS payments (
    payment_id BIGSERIAL PRIMARY KEY,
    date_init DATE NOT NULL,
    payday DATE NOT NULL
);

CREATE TABLE IF NOT EXISTS counts (
	count_id BIGSERIAL PRIMARY KEY,
	user_id BIGINT NOT NULL,
	debtor VARCHAR(100) NOT NULL,
	title VARCHAR(50) NOT NULL,
    description TEXT,
	value REAL NOT NULL,
    installment_paid BIGINT,
    portion BIGINT DEFAULT 1,
    payment_id BIGINT NOT NULL,
	status BOOLEAN NOT NULL,
	FOREIGN KEY (user_id) REFERENCES users,
    FOREIGN KEY (payment_id) REFERENCES payments
);