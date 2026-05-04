use pyo3::prelude::*;
use crate::python_objects::PyNodeType;

/// the Rule defines the punishment we will enact upon our poor text
#[pyclass]
#[derive(Clone)]
pub struct Rule {
    #[pyo3(get)]
    pub nodetype: PyNodeType,

    #[pyo3(get)]
    pub on_punishment: String,

    #[pyo3(get)]
    pub on_scale: usize,

    #[pyo3(get)]
    pub off_punishment: String,

    #[pyo3(get)]
    pub off_scale: usize,
}
