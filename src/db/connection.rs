use std::fmt::{Debug};
use std::ops::{Deref, DerefMut};

use anyhow::Result;
use rusqlite::{Connection, Params};
use serde::{Serialize, Serializer};
use serde::de::DeserializeOwned;
use serde::ser::{Impossible};
use serde_rusqlite::DeserRows;
use tracing::{debug, info};

use crate::db::field::Field;
use crate::ser::SerError;

pub fn db_init() -> Result<DbHandler> {
	info!("Creating in-memory database");
	let connection = Connection::open_in_memory()?;
	debug!("Create database successfully!");
	Ok(DbHandler { connection })
}

pub struct DbHandler {
	connection: Connection,
}

#[derive(serde::Deserialize, Debug)]
struct K {
	k: i64,
}

impl Deref for DbHandler {
	type Target = Connection;

	fn deref(&self) -> &Self::Target {
		&self.connection
	}
}

impl DerefMut for DbHandler {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.connection
	}
}

impl DbHandler {
	pub fn query<'de, T: DeserializeOwned>(&self, sql: &str, param: impl Params) -> Result<T> {
		Ok(self.query_row(sql, param, |row| Ok(serde_rusqlite::from_row::<T>(row)))??)
	}

	pub fn query_all<T: DeserializeOwned>(&self, sql: &str, param: impl Params) -> Result<Vec<T>> {
		let mut stmt = self.prepare(sql)?;
		let rows = stmt.query(param)?;
		let mut rows: DeserRows<T> = serde_rusqlite::from_rows::<T>(rows);
		let mut res = Vec::new();
		while let Some(row) = rows.next() {
			res.push(row?)
		}
		Ok(res)
	}

	pub fn create_table_from_sample<S: Serialize + DeserializeOwned>(s: S) -> Table {
		s.serialize(TableStructureGenerator).unwrap()
	}
	
	pub fn create_table<S: Serialize + DeserializeOwned + Default>() -> Table {
		let s = S::default();
		s.serialize(TableStructureGenerator).unwrap()
	}
}

#[derive(Debug)]
pub struct Table {
	pub name: String,
	pub fields: Vec<Field>,
}

struct TableStructureGenerator;

impl Serializer for TableStructureGenerator {
	type Ok = Table;
	type Error = SerError;
	type SerializeSeq = Impossible<Self::Ok, Self::Error>;
	type SerializeTuple = Impossible<Self::Ok, Self::Error>;
	type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
	type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
	type SerializeMap = Impossible<Self::Ok, Self::Error>;
	type SerializeStruct = Table;
	type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

	fn serialize_bool(self, _v: bool) -> std::result::Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_i8(self, _v: i8) -> std::result::Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_i16(self, _v: i16) -> std::result::Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_i32(self, _v: i32) -> std::result::Result<Self::Ok, Self::Error> {
		todo!()
	}

	fn serialize_i64(self, _v: i64) -> std::result::Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_u8(self, _v: u8) -> std::result::Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_u16(self, _v: u16) -> std::result::Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_u32(self, _v: u32) -> std::result::Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_u64(self, _v: u64) -> std::result::Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_f32(self, _v: f32) -> std::result::Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_f64(self, _v: f64) -> std::result::Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_char(self, _v: char) -> std::result::Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_str(self, _v: &str) -> std::result::Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_bytes(self, _v: &[u8]) -> std::result::Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_none(self) -> std::result::Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_some<T: ?Sized>(self, _value: &T) -> std::result::Result<Self::Ok, Self::Error> where T: Serialize {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_unit(self) -> std::result::Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_unit_struct(self, _name: &'static str) -> std::result::Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str) -> std::result::Result<Self::Ok, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, _value: &T) -> std::result::Result<Self::Ok, Self::Error> where T: Serialize {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_newtype_variant<T: ?Sized>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T) -> std::result::Result<Self::Ok, Self::Error> where T: Serialize {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_seq(self, _len: Option<usize>) -> std::result::Result<Self::SerializeSeq, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_tuple(self, _len: usize) -> std::result::Result<Self::SerializeTuple, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> std::result::Result<Self::SerializeTupleStruct, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> std::result::Result<Self::SerializeTupleVariant, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_map(self, _len: Option<usize>) -> std::result::Result<Self::SerializeMap, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}

	fn serialize_struct(self, name: &'static str, len: usize) -> std::result::Result<Self::SerializeStruct, Self::Error> {
		let mut name = name.to_string();
		name.reserve_exact(1);
		name.push('s');
		Ok(Table { name, fields: Vec::with_capacity(len) })
	}

	fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> std::result::Result<Self::SerializeStructVariant, Self::Error> {
		Err(SerError(String::from("Unsupported")))
	}
}