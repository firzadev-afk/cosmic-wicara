//! D-Bus notification transport.

use crate::notify::{BUS_NAME, NotifyError, OBJECT_PATH};
use std::collections::HashMap;

/// D-Bus notifications proxy.
#[zbus::proxy(interface = "org.freedesktop.Notifications")]
trait NotificationsProxy {
    #[allow(clippy::too_many_arguments)]
    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: &[&str],
        hints: HashMap<&str, zbus::Variant<'_>>,
        expire_timeout: i32,
    ) -> zbus::Result<u32>;
}

pub(crate) fn map_dbus(err: zbus::Error) -> NotifyError {
    NotifyError::Db(err.to_string())
}

pub(crate) async fn notify_async(
    app_name: &str,
    app_icon: &str,
    summary: &str,
    body: &str,
    expire_timeout_ms: i32,
) -> Result<u32, NotifyError> {
    let conn = zbus::Connection::session().await.map_err(map_dbus)?;
    let proxy = NotificationsProxyProxy::builder(&conn)
        .destination(BUS_NAME)
        .path(OBJECT_PATH)
        .build()
        .await
        .map_err(map_dbus)?;
    let hints: HashMap<&str, zbus::Variant<'_>> = HashMap::new();
    let actions: &[&str] = &[];
    proxy
        .notify(
            app_name,
            0,
            app_icon,
            summary,
            body,
            actions,
            hints,
            expire_timeout_ms,
        )
        .await
        .map_err(map_dbus)
}
