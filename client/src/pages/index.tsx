import type { NextPage } from 'next'
import { useEffect, useState } from 'react'
import dynamic from 'next/dynamic'

import { useContext } from "react"
import { WASMContext } from "../context/WASM"
import * as C from '../Category'
import { nc1, c3 } from '../../cypress/component/exampleModels'

// MUI:
import Box from '@mui/material/Box';
import SwipeableDrawer from '@mui/material/SwipeableDrawer';

import { Global } from '@emotion/react';
import { styled, ThemeProvider } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import { grey } from '@mui/material/colors';
import Skeleton from '@mui/material/Skeleton';
import Typography from '@mui/material/Typography';
import { Button } from '@mui/material'
// import Counter from './Counter'

const drawerBleeding = 56;

const Root = styled('div')(({ theme }) => ({
  height: '100%',
  backgroundColor: theme.palette.background.default,
}));

const StyledBox = styled(Box)(({ theme }) => ({
  backgroundColor: grey[800],
}));

const Puller = styled(Box)(({ theme }) => ({
  width: 30,
  height: 6,
  backgroundColor: grey[900],
  borderRadius: 3,
  position: 'absolute',
  top: 8,
  left: 'calc(50% - 15px)',
}));

// Import without SSR:
const ForceGraph3D = dynamic(() => import('react-force-graph-3d'), {
  ssr: false,
})

function genRandomTree(N = 50) {
  return {
    nodes: Array.from(Array(N).keys()).map(i => ({ id: i })),
    links: Array.from(Array(N).keys())
      .filter(id => id)
      .map(id => ({
        source: id,
        target: Math.round(Math.random() * (id - 1))
      }))
  };
}

const Home: NextPage = () => {

  const [open, setOpen] = useState(false);
  const [model, setModel] = useState(c3);

  const ctx = useContext(WASMContext)

  useEffect(() => {
    const output = C.checkIdMorphForAllObj(nc1) // True
    // const output = C.getMorphMap(c3)
    console.log(output)
  }, []);

  if (!ctx.wasm) {
    return <>...</>
  }

  const toggleDrawer = (newOpen: boolean) => () => {
    setOpen(newOpen);
  };


  return (
    <Root>
      <CssBaseline />
      <Global
        styles={{
          '.MuiDrawer-root > .MuiPaper-root': {
            height: `calc(50% - ${drawerBleeding}px)`,
            overflow: 'visible',
          },
        }}
      />
      <ForceGraph3D
        graphData={model}
        linkDirectionalArrowLength={3.5}
        linkDirectionalArrowRelPos={1}
        linkCurvature={0.25}
      />
      <SwipeableDrawer
        // container={container}
        anchor="bottom"
        open={open}
        onClose={toggleDrawer(false)}
        onOpen={toggleDrawer(true)}
        swipeAreaWidth={drawerBleeding}
        disableSwipeToOpen={false}
        ModalProps={{
          keepMounted: true,
        }}
      >
        <StyledBox
          sx={{
            position: 'absolute',
            top: -drawerBleeding,
            borderTopLeftRadius: 8,
            borderTopRightRadius: 8,
            visibility: 'visible',
            right: 0,
            left: 0,
          }}
        >
          <Puller />
          <Typography sx={{ p: 2, color: 'text.primary' }}>x objects, y morphisms</Typography>
        </StyledBox>
        <StyledBox
          sx={{
            px: 2,
            pb: 2,
            height: '100%',
            overflow: 'auto',
          }}
        >
          <button>Increment</button>
          {12}
          <button>Decrement</button>
          {/* <Counter /> */}
        </StyledBox>
      </SwipeableDrawer>
    </Root>
  );
}


//     <Root>
//       <CssBaseline />
//       <Global
//         styles={{
//           '.MuiDrawer-root > .MuiPaper-root': {
//             height: `calc(50% - ${drawerBleeding}px)`,
//             overflow: 'visible',
//           },
//         }}
//       />

//       <SwipeableDrawer
//         // container={container}
//         anchor="bottom"
//         open={open}
//         onClose={toggleDrawer(false)}
//         onOpen={toggleDrawer(true)}
//         swipeAreaWidth={drawerBleeding}
//         disableSwipeToOpen={false}
//         ModalProps={{
//           keepMounted: true,
//         }}

//         onClick={(e) => {
//           e.preventDefault()
//           toggleDrawer(!open)
//         }}
//       >

//         <StyledBox
//           sx={{
//             position: 'absolute',
//             top: -drawerBleeding,
//             borderTopLeftRadius: 8,
//             borderTopRightRadius: 8,
//             visibility: 'visible',
//             right: 0,
//             left: 0,
//           }}
//         >
//           <Puller />
//           <Typography sx={{ p: 2, color: 'text.primary' }}>x objects, y morphisms</Typography>
//         </StyledBox>
//         <StyledBox
//           sx={{
//             px: 2,
//             pb: 2,
//             height: '100%',
//             overflow: 'auto',
//           }}
//         >
//           {/* <button>Increment</button>
//         {12}
//         <button>Decrement</button> */}
//           {/* <Counter /> */}
//         </StyledBox>
//       </SwipeableDrawer>
//     </Root>
//   </div>



export default Home
