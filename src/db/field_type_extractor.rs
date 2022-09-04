use serde::{Serialize, Serializer};
use serde::ser::Impossible;

use crate::db::field::FieldType;
use crate::ser::SerError;

pub struct FieldTypeExtractor;

impl Serializer for FieldTypeExtractor {
	type Ok = FieldType;
	type Error = SerError;
	type SerializeSeq = Impossible<Self::Ok, Self::Error>;
	type SerializeTuple = Impossible<Self::Ok, Self::Error>;
	type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
	type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
	type SerializeMap = Impossible<Self::Ok, Self::Error>;
	type SerializeStruct = Impossible<Self::Ok, Self::Error>;
	type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

	fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
		Ok(FieldType::Bool)
	}

	fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
		Ok(FieldType::I8)
	}

	fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
		Ok(FieldType::I16)
	}

	fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
		Ok(FieldType::I32)
	}

	fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
		Ok(FieldType::I64)
	}

	fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
		Ok(FieldType::U8)
	}

	fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
		Ok(FieldType::U16)
	}

	fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
		Ok(FieldType::U32)
	}

	fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
		Ok(FieldType::U64)
	}

	fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
		Ok(FieldType::F32)
	}

	fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
		Ok(FieldType::F64)
	}

	fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
		Ok(FieldType::Char)
	}

	fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> {
		Ok(FieldType::String)
	}

	fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
		Ok(FieldType::Bytes)
	}

	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_some<T: ?Sized>(self, v: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
		v.serialize(self)
	}

	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_newtype_struct<T: ?Sized>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_newtype_variant<T: ?Sized>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}
}