import { useState } from 'react';

// Custom hook for counter
export function useCounter(initialValue: number = 0): [number, () => void, () => void] {
  const [counter, setCounter] = useState(initialValue);

  function handleIncrement() {
    setCounter(counter + 1);
  }

  function handleDecrement() {
    setCounter(counter <= 0 ? 0 : counter - 1);
  }

  return [counter, handleIncrement, handleDecrement];
}
