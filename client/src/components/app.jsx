import React from 'react';
import CssBaseline from '@material-ui/core/CssBaseline';
import Header from './header';
import Main from './main';
import Footer from './footer';

const App = () => {
  return (
    <React.Fragment>
      <CssBaseline />
      <Header />
      <Main />
      <Footer />
    </React.Fragment>
  );
};

export default App;