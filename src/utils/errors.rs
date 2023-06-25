#[derive(Debug)]
pub struct SerializeError {
    pub msg: String,
}
impl SerializeError {
    pub fn new_string(s: String) -> SerializeError {
        SerializeError { msg: s }
    }
}
