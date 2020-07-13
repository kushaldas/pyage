use std::io;
use std::io::{Read, Write};
use std::fs::File;

use age;
use age::cli_common::file_io;
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
/// ascii: boolean, default False.
#[pyfunction]
#[text_signature = "(data, keys, armor=False)"]
fn encrypt_bytes(py: Python, data: Vec<u8>, reps: Vec<String>, armor: Option<bool>) -> PyResult<PyObject> {
    let mut recipients: Vec<age::keys::RecipientKey> = Vec::new();

    for rep in &reps {
        let key = rep.parse().unwrap();
        recipients.push(key);
    }

    let encryptor = age::Encryptor::with_recipients(recipients);
    let format = match armor {
        Some(true) => age::Format::AsciiArmor,
        _ => age::Format::Binary,
    };

    let mut encrypted = vec![];
    let mut writer = encryptor
        .wrap_output(&mut encrypted, format)
        .unwrap();
    writer.write_all(&data[..]).unwrap();
    writer.finish().unwrap();

    Ok(PyBytes::new(py, &encrypted).into())
}

/// This function decrypts a given bytes using the secret key as str.
#[pyfunction]
#[text_signature = "(data, secret)"]
fn decrypt_bytes(py: Python, data: Vec<u8>, secret: String) -> PyResult<PyObject> {
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

/// This function encryptes the given file using a list of public keys (as str),
/// and writes the encrypted data to the outputfile.
/// filename: Input filename
/// outputfile: Output encrypted filename
/// keys: The list of public keys as str.
#[pyfunction]
#[text_signature = "(filename, outputfile, keys)"]
fn encrypt_file(filename: String, outputfile: String, keys: Vec<String>) -> PyResult<bool> {
    let mut recipients: Vec<age::keys::RecipientKey> = Vec::new();

    for rep in &keys {
        let key = rep.parse().unwrap();
        recipients.push(key);
    }

    //let format = file_io::OutputFormat::Binary;
    let encryptor = age::Encryptor::with_recipients(recipients);
    //let mut input = age::cli_common::file_io::InputReader::new(Some(filename)).unwrap();
    let mut input = File::open(filename).unwrap();
    //let output = file_io::OutputWriter::new(Some(outputfile), format, 0o666).unwrap();
    let output = File::create(outputfile).unwrap();
    let mut writer = encryptor.wrap_output(output, age::Format::Binary).unwrap();

    io::copy(&mut input, &mut writer).unwrap();
    writer.finish().unwrap();
    Ok(true)
}

/// This function encryptes the given file using a list of secret keys (as str),
/// and writes decrypted data to the outputfile.
/// filename: Input filename
/// outputfile: Output encrypted filename
/// secret: The secret key as str
#[pyfunction]
#[text_signature = "(filename, outputfile, secret)"]
fn decrypt_file(filename: String, outputfile: String, secret: String) -> PyResult<bool> {

    match age::Decryptor::new(file_io::InputReader::new(Some(filename)).unwrap()).unwrap() {
        age::Decryptor::Recipients(decryptor) => {
            let mut output = File::create(outputfile).unwrap();
            let key = age::keys::Identity::from_buffer(secret.as_bytes()).unwrap();
            let mut inc = decryptor.decrypt(&key).unwrap();

            io::copy(&mut inc, &mut output).unwrap();

        },
        _ => (),
    }
    Ok(true)
}

#[pymodule]
/// A Python module implemented in Rust.
fn pyage(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(create_newkey))?;
    m.add_wrapped(wrap_pyfunction!(encrypt_bytes))?;
    m.add_wrapped(wrap_pyfunction!(decrypt_bytes))?;
    m.add_wrapped(wrap_pyfunction!(encrypt_file))?;
    m.add_wrapped(wrap_pyfunction!(decrypt_file))?;
    Ok(())
}
