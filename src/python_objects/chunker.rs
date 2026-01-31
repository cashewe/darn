use pyo3::prelude::*;
use crate::python_objects::Chunk;
use crate::md_parser::MdParser;
use crate::rule_manager::RuleManager;
use crate::chunk_optimiser::{ChunkOptimiser, Granularity};

/// chunker is the wrapper on the logic for splitting text
/// it will become more elaborate with time, but theres a chance the users wont love it for that
#[pyclass]
pub struct Chunker;

#[pymethods]
impl Chunker {

    #[new]
    fn new() -> Self {
        Chunker
    }

    /// split text using the power of wonderous mathematics
    #[pyo3(signature = (text, chunk_size, granularity="characters"))]
    fn get_chunks(&self, text: &str, chunk_size: usize, granularity: &str) -> PyResult<Vec<Chunk>> {

        let node_ranges = MdParser::parse(text);
        let cost_vector =
            RuleManager::build_punishment_vector(&node_ranges, text.len());
        
        let optimiser = ChunkOptimiser::new(text, cost_vector);
        let granularity = match granularity { // prefer not to have pyo3 dep in other modules
            "characters" => Granularity::Characters,
            "tokens" => Granularity::Tokens,
            _ => {
                return Err(
                    pyo3::exceptions::PyValueError::new_err(
                        "granularity must be either 'characters' or 'tokens'"
                    )
                )
            }
        };
        let chunk_indices = optimiser.optimise_chunks(chunk_size, granularity);

        let mut chunks = Vec::new();

        for (i, &start) in chunk_indices.iter().enumerate() {

            let end = if i + 1 < chunk_indices.len() {
                chunk_indices[i + 1]
            } else {
                text.len()
            };

            let slice = text
                .get(start..end)
                .ok_or_else(|| {
                    pyo3::exceptions::PyValueError::new_err(
                        "Invalid UTF-8 slice"
                    )
                })?;

            chunks.push(Chunk {
                text: slice.to_string(),
                start_index: start,
                end_index: end,
            });
        }

        Ok(chunks)
    }
}
