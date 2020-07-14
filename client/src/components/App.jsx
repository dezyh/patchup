import React from 'react';
import CssBaseline from '@material-ui/core/CssBaseline';
import Header from './Header';
import Main from './Main';
import Footer from './Footer';

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