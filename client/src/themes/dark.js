import { createMuiTheme } from '@material-ui/core/styles';

const darkTheme = createMuiTheme({
  palette: {
    type: 'dark',
    background: {
      default: '#1c2025',
      paper: '#282c34',
      menu: '#282c34'
    },
    primary: {
      main: '#ff5426',
      // light: auto generated from main
      // dark: auto generated from main
      contrastText: '#ffffff',
    },
  },
});

export default darkTheme;