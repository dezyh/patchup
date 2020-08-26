import React from 'react';
import Main from './main';
import Footer from './footer';
import { Menu, MenuAuth } from './menu';

const App = () => {

  const auth = { token: null };

  if (auth.token) {
    return (
      <MenuAuth>
        <Main />
        <Footer />
      </MenuAuth>
    );
  } else {
    return (
      <Menu>
        <Main />
        <Footer />
      </Menu>
    );
  }
}

export default App;