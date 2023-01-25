import { Box, Button, ButtonGroup } from "@mui/material";
import { useState } from "react";

function Counter() {
  const [counter, updateCounter] = useState(0);

  function handleIncrement() {
    updateCounter(counter + 1);
  }

  function handleDecrement() {
    updateCounter(counter <= 0 ? 0 : counter - 1);
  }

  const third = 'calc(100% / 3)'

  return (
    <div style={{
      textAlign: 'center',
      height: '4em'
    }}>
      <ButtonGroup
        // variant="outlined"
        // aria-label="outlined button group"
        fullWidth={true} style={{ height: '100%' }}>
        <Button style={{ width: third }} onClick={handleDecrement}>-</Button>
        <Box
          component="span"
          sx={{ width: third }}>
          {counter}
        </Box>
        <Button style={{ width: third }} onClick={handleIncrement}>+</Button>
      </ButtonGroup>
    </div>
  );
}

export default Counter;