/// This module provides a [`super::PTY`] backend that uses
/// [conpty](https://docs.microsoft.com/en-us/windows/console/creating-a-pseudoconsole-session) as its implementation.
/// This backend is available on Windows 10 starting from build number 1809.
// Actual implementation if winpty is available
mod pty_impl;

pub use pty_impl::ConPTY;
