use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    username: String,
    password: String,
}

impl User {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    pub fn change_username(&mut self, username: String) {
        self.username = username;
    }

    pub fn change_password(&mut self, password: String) {
        self.password = password;
    }
}

type Database = Arc<Mutex<Vec<User>>>;

pub fn init_database(user_amount: usize) -> Database {
    let database = Arc::new(Mutex::new(Vec::new()));
    {
        let mut db = database.lock().unwrap();
        for i in 0..user_amount {
            db.push(User::new(format!("User{}", i), format!("password{}", i)));
        }
    }
    database
}

pub async fn register(user: web::Json<User>, database: web::Data<Database>) -> impl Responder {
    let mut db = database.lock().unwrap();
    if db.iter().any(|u| u.username == user.username) {
        return HttpResponse::BadRequest().body("User already exists");
    }
    db.push(user.into_inner());
    HttpResponse::Ok().body("Registered")
}

pub async fn login(user: web::Json<User>, database: web::Data<Database>) -> impl Responder {
    let db = database.lock().unwrap();
    if !db.iter().any(|u| u.username == user.username) {
        return HttpResponse::BadRequest().body("Bad credentials");
    }
    HttpResponse::Ok().body("Logged in")
}

pub async fn update_username(
    user: web::Json<User>,
    username: String,
    database: web::Data<Database>,
) -> impl Responder {
    if user.username == username {
        return HttpResponse::Ok().body("Username updated");
    }
    let mut db = database.lock().unwrap();
    if db.iter().any(|u| u.username == username) {
        return HttpResponse::BadRequest().body("Username already taken");
    }
    match db
        .iter_mut()
        .find(|u| u.username == user.username && u.password == user.password)
    {
        Some(u) => {
            u.change_username(username);
            HttpResponse::Ok().body("Username updated")
        }
        None => HttpResponse::BadRequest().body("Bad credentials"),
    }
}

pub async fn update_password(
    user: web::Json<User>,
    password: String,
    database: web::Data<Database>,
) -> impl Responder {
    if user.password == password {
        return HttpResponse::Ok().body("Password updated");
    }
    let mut db = database.lock().unwrap();
    if db.iter().any(|u| u.password == password) {
        return HttpResponse::BadRequest().body("Password already taken");
    }
    match db
        .iter_mut()
        .find(|u| u.username == user.username && u.password == user.password)
    {
        Some(u) => {
            u.change_password(password);
            HttpResponse::Ok().body("Password updated")
        }
        None => HttpResponse::BadRequest().body("Bad credentials"),
    }
}
