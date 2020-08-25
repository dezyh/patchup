import React from 'react';
import CssBaseline from '@material-ui/core/CssBaseline';
import DropzoneS3Uploader from 'react-dropzone-s3-uploader';
import { Typography } from '@material-ui/core';

const Upload = () => {
  
  const handleFinishedUpload = (info) => {
    console.log(info);
    console.log('File uploaded with filename', info.filename)
    console.log('Access it on s3 at', info.fileUrl)
  }

  const url = 'http://localhost:4566/test-bucket';
  
  const options = {
    server: 'http://localhost:3000',
    signingUrl: '/api/upload/presigned',
    signingUrlMethod: 'GET',
    uploadRequestHeaders: {'Access-Control-Allow-Headers': '*'},
  };

  return (
    <React.Fragment>
      <CssBaseline />
      <Typography>test</Typography>
      <DropzoneS3Uploader
        s3Url={url}
        upload={options}
        onFinish={handleFinishedUpload}
      />
    </React.Fragment>
  );
}

export default Upload;
