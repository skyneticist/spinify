// src/lib.rs
use indicatif::style::TemplateError;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{future::Future, time::Duration};
use tokio::task::JoinHandle;

// struct SpinnerTask<Fut, T> {
//     progress_bar: ProgressBar,
//     future: Fut,
//     _marker: std::marker::PhantomData<T>,
// }

// struct SpinnerTaskBuilder<Fut, T> {
//     multi: MultiProgress,
//     label: String,
//     style: ProgressStyle,
//     future: Fut,
//     _marker: std::marker::PhantomData<T>,
// }

#[derive(Clone)]
pub struct SpinnerTask<Fut, T> {
    pub future: Option<Fut>,
    pub label: String,
    pub progress_bar: MultiProgress,
    pub style: Option<ProgressStyle>,
    pub message: Option<String>,
    pub _marker: std::marker::PhantomData<T>,
}

// impl<Fut: Copy, T: Copy> Copy for SpinnerTask {}

impl<Fut: std::fmt::Debug, T: std::fmt::Debug> std::fmt::Debug for SpinnerTask<Fut, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpinnerTask")
            .field("future", &self.future)
            .field("label", &self.label)
            .field("progress_bar", &self.progress_bar)
            // .field("style", &self.style)
            .field("message", &self.message)
            .field("_marker", &self._marker)
            .finish()
    }
}

impl<Fut: std::future::Future<Output = T>, T> SpinnerTask<Fut, T> {
    pub fn new() -> Self {
        SpinnerTask {
            future: None,
            label: String::new(),
            progress_bar: MultiProgress::new(),
            style: None,
            message: None,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn progress_bar(&mut self, progress_bar: MultiProgress) -> &mut Self {
        self.progress_bar = progress_bar;
        self
    }

    pub fn future(&mut self, future: Fut) -> &mut Self {
        self.future = Some(future);
        self
    }

    pub fn future_mut(&mut self) -> &mut Option<Fut> {
        &mut self.future
    }

    pub fn set_style(&mut self, style: ProgressStyle) -> &mut Self {
        self.style = Some(style);
        self
    }

    pub fn set_message(&mut self, message: String) -> &mut Self {
        self.message = Some(message);
        self
    }

    //     pub async fn run(&self) -> Result<T, TemplateError>
    //     where
    //         Fut: Send + 'static,
    //         T: Send + 'static,
    //     {
    //         let prefix: String = self.label.into();
    //         let pb: ProgressBar = self.progress_bar.add(ProgressBar::new_spinner());

    //         pb.set_message(prefix.clone());
    //         pb.set_style(self.style.unwrap_or_else(|| {
    //             ProgressStyle::default_spinner()
    //                 .template("{spinner} {msg}")
    //                 .expect("default spinner template is valid")
    //         }));
    //         pb.enable_steady_tick(Duration::from_millis(100));

    //         Ok(tokio::spawn(async move {
    //             let result: T = self.future.unwrap().await;
    //             pb.finish_with_message(format!("{} – Done", prefix));
    //             result
    //         })
    //         .await
    //         .unwrap())
    //     }
    // }

    // JoinHandle<T>
    pub async fn run(&mut self) -> JoinHandle<T>
    where
        Fut: Send + 'static,
        T: Send + 'static,
    {
        let prefix = self.label.clone();
        let pb = self.progress_bar.add(ProgressBar::new_spinner());

        let style = self.style.clone().unwrap_or_else(|| {
            ProgressStyle::default_spinner()
                .template("{spinner} {msg}")
                .expect("default spinner template is valid")
        });

        pb.set_message(prefix.clone());
        pb.set_style(style);
        pb.enable_steady_tick(Duration::from_millis(100));

        let future = self
            .future
            .take()
            .expect("SpinnerTask::run called without a future");

        tokio::spawn(async move {
            let result = future.await;
            pb.finish_with_message(format!("{} – Done", prefix));
            result
        })
        // .unwrap()

        // handle
    }
}

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
        pb.finish_with_message(format!("{} – Done", prefix));
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use tokio::time::{sleep, Duration};

//     #[tokio::test]
//     async fn spawn_spinner_task_returns_future_output() {
//         let multi = MultiProgress::new();
//         let handle = spawn_spinner_task(&multi, "Test task", async {
//             sleep(Duration::from_millis(10)).await;
//             42
//         });

//         assert_eq!(handle.await.unwrap(), 42);
//     }

//     #[tokio::test]
//     async fn spawn_spinner_task_with_style_accepts_custom_style() {
//         let multi = MultiProgress::new();
//         let style = ProgressStyle::default_spinner()
//             .template("{spinner:.green} {msg}")
//             .unwrap();
//         let handle = spawn_spinner_task_with_style(&multi, "Styled task", style, async {
//             sleep(Duration::from_millis(10)).await;
//             99
//         });

//         assert_eq!(handle.await.unwrap(), 99);
//     }

//     #[tokio::test]
//     async fn spawn_spinner_task_with_template_returns_template_error() {
//         let multi = MultiProgress::new();
//         let result = spawn_spinner_task_with_template(&multi, "Bad template", "{:}", async { 1 });

//         assert!(result.is_err());
//     }
// }
