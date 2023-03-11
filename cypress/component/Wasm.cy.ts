import init, { check_json_model } from "../../wasm/pkg/wasm"
import { c0, c1, c2, c3 } from './exampleModels'

init();

describe('trivial', () => {
    it('single node', () => {
        let g = {
            "node_count": 1,
            "links": [
                {
                    "source": 1,
                    "target": 1,
                }
            ]
        }
        expect(() => check_json_model(g)).to.not.throw()
    })

    it('one node with no identity', () => {
        let g = {
            "node_count": 1,
            "links": []
        }
        expect(() => check_json_model(g)).to.throw('NoIdentity(1)')
    })
})

describe('given examples', () => {
    it('c0', () => {
        // TOOD: 4 solutions
        expect(() => check_json_model(c0)).to.not.throw()
    })

    it('nc1', () => {
        expect(() => check_json_model(c1)).to.throw()
    })

    it('c2', () => {
        expect(() => check_json_model(c2)).to.not.throw()
    })

    it('c3', () => {
        expect(() => check_json_model(c3)).to.not.throw()
    })

    // it('c4', () => {
    //     expect(() => check_json_model(c4)).to.throw()
    // })
})