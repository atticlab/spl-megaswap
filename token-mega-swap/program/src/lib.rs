//! binary oracle pair
//TODO: fix it #![deny(missing_docs)]

mod borsh;
pub mod error;
pub mod instruction;
mod invoke;
pub mod math;
pub mod processor;

#[cfg(test)]
mod processor_tests;
#[cfg(test)]
mod test;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;

solana_program::declare_id!("BDhwBerjCPBbT6NpcwwQ4m923JCB56vC1fauSxfdhYHy");
