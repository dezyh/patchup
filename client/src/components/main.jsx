import React from 'react';
import { Switch, Route } from 'react-router-dom';
import About from '../pages/about';
import Upload from '../pages/upload';
import Login from '../pages/login';
import SignUp from '../pages/signup';

const Main = () => {
  return (
    <main>
      <Switch>
        <Route exact path = '/about' component={About} />
        <Route exact path = '/upload' component={Upload} />
        <Route exact path = '/login' component={Login} />
        <Route exact path = '/signup' component={SignUp} />
      </Switch>
    </main>
  );
}

export default Main;
