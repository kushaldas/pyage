use std::io::{Read, Write};

use age;
use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::wrap_pyfunction;
use secrecy::ExposeSecret;

#[pyfunction]
fn create_newkey() -> PyResult<(String, String)> {
    let key = age::SecretKey::generate();
    let pubkey = key.to_public();
    let secstring = key.to_string();
    let secret = secstring.expose_secret();

    return Ok((pubkey.to_string(), secret.to_string()));
}

/// This function encryptes the given bytes using a list of secret keys (as str).
/// data: bytes
/// keys: a list of public keys as str
#[pyfunction]
#[text_signature = "(data, keys)"]
fn encrypt_bytes(py: Python, data: Vec<u8>, reps: Vec<String>) -> PyResult<PyObject>{
    let mut recipients: Vec<age::keys::RecipientKey> = Vec::new();

    for rep in &reps {
        let key = rep.parse().unwrap();
        recipients.push(key);
    }

    let encryptor = age::Encryptor::with_recipients(recipients);

    let mut encrypted = vec![];
    let mut writer = encryptor.wrap_output(&mut encrypted, age::Format::Binary).unwrap();
    writer.write_all(&data[..]).unwrap();
    writer.finish().unwrap();

    Ok(PyBytes::new(py, &encrypted).into())
}

/// This function decrypts a given bytes using the secret key as str.
#[pyfunction]
#[text_signature = "(data, secret)"]
fn decrypt_bytes(py: Python, data: Vec<u8>, secret: String) -> PyResult<PyObject>{
    let decryptor = match age::Decryptor::new(&data[..]).unwrap() {
        age::Decryptor::Recipients(d) => d,
        _ => unreachable!(),
    };

    let key = age::keys::Identity::from_buffer(secret.as_bytes()).unwrap();

    let mut decrypted = vec![];
    let mut reader = decryptor.decrypt(&key).unwrap();
    reader.read_to_end(&mut decrypted).unwrap();
    Ok(PyBytes::new(py, &decrypted).into())
}

#[pymodule]
/// A Python module implemented in Rust.
fn pyage(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(create_newkey))?;
    m.add_wrapped(wrap_pyfunction!(encrypt_bytes))?;
    m.add_wrapped(wrap_pyfunction!(decrypt_bytes))?;
    Ok(())
}
