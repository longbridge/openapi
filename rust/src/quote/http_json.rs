//! Lenient decoding of the gateway's proto-JSON responses into the `prost`
//! message types shared with the WebSocket transport.
//!
//! The protobuf types derive plain `serde` impls, which expect every field to
//! be present with its native JSON type. The gateway's proto-JSON differs in a
//! few ways that this deserializer absorbs, so the existing
//! `TryFrom<proto>` conversions can be reused unchanged:
//!
//! - 64-bit integers (and occasionally other scalars) are encoded as strings
//! - fields holding their default value may be omitted or `null`
//! - keys may use the lowerCamelCase JSON name instead of the proto name

use serde::{
    Deserializer,
    de::{
        self, DeserializeOwned, DeserializeSeed, IntoDeserializer, MapAccess, SeqAccess, Visitor,
    },
    forward_to_deserialize_any,
};
use serde_json::{Map, Value};

type Error = serde_json::Error;

/// Recursively remove `null` object members, so an absent `Option<Message>`
/// is omitted from a request body rather than sent as `null`.
pub(crate) fn strip_nulls(value: &mut Value) {
    match value {
        Value::Object(map) => {
            map.retain(|_, v| !v.is_null());
            map.values_mut().for_each(strip_nulls);
        }
        Value::Array(arr) => arr.iter_mut().for_each(strip_nulls),
        _ => {}
    }
}

/// Deserialize `T` from a gateway proto-JSON value.
pub(crate) fn from_value<T: DeserializeOwned>(value: Value) -> Result<T, Error> {
    T::deserialize(Lenient(value))
}

/// A `serde_json::Value` deserializer that treats `null` as the target type's
/// default value and accepts numbers / booleans encoded as strings.
struct Lenient(Value);

fn invalid(value: &Value, exp: &dyn de::Expected) -> Error {
    de::Error::invalid_type(
        match value {
            Value::Null => de::Unexpected::Unit,
            Value::Bool(b) => de::Unexpected::Bool(*b),
            Value::Number(_) => de::Unexpected::Other("number"),
            Value::String(s) => de::Unexpected::Str(s),
            Value::Array(_) => de::Unexpected::Seq,
            Value::Object(_) => de::Unexpected::Map,
        },
        exp,
    )
}

macro_rules! deserialize_int {
    ($($method:ident => $visit:ident: $ty:ty),* $(,)?) => {
        $(
            fn $method<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
                let n: $ty = match &self.0 {
                    Value::Null => 0 as $ty,
                    Value::Number(n) => n
                        .to_string()
                        .parse()
                        .map_err(|_| invalid(&self.0, &visitor))?,
                    Value::String(s) if s.is_empty() => 0 as $ty,
                    Value::String(s) => s.parse().map_err(|_| invalid(&self.0, &visitor))?,
                    Value::Bool(b) => *b as u8 as $ty,
                    _ => return Err(invalid(&self.0, &visitor)),
                };
                visitor.$visit(n)
            }
        )*
    };
}

impl<'de> Deserializer<'de> for Lenient {
    type Error = Error;

    deserialize_int! {
        deserialize_i8 => visit_i8: i8,
        deserialize_i16 => visit_i16: i16,
        deserialize_i32 => visit_i32: i32,
        deserialize_i64 => visit_i64: i64,
        deserialize_u8 => visit_u8: u8,
        deserialize_u16 => visit_u16: u16,
        deserialize_u32 => visit_u32: u32,
        deserialize_u64 => visit_u64: u64,
        deserialize_f32 => visit_f32: f32,
        deserialize_f64 => visit_f64: f64,
    }

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.0 {
            Value::Array(arr) => visitor.visit_seq(Seq(arr.into_iter())),
            Value::Object(map) => visitor.visit_map(Fields::new(map, &[])),
            other => other.deserialize_any(visitor),
        }
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match &self.0 {
            Value::Null => visitor.visit_bool(false),
            Value::Bool(b) => visitor.visit_bool(*b),
            Value::String(s) if s == "true" || s == "false" => visitor.visit_bool(s == "true"),
            Value::Number(n) => visitor.visit_bool(n.as_f64() != Some(0.0)),
            _ => Err(invalid(&self.0, &visitor)),
        }
    }

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.0 {
            Value::Null => visitor.visit_string(String::new()),
            Value::String(s) => visitor.visit_string(s),
            Value::Number(n) => visitor.visit_string(n.to_string()),
            Value::Bool(b) => visitor.visit_string(b.to_string()),
            other => Err(invalid(&other, &visitor)),
        }
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.0 {
            Value::Null => visitor.visit_none(),
            other => visitor.visit_some(Lenient(other)),
        }
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.0 {
            Value::Null => visitor.visit_seq(Seq(Vec::new().into_iter())),
            Value::Array(arr) => visitor.visit_seq(Seq(arr.into_iter())),
            other => Err(invalid(&other, &visitor)),
        }
    }

    fn deserialize_tuple<V: Visitor<'de>>(
        self,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Error> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Error> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.0 {
            Value::Null => visitor.visit_map(Fields::new(Map::new(), &[])),
            Value::Object(map) => visitor.visit_map(Fields::new(map, &[])),
            other => Err(invalid(&other, &visitor)),
        }
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error> {
        match self.0 {
            Value::Null => visitor.visit_map(Fields::new(Map::new(), fields)),
            Value::Object(map) => visitor.visit_map(Fields::new(map, fields)),
            other => Err(invalid(&other, &visitor)),
        }
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error> {
        self.0.deserialize_enum(name, variants, visitor)
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        visitor.visit_unit()
    }

    forward_to_deserialize_any! {
        char bytes byte_buf unit unit_struct identifier
    }
}

struct Seq(std::vec::IntoIter<Value>);

impl<'de> SeqAccess<'de> for Seq {
    type Error = Error;

    fn next_element_seed<T: DeserializeSeed<'de>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>, Error> {
        self.0
            .next()
            .map(|v| seed.deserialize(Lenient(v)))
            .transpose()
    }
}

/// Map access over an object's entries. For a struct, keys are normalized to
/// the expected field names and every expected field that is missing is
/// yielded as `null`, so it decodes to its default value.
struct Fields {
    entries: std::vec::IntoIter<(String, Value)>,
    value: Option<Value>,
}

impl Fields {
    fn new(map: Map<String, Value>, fields: &'static [&'static str]) -> Self {
        let mut entries: Vec<(String, Value)> = Vec::with_capacity(map.len() + fields.len());
        // camelCase aliases are applied after all exact names, so a payload
        // carrying both spellings never yields a duplicate: the proto name
        // wins.
        let mut aliases: Vec<(String, Value)> = Vec::new();
        for (key, value) in map {
            if fields.is_empty() || fields.contains(&key.as_str()) {
                entries.push((key, value));
                continue;
            }
            let snake = camel_to_snake(&key);
            if fields.contains(&snake.as_str()) {
                aliases.push((snake, value));
            } else {
                entries.push((key, value));
            }
        }
        for (key, value) in aliases {
            if !entries.iter().any(|(k, _)| *k == key) {
                entries.push((key, value));
            }
        }
        for field in fields {
            if !entries.iter().any(|(key, _)| key == field) {
                entries.push(((*field).to_string(), Value::Null));
            }
        }
        Self {
            entries: entries.into_iter(),
            value: None,
        }
    }
}

impl<'de> MapAccess<'de> for Fields {
    type Error = Error;

    fn next_key_seed<K: DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Error> {
        match self.entries.next() {
            Some((key, value)) => {
                self.value = Some(value);
                seed.deserialize(key.into_deserializer()).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Error> {
        seed.deserialize(Lenient(self.value.take().unwrap_or(Value::Null)))
    }
}

fn camel_to_snake(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for ch in s.chars() {
        if ch.is_ascii_uppercase() {
            out.push('_');
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use longbridge_proto::quote::{SecurityDepthResponse, SecurityQuoteResponse};
    use serde_json::json;

    use super::*;

    #[test]
    fn decode_quote_with_string_int64_and_missing_fields() {
        // int64 as string, enum as int, camelCase keys, omitted fields, null
        // sub-message, and an extra field unknown to the proto.
        let resp: SecurityQuoteResponse = from_value(json!({
            "secuQuote": [{
                "symbol": "700.HK",
                "lastDone": "432.200",
                "prev_close": "430.000",
                "volume": "12345678",
                "turnover": "5300000000.000",
                "timestamp": "1700000000",
                "trade_status": 0,
                "volume_str": "12345678",
                "pre_market_quote": null,
            }]
        }))
        .unwrap();
        let q = &resp.secu_quote[0];
        assert_eq!(q.symbol, "700.HK");
        assert_eq!(q.last_done, "432.200");
        assert_eq!(q.prev_close, "430.000");
        assert_eq!(q.volume, 12345678);
        assert_eq!(q.timestamp, 1700000000);
        assert_eq!(q.trade_status, 0);
        assert_eq!(q.open, "");
        assert!(q.pre_market_quote.is_none());
    }

    #[test]
    fn decode_depth() {
        let resp: SecurityDepthResponse = from_value(json!({
            "symbol": "700.HK",
            "ask": [{ "position": 1, "price": "432.400", "volume": "1000", "order_num": "3", "volume_str": "1000" }],
            "bid": [{ "position": "1", "price": "432.200", "volume": 2000, "orderNum": 5 }],
        }))
        .unwrap();
        assert_eq!(resp.ask[0].volume, 1000);
        assert_eq!(resp.ask[0].order_num, 3);
        assert_eq!(resp.bid[0].position, 1);
        assert_eq!(resp.bid[0].order_num, 5);
    }

    #[test]
    fn decode_edge_cases() {
        use longbridge_proto::quote::{Brokers, StrikePriceInfo};
        // null inside arrays, mixed string/number ints, bool spellings.
        let b: Brokers = from_value(json!({
            "position": "1", "broker_ids": ["1", 2, null, "-3"]
        }))
        .unwrap();
        assert_eq!(b.broker_ids, vec![1, 2, 0, -3]);
        for (v, want) in [
            (json!("true"), true),
            (json!(true), true),
            (json!(1), true),
            (json!(null), false),
        ] {
            let s: StrikePriceInfo = from_value(json!({ "price": "1", "standard": v })).unwrap();
            assert_eq!(s.standard, want);
        }
        // camelCase and proto name both present: proto name wins, no duplicate.
        for order in [
            json!({ "orderNum": 9, "order_num": "3", "position": 1 }),
            json!({ "order_num": "3", "orderNum": 9, "position": 1 }),
        ] {
            let d: SecurityDepthResponse = from_value(json!({ "ask": [order] })).unwrap();
            assert_eq!(d.ask[0].order_num, 3);
        }
        // Values that must not be silently accepted.
        assert!(
            from_value::<SecurityDepthResponse>(json!({ "ask": [{ "volume": "1.5" }] })).is_err()
        );
        assert!(
            from_value::<SecurityDepthResponse>(
                json!({ "ask": [{ "volume": "99999999999999999999" }] })
            )
            .is_err()
        );
        // Enum names are not accepted (the gateway sends numbers).
        assert!(
            from_value::<SecurityQuoteResponse>(json!({
                "secu_quote": [{ "trade_status": "Normal" }]
            }))
            .is_err()
        );
    }

    #[test]
    fn strip_nulls_on_history_request() {
        use longbridge_proto::quote::{
            SecurityHistoryCandlestickRequest, security_history_candlestick_request::OffsetQuery,
        };
        let req = SecurityHistoryCandlestickRequest {
            symbol: "AAPL.US".into(),
            offset_request: Some(OffsetQuery {
                count: 10,
                ..Default::default()
            }),
            date_request: None,
            ..Default::default()
        };
        let mut v = serde_json::to_value(&req).unwrap();
        strip_nulls(&mut v);
        assert!(v.get("date_request").is_none(), "{v}");
        assert_eq!(v["offset_request"]["count"], 10);
        assert_eq!(v["offset_request"]["date"], "");
    }

    #[test]
    fn strip_nulls_omits_absent_messages() {
        let mut v = json!({ "a": null, "b": { "c": null, "d": 1 }, "e": [ { "f": null } ] });
        strip_nulls(&mut v);
        assert_eq!(v, json!({ "b": { "d": 1 }, "e": [ {} ] }));
    }

    #[test]
    fn decode_empty_object() {
        let resp: SecurityDepthResponse = from_value(json!({})).unwrap();
        assert!(resp.ask.is_empty() && resp.bid.is_empty());
    }
}
