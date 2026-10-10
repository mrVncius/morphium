#![deny(missing_docs)]

//! Native model-provider wire types and the kernel adapters over them.
//!
//! [`provider`] exposes each vendor's own request, response, and streaming
//! types — one module per native wire. [`model`] implements the kernel's
//! [`Model`](morphium::model::Model) port over those wires.

pub mod provider;
