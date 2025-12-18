
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v2",
                block: "ADC",
                ir: &adc::REGISTERS,
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
                        register: "APBENR2",
                        field: "ADCEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR2",
                        field: "ADCRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN0",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN7",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IN8",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN9",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN10",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IN11",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IN12",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "IN13",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "IN14",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "IN15",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC1",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "ADC1",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "ADC1",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "ADC1",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "ADC1",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "ADC1",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    0,
                ),
            },
            PeripheralDmaChannel {
                signal: "ADC1",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    0,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "ADC_COMP",
            },
        ],
    },
    Peripheral {
        name: "CAN",
        address: 0x40006400,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "CAN",
            },
        ],
    },
    Peripheral {
        name: "DBGMCU",
        address: 0x40015800,
        registers: Some(
            PeripheralRegisters {
                kind: "dbgmcu",
                version: "f072",
                block: "DBGMCU",
                ir: &dbgmcu::REGISTERS,
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
                        register: "APBENR2",
                        field: "DBGEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR2",
                        field: "DBGRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA13",
                signal: "SWDIO",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "SWCLK",
                af: Some(
                    0,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EXTI",
        address: 0x40021800,
        registers: Some(
            PeripheralRegisters {
                kind: "exti",
                version: "v1",
                block: "EXTI",
                ir: &exti::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "PVD",
                interrupt: "PVD",
            },
            PeripheralInterrupt {
                signal: "EXTI0",
                interrupt: "EXTI0_1",
            },
            PeripheralInterrupt {
                signal: "EXTI1",
                interrupt: "EXTI0_1",
            },
            PeripheralInterrupt {
                signal: "EXTI2",
                interrupt: "EXTI2_3",
            },
            PeripheralInterrupt {
                signal: "EXTI3",
                interrupt: "EXTI2_3",
            },
            PeripheralInterrupt {
                signal: "EXTI4",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI5",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI6",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI7",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI8",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI9",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI10",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI11",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI12",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI13",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI14",
                interrupt: "EXTI4_15",
            },
            PeripheralInterrupt {
                signal: "EXTI15",
                interrupt: "EXTI4_15",
            },
        ],
    },
    Peripheral {
        name: "FLASH",
        address: 0x40022000,
        registers: Some(
            PeripheralRegisters {
                kind: "flash",
                version: "f072",
                block: "FLASH",
                ir: &flash::REGISTERS,
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
                        field: "FLASHEN",
                    },
                ),
                reset: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "FLASH",
            },
        ],
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
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOB",
        address: 0x50000400,
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
                        field: "GPIOBEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "IOPRSTR",
                        field: "GPIOBRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOC",
        address: 0x50000800,
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
                        field: "GPIOCEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "IOPRSTR",
                        field: "GPIOCRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOF",
        address: 0x50001400,
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
                        field: "GPIOFEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "IOPRSTR",
                        field: "GPIOFRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "I2C1",
        address: 0x40005400,
        registers: Some(
            PeripheralRegisters {
                kind: "i2c",
                version: "v1",
                block: "I2C",
                ir: &i2c::REGISTERS,
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
                        field: "I2C1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "I2C1RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "SMBA",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SCL",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "SDA",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SMBA",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDA",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCL",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SDA",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCL",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "SDA",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCL",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "SDA",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PF3",
                signal: "SCL",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "SDA",
                af: Some(
                    6,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    16,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    15,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    15,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    16,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    15,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    16,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    15,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    16,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    15,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    16,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    16,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    15,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    15,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    16,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "I2C1",
            },
        ],
    },
    Peripheral {
        name: "I2C2",
        address: 0x40005800,
        registers: Some(
            PeripheralRegisters {
                kind: "i2c",
                version: "v1",
                block: "I2C",
                ir: &i2c::REGISTERS,
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
                        field: "I2C2EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "I2C2RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA9",
                signal: "SCL",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "SDA",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDA",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCL",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SDA",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCL",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "SDA",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCL",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "SDA",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PF3",
                signal: "SCL",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "SDA",
                af: Some(
                    13,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    18,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    17,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    18,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    17,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    18,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    17,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    17,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    18,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    18,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    17,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    17,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    18,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    17,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    18,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "I2C2",
            },
        ],
    },
    Peripheral {
        name: "LPTIM1",
        address: 0x40007c00,
        registers: None,
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "TIM6_LPTIM1_DAC",
            },
        ],
    },
    Peripheral {
        name: "RCC",
        address: 0x40021000,
        registers: Some(
            PeripheralRegisters {
                kind: "rcc",
                version: "f072",
                block: "RCC",
                ir: &rcc::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "MCO",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "MCO",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "MCO",
                af: Some(
                    9,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "RCC_CTC",
            },
        ],
    },
    Peripheral {
        name: "SPI1",
        address: 0x40013000,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v2",
                block: "SPI",
                ir: &spi::REGISTERS,
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
                        register: "APBENR2",
                        field: "SPI1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR2",
                        field: "SPI1RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "SCK",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "MOSI",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SCK",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MSO",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MOSI",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "MISO",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "MOSI",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "NSS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "MISO",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "MOS",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "NSS",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "MOSI",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "SCK",
                af: Some(
                    11,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    4,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    3,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    4,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    3,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    4,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    3,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    3,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    4,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    3,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    4,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    4,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    3,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    3,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    4,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "SPI1",
            },
        ],
    },
    Peripheral {
        name: "SPI2",
        address: 0x40003800,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v2",
                block: "SPI",
                ir: &spi::REGISTERS,
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
                        field: "SPI2EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "SPI2RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "SCK",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "MOSI",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "MISO",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "MISO",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "NSS",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "MOSI",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "NSS",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "MISO",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "MOSI",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "MISO",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "MISO",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "MOSI",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCK",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "NSS",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCK",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "MOSI",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "NSS",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCK",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MISO",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "MOSI",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "MISO",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "MOSI",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "SCK",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "MISO",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "MOSI",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "NSS",
                af: Some(
                    11,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    5,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    5,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    5,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    5,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    5,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    5,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    6,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    5,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "SPI2",
            },
        ],
    },
    Peripheral {
        name: "SYSCFG",
        address: 0x40010000,
        registers: Some(
            PeripheralRegisters {
                kind: "syscfg",
                version: "f072",
                block: "SYSCFG",
                ir: &syscfg::REGISTERS,
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
                        register: "APBENR2",
                        field: "SYSCFGEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR2",
                        field: "SYSCFGRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TIM1",
        address: 0x40012c00,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
                block: "TIM_ADV",
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
                        register: "APBENR2",
                        field: "TIM1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR2",
                        field: "TIM1RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CH4",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "BKIN2",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "ETR",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH2",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH2N",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH3N",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH3",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH1",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH1N",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "BKIN",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH1N",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH2N",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH3N",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH2",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "CH3",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "CH4",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "BKIN",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "BKIN2",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PF0",
                signal: "BKIN",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PF1",
                signal: "CHIN",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "CH2",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PF5",
                signal: "BKIN2",
                af: Some(
                    2,
                ),
            },
        ],
        dma_channels: &[
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
                    21,
                ),
            },
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
                    20,
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
                    22,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    24,
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
                    19,
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
                    25,
                ),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    23,
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
                    21,
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
                    19,
                ),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    23,
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
                    20,
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
                    22,
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
                    25,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    24,
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
                    25,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    24,
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
                    20,
                ),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    23,
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
                    19,
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
                    21,
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
                    22,
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
                    20,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    24,
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
                    21,
                ),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    23,
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
                    22,
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
                    25,
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
                    19,
                ),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    23,
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
                    20,
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
                    19,
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
                    21,
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
                    22,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    24,
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
                    25,
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
                    20,
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
                    25,
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
                    19,
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
                    22,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    24,
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
                    21,
                ),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    23,
                ),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    23,
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
                    19,
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
                    25,
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
                    20,
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
                    22,
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
                    21,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    24,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_BRK_UP_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_BRK_UP_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_BRK_UP_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_BRK_UP_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_CC",
            },
        ],
    },
    Peripheral {
        name: "TIM3",
        address: 0x40000400,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
                block: "TIM_GP16",
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
                        field: "TIM3EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "TIM3RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH2",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH3",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH4",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH2",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH3",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CHA",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "ETR",
                af: Some(
                    0,
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
                    33,
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
                    36,
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
                    34,
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
                    32,
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
                    35,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    37,
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
                    33,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    37,
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
                    34,
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
                    35,
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
                    36,
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
                    32,
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
                    33,
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
                    34,
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
                    32,
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
                    35,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    37,
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
                    36,
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
                    35,
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
                    32,
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
                    33,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    37,
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
                    36,
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
                    34,
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
                    33,
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
                    34,
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
                    35,
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
                    32,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    37,
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
                    36,
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
                    34,
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
                    35,
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
                    36,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    37,
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
                    33,
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
                    32,
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
                    34,
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
                    32,
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
                    35,
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
                    36,
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
                    33,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    37,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM3",
            },
        ],
    },
    Peripheral {
        name: "TIM6",
        address: 0x40001000,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
                block: "TIM_BASIC",
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
                        field: "TIM6EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "TIM6RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[
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
                    38,
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
                    38,
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
                    38,
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
                    38,
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
                    38,
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
                    38,
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
                    38,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM6_LPTIM1_DAC",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM6_LPTIM1_DAC",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM6_LPTIM1_DAC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM6_LPTIM1_DAC",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM6_LPTIM1_DAC",
            },
        ],
    },
    Peripheral {
        name: "TIM7",
        address: 0x40001400,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
                block: "TIM_BASIC",
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
                        field: "TIM7EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "TIM7RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[
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
                    39,
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
                    39,
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
                    39,
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
                    39,
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
                    39,
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
                    39,
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
                    39,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM7",
            },
        ],
    },
    Peripheral {
        name: "TIM14",
        address: 0x40002000,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
                block: "TIM_1CH_CMP",
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
                        register: "APBENR2",
                        field: "TIM14EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR2",
                        field: "TIM14RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "CH1",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH1",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CH1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PF0",
                signal: "CH1",
                af: Some(
                    11,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM14",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM14",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM14",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM14",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM14",
            },
        ],
    },
    Peripheral {
        name: "TIM15",
        address: 0x40014000,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
                block: "TIM_1CH_CMP",
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
                        register: "APBENR2",
                        field: "TIM15EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR2",
                        field: "TIM15RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "CH1N",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH1",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH2",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "BKIN",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "BKIN",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "BKIN",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH1N",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH1",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH2",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH1N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH1",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CH2",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "BKIN",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PF1",
                signal: "CH1N",
                af: Some(
                    11,
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
                    41,
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
                    40,
                ),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    44,
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
                    42,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    43,
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
                    41,
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
                    40,
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
                    42,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    43,
                ),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    44,
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
                    40,
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
                    41,
                ),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    44,
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
                    42,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    43,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    43,
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
                    40,
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
                    41,
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
                    42,
                ),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    44,
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
                    41,
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
                    40,
                ),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    44,
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
                    42,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    43,
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
                    42,
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
                    41,
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
                    40,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    43,
                ),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    44,
                ),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    44,
                ),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    43,
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
                    41,
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
                    40,
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
                    42,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM15",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM15",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM15",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM15",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM15",
            },
        ],
    },
    Peripheral {
        name: "TIM16",
        address: 0x40014400,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
                block: "TIM_1CH_CMP",
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
                        register: "APBENR2",
                        field: "TIM16EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR2",
                        field: "TIM16RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "BKIN",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1N",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH1N",
                af: Some(
                    2,
                ),
            },
        ],
        dma_channels: &[
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
                    46,
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
                    45,
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
                    45,
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
                    46,
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
                    46,
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
                    45,
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
                    46,
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
                    45,
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
                    46,
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
                    45,
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
                    45,
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
                    46,
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
                    45,
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
                    46,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM16",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM16",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM16",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM16",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM16",
            },
        ],
    },
    Peripheral {
        name: "TIM17",
        address: 0x40014800,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
                block: "TIM_1CH_CMP",
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
                        register: "APBENR2",
                        field: "TIM17EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR2",
                        field: "TIM17RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA7",
                signal: "CH1",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "BKIN",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "BKIN",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH1N",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH1N",
                af: Some(
                    2,
                ),
            },
        ],
        dma_channels: &[
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
                    47,
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
                    48,
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
                    48,
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
                    47,
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
                    47,
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
                    48,
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
                    47,
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
                    48,
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
                    48,
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
                    47,
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
                    48,
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
                    47,
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
                    47,
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
                    48,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM17",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM17",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM17",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM17",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM17",
            },
        ],
    },
    Peripheral {
        name: "USART1",
        address: 0x40013800,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v1",
                block: "USART",
                ir: &usart::REGISTERS,
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
                        register: "APBENR2",
                        field: "USART1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR2",
                        field: "USART1RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "CK",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "TX",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "TX",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "RX",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CTS",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RTS",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "RX",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "TX",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RTS",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CTS",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CK",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CK",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "RTS",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "TX",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RX",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "TX",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "RX",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "TX",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "RX",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "CTS",
                af: Some(
                    4,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    8,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    8,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    8,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    8,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    8,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    8,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    7,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    8,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USART1",
            },
        ],
    },
    Peripheral {
        name: "USART2",
        address: 0x40004400,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v1",
                block: "USART",
                ir: &usart::REGISTERS,
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
                        field: "USART2EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "USART2RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CTS",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RTS",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CK",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "TX",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "TX",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RX",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "RTS",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CTS",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CTS",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "RTS",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PF0",
                signal: "TX",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PF1",
                signal: "RX",
                af: Some(
                    9,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    10,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    9,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    10,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    9,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    10,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    9,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    9,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    10,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    9,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    10,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    9,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    10,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    10,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    9,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USART2",
            },
        ],
    },
    Peripheral {
        name: "USART3",
        address: 0x40004800,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v1",
                block: "USART",
                ir: &usart::REGISTERS,
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
                        field: "USART3EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "USART3RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA5",
                signal: "TX",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CTS",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RTS",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CK",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "RX",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RTS",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "TX",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CTS",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "TX",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "RX",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "TX",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "RX",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CK",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CTS",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RTS",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "RTS",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CTS",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "TX",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "RX",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "RX",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "TX",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "TX",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "RX",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RX",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CK",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "RTS",
                af: Some(
                    1,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    12,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    11,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    12,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    11,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    12,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    11,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    12,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    11,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    11,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    12,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    11,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    12,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    12,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    11,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USART3_4",
            },
        ],
    },
    Peripheral {
        name: "USART4",
        address: 0x40004c00,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v1",
                block: "USART",
                ir: &usart::REGISTERS,
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
                        field: "USART4EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "USART4RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "TX",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RX",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RTS",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CTS",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "RX",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "TX",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CTS",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "RTS",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "TX",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RX",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CK",
                af: Some(
                    0,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    13,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    14,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    14,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    13,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    13,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    14,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    13,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    14,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    14,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    13,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    13,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    14,
                ),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    14,
                ),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    13,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USART3_4",
            },
        ],
    },
    Peripheral {
        name: "USB",
        address: 0x40005c00,
        registers: Some(
            PeripheralRegisters {
                kind: "usb",
                version: "v1",
                block: "USB",
                ir: &usb::REGISTERS,
            },
        ),
        rcc: Some(
            PeripheralRcc {
                bus_clock: "PLL",
                kernel_clock: Clock(
                    "PLL",
                ),
                enable: Some(
                    PeripheralRccRegister {
                        register: "APBENR1",
                        field: "USBEN",
                    },
                ),
                reset: None,
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "DM",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "DP",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "USB",
                channel: Some(
                    "DMA1_CH1",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    49,
                ),
            },
            PeripheralDmaChannel {
                signal: "USB",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    49,
                ),
            },
            PeripheralDmaChannel {
                signal: "USB",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    49,
                ),
            },
            PeripheralDmaChannel {
                signal: "USB",
                channel: Some(
                    "DMA1_CH4",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    49,
                ),
            },
            PeripheralDmaChannel {
                signal: "USB",
                channel: Some(
                    "DMA1_CH5",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    49,
                ),
            },
            PeripheralDmaChannel {
                signal: "USB",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    49,
                ),
            },
            PeripheralDmaChannel {
                signal: "USB",
                channel: Some(
                    "DMA1_CH7",
                ),
                dmamux: None,
                dma: Some(
                    "DMA1",
                ),
                request: Some(
                    49,
                ),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "HP",
                interrupt: "USB",
            },
            PeripheralInterrupt {
                signal: "LP",
                interrupt: "USB",
            },
            PeripheralInterrupt {
                signal: "WAKEUP",
                interrupt: "USB",
            },
        ],
    },
    Peripheral {
        name: "DMA1",
        address: 0x40020000,
        registers: Some(
            PeripheralRegisters {
                kind: "dma",
                version: "f072",
                block: "DMA",
                ir: &dma::REGISTERS,
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
                        field: "DMAEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "AHBRSTR",
                        field: "DMARST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "DMA1_CHANNEL1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA1_CHANNEL2_3",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA1_CHANNEL2_3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA1_CHANNEL4_5_6_7",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA1_CHANNEL4_5_6_7",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "DMA1_CHANNEL4_5_6_7",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "DMA1_CHANNEL4_5_6_7",
            },
        ],
    },
    Peripheral {
        name: "UID",
        address: 0x1fff3000,
        registers: Some(
            PeripheralRegisters {
                kind: "uid",
                version: "v1",
                block: "UID",
                ir: &uid::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CONFIGBYTES",
        address: 0x1fff3200,
        registers: Some(
            PeripheralRegisters {
                kind: "configbytes",
                version: "f072",
                block: "CONFIGBYTES",
                ir: &configbytes::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
];
                pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "WWDG",
        number: 0,
    },
    Interrupt {
        name: "PVD",
        number: 1,
    },
    Interrupt {
        name: "RTC",
        number: 2,
    },
    Interrupt {
        name: "FLASH",
        number: 3,
    },
    Interrupt {
        name: "RCC_CTC",
        number: 4,
    },
    Interrupt {
        name: "EXTI0_1",
        number: 5,
    },
    Interrupt {
        name: "EXTI2_3",
        number: 6,
    },
    Interrupt {
        name: "EXTI4_15",
        number: 7,
    },
    Interrupt {
        name: "LCD",
        number: 8,
    },
    Interrupt {
        name: "DMA1_CHANNEL1",
        number: 9,
    },
    Interrupt {
        name: "DMA1_CHANNEL2_3",
        number: 10,
    },
    Interrupt {
        name: "DMA1_CHANNEL4_5_6_7",
        number: 11,
    },
    Interrupt {
        name: "ADC_COMP",
        number: 12,
    },
    Interrupt {
        name: "TIM1_BRK_UP_TRG_COM",
        number: 13,
    },
    Interrupt {
        name: "TIM1_CC",
        number: 14,
    },
    Interrupt {
        name: "TIM2",
        number: 15,
    },
    Interrupt {
        name: "TIM3",
        number: 16,
    },
    Interrupt {
        name: "TIM6_LPTIM1_DAC",
        number: 17,
    },
    Interrupt {
        name: "TIM7",
        number: 18,
    },
    Interrupt {
        name: "TIM14",
        number: 19,
    },
    Interrupt {
        name: "TIM15",
        number: 20,
    },
    Interrupt {
        name: "TIM16",
        number: 21,
    },
    Interrupt {
        name: "TIM17",
        number: 22,
    },
    Interrupt {
        name: "I2C1",
        number: 23,
    },
    Interrupt {
        name: "I2C2",
        number: 24,
    },
    Interrupt {
        name: "SPI1",
        number: 25,
    },
    Interrupt {
        name: "SPI2",
        number: 26,
    },
    Interrupt {
        name: "USART1",
        number: 27,
    },
    Interrupt {
        name: "USART2",
        number: 28,
    },
    Interrupt {
        name: "USART3_4",
        number: 29,
    },
    Interrupt {
        name: "CAN",
        number: 30,
    },
    Interrupt {
        name: "USB",
        number: 31,
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
            #[path="../registers/adc_v2.rs"] pub mod adc;
#[path="../registers/configbytes_f072.rs"] pub mod configbytes;
#[path="../registers/dbgmcu_f072.rs"] pub mod dbgmcu;
#[path="../registers/dma_f072.rs"] pub mod dma;
#[path="../registers/exti_v1.rs"] pub mod exti;
#[path="../registers/flash_f072.rs"] pub mod flash;
#[path="../registers/gpio_v1.rs"] pub mod gpio;
#[path="../registers/i2c_v1.rs"] pub mod i2c;
#[path="../registers/rcc_f072.rs"] pub mod rcc;
#[path="../registers/spi_v2.rs"] pub mod spi;
#[path="../registers/syscfg_f072.rs"] pub mod syscfg;
#[path="../registers/timer_v1.rs"] pub mod timer;
#[path="../registers/uid_v1.rs"] pub mod uid;
#[path="../registers/usart_v1.rs"] pub mod usart;
#[path="../registers/usb_v1.rs"] pub mod usb;
