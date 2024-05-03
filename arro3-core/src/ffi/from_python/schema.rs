use crate::ffi::from_python::utils::import_arrow_c_schema;
use crate::schema::PySchema;
use pyo3::prelude::*;
use pyo3::{PyAny, PyResult};

impl<'a> FromPyObject<'a> for PySchema {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let schema = import_arrow_c_schema(ob)?;
        Ok(Self::new(schema))
    }
}