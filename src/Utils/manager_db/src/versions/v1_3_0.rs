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
    id_SES BIGSERIAL PRIMARY KEY,
    value float
);

CREATE TABLE IF NOT EXISTS Old_Counts (
    id_old_count BIGSERIAL PRIMARY KEY,
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
    id_fii BIGSERIAL PRIMARY KEY,
    last_yields_PK integer REFERENCES last_yields(last_yields_PK),
    dates_yield_PK integer REFERENCES dates_yield(dates_yield_PK),
    value float,
    dividend_yield float
);

CREATE TABLE IF NOT EXISTS Counts (
    id_count INTEGER,
    id_user integer,
    paid_installments integer DEFAULT 1 CHECK(paid_installments >= 0),
    installments integer DEFAULT 1 CHECK(installments >= 1),
    debtor varchar(100) NOT NULL,
    value float CHECK(value > 0.00),
    title varchar(50),
    date_out date not null,
    date_in date not null,
    proof blob,
    nature varchar(50) not null,
    description text,
    FOREIGN KEY (id_user) REFERENCES Users
);


CREATE TABLE IF NOT EXISTS History (
    id_history BIGSERIAL PRIMARY KEY,
    id_old_count integer REFERENCES Old_Counts(id_old_count),
    id_count integer,
    id_user integer,
    date datetime,
    FOREIGN KEY (id_count, id_user) REFERENCES Counts(id_count, id_user),
    FOREIGN KEY (id_user) REFERENCES Users(id_user)
);

CREATE TABLE IF NOT EXISTS Address (
    id_addres BIGSERIAL PRIMARY KEY,
    uid_property varchar(44) NULL,
    type varchar(30),
    public_place varchar(200),
    number integer,
    complement varchar(200),
    neighborhood varchar(200),
    district varchar(50),
    city varchar(70),
    state varchar(30),
    FOREIGN KEY (uid_property) REFERENCES Property(uid_property)
);

CREATE TABLE IF NOT EXISTS People (
    uid_people varchar(44) PRIMARY KEY,
    id_addres integer,
    id_user integer,
    provider varchar(44),
    wage float,
    name varchar(50),
    date_of_birth date,
    cell_phone char(12),
    voter_registration varchar(12),
    rg char(10),
    cpf char(11),
    surname varchar(100),
    FOREIGN KEY (id_addres) REFERENCES Address(id_addres),
    FOREIGN KEY (id_user) REFERENCES Users(id_user)
);

CREATE TABLE IF NOT EXISTS Investments (
    id_invest BIGSERIAL PRIMARY KEY,
    uid_people varchar(44),
    type_invest varchar(30),
    value_apply float,
    redeption_value float,
    date_apply date,
    redeption_date date,
    investment varchar(100),
    cnpj char(14),
    FOREIGN KEY (uid_people) REFERENCES People(uid_people)
);

CREATE TABLE IF NOT EXISTS Bank (
    uid_bank varchar(44) PRIMARY KEY,
    uid_people varchar(44),
    name varchar(50),
    code text,
    agency_number integer,
    FOREIGN KEY (uid_people) REFERENCES People(uid_people)
);

CREATE TABLE IF NOT EXISTS Receipts (
    uid_receipt varchar(44) PRIMARY KEY,
    uid_property varchar(44),
    type varchar(30),
    document blob,
    FOREIGN KEY (uid_property) REFERENCES Property(uid_property)
);

CREATE TABLE IF NOT EXISTS Goal (
    uid varchar(44) PRIMARY KEY,
    id_user integer,
    title varchar(30),
    description varchar(255),
    image blob,
    time date,
    start datetime,
    amount float,
    desired_value float,
    achivied boolean,
    FOREIGN KEY (id_user) REFERENCES Users(id_user)
);

 "
}

fn get_create_triggers() -> &'static str {
    "
    -- Active: 1718076179564@@127.0.0.1@3306

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
END;

CREATE TRIGGER register_history_counts_nature
AFTER UPDATE OF nature ON Counts
WHEN OLD.nature <> NEW.nature
BEGIN
    insert into Old_Counts(id_old_count, column, old_value, new_value)
    values ((select coalesce(max(id_old_count), 0)+1 from Old_Counts), 'nature', OLD.nature, NEW.nature);
END;

CREATE TRIGGER register_history_counts_value
AFTER UPDATE OF value ON Counts
WHEN OLD.value <> NEW.value
BEGIN
    insert into Old_Counts(id_old_count, column, old_value, new_value)
    values ((select coalesce(max(id_old_count), 0)+1 from Old_Counts), 'value', OLD.value, NEW.value);
END;

CREATE TRIGGER register_history_counts_paid_installments
AFTER UPDATE OF paid_installments ON Counts
WHEN OLD.paid_installments <> NEW.paid_installments
BEGIN
    insert into Old_Counts(id_old_count, column, old_value, new_value)
    values ((select coalesce(max(id_old_count), 0)+1 from Old_Counts), 'paid_installments', OLD.paid_installments, NEW.paid_installments);
END;

CREATE TRIGGER register_history_counts_installments
AFTER UPDATE OF installments ON Counts
WHEN OLD.installments <> NEW.installments
BEGIN
    insert into Old_Counts(id_old_count, column, old_value, new_value)
    values ((select coalesce(max(id_old_count), 0)+1 from Old_Counts), 'installments', OLD.installments, NEW.installments);
END;

CREATE TRIGGER register_history_counts_proof
AFTER UPDATE OF proof ON Counts
WHEN OLD.proof <> NEW.proof
BEGIN
    insert into Old_Counts(id_old_count, column, old_value, new_value)
    values ((select coalesce(max(id_old_count), 0)+1 from Old_Counts), 'proof', OLD.proof, NEW.proof);
END;

CREATE TRIGGER register_history_counts_title
AFTER UPDATE OF title ON Counts
WHEN OLD.title <> NEW.title
BEGIN
    insert into Old_Counts(id_old_count, column, old_value, new_value)
    values ((select coalesce(max(id_old_count), 0)+1 from Old_Counts), 'title', OLD.title, NEW.title);
END;

CREATE TRIGGER register_history_counts_date_out
AFTER UPDATE OF date_out ON Counts
WHEN OLD.date_out <> NEW.date_out
BEGIN
    insert into Old_Counts(id_old_count, column, old_value, new_value)
    values ((select coalesce(max(id_old_count), 0)+1 from Old_Counts), 'date_out', OLD.date_out, NEW.date_out);
END;

CREATE TRIGGER register_history_counts_date_in
AFTER UPDATE OF date_in ON Counts
WHEN OLD.date_in <> NEW.date_in
BEGIN
    insert into Old_Counts(id_old_count, column, old_value, new_value)
    values ((select coalesce(max(id_old_count), 0)+1 from Old_Counts), 'date_in', OLD.date_in, NEW.date_in);
END;

CREATE TRIGGER register_history_counts_description
AFTER UPDATE OF description ON Counts
WHEN OLD.description <> NEW.description
BEGIN
    insert into Old_Counts(id_old_count, column, old_value, new_value)
    values ((select coalesce(max(id_old_count), 0)+1 from Old_Counts), 'description', OLD.description, NEW.description);
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
