CREATE TABLE IF NOT EXISTS Users (
    id_user SERIAL PRIMARY KEY,
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
    id_history BIGSERIAL PRIMARY KEY,
    id_old_count integer REFERENCES Old_Counts(id_old_count) ON DELETE CASCADE,
    id_count integer REFERENCES Counts(id_count) ON DELETE CASCADE,
    id_user integer REFERENCES Users(id_user) ON DELETE CASCADE,
    date timestamp
);
CREATE TABLE IF NOT EXISTS Address (
    id_addres BIGSERIAL PRIMARY KEY,
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
    id_addres integer REFERENCES Address(id_addres) ON DELETE CASCADE,
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
    id_invest BIGSERIAL PRIMARY KEY,
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