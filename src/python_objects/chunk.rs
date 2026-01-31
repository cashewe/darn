use pyo3::prelude::*;


/// the Chunk is the user frienlyl wrapper on the output text.
/// it maintains enough context to provide value to the user, or so i hope...
#[pyclass]
#[derive(Clone)]
pub struct Chunk {
    #[pyo3(get)]
    pub text: String,

    #[pyo3(get)]
    pub start_index: usize,

    #[pyo3(get)]
    pub end_index: usize,
}


/// add the repr method? does this work?? papa dont know :(
#[pymethods]
impl Chunk {
    fn __repr__(&self) -> String {
        format!(
            "Chunk(start={}, end={}, content={})",
            self.start_index,
            self.end_index,
            self.text
        )
    }
}