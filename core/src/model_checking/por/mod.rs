//! Partial Order Reduction (POR) module.
//!
//! This module implements partial order reduction using ample sets to reduce
//! the state space while preserving all properties being checked.
//!
//! # Overview
//!
//! Partial order reduction is an optimization technique that exploits the
//! commutativity of independent transitions. Instead of exploring all possible
//! interleavings, we only explore a representative subset (the ample set) that
//! is sufficient to verify the properties.
//!
//! # Modules
//!
//! - `independence` - Defines the independence relation between transitions
//! - `ample_set` - Computes ample sets satisfying conditions C0-C3
//! - `explorer` - State space exploration using POR
//! - `result` - Result types with POR statistics

pub mod ample_set;
pub mod explorer;
pub mod independence;
pub mod result;

// Re-export commonly used types
pub use ample_set::{AmpleSetConfig, AmpleSetReason, AmpleSetResult, AmpleSetStats, ReasonCounts};
pub use explorer::run_with_por;
pub use independence::TransitionEffect;
pub use result::ModelCheckingResultWithStats;
