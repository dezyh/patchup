const User = require('../models/user');

const createUser = async (req, res) => {
  try {
    const user = new User(req.body);
    await user.save();
    const token = await user.generateAuthToken();
    res.status(201).send({
      user: user.toObject(),
      token,
    });
  } catch (error) {
    if (error.code === 11000) {
      res.status(400).send({
        error: 'Username or email already exists.',
      });
    } else {
      res.status(400).send({
        error: 'An unknown error has occured.',
      });
    }
  }
};

const loginUser = async (req, res) => {
  try {
    const { username, password } = req.body;
    const user = await User.findByCredentials(username, password);
    if (!user) {
      return res.status(401).send({
        error: 'Incorrect username or password.',
      });
    }
    // generate a new token and respond with the token and user data
    const token = await user.generateAuthToken();
    res.send({
      user: user.toObject(),
      token,
    });
  } catch (error) {
    res.status(400).send({
      error: 'An unknown error as occured.',
    });
  }
};

const getCurrentUser = async (req, res) => {
  res.status(200).send({
    user: await req.user.toObject(),
  });
};

const logoutUser = async (req, res) => {
  try {
    req.user.tokens = req.user.tokens.filter(token => {
      return token.token !== req.token;
    });
    await req.user.save();
    res.status(200).send();
  } catch (error) {
    res.status(400).send({
      error: 'An unknown error as occured.',
    });
  }
};

const logoutUserAll = async (req, res) => {
  try {
    req.user.tokens.splice(0, req.user.tokens.length);
    await req.user.save();
    res.send();
  } catch (error) {
    res.status(400).send({
      error: 'An unknown error as occured.',
    });
  }
};

const getAllUsers = async (req, res) => {
  try {
    const allUsers = await User.find();
    res.status(200).send(allUsers.map(user => user.toObject()));
  } catch (error) {
    req.status(400).send({
      error: 'Error found',
    });
  }
};

module.exports = {
  createUser,
  loginUser,
  getCurrentUser,
  logoutUser,
  logoutUserAll,
  getAllUsers,
};
