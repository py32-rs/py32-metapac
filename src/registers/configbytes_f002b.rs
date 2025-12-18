
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Configbytes",
            extends: None,
            description: Some(
                "Factory configuration bytes.",
            ),
            items: &[
                BlockItem {
                    name: "hsi_trimming",
                    description: Some(
                        "HSI 24 Hz frequency selection control and corresponding trimming value",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "HsiTrimming",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eppara",
                    description: Some(
                        "FLASH configuration values. 24MHz.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Eppara",
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Eppara",
            extends: None,
            description: None,
            items: &[
                BlockItem {
                    name: "eppara0",
                    description: Some(
                        "FLASH_TS0, FLASH_TS1, FLASH_TS3 configuration values.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eppara0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eppara1",
                    description: Some(
                        "FLASH_TS2P, FLASH_TPS3 configuration values.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eppara1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eppara2",
                    description: Some(
                        "FLASH_PERTPE configuration values.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eppara2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eppara3",
                    description: Some(
                        "FLASH_SMERTPE configuration values.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eppara3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eppara4",
                    description: Some(
                        "FLASH_PRGTPE, FLASH_PRETPE configuration values.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eppara4",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Eppara0",
            extends: None,
            description: Some(
                "FLASH_TSO, FLASH_TS1 configuration values.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ts0",
                    description: Some(
                        "TS0 value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ts3",
                    description: Some(
                        "TS3 value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ts1",
                    description: Some(
                        "TS1 value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Eppara1",
            extends: None,
            description: Some(
                "FLASH_TS2P, FLASH_TPS3 configuration values.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ts2p",
                    description: Some(
                        "TS2P value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tps3",
                    description: Some(
                        "TPS3 value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Eppara2",
            extends: None,
            description: Some(
                "FLASH_PERTPE configuration values.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pertpe",
                    description: Some(
                        "PERTPE value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 18,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Eppara3",
            extends: None,
            description: Some(
                "FLASH_SMERTPE configuration values.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smertpe",
                    description: Some(
                        "SMERTPE value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 18,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Eppara4",
            extends: None,
            description: Some(
                "FLASH_PRGTPE, FLASH_PRETPE configuration values.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "prgtpe",
                    description: Some(
                        "PRGTPE value.",
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
                    name: "pretpe",
                    description: Some(
                        "PRETPE value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HsiTrimming",
            extends: None,
            description: Some(
                "HSI trimming values.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsi_trim",
                    description: Some(
                        "HSI trimming value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsi_fs",
                    description: Some(
                        "HSI frequency selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                