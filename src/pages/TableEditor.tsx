import React from 'react';
import { Box, MenuItem, Select, styled, Table, TableBody, TableCell, TableHead, TableRow } from '@mui/material';
import { useTheme, Tab, Tabs } from '@mui/material';
import { F } from './components/TableMorphLabel';

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
      defaultValue={'-'}
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
      <MenuItem key={'-'} value={'-'} sx={{ justifyContent: 'center' }}>
        <F index={null} />
      </MenuItem>
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

  const [value, setValue] = React.useState(0);

  const handleChange = (event: React.SyntheticEvent, newValue: number) => {
    setValue(newValue);
  };
  interface StyledTabsProps {
    children?: React.ReactNode;
    value: number;
    onChange: (event: React.SyntheticEvent, newValue: number) => void;
  }

  const StyledTabs = styled((props: StyledTabsProps) => (
    <Tabs
      {...props}
      TabIndicatorProps={{ children: <span className="MuiTabs-indicatorSpan" /> }}
    />
  ))({
    '& .MuiTabs-indicator': {
      display: 'flex',
      justifyContent: 'center',
      backgroundColor: 'transparent',
    },
    '& .MuiTabs-indicatorSpan': {
      maxWidth: 40,
      width: '100%',
      backgroundColor: '#635ee7',
    },
  });

  interface StyledTabProps {
    label: string;
  }

  const StyledTab = styled((props: StyledTabProps) => (
    <Tab disabled disableRipple {...props} />
  ))(({ theme }) => ({
    textTransform: 'none',
    fontWeight: theme.typography.fontWeightRegular,
    fontSize: theme.typography.pxToRem(15),
    marginRight: theme.spacing(1),
    color: 'rgba(255, 255, 255, 0.7)',
    '&.Mui-selected': {
      color: '#fff',
    },
    '&.Mui-focusVisible': {
      backgroundColor: 'rgba(100, 95, 228, 0.32)',
    },
  }));

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
      <StyledTabs
        value={value}
        onChange={handleChange}
        aria-label="styled tabs example"
      >
        <StyledTab label="Category" />
        <StyledTab label="Monoidal" />
      </StyledTabs>
      <Table>
        <TableColHeadRow />
        <TableBody>{rows}</TableBody>
      </Table>
    </Box>
  );

};

export default TableEditor;
