use mandelbrot_common::generate;
use numpy::{IntoPyArray, PyArray2};
use pyo3::prelude::{pymodule, Py, PyModule, PyResult, Python};

/// A Python module implemented in Rust.
#[pymodule]
fn mandelbrot_rs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    /// Wrap the image generator from mandelbrot_common
    #[pyfn(m, "generate")]
    fn generate_wrapped(
        py: Python<'_>,
        n: usize,
        threshold: f64,
        max_steps: u8,
    ) -> Py<PyArray2<u8>> {
        generate(n, threshold, max_steps)
            .into_pyarray(py)
            .to_owned()
    }

    Ok(())
}
