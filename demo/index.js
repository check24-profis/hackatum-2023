const path = require('path');
const express = require('express');
const app = express();
const cors = require('cors');
const { getCraftsmen, updateRanking } = require('./src/simply-craftsmen-table');
const { getMappedCraftsman  } = require('./src/craftsmen-service');

app.use(cors());

app.get('/', (_, res) => {
  res.sendFile(path.join(__dirname, 'index.html'));
});

app.get('/craftsmen', async (req, res) => {
  // ideally should be validated & sanitzed.
  const { postalcode } = req.query;

  const craftsmen = await getCraftsmen(postalcode);

  return res.send({
    craftsmen: craftsmen.map(getMappedCraftsman)
  });
});

app.patch('/craftman/ranking', async (req, res) => {
  // ideally should be validated & sanitzed.
  const { id, ranking } = req.query;

  const updatedCraftsman = await updateRanking(+id, +ranking);

  res.send({
    updated: getMappedCraftsman(updatedCraftsman)
  });
});

const port = process.env['SERVER_PORT'] || 3000;

app.listen(port, () => {
  console.log(`Example app listening at http://localhost:${port}`);
});