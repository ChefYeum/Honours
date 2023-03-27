import { createTheme } from '@mui/material/styles';


const pickedColors = {
  eerieBlack: '#1C1D21',
  persianRed: '#BB4430',
}

export const globalTheme = createTheme({

  palette: {
    primary: {
      main: pickedColors.persianRed
    },
    secondary: {
      main: pickedColors.eerieBlack,
    },
    background: {
      // default: 'rgba(221, 255, 247, 0.3)'
      default: 'rgb(245,247,220, 0.66)'
    }
  }
});
