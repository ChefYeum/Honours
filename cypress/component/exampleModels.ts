import { DiMultGraph } from "../../src/DiMultGraph";

export const c0: DiMultGraph = {
  node_count: 3,
  links: [
    { source: 0, target: 0 },
    { source: 1, target: 1 },
    { source: 2, target: 2 },
    { source: 0, target: 1 },
    { source: 0, target: 1 },
    { source: 1, target: 2 },
    { source: 0, target: 2 },
    { source: 0, target: 2 },
  ]
}

// 4 solutions
export const c1: DiMultGraph = {
  node_count: 3,
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
  node_count: 3,
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

// No solution
export const c2: DiMultGraph = {
  node_count: 3,
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
  node_count: 4,
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
