include!("../metadata_0003.rs");
            use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
            pub static METADATA: Metadata = Metadata {
                name: "PY32T020G16",
                family: "Touch",
                line: "PY32T020",
                memory: &[
    MemoryRegion {
        name: "BANK_1",
        kind: MemoryRegionKind::Flash,
        address: 0x8000000,
        size: 32768,
        settings: Some(
            FlashSettings {
                erase_size: 128,
                write_size: 4,
                erase_value: 0,
            },
        ),
    },
    MemoryRegion {
        name: "SRAM",
        kind: MemoryRegionKind::Ram,
        address: 0x20000000,
        size: 4096,
        settings: None,
    },
],
                peripherals: PERIPHERALS,
                nvic_priority_bits: Some(2),
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };