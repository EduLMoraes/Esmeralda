
DROP VIEW IF EXISTS gen_uid;
CREATE VIEW gen_uid AS
    select lower(hex( randomblob(4)) || '-' || hex( randomblob(2))
         || '-' || '4' || substr( hex( randomblob(2)), 2) || '-'
         || substr('AB89', 1 + (abs(random()) % 4) , 1)  ||
         substr(hex(randomblob(2)), 2) || '-' || hex(randomblob(6)))
