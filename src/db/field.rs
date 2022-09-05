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

impl FieldType {
	pub fn sqlite_type(&self) -> &'static str {
		match self {
			FieldType::Char => {
				"CHARACTER(1)"
			}
			FieldType::String => {
				"TEXT"
			}
			FieldType::Bool => {
				"TINYINT"
			}
			FieldType::U8 | FieldType::I8 => {
				"TINYINT"
			}
			FieldType::U16 | FieldType::I16 => {
				"SMALLINT"
			}
			FieldType::U32 | FieldType::I32 => {
				"INTEGER"
			}
			FieldType::U64 | FieldType::I64 => {
				"BIGINT"
			}
			FieldType::F32 => {
				"FLOAT"
			}
			FieldType::F64 => {
				"DOUBLE"
			}
			FieldType::Bytes => {
				"BLOB"
			}
			_ => unreachable!()
		}
	}
}

trait FieldNameImpl {
	fn field_type(&self) -> FieldType {
		FieldType::Unsupported
	}
}