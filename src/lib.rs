mod extensions;

use pyo3::prelude::*;
use pyo3::types::PyList;

/// Returns a list of supported file extensions
#[pyfunction]
fn supported_extensions(py: Python<'_>) -> PyResult<Bound<'_, PyList>> {
    let exts = extensions::supported_extensions();
    let list = PyList::new(py, exts)?;
    return Ok(list);
}

/// A Python module implemented in Rust.
#[pymodule]
fn pearl(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(supported_extensions, m)?)?;
    Ok(())
}
