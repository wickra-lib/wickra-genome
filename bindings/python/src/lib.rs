//! Python bindings for `wickra-genome`, exposed under the `wickra_genome`
//! package.
//!
//! Thin glue over the genome core's command surface: construct a [`Genome`] from
//! a spec JSON, drive it with a command JSON and read back the response JSON. The
//! same command protocol crosses every binding, so a Python front-end drives the
//! exact same core — and gets the byte-identical similarity, clustering and
//! anomaly results — as the CLI.

// PyO3 protocol methods take `self` by ref regardless of use.
#![allow(clippy::needless_pass_by_value)]

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use genome_core::Genome;

/// A market genome driven by JSON commands.
///
/// `unsendable`: the handle caches per-command state, so it is bound to the
/// thread that created it.
#[pyclass(name = "Genome", unsendable)]
struct PyGenome {
    inner: Genome,
}

#[pymethods]
impl PyGenome {
    /// Construct a genome handle from a spec JSON (a non-empty `features` and
    /// `symbols`).
    #[new]
    fn new(spec_json: &str) -> PyResult<Self> {
        Genome::new(spec_json)
            .map(|inner| Self { inner })
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Apply a command JSON and return the response JSON. Domain errors (a bad
    /// spec, an unknown symbol) come back in-band as `{"ok":false,"error":...}`.
    fn command(&mut self, cmd_json: &str) -> String {
        self.inner.command_json(cmd_json)
    }

    /// The library version.
    #[staticmethod]
    fn version() -> &'static str {
        genome_core::version()
    }
}

/// The native module (`wickra_genome._wickra_genome`).
#[pymodule]
fn _wickra_genome(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add_class::<PyGenome>()?;
    Ok(())
}
