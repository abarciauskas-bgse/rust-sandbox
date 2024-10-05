use numpy::PyArray1;
use pyo3::prelude::*;
use pyo3::types::PyModule;

#[pyfunction]
fn numpy_to_vec<'py>(array: &Bound<'py, PyAny>) -> PyResult<Vec<f64>> {
    let array: &PyArray1<f64> = array.extract()?;
    let readonly: numpy::PyReadonlyArray<'_, f64, numpy::ndarray::Dim<[usize; 1]>> =
        array.readonly();
    let vec: Vec<f64> = readonly.as_slice()?.to_vec();
    Ok(vec)
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_example(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(numpy_to_vec, m)?)?;
    Ok(())
}
