include!("../metadata_0000.rs");
            use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
            pub static METADATA: Metadata = Metadata {
                name: "PY32F003L24",
                family: "Low-Cost",
                line: "PY32F003",
                memory: &[
    MemoryRegion {
        name: "BANK_1",
        kind: MemoryRegionKind::Flash,
        address: 0x8000000,
        size: 16384,
        settings: Some(
            FlashSettings {
                page_size: 128,
                sector_size: 4096,
                erase_value: 0,
            },
        ),
    },
    MemoryRegion {
        name: "SRAM",
        kind: MemoryRegionKind::Ram,
        address: 0x20000000,
        size: 2048,
        settings: None,
    },
],
                peripherals: PERIPHERALS,
                nvic_priority_bits: Some(2),
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };