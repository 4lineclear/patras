//! The actual api used by the core server

use derivative::Derivative;
use libreauth::pass::Hasher;

/// The central auth authority
#[derive(Derivative)]
#[derivative(Debug)]
pub struct AuthSession {
    #[derivative(Debug = "ignore")]
    hasher: Hasher,
}

// TODO: everything
