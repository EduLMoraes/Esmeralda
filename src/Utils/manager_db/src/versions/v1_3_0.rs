pub fn get_sql() -> String {
    format!(
        "{} {} {}",
        get_create_tables(),
        get_create_triggers(),
        get_create_views()
    )
}

fn get_create_tables() -> &'static str {
    "
CREATE TABLE IF NOT EXISTS Users (
    id_user INTEGER PRIMARY KEY,
    password varchar(200) NOT NULL,
    email varchar(100) NOT NULL UNIQUE,
    username varchar(100) NOT NULL UNIQUE,
    last_login date
);
CREATE TABLE IF NOT EXISTS Stock_Exchange_Shares (
    id_SES BIGINT PRIMARY KEY,
    value float
);
CREATE TABLE IF NOT EXISTS Old_Counts (
    id_old_count BIGINT PRIMARY KEY,
    column varchar(20),
    old_value text,
    new_value text
);
CREATE TABLE IF NOT EXISTS Property (
    uid_property varchar(44) PRIMARY KEY,
    category varchar(50),
    acquisition_date date,
    value float,
    cnpj char(14),
    cpf char(11),
    register_number varchar(200)
);
CREATE TABLE IF NOT EXISTS last_yields (
    last_yields_PK integer NOT NULL PRIMARY KEY,
    last_yields float
);
CREATE TABLE IF NOT EXISTS dates_yield (
    dates_yield_PK integer NOT NULL PRIMARY KEY,
    dates_yield date
);

CREATE TABLE IF NOT EXISTS FIIs (
    id_fii BIGINT PRIMARY KEY,
    last_yields_PK integer REFERENCES last_yields(last_yields_PK) ON DELETE CASCADE,
    dates_yield_PK integer REFERENCES dates_yield(dates_yield_PK) ON DELETE CASCADE,
    value float,
    dividend_yield float
);
CREATE TABLE IF NOT EXISTS Counts (
    id_count INTEGER PRIMARY KEY,
    id_user integer REFERENCES Users(id_user) ON DELETE CASCADE,
    paid_installments integer DEFAULT 1 CHECK(paid_installments >= 0),
    installments integer DEFAULT 1 CHECK(installments >= 1),
    debtor varchar(100) NOT NULL,
    value float CHECK(value > 0.00),
    title varchar(50),
    date_out date NOT NULL,
    date_in date NOT NULL,
    proof blob,
    nature varchar(50) NOT NULL,
    description text
);
CREATE TABLE IF NOT EXISTS History (
    id_history BIGINT PRIMARY KEY,
    id_old_count integer REFERENCES Old_Counts(id_old_count) ON DELETE CASCADE,
    id_count integer REFERENCES Counts(id_count) ON DELETE CASCADE,
    id_user integer REFERENCES Users(id_user) ON DELETE CASCADE,
    date timestamp
);
CREATE TABLE IF NOT EXISTS Address (
    id_addres BIGINT PRIMARY KEY,
    uid_property varchar(44) REFERENCES Property(uid_property) ON DELETE CASCADE,
    type varchar(30),
    public_place varchar(200),
    number integer,
    complement varchar(200),
    neighborhood varchar(200),
    district varchar(50),
    city varchar(70),
    state varchar(30)
);
CREATE TABLE IF NOT EXISTS People (
    uid_people varchar(44) PRIMARY KEY,
    id_addres integer REFERENCES Address(id_addres),
    id_user integer REFERENCES Users(id_user) ON DELETE CASCADE,
    provider varchar(44),
    wage float,
    name varchar(50),
    date_of_birth date,
    cell_phone char(12),
    voter_registration varchar(12),
    rg char(10),
    cpf char(11),
    surname varchar(100)
);
CREATE TABLE IF NOT EXISTS Investments (
    id_invest BIGINT PRIMARY KEY,
    uid_people varchar(44) REFERENCES People(uid_people) ON DELETE CASCADE,
    type_invest varchar(30),
    value_apply float,
    redeption_value float,
    date_apply date,
    redeption_date date,
    investment varchar(100),
    cnpj char(14)
);
CREATE TABLE IF NOT EXISTS Bank (
    uid_bank varchar(44) PRIMARY KEY,
    uid_people varchar(44) REFERENCES People(uid_people) ON DELETE CASCADE,
    name varchar(50),
    code text,
    agency_number integer
);
CREATE TABLE IF NOT EXISTS Receipts (
    uid_receipt varchar(44) PRIMARY KEY,
    uid_property varchar(44) REFERENCES Property(uid_property) ON DELETE CASCADE,
    type varchar(30),
    document blob
);
CREATE TABLE IF NOT EXISTS Goal (
    uid varchar(44) PRIMARY KEY,
    id_user integer REFERENCES Users(id_user) ON DELETE CASCADE,
    title varchar(30),
    description varchar(255),
    image blob,
    time date,
    start timestamp,
    amount float,
    desired_value float,
    achivied boolean
);
CREATE TABLE IF NOT EXISTS Investments_FIIs_Stock_Exchange_Shares (
    id_invest integer REFERENCES Investments(id_invest) ON DELETE CASCADE,
    id_fii integer REFERENCES FIIs(id_fii) ON DELETE CASCADE,
    id_SES integer REFERENCES Stock_Exchange_Shares(id_SES) ON DELETE CASCADE,
    title varchar(50),
    n_quotas integer,
    simbol varchar(10)
);
 "
}

fn get_create_triggers() -> &'static str {
    "

------------------------------------ TRIGGER OF ADD_PEOPLE ----------------------------------------------------------

DROP TRIGGER IF EXISTS add_people_user;
CREATE TRIGGER add_people_user
AFTER INSERT ON users
BEGIN
    INSERT INTO People 
    VALUES(
        (SELECT * FROM gen_uid),
         NULL, NEW.id_user, NULL, NULL, NULL, NULL, NULL, NULL, NULL, 
         NULL, NULL
    );
END;

-- INSERT INTO users VALUES(1, '1234', 'jhon@bol.com', 'Jhon', CURRENT_DATE);
-- select * from users NATURAL join People;

-- Active: 1718076179564@@127.0.0.1@3306
------------------------------------ TRIGGER OF OLD_COUNTS ----------------------------------------------------------

DROP TRIGGER IF EXISTS register_history_counts_debtor;
DROP TRIGGER IF EXISTS register_history_counts_nature;
DROP TRIGGER IF EXISTS register_history_counts_value;
DROP TRIGGER IF EXISTS register_history_counts_paid_installments;
DROP TRIGGER IF EXISTS register_history_counts_installments;
DROP TRIGGER IF EXISTS register_history_counts_proof;
DROP TRIGGER IF EXISTS register_history_counts_title;
DROP TRIGGER IF EXISTS register_history_counts_date_out;
DROP TRIGGER IF EXISTS register_history_counts_date_in;
DROP TRIGGER IF EXISTS register_history_counts_description;


CREATE TRIGGER register_history_counts_debtor
AFTER UPDATE OF debtor ON Counts
WHEN OLD.debtor <> NEW.debtor
BEGIN
    insert into Old_Counts(id_old_count, column, old_value, new_value)
    values ((select coalesce(max(id_old_count), 0)+1 from Old_Counts), 'debtor', OLD.debtor, NEW.debtor);

    insert into History (id_history, id_old_count, id_count, id_user, date)
    values((select coalesce(max(id_history), 0)+1 from history), (select max(id_old_count) from Old_Counts), new.id_count, new.id_user, CURRENT_DATE);
END;

CREATE TRIGGER register_history_counts_nature
AFTER UPDATE OF nature ON Counts
WHEN OLD.nature <> NEW.nature
BEGIN
    insert into Old_Counts(id_old_count, column, old_value, new_value)
    values ((select coalesce(max(id_old_count), 0)+1 from Old_Counts), 'nature', OLD.nature, NEW.nature);

    insert into History (id_history, id_old_count, id_count, id_user, date)
    values((select coalesce(max(id_history), 0)+1 from history), (select max(id_old_count) from Old_Counts), new.id_count, new.id_user, CURRENT_DATE);
END;

CREATE TRIGGER register_history_counts_value
AFTER UPDATE OF value ON Counts
WHEN OLD.value <> NEW.value
BEGIN
    insert into Old_Counts(id_old_count, column, old_value, new_value)
    values ((select coalesce(max(id_old_count), 0)+1 from Old_Counts), 'value', OLD.value, NEW.value);

    insert into History (id_history, id_old_count, id_count, id_user, date)
    values((select coalesce(max(id_history), 0)+1 from history), (select max(id_old_count) from Old_Counts), new.id_count, new.id_user, CURRENT_DATE);
END;

CREATE TRIGGER register_history_counts_paid_installments
AFTER UPDATE OF paid_installments ON Counts
WHEN OLD.paid_installments <> NEW.paid_installments
BEGIN
    insert into Old_Counts(id_old_count, column, old_value, new_value)
    values ((select coalesce(max(id_old_count), 0)+1 from Old_Counts), 'paid_installments', OLD.paid_installments, NEW.paid_installments);

    insert into History (id_history, id_old_count, id_count, id_user, date)
    values((select coalesce(max(id_history), 0)+1 from history), (select max(id_old_count) from Old_Counts), new.id_count, new.id_user, CURRENT_DATE);
END;

CREATE TRIGGER register_history_counts_installments
AFTER UPDATE OF installments ON Counts
WHEN OLD.installments <> NEW.installments
BEGIN
    insert into Old_Counts(id_old_count, column, old_value, new_value)
    values ((select coalesce(max(id_old_count), 0)+1 from Old_Counts), 'installments', OLD.installments, NEW.installments);

    insert into History (id_history, id_old_count, id_count, id_user, date)
    values((select coalesce(max(id_history), 0)+1 from history), (select max(id_old_count) from Old_Counts), new.id_count, new.id_user, CURRENT_DATE);
END;

CREATE TRIGGER register_history_counts_proof
AFTER UPDATE OF proof ON Counts
WHEN OLD.proof <> NEW.proof
BEGIN
    insert into Old_Counts(id_old_count, column, old_value, new_value)
    values ((select coalesce(max(id_old_count), 0)+1 from Old_Counts), 'proof', OLD.proof, NEW.proof);

    insert into History (id_history, id_old_count, id_count, id_user, date)
    values((select coalesce(max(id_history), 0)+1 from history), (select max(id_old_count) from Old_Counts), new.id_count, new.id_user, CURRENT_DATE);
END;

CREATE TRIGGER register_history_counts_title
AFTER UPDATE OF title ON Counts
WHEN OLD.title <> NEW.title
BEGIN
    insert into Old_Counts(id_old_count, column, old_value, new_value)
    values ((select coalesce(max(id_old_count), 0)+1 from Old_Counts), 'title', OLD.title, NEW.title);

    insert into History (id_history, id_old_count, id_count, id_user, date)
    values((select coalesce(max(id_history), 0)+1 from history), (select max(id_old_count) from Old_Counts), new.id_count, new.id_user, CURRENT_DATE);
END;

CREATE TRIGGER register_history_counts_date_out
AFTER UPDATE OF date_out ON Counts
WHEN OLD.date_out <> NEW.date_out
BEGIN
    insert into Old_Counts(id_old_count, column, old_value, new_value)
    values ((select coalesce(max(id_old_count), 0)+1 from Old_Counts), 'date_out', OLD.date_out, NEW.date_out);

    insert into History (id_history, id_old_count, id_count, id_user, date)
    values((select coalesce(max(id_history), 0)+1 from history), (select max(id_old_count) from Old_Counts), new.id_count, new.id_user, CURRENT_DATE);
END;

CREATE TRIGGER register_history_counts_date_in
AFTER UPDATE OF date_in ON Counts
WHEN OLD.date_in <> NEW.date_in
BEGIN
    insert into Old_Counts(id_old_count, column, old_value, new_value)
    values ((select coalesce(max(id_old_count), 0)+1 from Old_Counts), 'date_in', OLD.date_in, NEW.date_in);

    insert into History (id_history, id_old_count, id_count, id_user, date)
    values((select coalesce(max(id_history), 0)+1 from history), (select max(id_old_count) from Old_Counts), new.id_count, new.id_user, CURRENT_DATE);
END;

CREATE TRIGGER register_history_counts_description
AFTER UPDATE OF description ON Counts
WHEN OLD.description <> NEW.description
BEGIN
    insert into Old_Counts(id_old_count, column, old_value, new_value)
    values ((select coalesce(max(id_old_count), 0)+1 from Old_Counts), 'description', OLD.description, NEW.description);

    insert into History (id_history, id_old_count, id_count, id_user, date)
    values((select coalesce(max(id_history), 0)+1 from history), (select max(id_old_count) from Old_Counts), new.id_count, new.id_user, CURRENT_DATE);
END;
    "
}
// fn get_create_functions() -> &'static str{}
fn get_create_views() -> &'static str {
    "
    DROP VIEW IF EXISTS gen_uid;
    CREATE VIEW gen_uid AS
        select lower(hex( randomblob(4)) || '-' || hex( randomblob(2))
            || '-' || '4' || substr( hex( randomblob(2)), 2) || '-'
            || substr('AB89', 1 + (abs(random()) % 4) , 1)  ||
            substr(hex(randomblob(2)), 2) || '-' || hex(randomblob(6)));

    "
}
