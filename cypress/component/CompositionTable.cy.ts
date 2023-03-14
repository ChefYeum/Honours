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

// Use `n` to refer to null in the table
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

// Other solutions to c1
const c1_t2 = c1_t1[3][5] = 7;
const c1_t3 = c1_t1[4][5] = 6;
const c1_t4 = c1_t1[4][5] = 6;

describe('ID check', () => {
    it('id check', () => {
        expect(() => check_json_model({
            table: c1_t1,
        })).to.not.throw();
    })
});
