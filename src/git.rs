use std::process::Command;
use std::io::Error;
use repository::Repository;
use std::borrow::ToOwned;

pub fn init (id: &str, path: &str) -> Result<Repository, Error> {
    Command::new("git")
            .arg("init")
            .arg(path)
            .output()
            .map(|_| Repository {
                id: id.to_owned(),
                path: path.to_owned()
            })
}
