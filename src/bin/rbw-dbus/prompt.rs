//! # D-Bus interface proxy for: `org.freedesktop.Secret.Prompt`
//!
//! This code was generated by `zbus-xmlgen` `5.0.1` from D-Bus introspection data.
//! Source: `org.freedesktop.Secrets.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the [Writing a client proxy] section of the zbus
//! documentation.
//!
//!
//! [Writing a client proxy]: https://dbus2.github.io/zbus/client.html
//! [D-Bus standard interfaces]: https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces,
use zbus::proxy;
#[proxy(interface = "org.freedesktop.Secret.Prompt", assume_defaults = true)]
pub trait Prompt {
    /// Dismiss method
    fn dismiss(&self) -> zbus::Result<()>;

    /// Prompt method
    fn prompt(&self, window_id: &str) -> zbus::Result<()>;

    /// Completed signal
    #[zbus(signal)]
    fn completed(
        &self,
        dismissed: bool,
        result: zbus::zvariant::Value<'_>,
    ) -> zbus::Result<()>;
}