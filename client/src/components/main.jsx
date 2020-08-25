import React from 'react';
import { Switch, Route } from 'react-router-dom';
import About from '../pages/about';
import Upload from '../pages/upload';
import Login from '../pages/login';
import SignUp from '../pages/signup';
import Dashboard from '../pages/dashboard';

import { makeStyles } from '@material-ui/core/styles';

const useStyles = makeStyles(theme => ({
  content: {
    flexGrow: 1,
    padding: theme.spacing(3),
  },
}));

const Main = () => {
  const classes = useStyles();

  return (
    <main className={classes.content}>
      <Switch>
        <Route exact path='/about' component={About} />
        <Route exact path='/upload' component={Upload} />
        <Route exact path='/login' component={Login} />
        <Route exact path='/signup' component={SignUp} />
        <Route exact path='/dashboard' component={Dashboard} />
      </Switch>
    </main>
  );
}

export default Main;
