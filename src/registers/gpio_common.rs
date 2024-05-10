
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Gpio",
            extends: None,
            description: Some(
                "General-purpose I/Os.",
            ),
            items: &[
                BlockItem {
                    name: "moder",
                    description: Some(
                        "GPIO port mode register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Moder",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "otyper",
                    description: Some(
                        "GPIO port output type register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Otyper",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ospeedr",
                    description: Some(
                        "GPIO port output speed. register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ospeedr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pupdr",
                    description: Some(
                        "GPIO port pull-up/pull-down. register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pupdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "idr",
                    description: Some(
                        "GPIO port input data register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Idr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "odr",
                    description: Some(
                        "GPIO port output data register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Odr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bsrr",
                    description: Some(
                        "GPIO port bit set/reset. register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Bsrr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lckr",
                    description: Some(
                        "GPIO port configuration lock. register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lckr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "afrl",
                    description: Some(
                        "GPIO alternate function low. register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Afrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "afrh",
                    description: Some(
                        "GPIO alternate function high. register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Afrh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "brr",
                    description: Some(
                        "port bit reset register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Brr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Afrh",
            extends: None,
            description: Some(
                "GPIO alternate function high. register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "afsel",
                    description: Some(
                        "Alternate function selection for port x. bit y (y = 8..15).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Afrl",
            extends: None,
            description: Some(
                "GPIO alternate function low. register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "afsel",
                    description: Some(
                        "Alternate function selection for port x. bit y (y = 0..7).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Brr",
            extends: None,
            description: Some(
                "port bit reset register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "br",
                    description: Some(
                        "Port Reset bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bsrr",
            extends: None,
            description: Some(
                "GPIO port bit set/reset. register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bs",
                    description: Some(
                        "Port x set bit y (y=. 0..15).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "br",
                    description: Some(
                        "Port x set bit y (y=. 0..15).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Idr",
            extends: None,
            description: Some(
                "GPIO port input data register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "id",
                    description: Some(
                        "Port input data (y =. 0..15).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Lckr",
            extends: None,
            description: Some(
                "GPIO port configuration lock. register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lck",
                    description: Some(
                        "Port x lock bit y (y=. 0..15).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "lckk",
                    description: Some(
                        "Port x lock bit y (y=. 0..15).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Moder",
            extends: None,
            description: Some(
                "GPIO port mode register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mode",
                    description: Some(
                        "Port x configuration bits (y =. 0..15).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Moder",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Odr",
            extends: None,
            description: Some(
                "GPIO port output data register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "od",
                    description: Some(
                        "Port output data (y =. 0..15).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ospeedr",
            extends: None,
            description: Some(
                "GPIO port output speed. register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ospeed",
                    description: Some(
                        "Port x configuration bits (y =. 0..15).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Ospeedr",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Otyper",
            extends: None,
            description: Some(
                "GPIO port output type register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ot",
                    description: Some(
                        "Port x configuration bits (y =. 0..15).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Ot",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Pupdr",
            extends: None,
            description: Some(
                "GPIO port pull-up/pull-down. register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pupd",
                    description: Some(
                        "Port x configuration bits (y =. 0..15).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Pupdr",
                    ),
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Moder",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "INPUT",
                    description: Some(
                        "Input mode (reset state)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "OUTPUT",
                    description: Some(
                        "General purpose output mode",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ALTERNATE",
                    description: Some(
                        "Alternate function mode",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "ANALOG",
                    description: Some(
                        "Analog mode",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Ospeedr",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "LOWSPEED",
                    description: Some(
                        "Low speed",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MEDIUMSPEED",
                    description: Some(
                        "Medium speed",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HIGHSPEED",
                    description: Some(
                        "High speed",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "VERYHIGHSPEED",
                    description: Some(
                        "Very high speed",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Ot",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PUSHPULL",
                    description: Some(
                        "Output push-pull (reset state)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "OPENDRAIN",
                    description: Some(
                        "Output open-drain",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pupdr",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "FLOATING",
                    description: Some(
                        "No pull-up, pull-down",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PULLUP",
                    description: Some(
                        "Pull-up",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PULLDOWN",
                    description: Some(
                        "Pull-down",
                    ),
                    value: 2,
                },
            ],
        },
    ],
};
                