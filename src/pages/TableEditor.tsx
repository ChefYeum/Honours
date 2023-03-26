import React from 'react';
import { Box, Table, TableBody, TableCell, TableHead, TableRow, Typography } from '@mui/material';
import TableMorphSelect from './TableMorphSelect';

const F = (props: {index: number}) => (
    <Typography component="span" variant="subtitle2">
        f
        <Typography component="span" variant="caption" sx={{ verticalAlign: 'sub' }}>
            {props.index}
        </Typography>
    </Typography>
 )

const TableEditor = ( props : {n : number}) => {
    const { n } = props;
    const rows = [...Array(n)].map((_, rowIndex) => (
        <TableRow key={rowIndex}>
            <TableCell sx={{ borderBottom: '2px solid #ddd', padding: '10px', backgroundColor: '#f2f2f2' }}>
                <F index={rowIndex} />
            </TableCell>
            {[...Array(n)].map((_, colIndex) => (
                <TableMorphSelect key={colIndex} n={n} />
            ))}
        </TableRow>
    ));

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
                <TableHead>
                    <TableRow>
                        <TableCell />
                        {[...Array(n)].map((_, index) => (
                            <TableCell key={index} sx={{ borderBottom: '2px solid #ddd', padding: '10px', backgroundColor: '#f2f2f2' }}>
                                <F index={index} />
                            </TableCell>
                        ))}
                    </TableRow>
                </TableHead>
                <TableBody>{rows}</TableBody>
            </Table>
        </Box>
    );
};

export default TableEditor;
