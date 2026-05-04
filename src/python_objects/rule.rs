use pyo3::prelude::*;
use crate::python_objects::PyNodeType;
use crate::rule_manager::Rule as BackendRule;
use crate::rule_manager::punishments::lookup_punishment;

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


/// convert a python rule into a rust one
impl TryFrom<&Rule> for BackendRule {
    type Error = PyErr;

    fn try_from(r: &Rule) -> PyResult<Self> {
        let on_fn = lookup_punishment(&r.on_punishment)
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err(
                format!(
                    "Unknown on_punishment '{}'. Valid options: const, linear, inverse_triangular, reverse_linear",
                    r.on_punishment
                )
            ))?;

        let off_fn = lookup_punishment(&r.off_punishment)
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err(
                format!(
                    "Unknown off_punishment '{}'.",
                    r.off_punishment
                )
            ))?;

        Ok(BackendRule {
            node_type:      r.nodetype.into(),
            on_punishment:  on_fn,
            on_scale:       r.on_scale,
            off_punishment: off_fn,
            off_scale:      r.off_scale,
        })
    }
}