
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Ctc",
            extends: None,
            description: Some(
                "Clock trimming controller.",
            ),
            items: &[
                BlockItem {
                    name: "ctl0",
                    description: Some(
                        "Control register 0.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctl1",
                    description: Some(
                        "Control register 1.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "Status register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intc",
                    description: Some(
                        "Interrupt clear register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Intc",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ctl0",
            extends: None,
            description: Some(
                "Control register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ckokie",
                    description: Some(
                        "Clock OK interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckwarnie",
                    description: Some(
                        "Clock warning interrupt enable.",
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
                    name: "errie",
                    description: Some(
                        "Error interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "erefie",
                    description: Some(
                        "Reference signal presence interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnten",
                    description: Some(
                        "Counter enable. While set, CTL1 cannot be modified.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "autotrim",
                    description: Some(
                        "Automatic trimming enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "swrefpul",
                    description: Some(
                        "Software reference pulse. Cleared by hardware after the pulse is generated.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trimvalue",
                    description: Some(
                        "HSI48 clock trimming value. Hardware-controlled when AUTOTRIM is set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ctl1",
            extends: None,
            description: Some(
                "Control register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rlvalue",
                    description: Some(
                        "Counter reload value, captured on each reference pulse.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cklim",
                    description: Some(
                        "Clock limit value. The counter compares against multiples of this limit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "refpsc",
                    description: Some(
                        "Reference signal prescaler.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Refpsc",
                    ),
                },
                Field {
                    name: "refsel",
                    description: Some(
                        "Reference signal source selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Refsel",
                    ),
                },
                Field {
                    name: "refpol",
                    description: Some(
                        "Reference signal polarity.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Refpol",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Intc",
            extends: None,
            description: Some(
                "Interrupt clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ckokic",
                    description: Some(
                        "Clock OK flag clear.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckwarnic",
                    description: Some(
                        "Clock warning flag clear.",
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
                    name: "erric",
                    description: Some(
                        "Error flag clear.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "erefic",
                    description: Some(
                        "Reference signal presence flag clear.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some(
                "Status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ckokif",
                    description: Some(
                        "Clock OK flag. The counter was below 3 x CKLIM at the sync pulse.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckwarnif",
                    description: Some(
                        "Clock warning flag. The counter was between 3 x CKLIM and 128 x CKLIM at the sync pulse.",
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
                    name: "errif",
                    description: Some(
                        "Error flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "erefif",
                    description: Some(
                        "Reference signal presence flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckerr",
                    description: Some(
                        "Clock error. The counter reached 128 x CKLIM at a sync pulse, indicating the trimmed clock is too slow.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "refmiss",
                    description: Some(
                        "Reference signal missing. No sync pulse was detected within 128 x CKLIM counter cycles while counting up.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trimerr",
                    description: Some(
                        "Trimming error. TRIMVALUE overflowed or underflowed during automatic trimming.",
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
                    name: "refdir",
                    description: Some(
                        "Reference signal direction indicator.",
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
                    name: "refcap",
                    description: Some(
                        "Counter value captured on the last reference pulse.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Refpol",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RISINGEDGE",
                    description: Some(
                        "Reference edge is rising.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FALLINGEDGE",
                    description: Some(
                        "Reference edge is falling.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Refpsc",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "Reference signal not divided.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "Reference signal divided by 2.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "Reference signal divided by 4.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "Reference signal divided by 8.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "Reference signal divided by 16.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV32",
                    description: Some(
                        "Reference signal divided by 32.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV64",
                    description: Some(
                        "Reference signal divided by 64.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV128",
                    description: Some(
                        "Reference signal divided by 128.",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Refsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "GPIO",
                    description: Some(
                        "GPIO input as reference signal.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE oscillator as reference signal.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "USBDSOF",
                    description: Some(
                        "USB start-of-frame signal as reference signal.",
                    ),
                    value: 2,
                },
            ],
        },
    ],
};
                