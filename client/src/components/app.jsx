import React from 'react';
import CssBaseline from '@material-ui/core/CssBaseline';
import Main from './main';
import Footer from './footer';
import Menu from './menu';

// const useStyles = makeStyles(theme => ({
//   root
// }))

const App = () => {
  return (
    <React.Fragment>
      <CssBaseline />
      <Menu />
      <Main />
      <Footer />
    </React.Fragment>
  );
};

export default App;