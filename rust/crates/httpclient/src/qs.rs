use std::{borrow::Cow, fmt::Display, io::Write};

use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, percent_encode};
use serde::{
    Serialize, Serializer,
    ser::{Impossible, SerializeMap, SerializeSeq, SerializeStruct, SerializeTuple},
};

const QS_ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b' ')
    .remove(b'*')
    .remove(b'-')
    .remove(b'.')
    .remove(b'_');

/// Query string serialization error
#[derive(Debug, thiserror::Error)]
pub enum QsError {
    /// Custom string-based error
    #[error("{0}")]
    Custom(String),

    /// Insupported type for serialization
    #[error("unsupported type for serialization")]
    Unsupported,

    /// I/O error
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl serde::ser::Error for QsError {
    #[inline]
    fn custom<T: Display>(msg: T) -> Self {
        QsError::Custom(msg.to_string())
    }
}

fn replace_space(input: &str) -> Cow<'_, str> {
    match input.as_bytes().iter().position(|&b| b == b' ') {
        None => Cow::Borrowed(input),
        Some(first_position) => {
            let mut replaced = input.as_bytes().to_owned();
            replaced[first_position] = b'+';
            for byte in &mut replaced[first_position + 1..] {
                if *byte == b' ' {
                    *byte = b'+';
                }
            }
            Cow::Owned(String::from_utf8(replaced).expect("replacing ' ' with '+' cannot panic"))
        }
    }
}

macro_rules! unsupported_types {
    ($(($ty:ty, $meth:ident)),*) => {
        $(
            fn $meth(self, _: $ty) -> Result<Self::Ok, Self::Error> {
                Err(QsError::Unsupported)
            }
        )*
    };
}

macro_rules! serialize_value_to_string {
    ($(($ty:ty, $meth:ident)),*) => {
        $(
            fn $meth(self, v: $ty) -> Result<Self::Ok, Self::Error> {
                Ok(vec![v.to_string()])
            }
        )*
    };
}

macro_rules! serialize_primary_value_to_string {
    ($(($ty:ty, $meth:ident)),*) => {
        $(
            fn $meth(self, v: $ty) -> Result<Self::Ok, Self::Error> {
                Ok(Some(v.to_string()))
            }
        )*
    };
}

macro_rules! serialize_key_to_string {
    ($(($ty:ty, $meth:ident)),*) => {
        $(
            fn $meth(self, v: $ty) -> Result<Self::Ok, Self::Error> {
                Ok(v.to_string())
            }
        )*
    };
}

struct ValueWriter<'a, W> {
    writer: &'a mut W,
    first: bool,
}

impl<W: Write> ValueWriter<'_, W> {
    fn add_pair(&mut self, key: &str, value: &str) -> Result<(), QsError> {
        write!(
            self.writer,
            "{}{}={}",
            if self.first {
                self.first = false;
                ""
            } else {
                "&"
            },
            key,
            percent_encode(value.as_bytes(), QS_ENCODE_SET)
                .map(replace_space)
                .collect::<String>()
        )?;
        Ok(())
    }
}

struct QsSerializer<'a, 'b, W> {
    writer: &'a mut ValueWriter<'b, W>,
}

impl<'a, 'b, W: Write> Serializer for QsSerializer<'a, 'b, W> {
    type Ok = ();
    type Error = QsError;
    type SerializeSeq = QsSeqSerializer<'a, 'b, W>;
    type SerializeTuple = QsTupleSerializer<'a, 'b, W>;
    type SerializeTupleStruct = Impossible<(), QsError>;
    type SerializeTupleVariant = Impossible<(), QsError>;
    type SerializeMap = QsMapSerializer<'a, 'b, W>;
    type SerializeStruct = QsStructSerializer<'a, 'b, W>;
    type SerializeStructVariant = Impossible<(), QsError>;

    unsupported_types!(
        (bool, serialize_bool),
        (u8, serialize_u8),
        (u16, serialize_u16),
        (u32, serialize_u32),
        (u64, serialize_u64),
        (i8, serialize_i8),
        (i16, serialize_i16),
        (i32, serialize_i32),
        (i64, serialize_i64),
        (f32, serialize_f32),
        (f64, serialize_f64),
        (char, serialize_char),
        (&str, serialize_str),
        (&[u8], serialize_bytes)
    );

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize + ?Sized,
    {
        Err(QsError::Unsupported)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize + ?Sized,
    {
        Err(QsError::Unsupported)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize + ?Sized,
    {
        Err(QsError::Unsupported)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(QsSeqSerializer {
            writer: self.writer,
        })
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(QsTupleSerializer {
            writer: self.writer,
        })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(QsMapSerializer {
            writer: self.writer,
            key: None,
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(QsStructSerializer {
            writer: self.writer,
        })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(QsError::Unsupported)
    }
}

struct QsSeqSerializer<'a, 'b, W> {
    writer: &'a mut ValueWriter<'b, W>,
}

impl<W: Write> SerializeSeq for QsSeqSerializer<'_, '_, W> {
    type Ok = ();
    type Error = QsError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        value.serialize(QsSeqItemSerializer {
            writer: self.writer,
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

struct QsTupleSerializer<'a, 'b, W> {
    writer: &'a mut ValueWriter<'b, W>,
}

impl<W: Write> SerializeTuple for QsTupleSerializer<'_, '_, W> {
    type Ok = ();
    type Error = QsError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        value.serialize(QsSeqItemSerializer {
            writer: self.writer,
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

struct QsSeqItemSerializer<'a, 'b, W> {
    writer: &'a mut ValueWriter<'b, W>,
}

impl<'a, 'b, W: Write> Serializer for QsSeqItemSerializer<'a, 'b, W> {
    type Ok = ();
    type Error = QsError;

    type SerializeSeq = Impossible<(), QsError>;
    type SerializeTuple = QsTuplePairSerializer<'a, 'b, W>;
    type SerializeTupleStruct = Impossible<(), QsError>;
    type SerializeTupleVariant = Impossible<(), QsError>;
    type SerializeMap = Impossible<(), QsError>;
    type SerializeStruct = Impossible<(), QsError>;
    type SerializeStructVariant = Impossible<(), QsError>;

    unsupported_types!(
        (bool, serialize_bool),
        (u8, serialize_u8),
        (u16, serialize_u16),
        (u32, serialize_u32),
        (u64, serialize_u64),
        (i8, serialize_i8),
        (i16, serialize_i16),
        (i32, serialize_i32),
        (i64, serialize_i64),
        (f32, serialize_f32),
        (f64, serialize_f64),
        (char, serialize_char),
        (&str, serialize_str),
        (&[u8], serialize_bytes)
    );

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Err(QsError::Unsupported)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Err(QsError::Unsupported)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Err(QsError::Unsupported)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        if len != 2 {
            return Err(QsError::Unsupported);
        }
        Ok(QsTuplePairSerializer {
            writer: self.writer,
            key: None,
        })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(QsError::Unsupported)
    }
}

struct QsTuplePairSerializer<'a, 'b, W> {
    writer: &'a mut ValueWriter<'b, W>,
    key: Option<String>,
}

impl<W: Write> SerializeTuple for QsTuplePairSerializer<'_, '_, W> {
    type Ok = ();
    type Error = QsError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        if self.key.is_none() {
            self.key = Some(value.serialize(QsKeySerializer)?);
        } else {
            for value in value.serialize(QsValueSerializer)? {
                self.writer.add_pair(self.key.as_ref().unwrap(), &value)?;
            }
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

struct QsStructSerializer<'a, 'b, W> {
    writer: &'a mut ValueWriter<'b, W>,
}

impl<W: Write> SerializeStruct for QsStructSerializer<'_, '_, W> {
    type Ok = ();
    type Error = QsError;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        for value in value.serialize(QsValueSerializer)? {
            self.writer.add_pair(key, &value)?;
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

struct QsMapSerializer<'a, 'b, W> {
    writer: &'a mut ValueWriter<'b, W>,
    key: Option<String>,
}

impl<W: Write> SerializeMap for QsMapSerializer<'_, '_, W> {
    type Ok = ();
    type Error = QsError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        self.key = Some(key.serialize(QsKeySerializer)?);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        for value in value.serialize(QsValueSerializer)? {
            self.writer.add_pair(self.key.as_ref().unwrap(), &value)?;
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

struct QsKeySerializer;

impl Serializer for QsKeySerializer {
    type Ok = String;
    type Error = QsError;
    type SerializeSeq = Impossible<String, QsError>;
    type SerializeTuple = Impossible<String, QsError>;
    type SerializeTupleStruct = Impossible<String, QsError>;
    type SerializeTupleVariant = Impossible<String, QsError>;
    type SerializeMap = Impossible<String, QsError>;
    type SerializeStruct = Impossible<String, QsError>;
    type SerializeStructVariant = Impossible<String, QsError>;

    unsupported_types!(
        (&[u8], serialize_bytes),
        (bool, serialize_bool),
        (u8, serialize_u8),
        (u16, serialize_u16),
        (u32, serialize_u32),
        (u64, serialize_u64),
        (i8, serialize_i8),
        (i16, serialize_i16),
        (i32, serialize_i32),
        (i64, serialize_i64),
        (f32, serialize_f32),
        (f64, serialize_f64),
        (char, serialize_char)
    );

    serialize_key_to_string!((&str, serialize_str));

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Err(QsError::Unsupported)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Err(QsError::Unsupported)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Err(QsError::Unsupported)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(QsError::Unsupported)
    }
}

struct QsValueSerializer;

impl Serializer for QsValueSerializer {
    type Ok = Vec<String>;
    type Error = QsError;
    type SerializeSeq = QsArrayValueSerializer;
    type SerializeTuple = Impossible<Vec<String>, QsError>;
    type SerializeTupleStruct = Impossible<Vec<String>, QsError>;
    type SerializeTupleVariant = Impossible<Vec<String>, QsError>;
    type SerializeMap = Impossible<Vec<String>, QsError>;
    type SerializeStruct = Impossible<Vec<String>, QsError>;
    type SerializeStructVariant = Impossible<Vec<String>, QsError>;

    unsupported_types!((&[u8], serialize_bytes));

    serialize_value_to_string!(
        (bool, serialize_bool),
        (u8, serialize_u8),
        (u16, serialize_u16),
        (u32, serialize_u32),
        (u64, serialize_u64),
        (i8, serialize_i8),
        (i16, serialize_i16),
        (i32, serialize_i32),
        (i64, serialize_i64),
        (f32, serialize_f32),
        (f64, serialize_f64),
        (char, serialize_char),
        (&str, serialize_str)
    );

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(vec![])
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(vec![String::new()])
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Err(QsError::Unsupported)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Err(QsError::Unsupported)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(QsArrayValueSerializer {
            values: match len {
                Some(len) => Vec::with_capacity(len),
                None => vec![],
            },
        })
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(QsError::Unsupported)
    }
}

struct QsPrimaryValueSerializer;

impl Serializer for QsPrimaryValueSerializer {
    type Ok = Option<String>;
    type Error = QsError;
    type SerializeSeq = Impossible<Option<String>, QsError>;
    type SerializeTuple = Impossible<Option<String>, QsError>;
    type SerializeTupleStruct = Impossible<Option<String>, QsError>;
    type SerializeTupleVariant = Impossible<Option<String>, QsError>;
    type SerializeMap = Impossible<Option<String>, QsError>;
    type SerializeStruct = Impossible<Option<String>, QsError>;
    type SerializeStructVariant = Impossible<Option<String>, QsError>;

    unsupported_types!((&[u8], serialize_bytes));

    serialize_primary_value_to_string!(
        (bool, serialize_bool),
        (u8, serialize_u8),
        (u16, serialize_u16),
        (u32, serialize_u32),
        (u64, serialize_u64),
        (i8, serialize_i8),
        (i16, serialize_i16),
        (i32, serialize_i32),
        (i64, serialize_i64),
        (f32, serialize_f32),
        (f64, serialize_f64),
        (char, serialize_char),
        (&str, serialize_str)
    );

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(None)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Some(String::new()))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Err(QsError::Unsupported)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Err(QsError::Unsupported)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(QsError::Unsupported)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(QsError::Unsupported)
    }
}

struct QsArrayValueSerializer {
    values: Vec<String>,
}

impl SerializeSeq for QsArrayValueSerializer {
    type Ok = Vec<String>;
    type Error = QsError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        let value = value.serialize(QsPrimaryValueSerializer)?;
        self.values.extend(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.values)
    }
}

pub(crate) fn to_string<T: Serialize>(value: &T) -> Result<String, QsError> {
    let mut qs = Vec::new();
    let mut value_writer = ValueWriter {
        writer: &mut qs,
        first: true,
    };
    value.serialize(QsSerializer {
        writer: &mut value_writer,
    })?;
    Ok(String::from_utf8(qs).unwrap())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn serialize_struct() {
        #[derive(Serialize)]
        struct Test {
            a: i32,
            b: String,
            c: bool,
            d: Vec<i32>,
            e: Vec<String>,
        }

        let s = to_string(&Test {
            a: 100,
            b: "hehe".to_string(),
            c: true,
            d: vec![1, 2, 3, 4, 5],
            e: vec!["abc".to_string(), "def".to_string()],
        })
        .unwrap();
        assert_eq!(s, "a=100&b=hehe&c=true&d=1&d=2&d=3&d=4&d=5&e=abc&e=def");
    }

    #[test]
    fn serialize_map() {
        let map1 = {
            let mut map = BTreeMap::new();
            map.insert("a", 100);
            map.insert("b", 200);
            map
        };

        assert_eq!(to_string(&map1).unwrap(), "a=100&b=200");

        let map2 = {
            let mut map = BTreeMap::new();
            map.insert("a", vec![1, 2]);
            map.insert("b", vec![3, 4]);
            map
        };

        assert_eq!(to_string(&map2).unwrap(), "a=1&a=2&b=3&b=4");
    }

    #[test]
    fn serialize_optional() {
        #[derive(Serialize)]
        struct Test {
            a: i32,
            b: Option<i32>,
            c: bool,
        }

        assert_eq!(
            to_string(&Test {
                a: 100,
                b: None,
                c: true
            })
            .unwrap(),
            "a=100&c=true"
        );

        #[derive(Serialize)]
        struct Test2 {
            #[serde(rename = "v")]
            values: Vec<Option<i32>>,
        }

        assert_eq!(
            to_string(&Test2 {
                values: vec![Some(1), None, Some(3)]
            })
            .unwrap(),
            "v=1&v=3"
        );
    }

    #[test]
    fn serialize_seq() {
        assert_eq!(to_string(&[("a", 123), ("b", 456)]).unwrap(), "a=123&b=456");
        assert_eq!(
            to_string(&vec![("a", 123), ("b", 456)]).unwrap(),
            "a=123&b=456"
        );
    }

    #[test]
    fn serialize_tuple() {
        assert_eq!(
            to_string(&(("a", 123), ("b", 456), ("c", vec![1, 2, 3]))).unwrap(),
            "a=123&b=456&c=1&c=2&c=3"
        );
    }
}
