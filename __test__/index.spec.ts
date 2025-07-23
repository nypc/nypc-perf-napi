import test from 'ava'

import { calcPerf } from '../index'

test('calcPerf', (t) => {
  // Define our battle data with type safety
  const battles = [
    { i: 0, j: 1, wij: 3, wji: 1 }, // Player 0 vs Player 1: 3-1
    { i: 0, j: 2, wij: 2, wji: 0 }, // Player 0 vs Player 2: 2-0
    { i: 1, j: 2, wij: 1, wji: 2 }, // Player 1 vs Player 2: 1-2
  ]

  // Initialize performance ratings with explicit types
  const ratings = [
    { fixed: false, value: 0.0 }, // Player 0
    { fixed: false, value: 0.0 }, // Player 1
    { fixed: true, value: 0.0 }, // Player 2 (anchor)
  ]

  // Configure calculation options
  const options = { max_iterations: 100, epsilon: 1e-6 }

  // Run calculation with full type safety
  const result = calcPerf(ratings, battles, options)

  t.assert(Math.abs(result.ratings[0] - 0.71165756) < 1e-6)
  t.assert(Math.abs(result.ratings[1] + 0.31723877) < 1e-6)
  t.assert(Math.abs(result.ratings[2] - 0.0) < 1e-6)

  t.pass()
})

test('exception', (t) => {

  const battles = [{ i: 1, j: 2, wij: 0, wji: 0 }];
  const ratings = [{ fixed: false, value: 0.0 }, { fixed: false, value: 0.0 }];

  try {
    calcPerf(ratings, battles);
    t.fail();
  } catch (e) {
    t.pass();
  }
})
