use pyo3::prelude::*;

#[pyclass(eq)]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Method(rquest::Method);

macro_rules! define_constants {
    ($type:tt, $inner_type:ty, $($name:ident),*) => {
        #[allow(non_upper_case_globals)]
        #[pymethods]
        impl $type {
            $(
                #[classattr]
                pub const $name: $type = $type(<$inner_type>::$name);
            )*

            fn __str__(&self) -> &str {
                self.0.as_str()
            }

            fn __repr__(&self) -> &str {
                self.__str__()
            }
        }
    };
}

define_constants!(
    Method,
    rquest::Method,
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
);
