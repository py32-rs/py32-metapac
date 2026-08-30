
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Rtc",
            extends: None,
            description: Some(
                "Real-time clock.",
            ),
            items: &[
                BlockItem {
                    name: "crh",
                    description: Some(
                        "Control register high.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Crh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "crl",
                    description: Some(
                        "Control register low.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Crl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "prlh",
                    description: Some(
                        "Prescaler load register high.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Prlh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "prll",
                    description: Some(
                        "Prescaler load register low.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Prll",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "divh",
                    description: Some(
                        "Prescaler divider register high.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Divh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "divl",
                    description: Some(
                        "Prescaler divider register low.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Divl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnth",
                    description: Some(
                        "Counter register high.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cnth",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cntl",
                    description: Some(
                        "Counter register low.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cntl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "alrh",
                    description: Some(
                        "Alarm register high.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Alrh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "alrl",
                    description: Some(
                        "Alarm register low.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Alrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bkp_rtccr",
                    description: Some(
                        "Clock calibration and output configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BkpRtccr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Alrh",
            extends: None,
            description: Some(
                "Alarm register high.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alr",
                    description: Some(
                        "Alarm value, high bits (ALR[31:16]).",
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
            ],
        },
        FieldSet {
            name: "Alrl",
            extends: None,
            description: Some(
                "Alarm register low.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alr",
                    description: Some(
                        "Alarm value, low bits (ALR[15:0]).",
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
            ],
        },
        FieldSet {
            name: "BkpRtccr",
            extends: None,
            description: Some(
                "Clock calibration and output configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cal",
                    description: Some(
                        "Calibration value. Slows the clock by CAL x 1/2^20, up to 121 ppm.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cco",
                    description: Some(
                        "Calibration clock output. When set, outputs the RTC clock divided by 64.",
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
                    name: "asoe",
                    description: Some(
                        "Second or alarm pulse output enable.",
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
                    name: "asos",
                    description: Some(
                        "Second or alarm pulse output selection. 0: second pulse; 1: alarm pulse.",
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
            ],
        },
        FieldSet {
            name: "Cnth",
            extends: None,
            description: Some(
                "Counter register high.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some(
                        "Counter value, high bits (CNT[31:16]).",
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
            ],
        },
        FieldSet {
            name: "Cntl",
            extends: None,
            description: Some(
                "Counter register low.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some(
                        "Counter value, low bits (CNT[15:0]).",
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
            ],
        },
        FieldSet {
            name: "Crh",
            extends: None,
            description: Some(
                "Control register high.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "secie",
                    description: Some(
                        "Second interrupt enable.",
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
                    name: "alrie",
                    description: Some(
                        "Alarm interrupt enable.",
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
                    name: "owie",
                    description: Some(
                        "Overflow interrupt enable.",
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
            ],
        },
        FieldSet {
            name: "Crl",
            extends: None,
            description: Some(
                "Control register low.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "secf",
                    description: Some(
                        "Second flag. Set by hardware once per prescaler period; cleared by writing 0.",
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
                    name: "alrf",
                    description: Some(
                        "Alarm flag. Set by hardware when the counter reaches the alarm value; cleared by writing 0.",
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
                    name: "owf",
                    description: Some(
                        "Overflow flag. Set by hardware when the counter overflows; cleared by writing 0.",
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
                    name: "rsf",
                    description: Some(
                        "Registers synchronized flag. Set by hardware once CNT and DIV have been synchronized; cleared by writing 0.",
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
                    name: "cnf",
                    description: Some(
                        "Configuration flag. Set to enter configuration mode before writing PRL, CNT, ALR or BKP_RTCCR; clear it to commit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtoff",
                    description: Some(
                        "RTC operation off. Reads 1 when the last write has completed and a new write may start. Reset value is 1.",
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
            ],
        },
        FieldSet {
            name: "Divh",
            extends: None,
            description: Some(
                "Prescaler divider register high.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "div",
                    description: Some(
                        "Current prescaler divider value, high bits (DIV[19:16]).",
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
            ],
        },
        FieldSet {
            name: "Divl",
            extends: None,
            description: Some(
                "Prescaler divider register low.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "div",
                    description: Some(
                        "Current prescaler divider value, low bits (DIV[15:0]).",
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
            ],
        },
        FieldSet {
            name: "Prlh",
            extends: None,
            description: Some(
                "Prescaler load register high.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "prl",
                    description: Some(
                        "Prescaler reload value, high bits (PRL[19:16]).",
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
            ],
        },
        FieldSet {
            name: "Prll",
            extends: None,
            description: Some(
                "Prescaler load register low.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "prl",
                    description: Some(
                        "Prescaler reload value, low bits (PRL[15:0]).",
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
            ],
        },
    ],
    enums: &[],
};
                