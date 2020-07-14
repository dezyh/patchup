import React from 'react';
import { Switch, Route } from 'react-router-dom';
import About from './About';
import Upload from './Upload';

const Main = () => {
  return (
    <main>
      <Switch>
        <Route exact path = '/' component={About} />
        <Route exact path = '/upload' component={Upload} />
      </Switch>
    </main>
  );
}

export default Main;
