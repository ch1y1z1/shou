use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use log::info;

pub fn connect_wifi(
    wifi: &mut BlockingWifi<EspWifi<'static>>,
    ssid: &str,
    pass_word: &str,
) -> anyhow::Result<()> {
    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
        ssid: ssid.into(),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: pass_word.into(),
        channel: None,
    });

    wifi.set_configuration(&wifi_configuration)?;

    wifi.start()?;
    info!("Wifi started");

    wifi.connect()?;
    info!("Wifi connected");

    wifi.wait_netif_up()?;
    info!("Wifi netif up");

    Ok(())
}
