
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "TIM2",
        address: 0x40000000,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
                block: "TIM_GP32",
                ir: &timer::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK1_TIM",
                kernel_clock: Clock(
                    "PCLK1_TIM",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APBENR1",
                        field: "TIM2EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "TIM2RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH4",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH3",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CH4",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "CH1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "CH2",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH3",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH4",
                af: Some(
                    14,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    27,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    27,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    27,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    27,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    27,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    27,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    27,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    26,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    26,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    26,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    26,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    26,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    26,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    26,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    31,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    31,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    31,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    31,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    31,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    31,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    31,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    29,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    29,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    29,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    29,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    29,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    29,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    29,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    30,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    30,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    30,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    30,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    30,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    30,
                ),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    30,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    28,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    28,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    28,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    28,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    28,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    28,
                ),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    28,
                ),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM2",
            },
        ],
    },
    Peripheral {
        name: "RTC",
        address: 0x40002800,
        registers: Some(
            PeripheralRegisters {
                kind: "rtc",
                version: "v1",
                block: "RTC",
                ir: &rtc::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK1",
                kernel_clock: Clock(
                    "PCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "BDCR",
                        field: "RTCEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "RTCAPBRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "RTC",
            },
        ],
    },
    Peripheral {
        name: "WWDG",
        address: 0x40002c00,
        registers: Some(
            PeripheralRegisters {
                kind: "wwdg",
                version: "v1",
                block: "WWDG",
                ir: &wwdg::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK1",
                kernel_clock: Clock(
                    "PCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APBENR1",
                        field: "WWDGEN",
                    },
                ),
                reset: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "WWDG",
            },
        ],
    },
    Peripheral {
        name: "IWDG",
        address: 0x40003000,
        registers: Some(
            PeripheralRegisters {
                kind: "iwdg",
                version: "v1",
                block: "IWDG",
                ir: &iwdg::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CTC",
        address: 0x40006c00,
        registers: Some(
            PeripheralRegisters {
                kind: "ctc",
                version: "v1",
                block: "CTC",
                ir: &ctc::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK1",
                kernel_clock: Clock(
                    "PLL",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APBENR1",
                        field: "CTCEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "CTCRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "RCC_CTC",
            },
        ],
    },
    Peripheral {
        name: "PWR",
        address: 0x40007000,
        registers: Some(
            PeripheralRegisters {
                kind: "pwr",
                version: "f072",
                block: "PWR",
                ir: &pwr::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK1",
                kernel_clock: Clock(
                    "PCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APBENR1",
                        field: "PWREN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "PWRRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "OPA",
        address: 0x40010300,
        registers: Some(
            PeripheralRegisters {
                kind: "opa",
                version: "v1",
                block: "OPA",
                ir: &opa::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK1",
                kernel_clock: Clock(
                    "PCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APBENR1",
                        field: "OPAEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "OPARST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA9",
                signal: "VP1",
                af: None,
            },
            PeripheralPin {
                pin: "PA10",
                signal: "VN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "VOUT1",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "VP2",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "VN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "VOUT2",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "VP3",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "VN3",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "VOUT3",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CRC",
        address: 0x40023000,
        registers: Some(
            PeripheralRegisters {
                kind: "crc",
                version: "v1",
                block: "CRC",
                ir: &crc::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK1",
                kernel_clock: Clock(
                    "HCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHBENR",
                        field: "CRCEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHBRSTR",
                        field: "CRCRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DIV",
        address: 0x40023800,
        registers: Some(
            PeripheralRegisters {
                kind: "div",
                version: "v1",
                block: "DIV",
                ir: &div::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "HCLK1",
                kernel_clock: Clock(
                    "HCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "AHBENR",
                        field: "DIVEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHBRSTR",
                        field: "DIVRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOA",
        address: 0x50000000,
        registers: Some(
            PeripheralRegisters {
                kind: "gpio",
                version: "v1",
                block: "GPIO",
                ir: &gpio::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PCLK1",
                kernel_clock: Clock(
                    "PCLK1",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "IOPENR",
                        field: "GPIOAEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "IOPRSTR",
                        field: "GPIOARST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
    },
];
                pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "WWDG",
        number: 0,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[
    DmaChannel {
        name: "DMA1_CH1",
        dma: "DMA1",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH2",
        dma: "DMA1",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH3",
        dma: "DMA1",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH4",
        dma: "DMA1",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH5",
        dma: "DMA1",
        channel: 4,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH6",
        dma: "DMA1",
        channel: 5,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH7",
        dma: "DMA1",
        channel: 6,
        dmamux: None,
        dmamux_channel: None,
    },
];
            #[path="../registers/crc_v1.rs"] pub mod crc;
#[path="../registers/ctc_v1.rs"] pub mod ctc;
#[path="../registers/div_v1.rs"] pub mod div;
#[path="../registers/gpio_v1.rs"] pub mod gpio;
#[path="../registers/iwdg_v1.rs"] pub mod iwdg;
#[path="../registers/opa_v1.rs"] pub mod opa;
#[path="../registers/pwr_f072.rs"] pub mod pwr;
#[path="../registers/rtc_v1.rs"] pub mod rtc;
#[path="../registers/timer_v1.rs"] pub mod timer;
#[path="../registers/wwdg_v1.rs"] pub mod wwdg;
