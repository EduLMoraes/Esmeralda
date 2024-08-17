-- Active: 1722366331226@@127.0.0.1@3306
SELECT * FROM users;

SELECT MAX((SELECT MAX(count_id) FROM counts WHERE user_id = 2)) AS max_id, count_id, user_id,
strftime('%Y-%m-%d', date_in) AS date_in, strftime('%Y-%m-%d', date_out) AS date_out, debtor, title, description, value, paid_installments, installments, status, nature
FROM counts
WHERE user_id = 2
AND
(
    CAST(EXTRACT(YEAR FROM date_out) AS SMALLINT) = 2024
    OR
    CAST(EXTRACT(YEAR FROM date_out) AS SMALLINT) >= 2024
    AND
    CAST(EXTRACT(YEAR FROM date_in) AS SMALLINT) <= 2024
)
GROUP BY count_id
ORDER BY count_id

SELECT 
DISTINCT CAST(EXTRACT(YEAR FROM date_out) AS CHAR(4)) as date_out
FROM counts 
WHERE
user_id = 2


SELECT * FROM counts where user_id = 2;
select max(substr(count_id, 2)+1) from counts where user_id = 2;
SELECT user_id || max(substr(count_id, 2)) from counts where user_id = 2;
select (user_id || (max(substr(count_id, 2))+1)) AS count_id2 from counts where user_id = 4;

select
    (
        select user_id from users where user_id = 4
    ) || coalesce((
        max(
            cast(substr(count_id, 2) as int)
        +1)
    ), 1)
from counts 
where user_id = 4

select count_id, user_id, strftime('%Y-%m-%d', date_in) AS date_in, strftime('%Y-%m-%d', date_out) AS date_out, debtor, title,
description, value, paid_installments, installments, status, nature
from counts 
where user_id = '4' 
    and ( date_in like '2024%'
         or date_out like '2024%')
ORDER BY count_id;

select * from counts;
select cast(substr(count_id, 2) as int), * from counts;

select count_id from counts;

select
    (select user_id from users where user_id = 4) || coalesce((
        max(
            substr(count_id, 2)
        +1)
    ), 1)
from counts 
where user_id = 4;

select user_id || coalesce(max(substr(count_id, 2)), 0) + 1 from counts where user_id = 4;

SELECT debtor, user_id FROM counts
UNION
SELECT name, user_id FROM users;

SELECT debtor, user_id FROM counts
UNION ALL
SELECT name, user_id FROM users;


SELECT user_id, name, username, password, email, coalesce(strftime('%m', last_login), '') as last_login FROM users WHERE username = 'teste' LIMIT 1;

SELECT nature FROM counts WHERE user_id = 4;