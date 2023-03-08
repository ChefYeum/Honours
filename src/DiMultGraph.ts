export interface DiMultGraph {
    node_count: number;
    links: Link[];
}

export interface Node {
    id: number;
}

export interface Link {
    source: number;
    target: number;
}