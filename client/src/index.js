import React from 'react';
import App from './components/app';
import store from './store/index.js';
import { render } from 'react-dom';
import { Provider } from 'react-redux';
import { Router } from 'react-router-dom';
import { ThemeProvider } from '@material-ui/core/styles';
import history from './services/history';
import darkTheme from './themes/dark';

window.store = store;

render (
  <Provider store={store}>
    <ThemeProvider theme={darkTheme}>
      <Router history={history}>
        <App />
      </Router>
    </ThemeProvider>
  </Provider>,
  document.getElementById('root')
);