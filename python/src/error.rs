use pyo3::PyErr;

pyo3::import_exception!(longport.openapi, OpenApiException);

pub(crate) struct ErrorNewType(pub(crate) longport::Error);

impl std::convert::From<ErrorNewType> for PyErr {
    #[inline]
    fn from(err: ErrorNewType) -> PyErr {
        let err = err.0.into_simple_error();
        OpenApiException::new_err((err.code(), err.message().to_string()))
    }
}
