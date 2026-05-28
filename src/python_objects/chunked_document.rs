use pyo3::prelude::*;

use crate::python_objects::Chunk;


/// chunked document contains everything a muzungu needs to recieve their chunks.
#[pyclass]
#[derive(Clone)]
pub struct ChunkedDocument {
    #[pyo3(get)]
    pub chunks: Vec<Chunk>,

    #[pyo3(get)]
    pub(crate) punishments: Option<Vec<usize>>, // this is used for the anayliser... does it need to be public? does it help if it is??

    #[pyo3(get)]
    pub punishment_breakdown: Option<Vec<Vec<usize>>>, // the breakdowns xoxo
}


#[pymethods]
impl ChunkedDocument {
    fn __repr__(&self) -> String {
        format!(
            "ChunkedDocument(chunk_count={})",
            self.chunks.len()
        )
    }
}