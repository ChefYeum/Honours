import { c1 } from "../cypress/component/exampleModels";

export interface DiMultGraph {
    nodes: Node[];
    links: Link[];
}

export interface Node {
    id: number;
}

export interface Link {
    source: number;
    target: number;
}

type SourceID = number;
type TargetID = number;
type MorphCount = number;

// Maps source to target to count
export interface MorphMap {
    [source: SourceID]: TargetCountMap;
}

export interface TargetCountMap {
    [target: TargetID]: MorphCount;
}

// Returns the target map of a diagram
export function getMorphMap(g: DiMultGraph): MorphMap {
    const targetMap: MorphMap = {};
    g.links.forEach(l => {
        const countMap = targetMap[l.source] || {};
        targetMap[l.source] = countMap;
        countMap[l.target] = (countMap[l.target] || 0) + 1;
    })
    return targetMap;
}

// Check if all nodes have an identity morphism 
// TODO: return counterexample
export function checkIdMorphForAllObj(c: DiMultGraph): boolean {
    const nodes = c.nodes.map(n => n.id);
    const idMorphs = c.links.filter(l => l.source === l.target)
        .map(l => l.source)
    return nodes.every(n => idMorphs.includes(n));
}

export function checkComp(c: DiMultGraph): number {
    let totalEval: number = 1;

    const targetMap = getMorphMap(c);
    // A -> B -> C
    for (const objA of c.nodes) {
        const mapAfterObjA = targetMap[objA.id];
        if (!mapAfterObjA) continue;
        // TODO: why `any`?
        for (const objB in mapAfterObjA) {
            const mapAfterObjB = targetMap[objB];
            if (!mapAfterObjB) continue;
            let possibleEval: number = mapAfterObjB[objB];
            console.log("Eval: ", possibleEval);
            totalEval *= possibleEval;
        }
    }
    return totalEval;
}

// checkComp(c1);

// TODO: Check unitality (or by definition?)
// TODO: Check associativity of composition (ab.c = a.bc)
// Idea: composition table that displays number of possible assignment in the cell.

