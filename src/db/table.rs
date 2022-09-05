use std::marker::PhantomData;

use serde::{Serialize, Serializer};
use serde::de::DeserializeOwned;
use serde::ser::{Impossible, SerializeStruct};

use crate::db::field::{Field, FieldType};
use crate::db::field::FieldType::I64;
use crate::db::field_type_extractor::FieldTypeExtractor;
use crate::ser::SerError;

impl<'a, E: Serialize + DeserializeOwned> SerializeStruct for Table<E> {
	type Ok = Table<E>;
	type Error = SerError;

	fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where T: Serialize {
		let t = value.serialize(FieldTypeExtractor)?;
		if key == "rowid" {
			return if t == I64 {
				Ok(())
			} else {
				Err(SerError(String::from("typeof _rowid_ != i64")))
			};
		}
		if t == FieldType::Unsupported {
			Err(SerError(String::from("Unsupported Type")))
		} else {
			self.fields.push(Field { name: key.to_string(), typ: t });
			Ok(())
		}
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(self)
	}
}


#[derive(Debug)]
pub struct Table<T: Serialize + DeserializeOwned> {
	pub name: String,
	pub fields: Vec<Field>,
	typ: PhantomData<T>,
}

impl<T: Serialize + DeserializeOwned> Table<T> {
	pub fn pk(&self) -> &Field {
		self.fields.first().unwrap()
	}

	pub fn create_fts_script(&self) -> String {
		let mut res = String::new();
		res.push_str("CREATE VIRTUAL TABLE IF NOT EXISTS ");
		res.push_str(&self.name);
		res.push_str(" USING fts5 (");
		let mut iter = self.fields.iter();
		let first = iter.next().expect("Table should have at least 1 column");
		res.push_str(&first.name);
	//	res.push(' ');
	//	res.push_str(first.typ.sqlite_type());
		for f in iter {
			res.push(',');
			res.push_str(&f.name);
		//	res.push(' ');
		//	res.push_str(f.typ.sqlite_type());
		}
		res.push(')');
		res
	}
}

pub struct TableStructureGenerator<E: Serialize + DeserializeOwned>(pub PhantomData<E>);

impl<E: Serialize + DeserializeOwned> Serializer for TableStructureGenerator<E> {
	type Ok = Table<E>;
	type Error = SerError;
	type SerializeSeq = Impossible<Self::Ok, Self::Error>;
	type SerializeTuple = Impossible<Self::Ok, Self::Error>;
	type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
	type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
	type SerializeMap = Impossible<Self::Ok, Self::Error>;
	type SerializeStruct = Table<E>;
	type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

	fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, _value: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_newtype_variant<T: ?Sized>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
		let mut name = name.to_string();
		name.reserve_exact(1);
		name.push('s');
		Ok(Table { name, fields: Vec::with_capacity(len), typ: PhantomData })
	}

	fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}
}