use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use warp::reply::Reply;

#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    id: u64,
    name: String,
}

impl Employee {
    fn new() -> Self {
        Self::default()
    }

    fn with_name(self, name: String) -> Self {
        Self { name: name, ..self }
    }

    fn with_id(self, id: u64) -> Self {
        Self { id: id, ..self }
    }
}

pub type Database = Arc<Mutex<HashMap<u64, String>>>;

pub fn init_db() -> Database {
    Arc::new(Mutex::new(HashMap::new()))
}

impl Default for Employee {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
        }
    }
}

pub async fn get_employee(id: u64, database: Database) -> Result<impl Reply, warp::Rejection> {
    match database.lock().await.get(&id) {
        Some(name) => Ok(warp::reply::json(
            &Employee::new().with_id(id).with_name(name.into()),
        )),
        None => Ok(warp::reply::json(&"{}".to_string())),
    }
}
