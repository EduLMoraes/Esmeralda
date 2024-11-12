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