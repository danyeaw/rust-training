use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use std::collections::HashMap;
use std::string;

mod data;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_dna(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(count_kmers, m)?)?;
    m.add_function(wrap_pyfunction!(assert_valid_dna, m)?)?;
    m.add_function(wrap_pyfunction!(decode_orf, m)?)?;
    m.add_class::<Sequence>()?;
    m.add_class::<OpenReadingFrame>()?;
    Ok(())
}

#[pyfunction]
fn count_kmers(sequence: &str, k: usize) -> PyResult<HashMap<&str, usize>> {
    let mut kemers = HashMap::new();

    for i in 0..sequence.len() - k + 1 {
        let kmer = &sequence[i..i + k];
        *kemers.entry(kmer).or_insert(0) += 1;
    }

    Ok(kemers)
}

#[pyfunction]
fn assert_valid_dna(sequence: &str) -> PyResult<()> {
    if !sequence.chars().all(|c| "ATGC".contains(c)) {
        return Err(PyValueError::new_err("Sequence contains invalid characters"));
    }
    Ok(())
}

#[pyclass(str)]
struct Sequence {
    #[pyo3(get, set)]
    sequence: String,
}
impl std::fmt::Display for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.sequence)
    }
}

#[pymethods]
impl Sequence {
    #[new]
    fn new(sequence: String) -> Self {
        Self { sequence }
    }

    fn __len__(&self) -> usize {
        self.sequence.len()
    }

    fn kmers(&self, k: usize) -> HashMap<&str, usize> {
        count_kmers(&self.sequence, k).expect("Error")
    }
}

#[pyclass(eq, hash, frozen)]
#[derive(PartialEq, Hash)]
struct OpenReadingFrame {
    #[pyo3(get)]
    start: usize,
    #[pyo3(get)]
    end: usize,
    #[pyo3(get)]
    decoded: String,
}

#[pymethods]
impl OpenReadingFrame {
    #[new]
    fn new(start: usize, end: usize, decoded: String) -> Self {
        Self { start, end, decoded }
    }
}

#[pyfunction]
fn decode_orf(sequence: &Sequence, start: usize) -> PyResult<OpenReadingFrame {
    let sub_str = sequence.to_string()[start..].to_string();
    let mut decoded = String::new();

    let (chunks, rest) = sub_str.as_bytes().as_chunks::<3>();
    for chunk in chunks {
        if !rest.is_empty() {
            return Err(PyValueError::new_err("Sequence contains invalid characters"));
        }
        let amino = data::dna_to_amino(chunk).unwrap();
        if amino == "STOP" {
            decoded += "*";
            break;
        }
        decoded += amino;

    }
    OpenReadingFrame { start, end: (start + 3 * decoded.len()), decoded }
}
