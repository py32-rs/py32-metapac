
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v1",
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
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "ADC_COMP",
            },
        ],
    },
    Peripheral {
        name: "RCC",
        address: 0x40021000,
        registers: Some(
            PeripheralRegisters {
                kind: "rcc",
                version: "f030",
                block: "RCC",
                ir: &rcc::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "MCO",
                af: Some(
                    15,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "MCO",
                af: Some(
                    15,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "MCO",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "MCO",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "MCO",
                af: Some(
                    15,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "MCO",
                af: Some(
                    15,
                ),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "MCO",
                af: Some(
                    6,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "RCC",
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
                pin: "PA6",
                signal: "CK",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "TX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CK",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RX",
                af: Some(
                    8,
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
                pin: "PA9",
                signal: "RX",
                af: Some(
                    8,
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
                pin: "PA10",
                signal: "TX",
                af: Some(
                    8,
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
                    8,
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
                pin: "PB2",
                signal: "RX",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RTS",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CTS",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CK",
                af: Some(
                    3,
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
                    8,
                ),
            },
            PeripheralPin {
                pin: "PF0",
                signal: "RX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PF1",
                signal: "TX",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PF3",
                signal: "TX",
                af: Some(
                    0,
                ),
            },
        ],
        dma_channels: &[],
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
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "TX",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RTS",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RX",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CK",
                af: Some(
                    5,
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
                pin: "PA5",
                signal: "RX",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "TX",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CK",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RX",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "TX",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "RX",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CTS",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RTS",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "TX",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RX",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "RX",
                af: Some(
                    3,
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
                pin: "PB5",
                signal: "CK",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RX",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "TX",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PF0",
                signal: "RX",
                af: Some(
                    4,
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
                signal: "TX",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PF1",
                signal: "RX",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "RX",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PF3",
                signal: "TX",
                af: Some(
                    4,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USART2",
            },
        ],
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
                pin: "PA0",
                signal: "CH3",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "CH1N",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH4",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2N",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH1",
                af: Some(
                    13,
                ),
            },
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
                pin: "PA9",
                signal: "CH2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "BKIN",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CH3",
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
                pin: "PA12",
                signal: "ETR",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CH2",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH2",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH3",
                af: Some(
                    1,
                ),
            },
        ],
        dma_channels: &[],
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
                pin: "PA2",
                signal: "CH1",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH3",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH2",
                af: Some(
                    13,
                ),
            },
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
                pin: "PF3",
                signal: "CH3",
                af: Some(
                    13,
                ),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some(
                    "DMA1_CH2",
                ),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some(
                    "DMA1_CH3",
                ),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some(
                    "DMA1_CH6",
                ),
                dmamux: None,
                dma: None,
                request: None,
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
        name: "TIM14",
        address: 0x40002000,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1",
                block: "TIM_1CH",
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
                pin: "PF0",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PF1",
                signal: "CH1",
                af: Some(
                    13,
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
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
        ],
        dma_channels: &[],
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
                    5,
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
                pin: "PB8",
                signal: "CH1",
                af: Some(
                    13,
                ),
            },
        ],
        dma_channels: &[],
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
        name: "SYSCFG",
        address: 0x40010000,
        registers: Some(
            PeripheralRegisters {
                kind: "syscfg",
                version: "f030",
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
        name: "DMA1",
        address: 0x40020000,
        registers: Some(
            PeripheralRegisters {
                kind: "dma",
                version: "f030",
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
                interrupt: "DMA_CHANNEL1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA_CHANNEL2_3",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA_CHANNEL2_3",
            },
        ],
    },
    Peripheral {
        name: "FLASH",
        address: 0x40022000,
        registers: Some(
            PeripheralRegisters {
                kind: "flash",
                version: "f030",
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
        name: "SPI1",
        address: 0x40013000,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "v1",
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
                pin: "PA0",
                signal: "MISO",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "SCK",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "MOSI",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "MOSI",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "SCK",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "MOSI",
                af: Some(
                    10,
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
                signal: "MISO",
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
                pin: "PA7",
                signal: "MISO",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "MOSI",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SCK",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "NSS",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "MISO",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "MOSI",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "MISO",
                af: Some(
                    10,
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
                    0,
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
                pin: "PF1",
                signal: "NSS",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PF3",
                signal: "NSS",
                af: Some(
                    10,
                ),
            },
        ],
        dma_channels: &[],
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
                version: "v1",
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
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "MISO",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "MOSI",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "NSS",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "MISO",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "MOSI",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "SCK",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "MISO",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "MOSI",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCK",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "NSS",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PF0",
                signal: "SCK",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PF1",
                signal: "MISO",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "MOSI",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PF3",
                signal: "MISO",
                af: Some(
                    3,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "SPI2",
            },
        ],
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
                        field: "I2CEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "I2CRST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "SDA",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "SCL",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "SDA",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "SCL",
                af: Some(
                    12,
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
                pin: "PA9",
                signal: "SDA",
                af: Some(
                    12,
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
                pin: "PA10",
                signal: "SCL",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "SCL",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SDA",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDA",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCL",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SDA",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PF0",
                signal: "SDA",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PF1",
                signal: "SCL",
                af: Some(
                    12,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "I2C1",
            },
        ],
    },
    Peripheral {
        name: "DBGMCU",
        address: 0x40015800,
        registers: Some(
            PeripheralRegisters {
                kind: "dbgmcu",
                version: "f030",
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
                        register: "APBENR1",
                        field: "DBGEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
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
        name: "UID",
        address: 0x1fff0e00,
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
        address: 0x1fff0f00,
        registers: Some(
            PeripheralRegisters {
                kind: "configbytes",
                version: "f030",
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
        name: "RCC",
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
        name: "DMA_CHANNEL1",
        number: 9,
    },
    Interrupt {
        name: "DMA_CHANNEL2_3",
        number: 10,
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
        name: "TIM3",
        number: 16,
    },
    Interrupt {
        name: "TIM14",
        number: 19,
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
        name: "LED",
        number: 30,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
            #[path="../registers/adc_v1.rs"] pub mod adc;
#[path="../registers/configbytes_f030.rs"] pub mod configbytes;
#[path="../registers/dbgmcu_f030.rs"] pub mod dbgmcu;
#[path="../registers/dma_f030.rs"] pub mod dma;
#[path="../registers/exti_v1.rs"] pub mod exti;
#[path="../registers/flash_f030.rs"] pub mod flash;
#[path="../registers/gpio_v1.rs"] pub mod gpio;
#[path="../registers/i2c_v1.rs"] pub mod i2c;
#[path="../registers/rcc_f030.rs"] pub mod rcc;
#[path="../registers/spi_v1.rs"] pub mod spi;
#[path="../registers/syscfg_f030.rs"] pub mod syscfg;
#[path="../registers/timer_v1.rs"] pub mod timer;
#[path="../registers/uid_v1.rs"] pub mod uid;
#[path="../registers/usart_v1.rs"] pub mod usart;
