use std::time::Duration;

#[derive(Debug)]
pub enum ProgressEvent {
    Started(String),
    Finished(String, Duration),
    Failed(String, String),
    Cancelled(String),
}
