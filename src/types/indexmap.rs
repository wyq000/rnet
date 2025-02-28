use pyo3::FromPyObject;
use pyo3::{
    prelude::*,
    types::{PyBytes, PyDict},
};
use pyo3_stub_gen::{PyStubType, TypeInfo};
use std::hash::RandomState;
use std::ops::{Deref, DerefMut};

pub struct IndexMap<K, V, S = RandomState>(indexmap::IndexMap<K, V, S>);

impl<K, V, S> IndexMap<K, V, S>
where
    S: Default,
{
    pub fn new() -> IndexMap<K, V, S> {
        IndexMap(indexmap::IndexMap::with_hasher(S::default()))
    }
}

impl<K, V, S> Deref for IndexMap<K, V, S> {
    type Target = indexmap::IndexMap<K, V, S>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V, S> DerefMut for IndexMap<K, V, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K, V, S> PyStubType for IndexMap<K, V, S> {
    fn type_output() -> TypeInfo {
        TypeInfo::with_module("typing.Dict[str, bytes]", "typing".into())
    }
}

impl FromPyObject<'_> for IndexMap<String, String> {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        ob.extract().map(IndexMap)
    }
}

impl<'py> IntoPyObject<'py> for IndexMap<String, String> {
    type Target = PyDict;

    type Output = Bound<'py, Self::Target>;

    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let dict = PyDict::new(py);
        for (header, value) in &self.0 {
            dict.set_item(header.as_str(), PyBytes::new(py, value.as_ref()))?;
        }
        Ok(dict)
    }
}
