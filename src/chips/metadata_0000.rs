
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "UID",
        address: 0x1fff0000,
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
        address: 0x1fff0040,
        registers: Some(
            PeripheralRegisters {
                kind: "configbytes",
                version: "ms32c001b",
                block: "CONFIGBYTES",
                ir: &configbytes::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TIM14",
        address: 0x40002000,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1b",
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
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH1",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH1",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH1",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "CH1",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH1",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH1",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH1",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH1",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH1",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH1",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH1",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CH1",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH1",
                af: Some(
                    11,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM14",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM14",
            },
        ],
    },
    Peripheral {
        name: "PWM1",
        address: 0x40002800,
        registers: Some(
            PeripheralRegisters {
                kind: "pwm",
                version: "v1",
                block: "PWM",
                ir: &pwm::REGISTERS,
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
                        field: "PWMEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR2",
                        field: "PWMRST",
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
                    12,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CH1",
                af: Some(
                    12,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH1",
                af: Some(
                    12,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "PWM1",
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
        interrupts: &[],
    },
    Peripheral {
        name: "UART1",
        address: 0x40004800,
        registers: Some(
            PeripheralRegisters {
                kind: "usart",
                version: "v2",
                block: "UART",
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
                        field: "UART1EN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR1",
                        field: "UART1RST",
                    },
                ),
                stop_mode: StopMode::Stop1,
            },
        ),
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "RX",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "TX",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "RX",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "TX",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "TX",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "TX",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "RX",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "RX",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "TX",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "TX",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "TX",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "TX",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "TX",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "TX",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "TX",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "TX",
                af: Some(
                    13,
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
                pin: "PB7",
                signal: "TX",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "TX",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "TX",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "RX",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RX",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "RX",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "RX",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "RX",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RX",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "RX",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RX",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "RX",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "RX",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RX",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "RX",
                af: Some(
                    14,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "RX",
                af: Some(
                    14,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "UART1",
            },
        ],
    },
    Peripheral {
        name: "SYSCFG",
        address: 0x40010000,
        registers: Some(
            PeripheralRegisters {
                kind: "syscfg",
                version: "f002b",
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
        name: "VREF",
        address: 0x40010100,
        registers: Some(
            PeripheralRegisters {
                kind: "vref",
                version: "v1",
                block: "VREF",
                ir: &vref::REGISTERS,
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
                        field: "VREFBUFEN",
                    },
                ),
                reset: Some(
                    PeripheralRccRegister {
                        register: "APBRSTR2",
                        field: "VREFBUFRST",
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
        name: "ADC1",
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v3",
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
                pin: "PB1",
                signal: "IN0",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PA0",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IN7",
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "ADC1",
            },
        ],
    },
    Peripheral {
        name: "TIM1",
        address: 0x40012c00,
        registers: Some(
            PeripheralRegisters {
                kind: "timer",
                version: "v1b",
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
                pin: "PB1",
                signal: "CH2",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CH3N",
                af: Some(
                    1,
                ),
            },
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
                signal: "CH4",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH2",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH3",
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
                pin: "PA6",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH4",
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
                signal: "CH2N",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH1N",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "ETR",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "BKIN",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH3",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CH1N",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH1",
                af: Some(
                    2,
                ),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "CH1N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH1N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH1N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH1N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH1N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH1N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CH1N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH3N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH4",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH3",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH1N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH1N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH1N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CH1",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH1N",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "CH2",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH1",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH2",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH1",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH2",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH2",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CH2",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH1",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH2",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH2",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH2",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH2",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH2",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH2",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CH2",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH2",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "CH2N",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2N",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH2N",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CH2N",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH2N",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH1",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH2N",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH2N",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH2N",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH2N",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CH2N",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH2N",
                af: Some(
                    5,
                ),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "CH3",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH3",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH3",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH1",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH3",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CH3",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH3",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH3",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH1",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH3",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH3",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH1",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH3",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH3",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CH3",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH3",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "CH3N",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH3N",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3N",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH3N",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH3N",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH3N",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH3N",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH1N",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH3N",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH3N",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH3N",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH3N",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH3N",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH3N",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH3N",
                af: Some(
                    7,
                ),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "CH4",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH4",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH1",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH4",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH4",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH4",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CH4",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH2",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH4",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH1N",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH4",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH4",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH4",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH4",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH4",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH4",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CH4",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH4",
                af: Some(
                    8,
                ),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "BKIN",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "BKIN",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "BKIN",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "BKIN",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "BKIN",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "BKIN",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "BKIN",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "BKIN",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "BKIN",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "BKIN",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "BKIN",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "BKIN",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "BKIN",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "BKIN",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "BKIN",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "BKIN",
                af: Some(
                    9,
                ),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "ETR",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "ETR",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "ETR",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "ETR",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "ETR",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "ETR",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "ETR",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "ETR",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "ETR",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "ETR",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "ETR",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "ETR",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "ETR",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "ETR",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "ETR",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "ETR",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "ETR",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "ETR",
                af: Some(
                    10,
                ),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH2N",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH2N",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH2N",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH2N",
                af: Some(
                    11,
                ),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CH3N",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1",
                af: Some(
                    13,
                ),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                af: Some(
                    14,
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
        name: "DBGMCU",
        address: 0x40015800,
        registers: Some(
            PeripheralRegisters {
                kind: "dbgmcu",
                version: "f002b",
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
                pin: "PA2",
                signal: "SWCLK",
                af: Some(
                    0,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SWDIO",
                af: Some(
                    0,
                ),
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RCC",
        address: 0x40021000,
        registers: Some(
            PeripheralRegisters {
                kind: "rcc",
                version: "ms32c001b",
                block: "RCC",
                ir: &rcc::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA7",
                signal: "MCO",
                af: Some(
                    4,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "MCO",
                af: Some(
                    4,
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
        name: "EXTI",
        address: 0x40021800,
        registers: Some(
            PeripheralRegisters {
                kind: "exti",
                version: "v1b",
                block: "EXTI",
                ir: &exti::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
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
                interrupt: "EXTI4_7",
            },
            PeripheralInterrupt {
                signal: "EXTI5",
                interrupt: "EXTI4_7",
            },
            PeripheralInterrupt {
                signal: "EXTI6",
                interrupt: "EXTI4_7",
            },
            PeripheralInterrupt {
                signal: "EXTI7",
                interrupt: "EXTI4_7",
            },
            PeripheralInterrupt {
                signal: "EXTI16",
                interrupt: "PVD",
            },
            PeripheralInterrupt {
                signal: "EXTI29",
                interrupt: "LPTIM1",
            },
        ],
    },
    Peripheral {
        name: "FLASH",
        address: 0x40022000,
        registers: Some(
            PeripheralRegisters {
                kind: "flash",
                version: "ms32c001b",
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
                version: "v1b",
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
                version: "v1b",
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
                version: "v1b",
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
];
                pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "PVD",
        number: 0,
    },
    Interrupt {
        name: "FLASH",
        number: 1,
    },
    Interrupt {
        name: "RCC",
        number: 2,
    },
    Interrupt {
        name: "EXTI0_1",
        number: 3,
    },
    Interrupt {
        name: "EXTI2_3",
        number: 4,
    },
    Interrupt {
        name: "EXTI4_7",
        number: 5,
    },
    Interrupt {
        name: "ADC1",
        number: 8,
    },
    Interrupt {
        name: "TIM1_BRK_UP_TRG_COM",
        number: 9,
    },
    Interrupt {
        name: "TIM1_CC",
        number: 10,
    },
    Interrupt {
        name: "LPTIM1",
        number: 11,
    },
    Interrupt {
        name: "TIM14",
        number: 12,
    },
    Interrupt {
        name: "PWM1",
        number: 16,
    },
    Interrupt {
        name: "UART1",
        number: 17,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
            #[path="../registers/adc_v3.rs"] pub mod adc;
#[path="../registers/configbytes_ms32c001b.rs"] pub mod configbytes;
#[path="../registers/dbgmcu_f002b.rs"] pub mod dbgmcu;
#[path="../registers/exti_v1b.rs"] pub mod exti;
#[path="../registers/flash_ms32c001b.rs"] pub mod flash;
#[path="../registers/gpio_v1b.rs"] pub mod gpio;
#[path="../registers/iwdg_v1.rs"] pub mod iwdg;
#[path="../registers/pwm_v1.rs"] pub mod pwm;
#[path="../registers/rcc_ms32c001b.rs"] pub mod rcc;
#[path="../registers/syscfg_f002b.rs"] pub mod syscfg;
#[path="../registers/timer_v1b.rs"] pub mod timer;
#[path="../registers/uid_v1.rs"] pub mod uid;
#[path="../registers/usart_v2.rs"] pub mod usart;
#[path="../registers/vref_v1.rs"] pub mod vref;
