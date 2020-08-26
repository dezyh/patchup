import React from 'react';
import { Switch, Route } from 'react-router-dom';
import About from '../pages/about';
import Update from '../pages/update';
import Login from '../pages/login';
import SignUp from '../pages/signup';
import Dashboard from '../pages/dashboard';
import Test from '../pages/test';

const Main = () => {
  return (
    <React.Fragment>
        <Switch>
            <Route exact path='/about' component={About} />
            <Route exact path='/login' component={Login} />
            <Route exact path='/signup' component={SignUp} />
            <Route exact path='/dashboard' component={Dashboard} />
            <Route exact path='/update' component={Update} />
            <Route exact path='/test' component={Test} />
        </Switch>
    </React.Fragment>
  );
}

export default Main;
