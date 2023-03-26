import { TableCell, Select, MenuItem } from '@mui/material';

const tableDataStyles = {
  borderBottom: '1px solid #ddd',
  padding: '0',
  width: '50px',
  height: '50px',
  lineHeight: '50px',
  verticalAlign: 'middle',
};


const TableMorphSelect = (props: { n: number }) => (
  <TableCell style={tableDataStyles}>
    <Select
      sx={{
        display: 'flex',
        flexDirection: 'row',
        borderRadius: 0,
        width: '100%',
        height: '100%',
        border: 'none'
      }}
      defaultValue={10}
    >
      {[...Array(props.n)].map((_, index) => (
        <MenuItem value={index}>{index}</MenuItem>
      ))}
    </Select>
  </TableCell>
)

export default TableMorphSelect