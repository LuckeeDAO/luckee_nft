#![no_std]

//! # Luckee NFT Contract
//!
//! A no_std compatible NFT contract with synthesis capabilities for CosmWasm.
//! This contract supports minting, burning, transferring, and synthesizing NFTs.
//!
//! ## Features
//!
//! - Standard CW721 NFT functionality
//! - Batch minting operations
//! - NFT synthesis system
//! - Recipe-based crafting
//! - no_std compatibility for embedded environments
//!
//! ## Usage
//!
//! ```rust
//! use luckee_nft::{ExecuteMsg, InstantiateMsg, QueryMsg};
//! ```

extern crate alloc;

// Core modules
pub mod contract;
pub mod error;
pub mod msg;
pub mod state;
pub mod types;
pub mod cw721;
pub mod luckee;
pub mod admin;
pub mod events;
pub mod helpers;
pub mod recipes;

// Re-export main functionality
pub use crate::error::ContractError;
pub use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// Common result type
pub type Result<T> = core::result::Result<T, ContractError>;

// Unit tests for no_std compatibility
#[cfg(test)]
mod test;
