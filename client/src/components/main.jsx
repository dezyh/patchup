import React from 'react';
import { Switch, Route } from 'react-router-dom';
import About from '../pages/about';
import Update from '../pages/update';
import Login from '../pages/login';
import SignUp from '../pages/signup';
import Dashboard from '../pages/dashboard';
// import { useSelector } from 'react-redux';

const Main = () => {
  //const auth = useSelector(store => store.auth);
  const auth = {token: null};

  return (
    <React.Fragment>
        <Switch>
            <Route exact path='/' component={auth.token ? Dashboard : About} />
            <Route exact path='/login' component={Login} />
            <Route exact path='/signup' component={SignUp} />
            <Route exact path='/dashboard' component={Dashboard} />
            <Route exact path='/update' component={Update} />
        </Switch>
    </React.Fragment>
  );
}

export default Main;
