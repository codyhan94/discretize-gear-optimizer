/* eslint-disable react/jsx-filename-extension */

import React from 'react';
import { Provider } from 'react-redux';
import { ThemeProvider } from 'gw2-ui-bulk';
import baseTheme from './src/styles/baseTheme';

import 'typeface-menomonia';

import createStore from './src/state/createStore';

const store = createStore();

// eslint-disable-next-line import/prefer-default-export
export const wrapRootElement = ({ element }) => (
  <Provider store={store}>
    <ThemeProvider theme={baseTheme}>{element}</ThemeProvider>
  </Provider>
);
