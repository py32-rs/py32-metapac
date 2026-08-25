
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Div",
            extends: None,
            description: Some(
                "Hardware divider",
            ),
            items: &[
                BlockItem {
                    name: "dend",
                    description: Some(
                        "Dividend register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sor",
                    description: Some(
                        "Divisor register. Writing to it starts the division.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "quot",
                    description: Some(
                        "Quotient register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "remd",
                    description: Some(
                        "Remainder register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "sign",
                    description: Some(
                        "Sign register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sign",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "stat",
                    description: Some(
                        "Status register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Stat",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Sign",
            extends: None,
            description: Some(
                "Sign register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sign",
                    description: Some(
                        "Division sign. 0: unsigned division, 1: signed division.",
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
            ],
        },
        FieldSet {
            name: "Stat",
            extends: None,
            description: Some(
                "Status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "end",
                    description: Some(
                        "Division end flag. Set when the division is finished. Reading QUOT/REMD before this flag is set returns the result of the last completed division.",
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
                    name: "zero",
                    description: Some(
                        "Division by zero flag. Set when the divisor is 0. The operation ends immediately with QUOT and REMD both 0.",
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
    ],
    enums: &[],
};
                