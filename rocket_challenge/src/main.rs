#[macro_use]
extern crate rocket;
use rocket_challenge::*;

#[rocket::main]
async fn main() {
    let db: Database = init_database(10).await;
    rocket::build()
        .manage(db)
        .mount(
            "/",
            routes![create_user, get_user, update_user, delete_user],
        )
        .launch()
        .await
        .expect("Could not start server");
}
