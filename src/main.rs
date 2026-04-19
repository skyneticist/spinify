use indicatif::{MultiProgress, ProgressStyle};
use spinify::{spawn_spinner_task, spawn_spinner_task_with_style};
use tokio::time::{sleep, Duration};

// Dummy stand-ins for your real API call functions:
async fn fetch_users() -> usize {
    // Simulate database query with some processing
    sleep(Duration::from_millis(500)).await;
    let mut count = 0;
    for i in 0..1500 {
        count += i % 10; // Simulate some computation
        sleep(Duration::from_micros(100)).await; // Throttle to make it visible
    }
    count
}

async fn fetch_orders() -> usize {
    // Simulate API call with data aggregation
    sleep(Duration::from_millis(800)).await;
    let mut total = 0;
    for order in 0..42 {
        total += order * 2; // Simulate order value calculation
        sleep(Duration::from_millis(50)).await;
    }
    total
}

async fn fetch_inventory() -> usize {
    // Simulate inventory check with random fluctuations
    sleep(Duration::from_millis(300)).await;
    let base = 100;
    let mut inventory = base;
    for _ in 0..10 {
        inventory += (inventory as f64 * 0.1) as usize; // Simulate stock adjustments
        sleep(Duration::from_millis(70)).await;
    }
    inventory
}

async fn fetch_admin() -> Vec<String> {
    // Simulate admin user lookup with multiple steps
    sleep(Duration::from_secs(1)).await;
    let mut admins = Vec::new();
    let names = ["alice", "bob", "charlie", "diana"];
    for name in names {
        sleep(Duration::from_millis(500)).await; // Simulate permission check
        admins.push(name.to_string());
    }
    admins
}

async fn count_users() -> usize {
    // Simulate counting active users with filtering
    sleep(Duration::from_millis(400)).await;
    let mut active_count = 0;
    for user_id in 0..1515 {
        if user_id % 3 == 0 {
            // Simulate active user filter
            active_count += 1;
        }
        sleep(Duration::from_micros(50)).await;
    }
    active_count
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let multi = MultiProgress::new();

    println!("\n\n🚀 Demonstrating spinify spinners...\n");

    // Basic spinner with default style
    let users_handle = spawn_spinner_task(&multi, "📊 Fetch Users", fetch_users());

    // Custom spinner with dots
    let orders_style = ProgressStyle::default_spinner()
        .template("{spinner} {msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⠂");
    let orders_handle =
        spawn_spinner_task_with_style(&multi, "🛒 Fetch Orders", orders_style, fetch_orders());

    // Custom spinner with arrows
    let inventory_style = ProgressStyle::default_spinner()
        .template("{spinner:.blue} {msg}")
        .unwrap()
        .tick_chars("←↖↑↗→↘↓↙");
    let inventory_handle = spawn_spinner_task_with_style(
        &multi,
        "📦 Fetch Inventory",
        inventory_style,
        fetch_inventory(),
    );

    // Custom style spinner with Braille
    let custom_style = ProgressStyle::default_spinner()
        .template("{spinner:.green} {msg:.cyan.bold}")
        .unwrap()
        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏");
    let admin_handle =
        spawn_spinner_task_with_style(&multi, "👑 Fetch Admin", custom_style, fetch_admin());

    // Custom spinner with classic ASCII
    let count_style = ProgressStyle::default_spinner()
        .template("{spinner:.yellow} {msg:.magenta}")
        .unwrap()
        .tick_chars("|/-\\");
    let count_handle =
        spawn_spinner_task_with_style(&multi, "🔢 Count Users", count_style, count_users());

    // Await all of them and propagate any task failures.
    let users = users_handle.await?;
    let orders = orders_handle.await?;
    let inventory = inventory_handle.await?;
    let admin = admin_handle.await?;
    let count = count_handle.await?;

    // Clear finished progress bars from the terminal.
    // let _ = multi.clear();

    println!("\n\nAll tasks completed:");
    println!("- Users: {}", users);
    println!("- Orders: {}", orders);
    println!("- Inventory: {}", inventory);
    println!("- Admin: {:?}", admin);
    println!("- Count: {}", count);
    Ok(())
}
