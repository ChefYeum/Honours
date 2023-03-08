import { MorphMap, getMorphMap, checkComp } from "../../src/Category"
import { c1, c2, c3 } from "./exampleModels";

describe('Category.getMorphMap', () => {
  // it('c0', () => {
  //   ...
  // }

  it('c1', () => {
    const c1ExpectedMorphMap: MorphMap = {
      0: {
        0: 1,
        1: 2,
        2: 2
      },
      1: {
        // TODO: check how to represent no morphism; currently not included but zero should be equiv.
        1: 1,
        2: 1
      },
      2: {
        2: 1,
      }
    }
    expect(getMorphMap(c1)).to.deep.equal(c1ExpectedMorphMap)
  })

  it('c2', () => {
    const c2ExpectedMorphMap: MorphMap = {
      0: {
        0: 1,
        1: 2,
      },
      1: {
        1: 1,
        2: 2
      },
      2: {
        2: 1,
      }
    }
    expect(getMorphMap(c2)).to.deep.equal(c2ExpectedMorphMap)
  });

  it('c3', () => {
    const c3ExpectedMorphMap: MorphMap = {
      0: {
        0: 1,
        1: 1,
        2: 1,
        3: 1,
      },
      1: {
        1: 1,
        3: 1,
      },
      2: {
        2: 1,
        3: 1,
      },
      3: {
        3: 1,
      }
    };
    expect(getMorphMap(c3)).to.deep.equal(c3ExpectedMorphMap)
  });

  // TODO: add c4 and c5
});

describe('Category.checkComp', () => {
  // it('c0', () => {
  //   const result = checkComp(c0)
  //   expect(result).to.equal(2)
  //   // it('works', () => {
  //   //   expect(true).to.equal(true)
  //   // })
  // })

  it('c1', () => {
    const result = checkComp(c1)
    expect(result).to.equal(4)
  })
})
export { }