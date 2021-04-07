import React from 'react';
// import { makeStyles, useTheme } from '@material-ui/core/styles';
import AppBar from '@material-ui/core/AppBar';
import Toolbar from '@material-ui/core/Toolbar';
import Typography from '@material-ui/core/Typography';
import Button from '@material-ui/core/Button';
// import { Link } from 'react-router-dom';

//const useStyles = makeStyles((theme) => ({
//}));

const User = () => {
  const auth = { token: null };
  
  if (auth.token) {
    return (
      <Typography>
        Logged In
      </Typography>
    );
  } else {
    return (
      <div>
        <Button>
          Sign Up
        </Button>
        <Button>
          Login
        </Button>
      </div>
    );
  }
}


const Nav = () => {
    //const classes = useStyles();

  return (
    <AppBar position='flex'>
      <Toolbar>
        <Typography variant='h6'>
          Patchup
        </Typography>
        <User/>      
      </Toolbar>
    </AppBar>
  );
}

export { Nav };
