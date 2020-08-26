import React from 'react';
import Main from './main';
import Footer from './footer';
import Menu from './menu';

const App = () => {
  return (
      <Menu>
          <Main />
          <Footer />
      </Menu>
  );
};

export default App;