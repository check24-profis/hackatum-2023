const express = require('express');
const app = express();
const { getAllPersons } = require('./src/persons-table');

app.get('/greetings', (_, res) => {
  res.send("Hello world");
});

app.get('/persons', async (_, res) => {
  const persons = await getAllPersons();
  res.send(persons);
});

const port = process.env['SERVER_PORT'] || 3000;

app.listen(port, () => {
  console.log(`Example app listening at http://localhost:${port}`);
});