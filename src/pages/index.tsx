import type { NextPage } from 'next'
import { useEffect, useState } from 'react'
import dynamic from 'next/dynamic'

import { useContext } from "react"
import { WASMContext } from "../context/WASM"
import { nc1, c3 } from '../../cypress/component/exampleModels'

// MUI:
import Box from '@mui/material/Box';
import { styled } from '@mui/material/styles';
import { Fab, Modal, Typography } from '@mui/material'
import EditIcon from '@mui/icons-material/Edit';
import init, { check_json_model } from '../../wasm/pkg'
import TableEditor from './TableEditor'

const Root = styled('div')(({ theme }) => ({
  height: '100%',
  backgroundColor: theme.palette.background.default,
}));

// Import without SSR:
const ForceGraph3D = dynamic(() => import('react-force-graph-3d'), {
  ssr: false,
})

function genRandomTree(N = 50) {
  return {
    nodes: Array.from(Array(N).keys()),
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
  const [model, _] = useState(c3);

  const handleOpen = () => setOpen(true);
  const handleClose = () => setOpen(false);

  useEffect(() => {
    init().then(() => {
      // Get the example object from wasm.
      // let model = init_model(5);
      // model.node_count += 123 - 5;

      // model.add_node();
      // model.add_link();
      // model.add_node();

      const n = null;
      const c1_t1 = [ // unique solution
        [0, n, n, 3, 4, n, 6, 7],
        [n, 1, n, n, n, 5, n, n],
        [n, n, 2, n, n, n, n, n],
        [n, 3, n, n, n, 6, n, n],
        [n, 4, n, n, n, 7, n, n],
        [n, n, 5, n, n, n, n, n],
        [n, n, 6, n, n, n, n, n],
        [n, n, 7, n, n, n, n, n],
      ]
      // read from file examples.json

      // let output = check_json_model({
      //   "table": [
      //     [0, 1],
      //     [1, 0],
      //   ]
      // })
      let output = check_json_model({ "table": c1_t1 });
      console.log(output);
    })
  }, []);


  return (
    <Root>
      <Fab
        variant="extended"
        onClick={handleOpen}
        style={{
          margin: 0,
          position: 'fixed',

          // Size
          width: 96,
          height: 96,

          top: 'auto',
          right: 72,
          bottom: 72,
          left: 'auto',

          // Roundness
          borderRadius: '50%',
        }}>
        <EditIcon sx={{

        }} />
      </Fab>
      <ForceGraph3D
        graphData={
          // model but map node_count to nodes which is what the graph expects.
          {
            nodes: Array.from(Array(model.node_count).keys()
            ).map((id) => ({
              id: id
            })),
            links: model.links
          }
        }
        linkDirectionalArrowLength={3.5}
        linkDirectionalArrowRelPos={1}
        linkCurvature={0.25}
      />
      <Modal
        open={open}
        onClose={handleClose}
        aria-labelledby="modal-modal-title"
        aria-describedby="modal-modal-description"
      >
        <Box sx={{
          position: 'absolute' as 'absolute',
          top: '50%',
          left: '50%',
          transform: 'translate(-50%, -50%)',
          width: '80vw',
          height: '80vh',
          bgcolor: 'background.paper',
          border: '2px solid #000',
          boxShadow: 24,
          p: 4,
        }}>
          <TableEditor />
        </Box>
      </Modal>
    </Root>
  );
}

export default Home
