#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::tokio::sync::Mutex;
use rocket::State;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: u64,
    name: String,
}

impl User {
    pub fn new(id: u64, name: String) -> Self {
        Self { id, name }
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
        }
    }
}

pub type Database = Arc<Mutex<HashMap<u64, String>>>;

pub async fn init_database(user_amout: usize) -> Database {
    let database = Arc::new(Mutex::new(HashMap::new()));
    {
        let mut db = database.lock().await;
        for i in 0..user_amout {
            db.insert(i as u64, format!("User {}", i));
        }
    }
    database
}

#[get("/get_user/<id>")]
pub async fn get_user(database: &State<Database>, id: u64) -> Json<User> {
    let db = database.lock().await;
    if let Some(name) = db.get(&id) {
        Json(User::new(id, name.into()))
    } else {
        Json(User::default())
    }
}

#[post("/create_user", data = "<user>")]
pub async fn create_user(database: &State<Database>, user: Json<User>) -> Status {
    let mut db = database.lock().await;
    if db.get(&user.id).is_some() {
        return Status::from_code(404).unwrap();
    }
    db.insert(user.id, user.name.clone());
    Status::from_code(200).unwrap()
}

#[post("/update_user", data = "<user>")]
pub async fn update_user(database: &State<Database>, user: Json<User>) -> Status {
    let mut db = database.lock().await;
    if let Some(name) = db.get_mut(&user.id) {
        *name = user.name.clone();
        return Status::from_code(200).unwrap();
    }
    Status::from_code(404).unwrap()
}

#[delete("/delete_user/<user_id>")]
pub async fn delete_user(database: &State<Database>, user_id: u64) -> Status {
    let mut db = database.lock().await;
    match db.remove(&user_id) {
        Some(_) => Status::from_code(200).unwrap(),
        None => Status::from_code(404).unwrap(),
    }
}
