use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod indexing;
mod storage;
mod query;
mod utils;

/// CocoIndex - A high-performance document indexing and retrieval library
/// built with Rust for speed and Python for ease of use.

/// Index a collection of documents from a given path.
///
/// Args:
///     path: Path to the directory or file to index
///     index_name: Name for this index
///     options: Optional dict of indexing options
///
/// Returns:
///     Index ID string on success
#[pyfunction]
#[pyo3(signature = (path, index_name, options=None))]
fn index_documents(
    path: String,
    index_name: String,
    options: Option<std::collections::HashMap<String, String>>,
) -> PyResult<String> {
    indexing::create_index(&path, &index_name, options)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

/// Search an existing index for documents matching the query.
///
/// Args:
///     index_name: Name of the index to search
///     query: Search query string
///     top_k: Number of top results to return (default: 10)
///
/// Returns:
///     List of (doc_id, score, metadata) tuples
#[pyfunction]
#[pyo3(signature = (index_name, query, top_k=10))]
fn search(
    index_name: String,
    query: String,
    top_k: usize,
) -> PyResult<Vec<(String, f32, std::collections::HashMap<String, String>)>> {
    query::search_index(&index_name, &query, top_k)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

/// Delete an index by name.
///
/// Args:
///     index_name: Name of the index to delete
#[pyfunction]
fn delete_index(index_name: String) -> PyResult<()> {
    storage::delete_index(&index_name)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

/// List all available indexes.
///
/// Returns:
///     List of index name strings
#[pyfunction]
fn list_indexes() -> PyResult<Vec<String>> {
    storage::list_indexes()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

/// The main Python module definition for cocoindex.
#[pymodule]
fn _cocoindex(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(index_documents, m)?)?;
    m.add_function(wrap_pyfunction!(search, m)?)?;
    m.add_function(wrap_pyfunction!(delete_index, m)?)?;
    m.add_function(wrap_pyfunction!(list_indexes, m)?)?;

    // Expose version info
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
