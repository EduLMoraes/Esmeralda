SELECT * FROM users;

-- SELECT MAX((SELECT MAX(count_id) FROM counts WHERE user_id = $1)) AS max_id, count_id, user_id,
-- TO_CHAR(date_in, 'YYYY-MM-DD') AS date_in, TO_CHAR(date_out, 'YYYY-MM-DD') AS date_out, debtor, title, description, value, paid_installments, installments, status, nature
-- FROM counts
-- WHERE user_id = $1
-- AND
-- (
--     CAST(EXTRACT(YEAR FROM date_out) AS SMALLINT) = $2
--     OR
--     CAST(EXTRACT(YEAR FROM date_out) AS SMALLINT) >= $2
--     AND
--     CAST(EXTRACT(YEAR FROM date_in) AS SMALLINT) <= $2
-- )
-- GROUP BY count_id
-- ORDER BY count_id

-- SELECT 
-- DISTINCT CAST(EXTRACT(YEAR FROM date_out) AS CHAR(4)) as date_out
-- FROM counts 
-- WHERE
-- user_id = $1