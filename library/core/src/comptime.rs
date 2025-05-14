//! Defines items for interaction with the compiler during comptime evaluation.
#[lang = "ComptimeContext"]
#[unstable(feature = "comptime", issue = "none")]
#[cfg(not(bootstrap))]
pub struct Context;
