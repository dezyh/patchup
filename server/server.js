const express = require('express');

const router = require('./routes');
const database = require('./database');

const server = express();

server.use('/api', router);

const port = process.env.PORT || 8080;

server.listen(port, () => {
  console.log(`Port: ${port}`);
  console.log(`Build: ${process.env.NODE_ENV}`);
});


