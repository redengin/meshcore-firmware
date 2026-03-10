pub struct Repeater<LORA_RK, LORA_DLY>
where
    LORA_RK: lora_phy::mod_traits::RadioKind,
    LORA_DLY: lora_phy::DelayNs,
{
    lora_radio: lora_phy::LoRa<LORA_RK, LORA_DLY>,
}

impl<LORA_RK, LORA_DLY> Repeater<LORA_RK, LORA_DLY>
where
    LORA_RK: lora_phy::mod_traits::RadioKind,
    LORA_DLY: lora_phy::DelayNs,
{
    pub fn new(lora_radio: lora_phy::LoRa<LORA_RK, LORA_DLY>) -> Self {
        Self {
            lora_radio: lora_radio,
        }
    }

    /// handle repeater tasks forever
    pub async fn run(&mut self) -> ! {
        // configure radio
        // TODO use dynamically configured value
        const LORA_BAND_HZ: u32 = (910.525 * 1E6) as u32;
        let mod_params = self.lora_radio.create_modulation_params(
            lora_phy::mod_params::SpreadingFactor::_7,
            lora_phy::mod_params::Bandwidth::_62KHz,
            lora_phy::mod_params::CodingRate::_4_5,
            LORA_BAND_HZ,
        ).unwrap();


        loop {}
    }
}
