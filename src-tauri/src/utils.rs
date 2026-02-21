use std::env;

pub fn is_dev() -> bool {
    env::var("VITE_MODE").unwrap() == "dev"
}
