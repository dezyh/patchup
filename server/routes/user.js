const express = require('express');
const auth = require('../middleware/auth');

const userRouter = express.Router();
const userController = require('../controllers/user');

// create a new user
userRouter.post('/signup', userController.createUser);

// login to an existing user
userRouter.post('/login', userController.loginUser);

// get the current user
userRouter.get('/me', auth, userController.getCurrentUser);

// logout the current user on the current device
userRouter.post('/logout', auth, userController.logoutUser);

// logout the current user on all devices
userRouter.post('/logout/all', auth, userController.logoutUserAll);

// get all users
userRouter.get('/', userController.getAllUsers);

module.exports = userRouter;