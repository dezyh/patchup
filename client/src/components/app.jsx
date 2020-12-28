import React from 'react';
import Main from './main';
import Footer from './footer';
import { Nav } from './nav';

const App = () => {
  return (
    <Nav>
      <Main />
      <Footer />
    </Nav>
  );
}

export default App;
