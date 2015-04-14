use uuid::Uuid;
use repository::Repository;
use git::{self};

pub enum RepositoryState<'a> {
    NonExisting,
    Existing(&'a str)
}

static REPO_PREFIX :&'static str = "TEMP_REP_";

pub fn get_repository_handle (id: RepositoryState) -> Repository {
    match id {
        RepositoryState::NonExisting => {
            let id = &Uuid::new_v4().to_simple_string();
            git::init(id, &generate_path(id)).unwrap()
        },
        RepositoryState::Existing(id) => {
            Repository {
                id: id.to_string(),
                path: generate_path(id)
            }
        }
    }
}

pub fn generate_path (id: &str) -> String {
    String::new() + REPO_PREFIX + id
}
