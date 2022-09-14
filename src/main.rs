use crate::db::db_init;
use crate::wrapper::info::{InstalledPackage, list_installed};

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
	let repo = db.get_repository::<InstalledPackage>();
//	list_to_db(&repo)?;
	println!("{:?}", list_installed(&repo)?);
	println!("{:?}", repo.all());

	//println!("{:?}", repo.all());
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
