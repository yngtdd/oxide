use pyo3::prelude::*;
extern crate oxide as oxide_rs;
use oxide_rs::weibull;

#[pyclass]
struct WeibullModel {
    shape: f64,
    scale: f64,
    reliability: Vec<f64>,
}

#[pymethods]
impl WeibullModel {
    #[new]
    fn new(shape: f64, scale: f64, num_steps: u32) -> Self {
        let weibull = weibull::WeibullModel::new(shape, scale);
        let reliability = weibull::reliability(weibull.weibull, num_steps);

        Self {
            shape,
            scale,
            reliability,
        }
    }

    #[getter]
    fn reliability(&self) -> PyResult<Vec<f64>> {
        Ok(self.reliability.clone())
    }

    #[getter]
    fn shape(&self) -> PyResult<f64> {
        Ok(self.shape)
    }

    #[getter]
    fn scale(&self) -> PyResult<f64> {
        Ok(self.scale)
    }
} 

#[pymodule]
fn oxide(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WeibullModel>()?;
    Ok(())
}
