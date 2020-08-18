const express = require('express');
// const auth = require('../middleware/auth');

const uploadRouter = express.Router();
const uploadController = require('../controllers/upload');

// create a new user
uploadRouter.get('/presigned', uploadController.presigned);

module.exports = uploadRouter;