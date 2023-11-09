const { getDatabaseClient } = require('./database-client');

const TABLENAME = 'persons';
const COLUMNS_ID = 'id';
const COLUMNS_NAME = 'name';

const ensureTableAndGetClient = async () => {  
  const client = await getDatabaseClient();

  await client.query(`
CREATE TABLE IF NOT EXISTS ${TABLENAME} (
    ${COLUMNS_ID} SERIAL PRIMARY KEY,
    ${COLUMNS_NAME} VARCHAR(255)
);

DO $$
BEGIN
    IF (SELECT count(*) FROM ${TABLENAME}) = 0 THEN
        INSERT INTO ${TABLENAME} (${COLUMNS_NAME}) VALUES ('John');
        INSERT INTO ${TABLENAME} (${COLUMNS_NAME}) VALUES ('Jane');
    END IF;
END $$;
`);

  return client;
}

const getAllPersons = async () => {
  const client = await ensureTableAndGetClient();

  const result = await client.query(`SELECT * FROM ${TABLENAME};`);

  client.release();

  return result.rows;
};

module.exports = {
  getAllPersons,
}