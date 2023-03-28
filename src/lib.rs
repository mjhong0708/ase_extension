pub mod logfermi;

use numpy::{IntoPyArray, PyArray2, PyReadonlyArray2};
use pyo3::prelude::{pymodule, PyModule, PyResult, Python};
// NOTE
// * numpy defaults to np.float64, if you use other type than f64 in Rust
//   you will have to change type in Python before calling the Rust function.

// The name of the module must be the same as the rust package name
#[pymodule]
fn aseext(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    fn log_fermi<'py>(
        _py: Python<'py>,
        positions: PyReadonlyArray2<f64>,
        radius: f64,
        temperature: f64,
        beta: f64,
    ) -> (f64, &'py PyArray2<f64>) {
        let positions = positions.as_array();
        let (e, e_grad) = logfermi::log_fermi(&positions, radius, temperature, beta);
        (e, e_grad.into_pyarray(_py))
    }

    Ok(())
}
