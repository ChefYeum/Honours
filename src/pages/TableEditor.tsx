import React from 'react';
import { Box, MenuItem, Select, Table, TableBody, TableCell, TableHead, TableRow, Typography } from '@mui/material';
import { useTheme, Button } from '@mui/material';

const F = (props: { index: number }) => {
  return (
    <Typography
      component="span"
      variant="h6"
      sx={{
        fontWeight: 'bold',
      }}
    >
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
    </Typography>
  )
};



const cellStyles = {
  borderBottom: '1px solid #ddd',
  padding: '0',
  width: '64px',
  height: '64px',
  lineHeight: '64px',
  boxSizing: 'border-box',
  verticalAlign: 'middle',
  alignItems: 'center',
  textAlign: 'center',
}


const TableMorphSelect = (props: React.PropsWithChildren<{ n: number }>) => (
  <TableCell sx={cellStyles}>
    <Select
      sx={{
        display: 'flex',
        flexDirection: 'row',
        borderRadius: 0,
        width: '100%',
        height: '100%',
        border: 'none',
        backgroundColor: 'transparent',
        alignItems: 'center',
        textAlign: 'center',
        '& .MuiSelect-icon': {
          display: 'none',
        },
        '&:focus': {
          backgroundColor: 'transparent',
        },
        '& .MuiSelect-select.MuiSelect-select': {
          paddingRight: 0,
          paddingLeft: 0,
        },
      }}
      defaultValue={'0'}
      MenuProps={{
        anchorOrigin: {
          vertical: 'bottom',
          horizontal: 'left',
        },
        transformOrigin: {
          vertical: 'top',
          horizontal: 'left',
        },
        // getContentAnchorEl: null,
      }}
    >
      {[...Array(props.n)].map((_, index) => (
        <MenuItem key={index} value={index} sx={{ justifyContent: 'center' }}>
          <F index={index} />
        </MenuItem>
      ))}
    </Select>
  </TableCell>
);

const TableEditor = (props: { n: number }) => {
  const theme = useTheme();

  const headerStyles = {
    ...cellStyles,
    borderBottom: '2px solid #ddd',
    backgroundColor: theme.palette.primary.main,
  }

  const { n } = props;
  const TableColHead = (props: { rowIndex: number }) => (
    <TableCell sx={headerStyles}>
      <F index={props.rowIndex} />
    </TableCell>
  );


  const rows = [...Array(n)].map((_, rowIndex) => (
    <TableRow key={rowIndex}>
      {<TableColHead rowIndex={rowIndex} />}
      {[...Array(n)].map((_, colIndex) => (
        <TableMorphSelect key={colIndex} n={n} />
      ))}
    </TableRow>
  ));

  const TableColHeadRow = () => (
    <TableHead>
      <TableRow>
        {/* Empty cell: */}
        <TableCell sx={headerStyles} />
        {[...Array(n)].map((_, index) => (
          <TableCell key={index} sx={{
            ...cellStyles,
            borderBottom: '2px solid #ddd',
            backgroundColor: theme.palette.primary.main,
          }}>
            <F index={index} />
          </TableCell>
        ))}
      </TableRow>
    </TableHead>
  );

  return (
    <Box
      sx={{
        borderCollapse: 'collapse',
        width: '100%',
        textAlign: 'center',
        margin: 'auto',
        maxWidth: '100%',
        overflowX: 'auto',
        fontFamily: 'Arial, sans-serif',
      }}
    >
      <Table>
        <TableColHeadRow />
        <TableBody>{rows}</TableBody>
      </Table>
    </Box>
  );

};

export default TableEditor;
