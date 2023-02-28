use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

/// Returns a random alphanumeric string of the given length.
pub fn random_alphanumeric_string(length: usize) -> String {
    return thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
}
