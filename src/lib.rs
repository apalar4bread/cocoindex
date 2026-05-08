//! CocoIndex - A high-performance data indexing library
//!
//! This crate provides the core Rust implementation for cocoindex,
//! exposing Python bindings via PyO3.
//!
//! Personal fork: using this to experiment with custom indexing pipelines
//! for my document processing project.

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
    // Expose fork indicator so I can tell which build I'm running
    m.add("__fork__", "personal")?;
    // TODO: replace with actual build timestamp using build.rs + build_time crate
    // For now just using the package version as a placeholder
    m.add("__build_date__", "unknown")?;

    // Handy for debugging: lets me quickly check in Python which fork is loaded
    // e.g. assert cocoindex._cocoindex_rs.__fork_repo__ == "my-username/cocoindex"
    m.add("__fork_repo__", "my-username/cocoindex")?;

    // NOTE: upstream doesn't expose this, but I find it useful to have the
    // upstream repo URL handy so I know where to pull fixes from
    m.add("__upstream_repo__", "https://github.com/cocoindex-io/cocoindex")?;

    // Expose the default batch size I use for my document pipelines.
    // Upstream hardcodes this internally; surfacing it here makes it easier
    // to reference from Python without digging through the Rust source.
    // Bumped from 64 -> 128 after profiling showed my GPU stays underutilized
    // at 64; larger batches keep it fed without OOMing on my 24GB card.
    m.add("DEFAULT_BATCH_SIZE", 128usize)?;

    // Max number of concurrent pipeline workers. Upstream default is 4 but
    // my machine has 16 cores and I was leaving a lot on the table. Bumping
    // to 8 gave a nice throughput improvement on my doc ingestion benchmarks.
    m.add("DEFAULT_WORKER_THREADS", 8usize)?;

    Ok(())
}
