import React from 'react';
import AppBar from '@material-ui/core/AppBar';
import Toolbar from '@material-ui/core/Toolbar';
import Typography from '@material-ui/core/Typography';
import OctopusLogo from '../assets/OctopusLogo';
import { makeStyles } from '@material-ui/core/styles';

const useStyles = makeStyles(theme => ({
  title: {
    paddingLeft: 20,
  },
}));

const Nav = () => {
  const classes = useStyles();
  return (
    <AppBar position='relative'>
      <Toolbar align='center'>
        <OctopusLogo height={35} width={35}/>
        <Typography variant='h6' color='inherit' noWrap className={classes.title}>
          Patchup
        </Typography>
      </Toolbar>
    </AppBar>
  );
};

export default Nav;