
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Uid",
            extends: None,
            description: Some(
                "Unique Device ID.",
            ),
            items: &[
                BlockItem {
                    name: "uid",
                    description: Some(
                        "Factory programmed 96-bit unique device identifier words.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "word0",
                    description: Some(
                        "UID Word 0 (Offset 0x00).",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "UidWord0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "word1",
                    description: Some(
                        "UID Word 1 (Offset 0x04).",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "UidWord1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "word2",
                    description: Some(
                        "UID Word 2 (Offset 0x08).",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "UidWord2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "word3",
                    description: Some(
                        "UID Word 3 (Offset 0x0C).",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "UidWord3",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "UidWord0",
            extends: None,
            description: Some(
                "Lot Number (Bytes 0-3).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lot_num_ascii_0",
                    description: Some(
                        "Lot Number ASCII Code (Byte 0).",
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
                    name: "lot_num_ascii_1",
                    description: Some(
                        "Lot Number ASCII Code (Byte 1).",
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
                    name: "lot_num_ascii_2",
                    description: Some(
                        "Lot Number ASCII Code (Byte 2).",
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
                    name: "lot_num_ascii_3",
                    description: Some(
                        "Lot Number ASCII Code (Byte 3).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UidWord1",
            extends: None,
            description: Some(
                "Wafer Number and Lot Number (Bytes 4-7).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wafer_num",
                    description: Some(
                        "Wafer Number.",
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
                    name: "lot_num_ascii_4",
                    description: Some(
                        "Lot Number ASCII Code (Byte 4).",
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
                    name: "lot_num_ascii_5",
                    description: Some(
                        "Lot Number ASCII Code (Byte 5).",
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
                    name: "lot_num_ascii_6",
                    description: Some(
                        "Lot Number ASCII Code (Byte 6).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UidWord2",
            extends: None,
            description: Some(
                "Coordinates and Internal Coding (Bytes 8-11).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "internal_code_0",
                    description: Some(
                        "Internal Coding (Byte 0).",
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
                    name: "y_coord_low",
                    description: Some(
                        "Y Coordinate Low bits.",
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
                    name: "x_coord_low",
                    description: Some(
                        "X Coordinate Low bits.",
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
                    name: "x_coord_high",
                    description: Some(
                        "X Coordinate High bits.",
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
                Field {
                    name: "y_coord_high",
                    description: Some(
                        "Y Coordinate High bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UidWord3",
            extends: None,
            description: Some(
                "Internal Coding and Fixed Code (Bytes 12-15).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fixed_code",
                    description: Some(
                        "Fixed Code (0x78).",
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
                    name: "internal_code_1",
                    description: Some(
                        "Internal Coding (Byte 1).",
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
                    name: "internal_code_2",
                    description: Some(
                        "Internal Coding (Byte 2).",
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
                    name: "internal_code_3",
                    description: Some(
                        "Internal Coding (Byte 3).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                