async fn fast_alarm() {
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    println!("The fast ararm is working!");
}
async fn slow_alarm() {
    tokio::time::sleep(std::time::Duration::from_millis(3000)).await;
    println!("You don't saw this message!");
}
#[tokio::main]
async fn main() {
    tokio::select! {
        _ = fast_alarm() => {
            println!("fast alarm work first");
        }
        _ = slow_alarm() => {
            println!("fast alarm work first");
        }
    }
}