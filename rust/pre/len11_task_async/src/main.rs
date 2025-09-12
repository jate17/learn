
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(10);

    tokio::spawn(async move {
        for i in 0..5 {
            tx.send(format!("Messaggio {}", i)).await.unwrap();
        }
    });

    while let Some(msg) = rx.recv().await {
        println!("Ricevuto: {}", msg);
    }
}
