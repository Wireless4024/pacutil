use serde::ser::SerializeStruct;
use serde::Serialize;

use crate::db::field_type_extractor::FieldTypeExtractor;
use crate::db::Table;
use crate::ser::SerError;

#[derive(Debug)]
pub struct Field {
	pub name: String,
	pub typ: FieldType,
}

#[derive(Debug, Eq, PartialEq)]
pub enum FieldType {
	Char,
	String,
	Bool,
	U8,
	U16,
	U32,
	U64,
	I8,
	I16,
	I32,
	I64,
	F32,
	F64,
	Bytes,
	Unsupported,
}

trait FieldNameImpl {
	fn field_type(&self) -> FieldType {
		FieldType::Unsupported
	}
}

impl<'a> SerializeStruct for Table {
	type Ok = Table;
	type Error = SerError;

	fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where T: Serialize {
		let t = value.serialize(FieldTypeExtractor)?;
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