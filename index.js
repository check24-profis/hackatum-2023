const express = require('express');
const app = express();
const { getAllTimestamps, addCurrentTimestamp } = require('./src/database-timestamp-table');

// Get all timestamps added
app.get('/timestamps', async (_, res) => {
  const result = await getAllTimestamps();
  res.send(result);
});

// Add current timestamp to database
app.post('/current-timestamp', async (_, res) => {
  const result = await addCurrentTimestamp();
  res.send(result);
});

const port = process.env['SERVER_PORT'] || 3000;

app.listen(port, () => {
  console.log(`Example app listening at http://localhost:${port}`);
});