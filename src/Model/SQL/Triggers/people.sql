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
