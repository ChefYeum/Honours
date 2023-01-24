import { DiMultGraph } from "../../src/Category";

// const c0: DiMultGraph = {
//   nodes: [
//     { id: 0 },
//     { id: 1 },
//     { id: 2 },
//   ],
//   links: [
//     { source: 0, target: 0 },
//     { source: 1, target: 1 },
//     { source: 2, target: 2 },
//     { source: 0, target: 1 },
//     { source: 0, target: 1 },
//     { source: 1, target: 2 },
//     { source: 0, target: 2 },
//     { source: 0, target: 2 },
//   ]
// }
export const c1: DiMultGraph = {
  nodes: [
    { id: 0 },
    { id: 1 },
    { id: 2 },
  ],
  links: [
    { source: 0, target: 0 },
    { source: 1, target: 1 },
    { source: 2, target: 2 },
    { source: 0, target: 1 },
    { source: 0, target: 1 },
    { source: 1, target: 2 },
    { source: 0, target: 2 },
    { source: 0, target: 2 } // f7
  ]
};

// Non category; Same as c1 but without identity on node 0.
export const nc1: DiMultGraph = {
  nodes: [
    { id: 0 },
    { id: 1 },
    { id: 2 },
  ],
  links: [
    // { source: 0, target: 0 },
    { source: 1, target: 1 },
    { source: 2, target: 2 },
    { source: 0, target: 1 },
    { source: 0, target: 1 },
    { source: 1, target: 2 },
    { source: 0, target: 2 },
    { source: 0, target: 2 } // f7
  ]
};

export const c2: DiMultGraph = {
  nodes: [
    { id: 0 },
    { id: 1 },
    { id: 2 },
  ],
  links: [
    { source: 0, target: 0 },
    { source: 1, target: 1 },
    { source: 2, target: 2 },
    { source: 0, target: 1 },
    { source: 0, target: 1 },
    { source: 1, target: 2 },
    { source: 1, target: 2 }, // f6
  ]
};
export const c3: DiMultGraph = {
  nodes: [
    { id: 0 },
    { id: 1 },
    { id: 2 },
    { id: 3 },
  ],
  links: [
    { source: 0, target: 0 },
    { source: 1, target: 1 },
    { source: 2, target: 2 },
    { source: 3, target: 3 },
    { source: 0, target: 1 },
    { source: 2, target: 3 },
    { source: 0, target: 2 },
    { source: 1, target: 3 },
    { source: 0, target: 3 }, // f8
  ]
};
