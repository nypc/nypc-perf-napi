# `@nypc-perf/perf`

[![npm version](https://img.shields.io/npm/v/@nypc-perf/perf.svg)](https://www.npmjs.com/package/@nypc-perf/perf)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Node.js binding for calculating player performance using the Bradley-Terry model. This package provides a high-performance interface to the [nypc-perf](https://crates.io/crates/nypc-perf) Rust library.

## Overview

This library implements a Bradley-Terry model based performance system that estimates player performance levels from head-to-head battle outcomes. The algorithm uses Newton-Raphson iteration to find maximum likelihood estimates of player performances that best explain the observed win/loss patterns. For more information, please refer to [Github nypc/nypc-perf repository](https://github.com/nypc/nypc-perf).

## Usage

### Basic Example

```typescript
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

const result = calcPerf(ratings, battles, options);

/*
  The result is shown like this
  {
    ratings: [ 0.7116575573823986, -0.3172387704122441, 0 ],
    iterations: 14
  }
*/
```

## Authors

**NEXON Algorithm Research Team** - [_algorithm@nexon.co.kr](mailto:_algorithm@nexon.co.kr)

## License

This project is licensed under the MIT License.