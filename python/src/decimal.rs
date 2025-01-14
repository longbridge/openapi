use std::{
    convert::Infallible,
    fmt::{self, Debug},
};

use once_cell::sync::Lazy;
use pyo3::{exceptions::PyBaseException, prelude::*, types::PyType};
use rust_decimal::Decimal;

static DECIMAL_TYPE: Lazy<PyObject> = Lazy::new(|| {
    Python::with_gil(|py| {
        let decimal_module = py.import("decimal")?;
        let decimal_type = decimal_module.getattr("Decimal")?;
        Ok::<_, PyErr>(decimal_type.into_pyobject(py)?.unbind())
    })
    .expect("import decimal")
});

#[derive(Copy, Clone)]
pub(crate) struct PyDecimal(Decimal);

impl Debug for PyDecimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Decimal> for PyDecimal {
    #[inline]
    fn from(value: Decimal) -> Self {
        PyDecimal(value)
    }
}

impl From<PyDecimal> for Decimal {
    #[inline]
    fn from(value: PyDecimal) -> Self {
        value.0
    }
}

impl<'py> FromPyObject<'py> for PyDecimal {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(value) = ob.extract::<f64>() {
            // convert from PyFloat
            Ok(Self(Decimal::try_from(value).map_err(|err| {
                PyBaseException::new_err(format!("cannot create decimal value: {err}"))
            })?))
        } else if let Ok(value) = ob.extract::<i64>() {
            // convert from PyInt/PyLong
            Ok(Self(Decimal::from(value)))
        } else {
            // convert from decimal.Decimal
            Python::with_gil(|py| {
                let decimal_type = DECIMAL_TYPE
                    .downcast_bound::<PyType>(py)
                    .expect("decimal type");
                if ob.is_instance(decimal_type)? {
                    let value = ob
                        .str()
                        .and_then(|value| Ok(value.to_str()?.to_string()))
                        .expect("decimal str");
                    Ok(Self(value.parse().expect("valid decimal")))
                } else {
                    Err(PyBaseException::new_err("expected a decimal"))
                }
            })
        }
    }
}

impl<'py> IntoPyObject<'py> for PyDecimal {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(DECIMAL_TYPE
            .call1(py, (self.0.to_string(),))
            .expect("new decimal")
            .into_bound(py))
    }
}
