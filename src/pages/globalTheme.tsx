import { createTheme } from '@mui/material/styles';


const pickedColors = {
  eerieBlack: '#1C1D21',
  persianRed: '#BB4430',
  mintGreen: '#DDFFF7', // or rgb(221, 255, 247)
  peach: '#FFD3B5',
  caribbeanGreen: '#0F7173'
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
      default: 'rgba(221, 255, 247, 0.3)'
    }
  }
});
