mod bindings;

pub use self::bindings::*;

#[cfg(target_os = "macos")]
#[link(name = "Foundation", kind = "framework")]
extern "C" {}
