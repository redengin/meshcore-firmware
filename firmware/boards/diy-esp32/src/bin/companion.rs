#![no_std]
#![no_main]

use common::trouble_host::peripheral;
// provide the shared crates via re-export
use common::*;
use soc_esp32::*; // provides the panic handler

// provide logging primitives
use log::*;

// provice scheduling primitives
use common::embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use common::embassy_sync::mutex::Mutex;
use common::embassy_time::{Delay, Duration, Timer};

/// LoRa radio SPI bus
static LORA_SPI_BUS: static_cell::StaticCell<
    Mutex<CriticalSectionRawMutex, esp_hal::spi::master::Spi<'static, esp_hal::Async>>,
> = static_cell::StaticCell::new();

#[soc_esp32::esp_rtos::main]
// async fn main(spawner: soc_esp32::embassy_executor::Spawner) -> ! {
async fn main(spawner: embassy_executor::Spawner) -> ! {
    // initialize the SoC interface
    let peripherals = esp_hal::init(
        // max out clock to support radio
        esp_hal::Config::default().with_cpu_clock(esp_hal::clock::CpuClock::max()),
    );

    // initialize logging
    esp_println::logger::init_logger_from_env();
    info!("initializing...");

    // initialize the rtos
    use esp_hal::timer::timg::TimerGroup;
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    use esp_hal::interrupt::software::SoftwareInterruptControl;
    let sw_int = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_int.software_interrupt0);

    // initialize LoRa radio
    //==============================================================================
    info!("initializing LoRA radio...");
    // the following initializes a heltec v3 sx1262
    // heltec v3 pins https://heltec.org/wp-content/uploads/2023/09/pin.png
    //--------------------------------------------------------------------------
    info!("creating lora_nss");
    let lora_nss = esp_hal::gpio::Output::new(
        // peripherals.GPIO8,
        // FIXME GPIO8 not allowed on esp32 (hangs rather than panics), so remapping
        peripherals.GPIO26,
        esp_hal::gpio::Level::High,
        esp_hal::gpio::OutputConfig::default(),
    );
    let lora_sck = peripherals.GPIO9;
    let lora_mosi = peripherals.GPIO10;
    let lora_miso = peripherals.GPIO11;
    let lora_reset = esp_hal::gpio::Output::new(
        peripherals.GPIO12,
        esp_hal::gpio::Level::Low,
        esp_hal::gpio::OutputConfig::default(),
    );
    let lora_busy =
        esp_hal::gpio::Input::new(peripherals.GPIO13, esp_hal::gpio::InputConfig::default());
    let lora_dio1 =
        esp_hal::gpio::Input::new(peripherals.GPIO14, esp_hal::gpio::InputConfig::default());
    //--------------------------------------------------------------------------
    let lora_spi = esp_hal::spi::master::Spi::new(
        peripherals.SPI2,
        esp_hal::spi::master::Config::default()
            .with_frequency(esp_hal::time::Rate::from_khz(100))
            .with_mode(esp_hal::spi::Mode::_0),
    )
    .unwrap()
    .with_sck(lora_sck)
    .with_mosi(lora_mosi)
    .with_miso(lora_miso)
    .into_async();
    info!("LoRa SPI initialized");
    let lora_spi_bus = LORA_SPI_BUS.init(Mutex::new(lora_spi));
    let lora_spi_device =
        embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice::new(lora_spi_bus, lora_nss);
    // create a radio instance
    let lora_interface = lora_phy::iv::GenericSx126xInterfaceVariant::new(
        lora_reset, lora_dio1, lora_busy, None, None,
    )
    .unwrap();
    let sx126x_config = lora_phy::sx126x::Config {
        chip: lora_phy::sx126x::Sx1262,
        // TODO are these the correct parameters?
        //----------------------------------------------------------------------
        tcxo_ctrl: Some(lora_phy::sx126x::TcxoCtrlVoltage::Ctrl1V7),
        use_dcdc: false,
        rx_boost: true,
        //----------------------------------------------------------------------
    };
    // FIXME disabling as I have no radio for development
    // let mut lora_radio = lora_phy::LoRa::new(
    //     lora_phy::sx126x::Sx126x::new(lora_spi_device, lora_interface, sx126x_config),
    //     false,
    //     Delay,
    // )
    // .await
    // .unwrap();
    info!("LoRa radio initialized");
    //==============================================================================

    // initialize the bluetooth hardware
    // https://github.com/esp-rs/esp-hal/tree/main/examples/ble/bas_peripheral
    create_heap!(); // required by radio (use 64K reclaimed from bootloader)
    let ble_connector = esp_radio::ble::controller::BleConnector::new(
        peripherals.BT,
        esp_radio::ble::Config::default().with_max_connections(1),
    )
    .unwrap();
    info!("BLE initialized");
    spawner.spawn(task_ble_host(ble_connector)).unwrap();

    // initialize WiFi hardware
    // https://github.com/esp-rs/esp-hal/blob/main/examples/wifi/80211_tx/
    let (mut wifi_controller, _interfaces) =
        esp_radio::wifi::new(peripherals.WIFI, Default::default()).unwrap();
    wifi_controller
        .set_mode(esp_radio::wifi::WifiMode::Station)
        .unwrap();
    wifi_controller.start_async().await.unwrap();
    info!("WiFi initialized");

    //------------------------------------------------------------------------------

    loop {
        info!("Hello world!");
        Timer::after(Duration::from_secs(1)).await;
    }
}

// FIXME is trouble_host able to be used commonly across hardware?
#[embassy_executor::task]
async fn task_ble_host(_ble_connector: esp_radio::ble::controller::BleConnector<'static>) {
    // /// Max number of connections
    // const CONNECTIONS_MAX: usize = 1;
    // /// Max number of L2CAP channels.
    // const L2CAP_CHANNELS_MAX: usize = 2; // Signal + att

    // // create the BLE Host controller (i.e trouble_host)
    // use trouble_host::prelude::*;
    // let ble_controller: ExternalController<_, 1> = ExternalController::new(ble_connector);

    // // configure trouble_host
    // let mut resources: HostResources<DefaultPacketPool, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX> =
    //     HostResources::new();
    // let mac = esp_hal::efuse::Efuse::mac_address();
    // let stack =
    //     trouble_host::new(ble_controller, &mut resources).set_random_address(Address::random(mac));
    // let Host {
    //     mut peripheral,
    //     runner,
    //     ..
    // } = stack.build();

    info!("Starting advertising and GATT service");
    // TODO implement

    error!("BLE host stopped");
}
