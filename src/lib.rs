//! CocoIndex - A high-performance data indexing library
//!
//! This crate provides the core Rust implementation for cocoindex,
//! exposing Python bindings via PyO3.

use pyo3::prelude::*;

pub mod index;
pub mod pipeline;
pub mod storage;
pub mod transform;
pub mod types;

/// Python module entry point
#[pymodule]
fn _cocoindex_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register core types
    m.add_class::<types::DataType>()?;
    m.add_class::<types::FieldSchema>()?;
    m.add_class::<types::RecordSchema>()?;

    // Register pipeline components
    m.add_class::<pipeline::Pipeline>()?;
    m.add_class::<pipeline::PipelineConfig>()?;

    // Register index operations
    m.add_class::<index::IndexSpec>()?;
    m.add_class::<index::IndexManager>()?;

    // Register storage backends
    m.add_class::<storage::StorageConfig>()?;

    // Register version info
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
