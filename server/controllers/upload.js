const aws = require('aws-sdk');

aws.config.update({
  accessKeyId: 'patchup', 
  secretAccessKey: 'quilt',
  region: 'us-east-1'
});

const s3 = new aws.S3();

const presigned = async (req, res) => {
  try {
    const url = s3.getSignedUrl('putObject', {
      Bucket: 'test-bucket',
      Expires: 60*5,
      Key: 'testKey',
    });
    res.status(200).send(url);
  } catch (error) {
    console.log(error);
    res.status(400).send({
      error: 'An unknown error has occurred.',
    });
  }
}

module.exports = {
  presigned,
};
