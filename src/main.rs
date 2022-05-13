use std::{collections::HashMap, sync::Mutex, time::Duration};

use once_cell::sync::Lazy;
use tokio::{
    sync::oneshot::{self, Sender},
    time::sleep,
};
static MAP: Lazy<Mutex<HashMap<usize, Sender<String>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

// fetch

#[tokio::main]
async fn main() {
    let task1 = {
        tokio::spawn(async move {
            let res = fetch("some_thing".into()).await;
            println!("await res success:{}", res);
        })
    };

    let task2 = {
        tokio::spawn(async move {
            // fake net delay
            sleep(Duration::new(2, 0)).await;

            // got id and res from remote
            let id = 123;
            let res_from_remote = "hi,this is result".to_string();
            if let Some(tx) = MAP.lock().unwrap().remove(&id) {
                tx.send(res_from_remote).unwrap();
            };
        })
    };

    task1.await.unwrap();
    task2.await.unwrap();
}

async fn fetch(_some_action: String) -> String {
    let req_id = 123;
    let (tx, rx) = oneshot::channel::<String>();
    MAP.lock().unwrap().insert(req_id, tx);

    // fake request with id
    // xx.send(req_id, _some_action);

    println!("start request and wait result");
    let res = rx.await.unwrap();
    res
}
