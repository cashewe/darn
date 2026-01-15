use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod darn_it {
    use pyo3::prelude::*;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }

    /// function to create and display the EnumMap for a given md file - used for debugging
    #[pyfunction]
    fn bs_func(a: &str) {
        // read the file from &a in as string
        // run the relevant code to parse it
        // println it with wonderous skill
        println!("hello world")
    }
}
