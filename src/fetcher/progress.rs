use indicatif::{ProgressBar, ProgressStyle};

pub struct ProgressBarHandler {
    pub progress_bar: ProgressBar,
}

impl ProgressBarHandler {
    pub fn new(total_items: u64) -> Self {
        let progress_bar = ProgressBar::new(total_items);

        // Style the progress bar
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .expect("Unable to create progress bar")
                .progress_chars("#>-"),
        );

        Self { progress_bar }
    }
}
