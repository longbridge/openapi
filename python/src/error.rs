use longport_python_macros::PyEnum;
use pyo3::{PyErr, pyclass};

pyo3::import_exception!(longport.openapi, OpenApiException);

pub(crate) struct ErrorNewType(pub(crate) longport::Error);

impl std::convert::From<ErrorNewType> for PyErr {
    #[inline]
    fn from(err: ErrorNewType) -> PyErr {
        let err = err.0.into_simple_error();
        OpenApiException::new_err((
            ErrorKind::from(err.kind()),
            err.code(),
            err.trace_id().map(ToString::to_string),
            err.message().to_string(),
        ))
    }
}

#[pyclass(eq, eq_int)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longport::SimpleErrorKind")]
pub(crate) enum ErrorKind {
    /// HTTP error
    Http,
    /// OpenAPI error
    OpenApi,
    /// Other error
    Other,
}
