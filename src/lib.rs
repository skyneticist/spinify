// src/lib.rs
use indicatif::style::TemplateError;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{future::Future, time::Duration};
use tokio::task::JoinHandle;

/// Spawn a task with a spinner using the default style.
///
/// * `multi`  – your shared `MultiProgress`
/// * `label`  – shown next to the spinner
/// * `fut`    – any future: e.g. an API call
///
/// Returns a `JoinHandle` of the future’s output.
///
/// This helper uses a simple default spinner style. If you want a custom
/// appearance, use `spawn_spinner_task_with_style` or
/// `spawn_spinner_task_with_template`.
///
/// # Example
///
/// ```no_run
/// use indicatif::MultiProgress;
/// use spinify::spawn_spinner_task;
/// use tokio::time::{sleep, Duration};
///
/// #[tokio::main]
/// async fn main() {
///     let multi = MultiProgress::new();
///     let handle = spawn_spinner_task(&multi, "Loading", async {
///         sleep(Duration::from_millis(10)).await;
///         "done"
///     });
///
///     assert_eq!(handle.await.unwrap(), "done");
///     // Spinner will show: ✅ Loading - Done
/// }
/// ```
pub fn spawn_spinner_task<Fut, T>(
    multi: &MultiProgress,
    label: impl Into<String>,
    fut: Fut,
) -> JoinHandle<T>
where
    Fut: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    spawn_spinner_task_with_style(
        multi,
        label,
        ProgressStyle::default_spinner()
            .template("{spinner} {msg}")
            .expect("default spinner template is valid"),
        fut,
    )
}

/// Spawn a task with a spinner using a caller-provided style.
///
/// This is useful when you want a different template, colors, or progress
/// characters.
pub fn spawn_spinner_task_with_style<Fut, T>(
    multi: &MultiProgress,
    label: impl Into<String>,
    style: ProgressStyle,
    fut: Fut,
) -> JoinHandle<T>
where
    Fut: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let prefix = label.into();
    let pb: ProgressBar = multi.add(ProgressBar::new_spinner());
    pb.set_message(prefix.clone());
    pb.set_style(style);
    pb.enable_steady_tick(Duration::from_millis(100));

    tokio::spawn(async move {
        let result: T = fut.await;
        pb.finish_with_message(format!("✅ {} - Done", prefix));
        result
    })
}

/// Spawn a task with a spinner using a custom style template.
///
/// This returns a typed result to preserve template parsing errors instead of
/// panicking on invalid format strings.
pub fn spawn_spinner_task_with_template<Fut, T>(
    multi: &MultiProgress,
    label: impl Into<String>,
    template: impl AsRef<str>,
    fut: Fut,
) -> Result<JoinHandle<T>, TemplateError>
where
    Fut: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let style = ProgressStyle::default_spinner().template(template.as_ref())?;
    Ok(spawn_spinner_task_with_style(multi, label, style, fut))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn spawn_spinner_task_returns_future_output() {
        let multi = MultiProgress::new();
        let handle = spawn_spinner_task(&multi, "Test task", async {
            sleep(Duration::from_millis(10)).await;
            42
        });

        assert_eq!(handle.await.unwrap(), 42);
    }

    #[tokio::test]
    async fn spawn_spinner_task_with_style_accepts_custom_style() {
        let multi = MultiProgress::new();
        let style = ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap();
        let handle = spawn_spinner_task_with_style(&multi, "Styled task", style, async {
            sleep(Duration::from_millis(10)).await;
            99
        });

        assert_eq!(handle.await.unwrap(), 99);
    }

    #[tokio::test]
    async fn spawn_spinner_task_with_template_returns_template_error() {
        let multi = MultiProgress::new();
        let result = spawn_spinner_task_with_template(&multi, "Bad template", "{:}", async { 1 });

        assert!(result.is_err());
    }
}
