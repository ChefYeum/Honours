import init, { check_json_model } from "../../wasm/pkg/wasm"

init();

describe('Table size check', () => {
    // we want square matrix

    it('dummy pass', () => { })

    it('sq mtx 1', () => {
        expect(() => check_json_model({
            table: [
                [1, 1],
                [1, 1]
            ]
        })).to.not.throw();
    })

    it('non-sq mtx', () => {
        expect(() => check_json_model({
            table: [
                [1, 1],
                [1, 1],
                [1, 1]
            ]
        })).to.throw('TableSizeError');
    })

    it('error msg check', () => {
        expect(() => check_json_model({
            table: [
                [1, 1],
                [1, 1],
                [1, 1]
            ]
        })).to.not.throw('somedifferenterror');
    })

})
