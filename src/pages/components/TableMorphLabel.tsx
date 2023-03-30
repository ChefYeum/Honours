import React from 'react';
import { Typography } from '@mui/material';

function TableMorphLabel({ children }: { children: React.ReactNode }) {
  return (
    <Typography
      component="span"
      variant="h6"
      sx={{
        fontWeight: 'bold',
      }}
    >
      {children}
    </Typography>
  );
}

const F = (props: { index: number | null; }) => {
  if (props.index === null) {
    return (
      <TableMorphLabel>
        -
      </TableMorphLabel>
    );
  }
  return (
    <TableMorphLabel>
      f
      <Typography
        component="span"
        variant="subtitle1"
        sx={{
          verticalAlign: 'sub',
          fontWeight: 'bold',
        }}
      >
        {props.index}
      </Typography>
    </TableMorphLabel>
  );
};


export { TableMorphLabel as MorphLabel, F };