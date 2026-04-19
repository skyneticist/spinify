use indicatif::MultiProgress;
use spinify::spawn_spinner_task;
use tokio::time::{sleep, Duration};

// Dummy stand-ins for your real API call functions:
async fn fetch_users() -> usize {
    sleep(Duration::from_secs(2)).await;
    1515
}
async fn fetch_orders() -> usize {
    sleep(Duration::from_secs(3)).await;
    42
}
async fn fetch_inventory() -> usize {
    sleep(Duration::from_secs(1)).await;
    100
}

async fn fetch_admin() -> Vec<String> {
    sleep(Duration::from_secs(10)).await;
    vec!["alice".into(), "bob".into()]
}

async fn count_users() -> usize {
    sleep(Duration::from_secs(1)).await;
    1515
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let multi = MultiProgress::new();

    // Spawn concurrent spinners:
    let users_handle: tokio::task::JoinHandle<usize> =
        spawn_spinner_task(&multi, "Fetch Users", fetch_users());
    let orders_handle: tokio::task::JoinHandle<usize> =
        spawn_spinner_task(&multi, "Fetch Orders", fetch_orders());
    let inventory_handle: tokio::task::JoinHandle<usize> =
        spawn_spinner_task(&multi, "Fetch Inventory", fetch_inventory());
    let admin_handle: tokio::task::JoinHandle<Vec<String>> =
        spawn_spinner_task(&multi, "Fetch Admin", fetch_admin());
    let count_handle: tokio::task::JoinHandle<usize> =
        spawn_spinner_task(&multi, "Count Users", count_users());

    // Await all of them and propagate any task failures.
    let users = users_handle.await?;
    let orders = orders_handle.await?;
    let inventory = inventory_handle.await?;
    let admin = admin_handle.await?;
    let count = count_handle.await?;

    // Clear finished progress bars from the terminal.
    // let _ = multi.clear();

    println!(
        "All tasks completed:\n users={},\n orders={},\n inventory={},\n admin={:?},\n count={}",
        users, orders, inventory, admin, count
    );
    Ok(())
}
