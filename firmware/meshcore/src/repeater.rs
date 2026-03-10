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

    pub async fn run(&self) -> ! {
        loop {}
    }
}
