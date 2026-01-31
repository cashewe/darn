use pyo3::prelude::*;
mod md_parser;
mod rule_manager;
mod chunk_optimiser;
mod python_objects;

use python_objects::{Chunk, Chunker};


#[pymodule]
fn darn_it(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Chunk>()?;
    m.add_class::<Chunker>()?;
    Ok(())
}
