use pyo3::prelude::*;
use aga8::gerg2008::Gerg2008;
use aga8::composition::Composition;

#[pyclass]
struct PyComposition {
    inner: Composition,
}

#[pymethods]
impl PyComposition {
    #[new]
    #[args(
        methane = "0.0", nitrogen = "0.0", carbon_dioxide = "0.0", ethane = "0.0",
        propane = "0.0", isobutane = "0.0", n_butane = "0.0", isopentane = "0.0",
        n_pentane = "0.0", hexane = "0.0", heptane = "0.0", octane = "0.0",
        nonane = "0.0", decane = "0.0", hydrogen = "0.0", oxygen = "0.0",
        carbon_monoxide = "0.0", water = "0.0", hydrogen_sulfide = "0.0",
        helium = "0.0", argon = "0.0"
    )]
    fn new(
        methane: f64, nitrogen: f64, carbon_dioxide: f64, ethane: f64,
        propane: f64, isobutane: f64, n_butane: f64, isopentane: f64,
        n_pentane: f64, hexane: f64, heptane: f64, octane: f64,
        nonane: f64, decane: f64, hydrogen: f64, oxygen: f64,
        carbon_monoxide: f64, water: f64, hydrogen_sulfide: f64,
        helium: f64, argon: f64
    ) -> Self {
        PyComposition {
            inner: Composition {
                methane, nitrogen, carbon_dioxide, ethane, propane, isobutane,
                n_butane, isopentane, n_pentane, hexane, heptane, octane,
                nonane, decane, hydrogen, oxygen, carbon_monoxide, water,
                hydrogen_sulfide, helium, argon,
            }
        }
    }
}

#[pyclass]
struct PyGerg2008 {
    inner: Gerg2008,
}

#[pymethods]
impl PyGerg2008 {
    #[new]
    fn new() -> Self {
        PyGerg2008 { inner: Gerg2008::new() }
    }

    fn set_composition(&mut self, comp: &PyComposition) {
        self.inner.set_composition(&comp.inner);
    }

    #[setter]
    fn set_p(&mut self, p: f64) {
        self.inner.p = p;
    }

    #[setter]
    fn set_t(&mut self, t: f64) {
        self.inner.t = t;
    }

    fn density(&mut self, init_type: i32) {
        self.inner.density(init_type);
    }

    fn properties(&mut self) {
        self.inner.properties();
    }

    #[getter]
    fn get_d(&self) -> f64 {
        self.inner.d
    }

    #[getter]
    fn get_z(&self) -> f64 {
        self.inner.z
    }
}

#[pymodule]
fn  PyGERG(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyComposition>()?;
    m.add_class::<PyGerg2008>()?;
    Ok(())
}
