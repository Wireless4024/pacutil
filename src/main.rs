use serde_json::json;
use crate::db::db_init;
use crate::util::right_join;
use crate::wrapper::info::list_installed;
use crate::wrapper::repo::{list_all, list_to_db, Package};

mod wrapper;
mod util;
mod db;
mod ser;

fn main() -> anyhow::Result<()> {
	tracing_subscriber::fmt::init();
	//exec().expect("1");
	/*for x in list().unwrap().into_iter().map(|it| it.to_object()) {
		println!("{:?}", x);
	}*/
	let db = db_init()?;
	let repo = db.get_repository::<Package>();
	//list_to_db(&repo)?;
	repo.find(json!({"name":"hello","repo":null}));
	println!("{:?}", repo.all());
	/*
		for x in list_installed_with_detail() {
			if let Multiple(arr) = x {
				if arr.iter().any(|it| obj_match(&json!({"repo":"cachyos*"}), it)) {
					let pkg = arr.into_iter().find(|it| !obj_match(&json!({"repo":"cachyos*"}), it))
						.and_then(|it| from_value::<InstallablePackage>(it).ok());
					if let Some(pkg) = pkg {
						install_pkg(&pkg)?;
					}
				}
			}
		}*/
	Ok(())
}
