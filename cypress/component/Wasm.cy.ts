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
        expect(() => check_json_model(c0)).to.not.throw()
    })
})