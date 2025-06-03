#[derive(Debug)]
pub enum TelemetryError {
    DeserialisationError(String),
    ConnectionError(String),
    TimeoutError(String),
    Other(String),
    Default,
}
