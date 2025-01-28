
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
                        "HSI trimming value, 4, 8, 16, 22.12, 24MHz.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 8,
                            },
                        ),
                    ),
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
                    name: "ts_cal_30c",
                    description: Some(
                        "Temperature sensor calibration value at 30°C.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Tscal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ts_cal_85c",
                    description: Some(
                        "Temperature sensor calibration value at 85°C.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Tscal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eppara",
                    description: Some(
                        "FLASH configuration values. 4,8,16,22.12,24MHz.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 40,
                            },
                        ),
                    ),
                    byte_offset: 0x38,
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
                    byte_offset: 0x8,
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
                    byte_offset: 0x10,
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
                    byte_offset: 0x18,
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
                    byte_offset: 0x20,
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
                    bit_size: 8,
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
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
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
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
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
                    bit_size: 8,
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
                    bit_size: 11,
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
                    bit_size: 17,
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
                    bit_size: 17,
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
                    bit_size: 12,
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
        FieldSet {
            name: "Tscal",
            extends: None,
            description: Some(
                "Temperature sensor calibration values.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscal",
                    description: Some(
                        "Temperature sensor calibration value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                