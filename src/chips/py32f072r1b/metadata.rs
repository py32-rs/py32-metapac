include!("../metadata_0000.rs");
            use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
            pub static METADATA: Metadata = Metadata {
                name: "PY32F072R1B",
                family: "Mainstream",
                line: "PY32F072",
                memory: &[
    MemoryRegion {
        name: "BANK_1",
        kind: MemoryRegionKind::Flash,
        address: 0x8000000,
        size: 131072,
        settings: Some(
            FlashSettings {
                page_size: 256,
                sector_size: 8192,
                erase_value: 0,
            },
        ),
    },
    MemoryRegion {
        name: "SRAM",
        kind: MemoryRegionKind::Ram,
        address: 0x20000000,
        size: 16384,
        settings: None,
    },
],
                peripherals: PERIPHERALS,
                nvic_priority_bits: Some(2),
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };