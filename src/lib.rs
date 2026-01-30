use pyo3::prelude::*;
mod md_parser;
mod rule_manager;
mod chunk_optimiser;

use md_parser::MdParser;
use rule_manager::RuleManager;
use chunk_optimiser::cheapest_path_indices;

#[pymodule]
fn darn_it(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_md_node_ranges, m)?)?;
    m.add_function(wrap_pyfunction!(get_cost_vector, m)?)?;
    m.add_function(wrap_pyfunction!(get_chunk_indices, m)?)?;
    Ok(())
}

/// function to create and display the EnumMap for a given md file - used for debugging
/// this method is disgustingly fast
#[pyfunction]
fn get_md_node_ranges(file_path: &str) -> PyResult<()> {
    let markdown_string = std::fs::read_to_string(file_path)
        .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Failed to read file: {}", e)))?;
    let node_ranges = MdParser::parse(&markdown_string);
    println!("{}", node_ranges);
    Ok(())
}

/// function to create and display the EnumMap for a given md file - used for debugging
/// this method is disgustingly fast
#[pyfunction]
fn get_cost_vector(file_path: &str) -> PyResult<()> {
    let markdown_string = std::fs::read_to_string(file_path)
        .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Failed to read file: {}", e)))?;
    let node_ranges = MdParser::parse(&markdown_string);
    let cost_vector = RuleManager::build_punishment_vector(&node_ranges, markdown_string.len());
    println!("{:?}", cost_vector);
    Ok(())
}

/// end to end with 0 bells or whistles
/// if this works, we will indeed be blessed
#[pyfunction]
fn get_chunk_indices(file_path: &str, n: usize) -> PyResult<()> {
    let markdown_string = std::fs::read_to_string(file_path)
        .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Failed to read file: {}", e)))?;
    let node_ranges = MdParser::parse(&markdown_string);
    let cost_vector = RuleManager::build_punishment_vector(&node_ranges, markdown_string.len());
    let chunk_indices = cheapest_path_indices(&cost_vector, n);
    println!("{:?}", chunk_indices);
    Ok(())
}