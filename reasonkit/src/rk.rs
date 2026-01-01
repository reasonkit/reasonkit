//! Short alias for the ReasonKit CLI.
//!
//! This is a convenience binary that provides `rk` as a shorter alternative
//! to `reasonkit`. Both binaries are functionally identical.
//!
//! ```bash
//! # These are equivalent:
//! rk think "Your question"
//! reasonkit think "Your question"
//! ```

// Simply re-export the main binary
include!("main.rs");
