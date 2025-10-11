-- Active: 1718076179564@@127.0.0.1@3306
-- ALTER TABLE counts ADD COLUMN nature VARCHAR(15) DEFAULT ' ';

-- UPDATE counts SET nature = ' ' WHERE nature IS NULL;

-- ALTER TABLE users ADD COLUMN name VARCHAR(100) NOT NULL DEFAULT ' ';

ALTER TABLE counts RENAME TO old_table_counts;

CREATE TABLE natures(
    nature_id SERIAL NOT NULL PRIMARY KEY,
    nature VARCHAR(25) NOT NULL
);

CREATE TABLE counts(
    count_id NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users,
    nature_id INTEGER NOT NULL REFERENCES natures,
    debtor VARCHAR(100) NOT NULL,
    title VARCHAR(50) NOT NULL,
    description TEXT,
    value REAL DEFAULT 0.01,
    paid_installments INTEGER NOT NULL DEFAULT 0,
    installments INTEGER NOT NULL DEFAULT 1,
    date_in DATE NOT NULL CHECK(date_in <= date_out),
    date_out DATE NOT NULL CHECK(date_out >= date_in),
    status BOOLEAN NOT NULL
);

INSERT INTO natures
(nature_id, nature)
SELECT ROW_NUMBER() OVER (ORDER BY nature) AS nature_id,
nature
FROM old_table_counts
GROUP BY nature;

INSERT INTO natures VALUES ('aaa');

INSERT INTO counts
(
    count_id, user_id, nature_id, debtor, 
    title, description, value, paid_installments, 
    installments, date_in, date_out, status
)
SELECT ROW_NUMBER() OVER (PARTITION BY user_id ORDER BY user_id) AS count_id,
user_id, nature_id, debtor, title, 
description, value, paid_installments, 
installments, date_in, date_out, status
FROM old_table_counts, natures
WHERE old_table_counts.nature = natures.nature
ORDER BY old_table_counts.user_id;

alter table users add column last_login date;

