#[macro_use]
extern crate pyo3;
extern crate zenroom;

use pyo3::{exceptions::TypeError, prelude::*};
use zenroom::{prelude::*, FileScenarioLinker, ScenarioLoader, ZencodeRuntime};

#[pyclass]
struct Zenroom {
    runtime: ZencodeRuntime,
}

#[pymethods]
impl Zenroom {
    #[new]
    fn new(obj: &PyRawObject, scenarios: &str) {
        let loader = ScenarioLoader::new(FileScenarioLinker::new(scenarios));
        obj.init(Zenroom {
            runtime: ZencodeRuntime::new(loader),
        })
    }

    fn load(&mut self, source: String) -> PyResult<()> {
        self.runtime
            .load(&source)
            .map_err(|_| PyErr::new::<TypeError, _>("could not load source"))?;
        Ok(())
    }

    fn eval(&self) -> PyResult<Option<String>> {
        // TODO better error codes
        let result = self
            .runtime
            .eval()
            .map_err(|_| PyErr::new::<TypeError, _>("something happened"))?;
        Ok(result)
    }
}

#[pymodule]
fn zenroom_minimal(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Zenroom>()?;
    Ok(())
}
