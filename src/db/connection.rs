use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use anyhow::Result;
use rusqlite::{Connection, Params};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_rusqlite::DeserRows;
use tracing::{debug, info};

use crate::db::table::{Table, TableStructureGenerator};

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
		let rows: DeserRows<T> = serde_rusqlite::from_rows::<T>(rows);
		let mut res = Vec::new();
		for row in rows {
			res.push(row?)
		}
		Ok(res)
	}

	pub fn get_repository_from<S: Serialize + DeserializeOwned>(&self, s: S) -> Repository<S> {
		let table = s.serialize(TableStructureGenerator(PhantomData)).unwrap();
		Repository { connection: self, table }.init()
	}

	pub fn get_repository<S: Serialize + DeserializeOwned + Default>(&self) -> Repository<S> {
		let s = S::default();
		Self::get_repository_from(self, s)
	}
}

pub struct Repository<'a, T: Serialize + DeserializeOwned> {
	connection: &'a DbHandler,
	table: Table<T>,
}

impl<'a, T: Serialize + DeserializeOwned> Repository<'a, T> {
	fn init(self) -> Self {
		println!("{}", self.table.create_fts_script());
		self.connection.execute(&self.table.create_fts_script(), []).expect("Create table");
		self
	}

	pub fn all(&self) -> Vec<T> {
		self.connection.query_all(&format!("SELECT * FROM {}", &self.table.name), []).unwrap()
	}

	pub fn add(&self, obj: T) -> T {
		//serde_rusqlite::
		let mut params = String::new();
		let mut vals = String::new();
		for x in &self.table.fields {
			params.push_str(&x.name);
			params.push(',');
			vals.push(':');
			vals.push_str(&x.name);
			vals.push(',');
		}
		params.pop();
		vals.pop();
		self.connection.query(&format!("INSERT INTO {} ({}) VALUES ({}) RETURNING *", &self.table.name, params, vals),
		                      serde_rusqlite::to_params_named(&obj).unwrap().to_slice().as_slice()).unwrap()
	}

	pub fn add_all(&self, objs: Vec<T>) -> Vec<T> {
		objs.into_iter().map(|it| self.add(it)).collect()
	}
}