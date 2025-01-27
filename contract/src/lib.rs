pub mod contract; // Public module for handling contract execution logic
mod error;        // Private module for defining custom error types
pub mod msg;       // Public module for message and query definitions
pub mod state;      // Public module for state management

// Re-export ContractError for convenient use
pub use crate::error::ContractError;  