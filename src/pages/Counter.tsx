import { Box, Button, ButtonGroup, useTheme } from "@mui/material";
import { useState } from "react";

function Counter(
  props: {count: number, plus: () => void, minus: () => void}
) {  
  const theme = useTheme();

  const { count, plus, minus } = props;

  const buttonWidth = 'calc(100% / 3)';

  return (
    <div style={{
      textAlign: 'center',
      height: '4em',
      color: theme.palette.primary.main,
      paddingTop: '0.5em',
      paddingBottom: '0.5em',
    }}>
      <ButtonGroup fullWidth style={{ height: '100%' }}>
        <Button
          style={{ width: buttonWidth }}
          onClick={minus}
        >
          -
        </Button>
        <Box
          component="span"
          sx={{ width: buttonWidth }}
          style={{
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
          }}
        >
          {count}
        </Box>
        <Button
          style={{ width: buttonWidth }}
          onClick={plus}
        >
          +
        </Button>
      </ButtonGroup>
    </div>
  );
}

export default Counter;
