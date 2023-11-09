const { getDatabaseClient } = require('./database-client');

const tableName = 'timestamps';
const columnsNames = {
  id: 'id',
  createdAt: 'created_at',
};

let ensureTablePromise;
const ensureTable = () => {  
  if (ensureTablePromise) {
    return ensureTablePromise;
  }

  ensureTablePromise = getDatabaseClient().then((client) => client.query(`
CREATE TABLE IF NOT EXISTS ${tableName} (
    ${columnsNames.id} SERIAL PRIMARY KEY,
    ${columnsNames.createdAt} TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
`));

  return ensureTablePromise;
}

const getAllTimestamps = async () => {
  await ensureTable();
  const client = await getDatabaseClient();

  const result = await client.query(`SELECT * FROM ${tableName};`);

  return result.rows;
};

const addCurrentTimestamp = async () => {
  await ensureTable();
  const client = await getDatabaseClient();

  const result = await client.query(`INSERT INTO ${tableName}(${columnsNames.createdAt}) VALUES (DEFAULT) RETURNING ${columnsNames.id}, ${columnsNames.createdAt};`);

  return {
    [columnsNames.id]: result.rows[0][columnsNames.id],
    [columnsNames.createdAt]: result.rows[0][columnsNames.createdAt]
  };
};

module.exports = {
  getAllTimestamps,
  addCurrentTimestamp,
}