
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Dbgmcu",
            extends: None,
            description: Some(
                "Debug support.",
            ),
            items: &[
                BlockItem {
                    name: "idcode",
                    description: Some(
                        "MCU Device ID Code Register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Idcode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr",
                    description: Some(
                        "Debug MCU Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb_fz1",
                    description: Some(
                        "APB Freeze Register1.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ApbFz1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb_fz2",
                    description: Some(
                        "APB Freeze Register2.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ApbFz2",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "ApbFz1",
            extends: None,
            description: Some(
                "APB Freeze Register1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbg_timer3_stop",
                    description: Some(
                        "Debug Timer 3 stopped when Core is halted.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dbg_rtc_stop",
                    description: Some(
                        "Debug RTC stopped when Core is halted.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dbg_wwdg_stop",
                    description: Some(
                        "Debug Window Wachdog stopped when Core is halted.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dbg_iwdg_stop",
                    description: Some(
                        "Debug Independent Wachdog stopped when Core is halted.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dbg_lptim_stop",
                    description: Some(
                        "Debug LPTIM stopped when Core is halted.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "ApbFz2",
            extends: None,
            description: Some(
                "APB Freeze Register2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbg_timer1_stop",
                    description: Some(
                        "Debug Timer 1 stopped when Core is halted.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dbg_timer14_stop",
                    description: Some(
                        "Debug Timer 14 stopped when Core is halted.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dbg_timer16_stop",
                    description: Some(
                        "Debug Timer 16 stopped when Core is halted.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dbg_timer17_stop",
                    description: Some(
                        "Debug Timer 17 stopped when Core is halted.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "Debug MCU Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbg_stop",
                    description: Some(
                        "Debug Stop Mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Idcode",
            extends: None,
            description: Some(
                "MCU Device ID Code Register.",
            ),
            bit_size: 32,
            fields: &[],
        },
    ],
    enums: &[],
};
                