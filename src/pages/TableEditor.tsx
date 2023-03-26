import React from 'react';
import { Box, MenuItem, Select, Table, TableBody, TableCell, TableHead, TableRow, Typography } from '@mui/material';

const F = (props: { index: number }) => (
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
);

const TableMorphSelect = (props: { n: number }) => (
    <TableCell sx={{
        borderBottom: '1px solid #ddd',
        padding: '0',
        width: '50px',
        height: '50px',
        lineHeight: '50px',
        verticalAlign: 'middle',
    }}>
        <Select
            sx={{
                display: 'flex',
                flexDirection: 'row',
                borderRadius: 0,
                width: '100%',
                height: '100%',
                border: 'none'
            }}
            defaultValue={"-"}
        >
            {[...Array(props.n)].map((_, index) => (
                <MenuItem value={index}>{index}</MenuItem>
            ))}
        </Select>
    </TableCell>
)

const TableEditor = (props: { n: number }) => {
    const { n } = props;
    const TableColHead = (props: { rowIndex: number }) => (<TableCell sx={{ borderBottom: '2px solid #ddd', padding: '10px', backgroundColor: '#f2f2f2' }}>
        <F index={props.rowIndex} />
    </TableCell>);

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
                <TableCell />

                {[...Array(n)].map((_, index) => (
                    <TableCell key={index} sx={{ borderBottom: '2px solid #ddd', padding: '10px', backgroundColor: '#f2f2f2' }}>
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
