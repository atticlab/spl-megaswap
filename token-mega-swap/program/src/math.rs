// use std::convert::TryInto;

// use spl_math::precise_number::{pnum, PreciseNumber};
// use spl_token_swap::curve::calculator::map_zero_to_none;

// use crate::error::PoolError;

// ///  return amount of token B the same in value as `pool_ab_provided` amount in AB swap liquidity tokens
// ///
// /// parameters:
// /// - `pool_ab_b_total`   total amount of B tokens in AB swap liquidity  pool
// /// - `pool_ab_provided`  exact amount of AB liquidity tokens user wants to put in the new pool
// /// - `pool_ab_total`     total minted amount of AB swap liquidity tokens
// pub fn amount_of_b_for_ab(
//     pool_ab_b_total: u64,
//     pool_ab_provided: u64,
//     pool_ab_total: u64,
// ) -> Option<PreciseNumber> {
//     (pnum(pool_ab_b_total) * pnum(pool_ab_provided))? / pnum(pool_ab_total)
// }

// /// returns exact amount of BC swap LP tokens required to create megaswap in correct LP token ratio
// ///
// /// parameters:
// ///  - `pool_bc_total` is total minted amount of BC swap liquidity tokens
// ///  - `pool_ab_b` is amount of token B calculated in the `amount_of_b_for_ab` step
// ///  - `pool_bc_b_total` is total amount of token B in BC swap liquidity pool
// pub fn amount_of_bc_for_b(
//     pool_bc_total: u64,
//     pool_ab_b: u128,
//     pool_bc_b_total: u64,
// ) -> Option<PreciseNumber> {
//     (pnum(pool_bc_total) * pnum(pool_ab_b))? / pnum(pool_bc_b_total)
// }

// pub fn prepare_amounts(
//     pool_ab_b_total: u64,
//     pool_ab_provided: u64,
//     pool_ab_total: u64,
//     pool_bc_total: u64,
//     pool_bc_b_total: u64,
// ) -> Option<u128> {
//     // we could plug  it as one big formula so will not need to cast to imprecise
//     let pool_ab_b =
//         amount_of_b_for_ab(pool_ab_b_total, pool_ab_provided, pool_ab_total)?.to_imprecise()?;
//     map_zero_to_none(amount_of_bc_for_b(pool_bc_total, pool_ab_b, pool_bc_b_total)?.to_imprecise()?)
// }

// pub fn to_u64(val: u128) -> Result<u64, PoolError> {
//     val.try_into().map_err(|_| PoolError::Overflow)
// }

// #[cfg(test)]
// pub mod test {
//     use super::*;

//     #[test]
//     pub fn base_amount_of_b_for_ab() {
//         assert_eq!(
//             amount_of_b_for_ab(10, 5, 20)
//                 .unwrap()
//                 .to_imprecise()
//                 .unwrap(),
//             3
//         );
//         assert_eq!(
//             amount_of_b_for_ab(10_000, 5_000, 20_000)
//                 .unwrap()
//                 .to_imprecise()
//                 .unwrap(),
//             2_500
//         );
//         assert_eq!(
//             amount_of_b_for_ab(10, 10, 10)
//                 .unwrap()
//                 .to_imprecise()
//                 .unwrap(),
//             10
//         );
//     }

//     #[test]
//     pub fn base_amount_of_bc_for_b() {
//         assert_eq!(
//             amount_of_bc_for_b(20, 3, 10)
//                 .unwrap()
//                 .to_imprecise()
//                 .unwrap(),
//             6
//         );
//         assert_eq!(
//             amount_of_bc_for_b(20_000, 2_500, 10_000)
//                 .unwrap()
//                 .to_imprecise()
//                 .unwrap(),
//             5_000
//         );
//         assert_eq!(
//             amount_of_bc_for_b(10, 10, 10)
//                 .unwrap()
//                 .to_imprecise()
//                 .unwrap(),
//             10
//         );
//     }

//     #[test]
//     pub fn back_and_forth() {
//         assert_eq!(prepare_amounts(10, 10, 10, 10, 10).unwrap(), 10);

//         assert_eq!(
//             prepare_amounts(
//                 100_000,
//                 2_000_000,
//                 1_000_000_000_000,
//                 1_000_000_000_000,
//                 2_000_000_000
//             ),
//             None,
//             "A in AB token is way more costly C in BC, given B = B, making (BC price per token)/(AB price per token) zero"
//         );

//         assert_eq!(
//             prepare_amounts(
//                 2_000_000_000,
//                 2_000_000,
//                 1_000_000_000_000,
//                 1_000_000_000_000,
//                 100_000,
//             )
//             .unwrap(),
//             40_000_000_000,
//             "A in AB token is way more cheaper C in BC, given B = B"
//         );
//         let prepared = prepare_amounts(1000, 100, 1000000000, 1000000000, 1500);
//         assert_eq!(prepared, None);
//     }

//     // can prop test here or run big values to choose best math
// }
