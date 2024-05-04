use crate::environment::Args;

#[derive(Clone, Debug)]
pub struct Argon {
    secret: String,
    memory_size: Option<u32>,
    iterations: Option<u32>,
}