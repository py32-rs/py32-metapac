
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Configbytes",
            extends: None,
            description: Some(
                "Flash option bytes (MS32C001B, mapped at 0x1FFF0040)",
            ),
            items: &[
                BlockItem {
                    name: "optr",
                    description: Some(
                        "User option bytes and complements (loaded into FLASH_OPTR at reset)",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Optr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sdkr",
                    description: Some(
                        "SDK area address option bytes and complements (loaded into FLASH_SDKR at reset)",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Sdkr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wrpr",
                    description: Some(
                        "WRP address option bytes and complements (loaded into FLASH_WRPR at reset)",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Wrpr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Optr",
            extends: None,
            description: Some(
                "User option bytes and complements",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdp",
                    description: Some(
                        "Read protection level (0xAA: level 0, no protection; otherwise: level 1, protection active)",
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
                    name: "bor_en",
                    description: Some(
                        "BOR enable (0: disabled; 1: enabled, BOR_LEV active)",
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
                    name: "bor_lev",
                    description: Some(
                        "BOR threshold level (000: 2.2V/2.1V ... 111: 3.6V/3.5V)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iwdg_sw",
                    description: Some(
                        "IWDG mode (0: hardware watchdog; 1: software watchdog)",
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
                    name: "nrst_mode",
                    description: Some(
                        "NRST pin mode (0: reset input only; 1: GPIO function)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iwdg_stop",
                    description: Some(
                        "IWDG behavior in Stop mode (0: freeze; 1: keep running)",
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
                    name: "nrdp",
                    description: Some(
                        "Complemented read protection level",
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
                    name: "nbor_en",
                    description: Some(
                        "Complemented BOR enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nbor_lev",
                    description: Some(
                        "Complemented BOR threshold level",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "niwdg_sw",
                    description: Some(
                        "Complemented IWDG mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nnrst_mode",
                    description: Some(
                        "Complemented NRST pin mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "niwdg_stop",
                    description: Some(
                        "Complemented IWDG Stop mode behavior",
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
            name: "Sdkr",
            extends: None,
            description: Some(
                "SDK area address option bytes and complements",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sdk_strt",
                    description: Some(
                        "SDK area start address (step = 1 KB)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdk_end",
                    description: Some(
                        "SDK area end address (step = 1 KB)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nsdk_strt",
                    description: Some(
                        "Complemented SDK area start address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nsdk_end",
                    description: Some(
                        "Complemented SDK area end address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wrpr",
            extends: None,
            description: Some(
                "WRP address option bytes and complements",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrp",
                    description: Some(
                        "Write protection bits (0: sector[y] protected; 1: unprotected; y = 0..15)",
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
                    name: "nwrp",
                    description: Some(
                        "Complemented write protection bits",
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
    enums: &[],
};
                