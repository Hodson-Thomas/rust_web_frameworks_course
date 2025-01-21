use tokio;
use warp::Filter;
use warp_challenge::*;

#[tokio::main]
async fn main() {
    let db: Database = init_db();
    {
        let mut guard = db.lock().await;
        guard.insert(1, "Thomas".to_string());
        guard.insert(2, "Nathan".to_string());
        guard.insert(3, "Leon".to_string());
        guard.insert(3, "Amelie".to_string());
    }

    let routes = warp::path!("employees" / u64)
        .and(warp::get())
        .and(warp::any().map(move || db.clone()))
        .and_then(get_employee);
    println!("Listening at 127.0.0.1:8000 ...");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
