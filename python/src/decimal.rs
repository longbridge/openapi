use std::fmt::{self, Debug};

use once_cell::sync::Lazy;
use pyo3::{exceptions::PyBaseException, prelude::*, types::PyType};
use rust_decimal::Decimal;

static DECIMAL_TYPE: Lazy<PyObject> = Lazy::new(|| {
    Python::with_gil(|py| {
        let decimal_module = py.import("decimal")?;
        let decimal_type = decimal_module.getattr("Decimal")?;
        Ok::<_, PyErr>(decimal_type.to_object(py))
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
    fn extract(ob: &'py PyAny) -> PyResult<Self> {
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
                let decimal_type = DECIMAL_TYPE.downcast::<PyType>(py).expect("decimal type");
                if ob.is_instance(decimal_type)? {
                    let value = ob
                        .str()
                        .and_then(|value| value.to_str())
                        .expect("decimal str");
                    Ok(Self(value.parse().expect("valid decimal")))
                } else {
                    Err(PyBaseException::new_err("expected a decimal"))
                }
            })
        }
    }
}

impl IntoPy<PyObject> for PyDecimal {
    fn into_py(self, py: Python<'_>) -> PyObject {
        DECIMAL_TYPE
            .call1(py, (self.0.to_string(),))
            .expect("new decimal")
            .to_object(py)
    }
}
