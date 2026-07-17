//! WebAssembly bindings for `wickra-genome` (wasm-bindgen).
//!
//! Run the market vector database in the browser: create a `Genome` from a spec
//! JSON, drive it with a command JSON (`build`, `feed`, `vector`, `similar`,
//! `cluster`, `anomaly`, `version`) and read back the response JSON. The same
//! command protocol crosses every binding, so a browser front-end runs against
//! the exact same core as the native CLI.
//!
//! The engine runs single-threaded here (no rayon thread pool in a browser
//! sandbox); because every cross-section reduction runs serially in key order,
//! that is byte-identical to the native parallel run — the exact cross-language
//! golden check.

use wasm_bindgen::prelude::*;

use genome_core::Genome as CoreGenome;

/// A market genome driven by JSON commands.
#[wasm_bindgen]
pub struct Genome {
    inner: CoreGenome,
}

#[wasm_bindgen]
impl Genome {
    /// Construct a genome handle from a spec JSON (a non-empty `features` and
    /// `symbols`). Throws on an invalid spec.
    #[wasm_bindgen(constructor)]
    pub fn new(spec_json: &str) -> Result<Genome, JsError> {
        CoreGenome::new(spec_json)
            .map(|inner| Self { inner })
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// Apply a command JSON (`{"cmd":"...", ...}`) and return the response JSON.
    /// Domain errors come back in-band as `{"ok":false,"error":...}`.
    pub fn command(&mut self, cmd_json: &str) -> String {
        self.inner.command_json(cmd_json)
    }

    /// The library version.
    #[wasm_bindgen(js_name = version)]
    pub fn instance_version(&self) -> String {
        genome_core::version().to_string()
    }
}

/// The library version.
#[wasm_bindgen]
pub fn version() -> String {
    genome_core::version().to_string()
}
