use tokio::time::{sleep, Duration};

async fn lavoro(nome: &str) {
    println!("Inizio lavoro: {}", nome);
    sleep(Duration::from_secs(2)).await;
    println!("Fine lavoro: {}", nome);
}

#[tokio::main]
async fn main() {
    let t1 = lavoro("Task 1");
    let t2 = lavoro("Task 2");

    tokio::join!(t1, t2);
}
