const jwt = require('jsonwebtoken');
const User = require('../models/user');

// intercepts a request and checks if an authorized jwt was supplied
const auth = async (req, res, next) => {
  try {
    // extract just the token from the authorization header
    const token = req.header('Authorization').replace('Bearer ', '');
    // verify that the token was created by this server
    const data = jwt.verify(token, process.env.SECRET_KEY || process.env.NODE_ENV || 'development');
    // check if the user and token from the request match a user and token in the database
    const user = await User.findOne({ _id: data._id, 'tokens.token': token });
    if (!user) {
      throw new Error();
    }
    req.user = user;
    req.token = token;
    if (process.env.NODE_ENV === 'test') {
      await next(req, res);
    } else {
      next();
    }
  } catch (error) {
    res.status(401).send({ error: 'Not authorized to access this resource' });
  }
};

module.exports = auth;
