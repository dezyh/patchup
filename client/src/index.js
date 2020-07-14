import React from 'react';
import './css/index.css';
import App from './components/App';
import store from './store/index.js';
import { render } from 'react-dom';
import { Provider } from 'react-redux';
import { Router } from 'react-router-dom';
import history from './services/history';

window.store = store;

render (
  <Provider store={store}>
    <Router history={history}>
      <App />
    </Router>
  </Provider>,
  document.getElementById('root')
);