import { Button, ButtonGroup } from "@mui/material";
import { useState } from "react";

function Counter() {
  const [counter, updateCounter] = useState(0);

  function handleIncrement() {
    updateCounter(counter + 1);
  }

  function handleDecrement() {
    updateCounter(counter <= 0 ? 0 : counter - 1);
  }

  function handleReset() {
    updateCounter(0);
  }

  return (
    <div style={{
      textAlign: 'center',
    }}>
      <span>
        <ButtonGroup variant="outlined" aria-label="outlined button group">
          <Button onClick={handleDecrement}>-</Button>
          <Button disabled={true}>{counter}</Button>
          <Button onClick={handleIncrement}>+</Button>
        </ButtonGroup>
      </span>
    </div>
  );
}

export default Counter;