#[derive(Debug)]
pub enum BootError {
    Bind { address: String, error: String },
    Server { error: String },
}
