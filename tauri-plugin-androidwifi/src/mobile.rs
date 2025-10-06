use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_androidwifi);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Androidwifi<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.plugin.androidwifi", "WifiPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_androidwifi)?;
    Ok(Androidwifi(handle))
}

/// Access to the androidwifi APIs.
pub struct Androidwifi<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Androidwifi<R> {
    pub fn get_wifi_details(&self, payload: Empty) -> crate::Result<WifiDetailsResponse> {
        self.0
            .run_mobile_plugin("getWifiDetails", payload)
            .map_err(Into::into)
    }

    pub fn connect_wifi(&self, payload: ConnectWifiPayload) -> crate::Result<ConnectWifiResponse> {
        self.0
            .run_mobile_plugin("connectWifi", payload)
            .map_err(Into::into)
    }

    pub fn get_current_wifi_details(&self, payload: Empty) -> crate::Result<CurrentWifiResponse> {
        self.0
            .run_mobile_plugin("getCurrentWifiDetails", payload)
            .map_err(Into::into)
    }

    pub fn get_mac_address(
        &self,
        payload: GetMacAddressPayload,
    ) -> crate::Result<MacAddressResponse> {
        self.0
            .run_mobile_plugin("getMacAddress", payload)
            .map_err(Into::into)
    }

    pub fn get_gateway_ip(&self, payload: Empty) -> crate::Result<GatewayIpResponse> {
        self.0
            .run_mobile_plugin("getGatewayIp", payload)
            .map_err(Into::into)
    }

    pub fn mark_captive_portal_dismissed(
        &self,
        payload: Empty,
    ) -> crate::Result<MacAddressResponse> {
        self.0
            .run_mobile_plugin("markCaptivePortalDismissed", payload)
            .map_err(Into::into)
    }

    pub fn detect_tollgate(&self, payload: Empty) -> crate::Result<TollgateDetectionResponse> {
        self.0
            .run_mobile_plugin("detectTollgate", payload)
            .map_err(Into::into)
    }

    pub async fn get_network_status(&self, payload: Empty) -> crate::Result<NetworkStatusResponse> {
        // For mobile, we'll call the plugin synchronously and convert to async
        tokio::task::spawn_blocking({
            let handle = self.0.clone();
            move || {
                handle
                    .run_mobile_plugin("getNetworkStatus", payload)
                    .map_err(Into::into)
            }
        })
        .await
        .unwrap_or_else(|_| {
            Err(crate::Error::Anyhow(
                "Failed to execute network status check".into(),
            ))
        })
    }
}
