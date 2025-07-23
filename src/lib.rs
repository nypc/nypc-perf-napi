#![deny(clippy::all)]

use napi::{Error, Result, Status};
use napi_derive::napi;
use nypc_perf::PerfCalc;

/// Represents the outcome of battles between two players.
///
/// This struct holds the battle statistics between players `i` and `j`,
/// including the number of wins each player achieved against the other.
///
/// # Fields
///
/// * `i` - Index of the first player (0-based)
/// * `j` - Index of the second player (0-based)
/// * `wij` - Number of wins by player `i` against player `j`
/// * `wji` - Number of wins by player `j` against player `i`
///
/// # Example
///
/// ```javascript
/// // Player 0 won 3 times against Player 1, Player 1 won 1 time against Player 0
/// const battle = new BattleResult({
///   i: 0,
///   j: 1,
///   wij: 3.0,
///   wji: 1.0,
/// });
/// ```
#[napi(object)]
#[derive(Debug, Clone, Copy)]
pub struct BattleResult {
  pub i: u32,
  pub j: u32,
  pub wij: f64,
  pub wji: f64,
}

impl From<BattleResult> for nypc_perf::BattleResult {
  fn from(value: BattleResult) -> Self {
    nypc_perf::BattleResult {
      i: value.i as usize,
      j: value.j as usize,
      wij: value.wij,
      wji: value.wji,
    }
  }
}

/// Represents a player's performance rating.
///
/// Each rating consists of a numerical value (on a log-scale) and a flag
/// indicating whether the rating should remain fixed during calculation.
/// Fixed ratings are typically used for anchor players to establish a
/// reference point in the rating system.
///
/// # Fields
///
/// * `fixed` - Whether this rating should remain constant during calculation
/// * `value` - The performance rating value (log-scale, higher = better)
///
/// # Example
///
/// ```javascript
/// // Variable rating that can be updated
/// const playerRating = new Rating({ fixed: false, value: 0.0 });
///
/// // Fixed anchor rating
/// const anchorRating = new Rating({ fixed: true, value: 0.0 });
/// ```
#[napi(object)]
#[derive(Debug, Clone, Copy)]
pub struct Rating {
  pub fixed: bool,
  pub value: f64,
}

impl From<Rating> for nypc_perf::Rating {
  fn from(value: Rating) -> Self {
    nypc_perf::Rating {
      fixed: value.fixed,
      value: value.value,
    }
  }
}

/// Configuration options for the performance calculation algorithm.
///
/// These options control the behavior of the Newton-Raphson iteration
/// used to find maximum likelihood estimates of player performances.
///
/// # Fields
///
/// * `max_iterations` - Maximum number of iterations before giving up (default: 100)
/// * `epsilon` - Convergence threshold - algorithm stops when error < epsilon (default: 1e-6)
///
/// # Example
///
/// ```javascript
/// const options = new CalcOptions({
///   max_iterations: 200,  // Allow more iterations for difficult cases
///   epsilon: 1e-8        // Require higher precision
/// });
/// ```
#[napi(object)]
#[derive(Debug, Clone, Copy)]
pub struct CalcOptions {
  pub max_iterations: Option<u32>,
  pub epsilon: Option<f64>,
}

/// Result of a performance calculation.
///
/// Contains the updated ratings and information about the algorithm's
/// convergence behavior. If the algorithm converged successfully,
/// `iterations` will contain the number of iterations required.
/// If it failed to converge, `error` will contain the final error value.
///
/// # Fields
///
/// * `ratings` - Updated performance ratings for all players
/// * `iterations` - Number of iterations if converged, null otherwise  
/// * `error` - Final error value if did not converge, null otherwise
#[napi(object)]
pub struct CalcResult {
  pub ratings: Vec<f64>,
  pub iterations: Option<u32>,
  pub error: Option<f64>,
}

/// Calculates player performance ratings using the Bradley-Terry model.
///
/// This function implements a Newton-Raphson algorithm to find maximum
/// likelihood estimates of player performance levels based on head-to-head
/// battle outcomes. The algorithm iteratively adjusts ratings to best
/// explain the observed win/loss patterns.
///
/// # Parameters
///
/// * `ratings` - Initial performance ratings for all players. At least one
///               rating should be fixed to serve as an anchor point.
/// * `battles` - Battle outcomes between players. Each battle specifies
///               two players and their win counts against each other.
/// * `options` - Algorithm configuration including iteration limits and
///               convergence criteria.
///
/// # Returns
///
/// A `CalcResult` object containing:
/// - Updated performance ratings
/// - Number of iterations if converged
/// - Final error if convergence failed
///
/// # Algorithm Details
///
/// The Bradley-Terry model estimates P(i beats j) = exp(π_i) / (exp(π_i) + exp(π_j))
/// where π_i and π_j are log-performance ratings. The algorithm finds ratings
/// that maximize the likelihood of the observed battle outcomes.
///
/// # Example
///
/// ```javascript
/// import { calc_perf, BattleResult, Rating, CalcOptions } from 'nypc-perf-wasm';
///
/// const ratings = [
///   new Rating({ fixed: false, value: 0.0 }),
///   new Rating({ fixed: false, value: 0.0 }),
///   new Rating({ fixed: true, value: 0.0 })  // Anchor
/// ];
///
/// const battles = [
///   new BattleResult({ i: 0, j: 1, wij: 2.0, wji: 1.0 }),
///   new BattleResult({ i: 0, j: 2, wij: 1.0, wji: 0.0 })
/// ];
///
/// const options = new CalcOptions({ max_iterations: 100, epsilon: 1e-6 });
/// const result = calc_perf(ratings, battles, options);
///
/// if (result.iterations !== null) {
///   console.log(`Converged in ${result.iterations} iterations`);
///   console.log('Updated ratings:', result.ratings);
/// }
/// ```
#[napi]
pub fn calc_perf(
  ratings: Vec<Rating>,
  battles: Vec<BattleResult>,
  options: Option<CalcOptions>,
) -> Result<CalcResult> {
  for rating in &ratings {
    if !rating.value.is_finite() {
      return Err(Error::new(Status::InvalidArg, "Invalid rating value"));
    }
  }
  for battle in &battles {
    if battle.i >= ratings.len() as u32 || battle.j >= ratings.len() as u32 {
      return Err(Error::new(Status::InvalidArg, "Invalid player index"));
    }
    if !(battle.wij.is_finite() && battle.wji.is_finite() && battle.wij >= 0.0 && battle.wji >= 0.0)
    {
      return Err(Error::new(Status::InvalidArg, "Invalid battle result"));
    }
  }
  let max_iterations = options.and_then(|o| o.max_iterations);
  let epsilon = options.and_then(|o| o.epsilon);
  if max_iterations.is_some_and(|m| m == 0) {
    return Err(Error::new(
      Status::InvalidArg,
      "Max iterations must be greater than 0",
    ));
  }
  if epsilon.is_some_and(|e| !(e > 0.0 && e.is_finite())) {
    return Err(Error::new(
      Status::InvalidArg,
      "Epsilon must be greater than 0",
    ));
  }

  let mut calc = PerfCalc::new();
  if let Some(max_iterations) = max_iterations {
    calc = calc.max_iters(max_iterations as usize);
  }
  if let Some(epsilon) = epsilon {
    calc = calc.epsilon(epsilon);
  }
  let mut ratings = ratings.into_iter().map(|r| r.into()).collect::<Vec<_>>();
  let battles = battles.into_iter().map(|b| b.into()).collect::<Vec<_>>();
  let result = calc.run(&mut ratings, &battles);
  Ok(CalcResult {
    ratings: ratings.into_iter().map(|r| r.value).collect(),
    iterations: result.ok().map(|i| i as u32),
    error: result.err(),
  })
}
