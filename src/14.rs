use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{Filter, Rejection, Reply};

#[tokio::main]
async fn main() {
    let mut map = HashMap::new();
    map.insert("key".to_string(), "value".to_string());

    let arc_map = Arc::new(Mutex::new(map));

    let route = warp::path!("my-route")
        .and(warp::get())
        .and_then(move || {
            let arc_map = arc_map.clone();
            async move {
                let mut map = arc_map.lock().await;
                if let Some(value) = map.get("key") {
                    return Ok((warp::reply::json(&value),));
                }
                Err(Rejection::from(warp::reject::not_found()))
            }
        });

    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}
