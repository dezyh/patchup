const mongoose = require('mongoose');

const collection = process.env.NODE_ENV || 'development';
const hostname = process.env.MONGO_HOST || 'localhost';
const port = process.env.MONGO_POST || 27017;


const connection = `mongodb://${hostname}:${port}/${collection}`;

const options = {
  useNewUrlParser: true,
  useCreateIndex: true,
  useUnifiedTopology: true,
};

mongoose.connect(connection, options).then(
  () => {
    console.log('Database: Connected');
  },
  err => {
    console.log('Database: Connection Error');
  }
);
