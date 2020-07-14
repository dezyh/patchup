import React from 'react';
import CssBaseline from '@material-ui/core/CssBaseline';
import DropzoneS3Uploader from 'react-dropzone-s3-uploader';

const Upload = () => {

  const handleFinishedUpload = info => {
    console.log('File uploaded with filename', info.filename);
    console.log('Access it on s3 at', info.fileUrl);
  };

  const uploadOptions = {
    server: 'http://localhost:3000',
  };
  const s3Url = 'http://patchup-user-uploads.s3-website-us-east-1.amazonaws.com';

  return (
    <React.Fragment>
      <CssBaseline />
      <main>
        <DropzoneS3Uploader
          onFinish={handleFinishedUpload}
          s3Url={s3Url}
          upload={uploadOptions}
        />
      </main>
    </React.Fragment>
  );
}

export default Upload;

