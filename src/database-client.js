const { Pool } = require('pg');

const pool = new Pool({
  host: process.env['DB_HOSTNAME'],
  database: process.env['DB_DATABASE_NAME'],
  user: "postgres"
});

const getDatabaseClient = async () => await pool.connect();

module.exports = {
  getDatabaseClient
};