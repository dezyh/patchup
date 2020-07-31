const express = require('express');

const server = express();

const port = process.env.PORT || 8080;

server.listen(port, () => {
  console.log(`Port: ${port}`);
  console.log(`Build: ${process.env.NODE_ENV}`);
})
