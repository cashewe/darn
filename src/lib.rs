use pyo3::prelude::*;
mod md_parser;

use md_parser::MdParser;

#[pymodule]
fn darn_it(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_md_node_ranges, m)?)?;
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
