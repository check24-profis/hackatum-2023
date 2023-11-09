const express = require('express');
const app = express();

app.get('/greetings', (_, res) => {
  res.send("Hello World!");
});

const port = process.env['SERVER_PORT'] || 3000;

app.listen(port, () => {
  console.log(`Example app listening at http://localhost:${port}`);
});