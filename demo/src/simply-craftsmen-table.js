const { getDatabaseClient } = require('./database-client');

const TABLENAME = 'craftsmen';
const COLUMNS_ID = 'id';
const COLUMNS_FIRSTNAME = 'first_name';
const COLUMNS_LASTNAME = 'last_name';
const COLUMNS_POSTALCODE = 'postalcode';
const COLUMNS_RANKING_CORE = 'ranking_score';

const ensureTableAndGetClient = async () => {  
  const client = await getDatabaseClient();

  await client.query(`
CREATE TABLE IF NOT EXISTS ${TABLENAME} (
    ${COLUMNS_ID} SERIAL PRIMARY KEY,
    ${COLUMNS_FIRSTNAME} VARCHAR(255),
    ${COLUMNS_LASTNAME} VARCHAR(255),
    ${COLUMNS_POSTALCODE} VARCHAR(255),
    ${COLUMNS_RANKING_CORE} DECIMAL(4, 2)
);

DO $$
BEGIN
    IF (SELECT count(*) FROM ${TABLENAME}) = 0 THEN
        INSERT INTO ${TABLENAME} (${COLUMNS_FIRSTNAME}, ${COLUMNS_LASTNAME}, ${COLUMNS_POSTALCODE}, ${COLUMNS_RANKING_CORE}) VALUES ('Leon', 'Ramos','80686', 5.46);
        INSERT INTO ${TABLENAME} (${COLUMNS_FIRSTNAME}, ${COLUMNS_LASTNAME}, ${COLUMNS_POSTALCODE}, ${COLUMNS_RANKING_CORE}) VALUES ('Constant', 'Woolery','80687', 3.27);
        INSERT INTO ${TABLENAME} (${COLUMNS_FIRSTNAME}, ${COLUMNS_LASTNAME}, ${COLUMNS_POSTALCODE}, ${COLUMNS_RANKING_CORE}) VALUES ('Eva', 'Malone', '80689', 4.43);
    END IF;
END $$;
`);

  return client;
}

const getCraftsmen = async (postalCode) => {
  const client = await ensureTableAndGetClient();

  const query = !!postalCode ?
    `SELECT * FROM ${TABLENAME} WHERE ${COLUMNS_POSTALCODE}='${postalCode}';`:
    `SELECT * FROM ${TABLENAME};`;
    
  const result = await client.query(query);
  client.release();

  return result.rows;
};

const updateRanking = async (craftsmanId, ranking) => {
  const client = await ensureTableAndGetClient();
  const result = await client.query(`UPDATE ${TABLENAME} SET ${COLUMNS_RANKING_CORE}=${ranking} WHERE ${COLUMNS_ID}=${craftsmanId} RETURNING *;`);
  
  client.release();

  return result.rows[0];
};

module.exports = {
  getCraftsmen,
  updateRanking
}