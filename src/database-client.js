const { Client } = require('pg');

let getClientPromise;

const getDatabaseClient = () => {
  if(getClientPromise) {
    return getClientPromise;
  }

  const client = new Client({
    host: process.env['DB_HOSTNAME'] || 'localhost',
    database: process.env['DB_DATABASE_NAME'] || 'postgres',
  });

  getClientPromise = client.connect().then(() => client);

  return getClientPromise;
};

module.exports = {
  getDatabaseClient
};