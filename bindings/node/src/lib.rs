//! Node.js bindings for `wickra-genome` via napi-rs.
//!
//! A `Genome` is built from a spec JSON; `command` takes a request JSON and
//! returns the response JSON, so Node drives the exact same byte-identical
//! surface — and gets the byte-identical similarity, clustering and anomaly
//! results — as every other binding.

use napi_derive::napi;

/// A market genome driven by JSON commands.
#[napi]
pub struct Genome(genome_core::Genome);

#[napi]
impl Genome {
    /// Construct a genome handle from a spec JSON (a non-empty `features` and
    /// `symbols`). Throws on an invalid spec.
    #[napi(constructor)]
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(spec_json: String) -> napi::Result<Self> {
        genome_core::Genome::new(&spec_json)
            .map(Genome)
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }

    /// Apply a command envelope (`{"cmd":"...", ...}`) and return the response
    /// JSON. Commands: `set_spec`, `feed`, `feed_batch`, `build`, `vector`,
    /// `similar`, `cluster`, `anomaly`, `reset`, `version`. Domain errors come
    /// back in-band as `{"ok":false,"error":...}`.
    #[napi]
    #[allow(clippy::needless_pass_by_value)]
    pub fn command(&mut self, cmd_json: String) -> String {
        self.0.command_json(&cmd_json)
    }

    /// The crate version.
    #[napi]
    pub fn version(&self) -> &'static str {
        genome_core::version()
    }
}
