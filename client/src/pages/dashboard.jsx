import React from 'react';

//import Typography from '@material-ui/core/Typography';
import Container from '@material-ui/core/Container';
//import Grid from '@material-ui/core/Grid';
import CssBaseline from '@material-ui/core/CssBaseline';

import  { makeStyles } from '@material-ui/core/styles';

const useStyles = makeStyles(theme => ({
  paper: {
    marginTop: theme.spacing(8),
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
  },
}));

const Dashboard = () => {
  const classes = useStyles();

  return (
    <div className={classes.paper}>
      <Container>
        left
      </Container>
      <Container>
        right
      </Container>
    </div>
  );
}

export default Dashboard;

