
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Dma",
            extends: None,
            description: Some(
                "Direct memory access.",
            ),
            items: &[
                BlockItem {
                    name: "isr",
                    description: Some(
                        "DMA interrupt status register (DMA_ISR).",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ixr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ifcr",
                    description: Some(
                        "DMA interrupt flag clear register (DMA_IFCR).",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Ixr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "st",
                    description: Some(
                        "Stream cluster: S?CR, S?NDTR, S?M0AR, S?PAR and S?MAR registers",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 20,
                            },
                        ),
                    ),
                    byte_offset: 0x8,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "St",
                        },
                    ),
                },
            ],
        },
        Block {
            name: "St",
            extends: None,
            description: Some(
                "Stream cluster: S?CR, S?NDTR, S?M0AR, S?PAR and S?MAR registers",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "DMA channel x configuration register (DMA_CCR).",
                    ),
                    array: None,
                    byte_offset: 0x0,
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
                    name: "ndtr",
                    description: Some(
                        "DMA channel x number of data register.",
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
                    name: "par",
                    description: Some(
                        "DMA channel x peripheral address register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "mar",
                    description: Some(
                        "DMA channel x memory address register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "DMA channel configuration register (DMA_CCR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Channel enable.",
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
                    name: "tcie",
                    description: Some(
                        "Transfer complete interrupt enable.",
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
                    name: "htie",
                    description: Some(
                        "Half Transfer interrupt enable.",
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
                    name: "teie",
                    description: Some(
                        "Transfer error interrupt enable.",
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
                    name: "dir",
                    description: Some(
                        "Data transfer direction.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dir",
                    ),
                },
                Field {
                    name: "circ",
                    description: Some(
                        "Circular mode.",
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
                    name: "pinc",
                    description: Some(
                        "Peripheral increment mode.",
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
                    name: "minc",
                    description: Some(
                        "Memory increment mode.",
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
                    name: "psize",
                    description: Some(
                        "Peripheral size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Size",
                    ),
                },
                Field {
                    name: "msize",
                    description: Some(
                        "Memory size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Size",
                    ),
                },
                Field {
                    name: "pl",
                    description: Some(
                        "Channel Priority level.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pl",
                    ),
                },
                Field {
                    name: "mem2mem",
                    description: Some(
                        "Memory to memory mode.",
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
            ],
        },
        FieldSet {
            name: "Ixr",
            extends: None,
            description: Some(
                "DMA interrupt register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gif",
                    description: Some(
                        "Channel 1 Global interrupt flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    0,
                                    4,
                                    8,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "htif",
                    description: Some(
                        "Channel x Half Transfer Complete. flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    2,
                                    6,
                                    10,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "tcif",
                    description: Some(
                        "Channel x Transfer Complete. flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    1,
                                    5,
                                    9,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "teif",
                    description: Some(
                        "Channel x Transfer Error. flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Cursed(
                            CursedArray {
                                offsets: &[
                                    3,
                                    7,
                                    11,
                                ],
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Dir",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PERIPHERALTOMEMORY",
                    description: Some(
                        "Peripheral-to-memory",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MEMORYTOPERIPHERAL",
                    description: Some(
                        "Memory-to-peripheral",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pl",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "LOW",
                    description: Some(
                        "Low",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MEDIUM",
                    description: Some(
                        "Medium",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HIGH",
                    description: Some(
                        "High",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "VERYHIGH",
                    description: Some(
                        "Very high",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Size",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BITS8",
                    description: Some(
                        "Byte (8-bit)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BITS16",
                    description: Some(
                        "Half-word (16-bit)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BITS32",
                    description: Some(
                        "Word (32-bit)",
                    ),
                    value: 2,
                },
            ],
        },
    ],
};
                