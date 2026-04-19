# spinify

A Rust library providing utilities for spawning asynchronous tasks with progress spinners using the `indicatif` crate.

## Demo

![Demo](demo.gif)

*Run `cargo run` to see the spinners in action!*

To create the demo GIF:
1. Install [asciinema](https://asciinema.org/) or [terminalizer](https://github.com/faressoft/terminalizer)
2. Run `asciinema rec demo.cast` (or equivalent)
3. Execute `cargo run`
4. Stop recording and convert to GIF: `asciinema gif demo.cast demo.gif`

## Description

`spinify` simplifies running concurrent async tasks with visual progress indicators. It integrates seamlessly with `tokio` and `indicatif` to display spinners for long-running operations like API calls or data processing.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
spinify = "0.1.0"
tokio = { version = "1", features = ["full"] }
indicatif = "0.17"
```

## Usage

### Basic Spinner Task

Use `spawn_spinner_task` for a default spinner style:

```rust
use indicatif::MultiProgress;
use spinify::spawn_spinner_task;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let multi = MultiProgress::new();
    let handle = spawn_spinner_task(&multi, "Loading data", async {
        sleep(Duration::from_secs(2)).await;
        "Data loaded"
    });

    let result = handle.await?;
    println!("Result: {}", result);
    Ok(())
}
```

### Using Named Async Functions

You can define async functions separately and pass them to `spawn_spinner_task`:

```rust
use indicatif::MultiProgress;
use spinify::spawn_spinner_task;
use tokio::time::{sleep, Duration};

async fn load_data() -> String {
    sleep(Duration::from_secs(2)).await;
    "Data loaded".to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let multi = MultiProgress::new();
    let handle = spawn_spinner_task(&multi, "Loading data", load_data());

    let result = handle.await?;
    println!("Result: {}", result);
    Ok(())
}
```

### Custom Style Spinner Task

Use `spawn_spinner_task_with_style` for customized appearance:

```rust
use indicatif::{MultiProgress, ProgressStyle};
use spinify::spawn_spinner_task_with_style;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let multi = MultiProgress::new();
    let style = ProgressStyle::default_spinner()
        .template("{spinner:.green} {msg}")
        .unwrap();
    let handle = spawn_spinner_task_with_style(&multi, "Processing", style, async {
        sleep(Duration::from_secs(1)).await;
        42
    });

    let result = handle.await?;
    println!("Processed: {}", result);
    Ok(())
}
```

### Template-Based Spinner Task

Use `spawn_spinner_task_with_template` for template strings with error handling:

```rust
use indicatif::MultiProgress;
use spinify::spawn_spinner_task_with_template;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let multi = MultiProgress::new();
    let handle = spawn_spinner_task_with_template(&multi, "Fetching", "{spinner:.blue} {msg}", async {
        sleep(Duration::from_secs(3)).await;
        vec![1, 2, 3]
    })?;

    let result = handle.await?;
    println!("Fetched: {:?}", result);
    Ok(())
}
```

## API Reference

- `spawn_spinner_task(multi, label, future)`: Spawns a task with default spinner style.
- `spawn_spinner_task_with_style(multi, label, style, future)`: Spawns a task with custom `ProgressStyle`.
- `spawn_spinner_task_with_template(multi, label, template, future)`: Spawns a task with template string, returns `Result`.

For full documentation, run `cargo doc --open`.

## License

This project is licensed under the MIT License.