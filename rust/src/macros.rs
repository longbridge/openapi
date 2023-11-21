macro_rules! impl_default_for_enum_string {
    ($($ty:ty),*) => {
        $(
            impl Default for $ty {
                fn default() -> Self {
                    Self::Unknown
                }
            }
        )*
    };
}

macro_rules! impl_serde_for_enum_string {
    ($($ty:ty),*) => {
        $(
            impl<'de> ::serde::Deserialize<'de> for $ty {
                fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> ::std::result::Result<Self, D::Error> {
                    use std::str::FromStr;
                    let value: ::std::string::String = ::serde::Deserialize::deserialize(deserializer)?;
                    Ok(<$ty>::from_str(&value).unwrap_or_default())
                }
            }

            impl ::serde::Serialize for $ty {
                fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                    use std::string::ToString;
                    let value = self.to_string();
                    ::serde::Serialize::serialize(&value, serializer)
                }
            }
        )*
    };
}

/// A macro to construct decimal.
///
/// # Examples
///
/// ```
/// # use longport::decimal;
/// # use rust_decimal::Decimal;
///
/// assert_eq!(decimal!(1.23), Decimal::try_from(1.23).unwrap());
/// ```
///
/// # Panics
///
/// Panic if the input value is invalid.
#[macro_export]
macro_rules! decimal {
    ($value:expr) => {
        $crate::Decimal::try_from($value).expect("valid decimal")
    };
}
