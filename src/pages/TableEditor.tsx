import React from 'react';
import { css, ClassNames, Global } from '@emotion/react';
import { Select, MenuItem } from '@mui/material';

const tableStyles = css({
  borderCollapse: 'collapse',
  width: '100%',
  textAlign: 'center',
  margin: 'auto',
  maxWidth: '100%',
  overflowX: 'auto',
});

const tableHeadStyles = css({
  borderBottom: '2px solid #ddd',
  padding: '10px',
  backgroundColor: '#f2f2f2',
  width: '50px',
  height: '50px',
  lineHeight: '50px',
});

const tableDataStyles = css({
  borderBottom: '1px solid #ddd',
  padding: '0',
  width: '50px',
  height: '50px',
  lineHeight: '50px',
  verticalAlign: 'middle',
});

const subscriptStyle = css({
  fontSize: '0.8em',
  verticalAlign: 'sub',
});

const selectStyles = css({
  width: '100%',
  height: '100%',
  border: 'none',
  textAlign: 'center',
  textAlignLast: 'center',
  appearance: 'none',
  background: 'transparent',
});

const TableEditor = ({ n }: { n: number }) => {
  const rows = [...Array(n)].map((_, rowIndex) => (
    <tr key={rowIndex}>
      <ClassNames>
        {({ css }) => (
          <th className={css(tableHeadStyles)}>
            f<span className={css(subscriptStyle)}>{rowIndex}</span>
          </th>
        )}
      </ClassNames>
      {[...Array(n)].map((_, colIndex) => (
        <ClassNames key={colIndex}>
          {({ css }) => (
            <td className={css(tableDataStyles)}>
              <Select
                IconComponent={null}
                sx={{
                  display: 'flex',
                  flexDirection: 'row',
                  borderRadius: 0,
                }}
                sx={{ width: '100%', height: '100%', border: 'none' }}
                defaultValue={10}
              >
                <MenuItem value={10}>Ten</MenuItem>
                <MenuItem value={20}>Twenty</MenuItem>
                <MenuItem value={30}>Thirty</MenuItem>
              </Select>
            </td>
          )}
        </ClassNames>
      ))}
    </tr>
  ));

  return (
    <>
      <Global
        styles={{
          body: {
            fontFamily: 'Arial, sans-serif',
          },
        }}
      />
      <ClassNames>
        {({ css }) => (
          <div className={css(tableStyles)}>
            <table>
              <thead>
                <tr>
                  <th />
                  {[...Array(n)].map((_, index) => (
                    <ClassNames key={index}>
                      {({ css }) => (
                        <th className={css(tableHeadStyles)}>
                          f<span className={css(subscriptStyle)}>{index}</span>
                        </th>
                      )}
                    </ClassNames>
                  ))}
                </tr>
              </thead>
              <tbody>{rows}</tbody>
            </table>
          </div>
        )}
      </ClassNames>
    </>
  );
};

export default TableEditor;
