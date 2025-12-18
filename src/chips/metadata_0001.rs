
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "adc",
                version: "v1b",
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
                pin: "PB6",
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
                version: "f002b",
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
                pin: "PA5",
                signal: "CK",
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
                pin: "PA7",
                signal: "RX",
                af: Some(
                    3,
                ),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CK",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RTS",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CTS",
                af: Some(
                    1,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CK",
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
                pin: "PB0",
                signal: "CH3N",
                af: Some(
                    3,
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
                pin: "PB1",
                signal: "CH4",
                af: Some(
                    3,
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
                pin: "PB2",
                signal: "CH3",
                af: Some(
                    3,
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
                pin: "PC0",
                signal: "CH1N",
                af: Some(
                    2,
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
        name: "FLASH",
        address: 0x40022000,
        registers: Some(
            PeripheralRegisters {
                kind: "flash",
                version: "f002b",
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
                signal: "SCL",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCL",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "SDA",
                af: Some(
                    6,
                ),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SDA",
                af: Some(
                    6,
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
        address: 0x1fff0100,
        registers: Some(
            PeripheralRegisters {
                kind: "configbytes",
                version: "f002b",
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
        name: "LPTIM1",
        number: 17,
    },
    Interrupt {
        name: "TIM14",
        number: 19,
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
        name: "USART1",
        number: 27,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
            #[path="../registers/adc_v1b.rs"] pub mod adc;
#[path="../registers/configbytes_f002b.rs"] pub mod configbytes;
#[path="../registers/dbgmcu_f002b.rs"] pub mod dbgmcu;
#[path="../registers/exti_v1.rs"] pub mod exti;
#[path="../registers/flash_f002b.rs"] pub mod flash;
#[path="../registers/gpio_v1.rs"] pub mod gpio;
#[path="../registers/i2c_v1.rs"] pub mod i2c;
#[path="../registers/rcc_f002b.rs"] pub mod rcc;
#[path="../registers/timer_v1.rs"] pub mod timer;
#[path="../registers/uid_v1.rs"] pub mod uid;
#[path="../registers/usart_v1.rs"] pub mod usart;
