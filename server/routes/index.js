const express = require('express');

const router = express.Router();

const userRouter = require('./user');
const uploadRouter = require('./upload');

router.use('/user', userRouter);
router.use('/upload', uploadRouter);

module.exports = router;