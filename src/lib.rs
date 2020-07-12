use age;
use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::wrap_pyfunction;
use secrecy::ExposeSecret;

#[pyfunction]
fn create_newkey() -> PyResult<(String, String)>{
    let key = age::SecretKey::generate();
    let pubkey = key.to_public();
    let secstring = key.to_string();
    let secret = secstring.expose_secret();

    return Ok((pubkey.to_string(), secret.to_string()));
}

#[pymodule]
/// A Python module implemented in Rust.
fn pyage(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(create_newkey))?;
    Ok(())
}

