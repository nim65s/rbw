//! # D-Bus interface proxy for: `org.freedesktop.Secret.Item`
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
#[proxy(interface = "org.freedesktop.Secret.Item", assume_defaults = true)]
pub trait Item {
    /// Delete method
    fn delete(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// GetSecret method
    fn get_secret(
        &self,
        session: &zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<(
        zbus::zvariant::OwnedObjectPath,
        Vec<u8>,
        Vec<u8>,
        String,
    )>;

    /// SetSecret method
    fn set_secret(
        &self,
        secret: &(&zbus::zvariant::ObjectPath<'_>, &[u8], &[u8], &str),
    ) -> zbus::Result<()>;

    /// Attributes property
    #[zbus(property)]
    fn attributes(
        &self,
    ) -> zbus::Result<std::collections::HashMap<String, String>>;
    #[zbus(property)]
    fn set_attributes(
        &self,
        value: std::collections::HashMap<&str, &str>,
    ) -> zbus::Result<()>;

    /// Created property
    #[zbus(property)]
    fn created(&self) -> zbus::Result<u64>;

    /// Label property
    #[zbus(property)]
    fn label(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn set_label(&self, value: &str) -> zbus::Result<()>;

    /// Locked property
    #[zbus(property)]
    fn locked(&self) -> zbus::Result<bool>;

    /// Modified property
    #[zbus(property)]
    fn modified(&self) -> zbus::Result<u64>;
}
