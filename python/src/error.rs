use pyo3::{exceptions::PyException, PyErr};

pyo3::create_exception!(
    longbridge,
    LongbridgeSDKException,
    PyException,
    "Some description."
);

pub(crate) struct ErrorNewType(pub(crate) longbridge::Error);

impl std::convert::From<ErrorNewType> for PyErr {
    #[inline]
    fn from(err: ErrorNewType) -> PyErr {
        LongbridgeSDKException::new_err(err.0.to_string())
    }
}
