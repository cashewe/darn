use pyo3::prelude::*;
use crate::python_objects::{Chunk, ChunkedDocument, Rule};
use crate::md_parser::MdParser;
use crate::rule_manager::{RuleManager, Rule as BackendRule};
use crate::chunk_optimiser::{ChunkOptimiser, Granularity};

/// chunker is the wrapper on the logic for splitting text
/// it will become more elaborate with time, but theres a chance the users wont love it for that
#[pyclass]
pub struct Chunker {
    rules: Vec<BackendRule>,
}

#[pymethods]
impl Chunker {

    #[new]
    #[pyo3(signature = (rules=None))]
    fn new(rules: Option<Vec<Rule>>) -> PyResult<Self> {
        let backend_rules = rules
            .unwrap_or_default()
            .iter()
            .map(BackendRule::try_from)
            .collect::<PyResult<Vec<_>>>()?;

        Ok(Chunker { rules: backend_rules })
    }

    /// split text using the power of wonderous mathematics
    #[pyo3(signature = (text, chunk_size, granularity="characters", model="gpt-4o-mini", overlap=0, return_vectors=false))]
    fn get_chunks(&self, text: &str, chunk_size: usize, granularity: &str, model: &str, overlap: usize, return_vectors: bool) -> PyResult<ChunkedDocument> {
        let rules_slice = if self.rules.is_empty() {
            None
        } else {
            Some(self.rules.as_slice())
        };
        let node_ranges = MdParser::parse(text);
        let cost_vector =
            RuleManager::build_punishment_vector(&node_ranges, text.len(), rules_slice, return_vectors);
        
        let optimiser = ChunkOptimiser::new(text, &cost_vector.totals, model);
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

            let base_end = if i + 1 < chunk_indices.len() {
                chunk_indices[i + 1]
            } else {
                text.len()
            };

            let end = if overlap > 0 && i + 1 < chunk_indices.len() {
                match granularity {
                    Granularity::Characters => base_end.saturating_add(overlap).min(text.len()),
                    Granularity::Tokens => {
                        optimiser.char_index_after_token_offset(base_end, overlap)
                    }
                }
            } else {
                base_end
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

        let chunked_document = ChunkedDocument {
            chunks: chunks,
            punishments: return_vectors.then_some(cost_vector.totals.clone()),
            punishment_breakdown: cost_vector.per_rule.clone()
        };

        Ok(chunked_document)
    }
}
