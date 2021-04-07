import React from 'react';
import App from './components/app';
import store from './store/index.js';
import { render } from 'react-dom';
import { Provider } from 'react-redux';
import { Router } from 'react-router-dom';
import { ThemeProvider } from '@material-ui/core/styles';
import history from './services/history';
import darkTheme from './themes/dark';
import CssBaseline from '@material-ui/core/CssBaseline';


window.store = store;

render (
  <Provider store={store}>
    <ThemeProvider theme={darkTheme}>
      <Router history={history}>
        <CssBaseline>
          <App />
        </CssBaseline>
      </Router>
    </ThemeProvider>
  </Provider>,
  document.getElementById('root')
);
