
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Spi",
            extends: None,
            description: Some(
                "Serial peripheral interface.",
            ),
            items: &[
                BlockItem {
                    name: "cr1",
                    description: Some(
                        "control register 1.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr2",
                    description: Some(
                        "control register 2.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "status register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dr",
                    description: Some(
                        "data register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cr1",
            extends: None,
            description: Some(
                "control register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cpha",
                    description: Some(
                        "Clock phase.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cpha",
                    ),
                },
                Field {
                    name: "cpol",
                    description: Some(
                        "Clock polarity.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cpol",
                    ),
                },
                Field {
                    name: "mstr",
                    description: Some(
                        "Master selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Mstr",
                    ),
                },
                Field {
                    name: "br",
                    description: Some(
                        "Baud rate control.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Br",
                    ),
                },
                Field {
                    name: "spe",
                    description: Some(
                        "SPI enable.",
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
                    name: "lsbfirst",
                    description: Some(
                        "Frame format.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lsbfirst",
                    ),
                },
                Field {
                    name: "ssi",
                    description: Some(
                        "Internal slave select.",
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
                    name: "ssm",
                    description: Some(
                        "Software slave management.",
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
                    name: "rxonly",
                    description: Some(
                        "Receive only.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Rxonly",
                    ),
                },
                Field {
                    name: "bidioe",
                    description: Some(
                        "Select the direction of transfer in bidirectional mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Bidioe",
                    ),
                },
                Field {
                    name: "bidimode",
                    description: Some(
                        "Bidirectional data mode enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Bidimode",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cr2",
            extends: None,
            description: Some(
                "control register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxdmaen",
                    description: Some(
                        "Rx buffer DMA enable.",
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
                    name: "txdmaen",
                    description: Some(
                        "Tx buffer DMA enable.",
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
                    name: "ssoe",
                    description: Some(
                        "SS output enable.",
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
                    name: "errie",
                    description: Some(
                        "Error interrupt enable.",
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
                    name: "rxneie",
                    description: Some(
                        "RX buffer not empty interrupt enable.",
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
                    name: "txeie",
                    description: Some(
                        "Tx buffer empty interrupt enable.",
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
                    name: "ds",
                    description: Some(
                        "Data length.",
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
                    name: "frxth",
                    description: Some(
                        "FIFO reception threshold.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Frxth",
                    ),
                },
                Field {
                    name: "ldma_rx",
                    description: Some(
                        "Last DMA transfer for reception.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "LdmaRx",
                    ),
                },
                Field {
                    name: "ldma_tx",
                    description: Some(
                        "Last DMA transfer for transmission.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "LdmaTx",
                    ),
                },
                Field {
                    name: "slvfm",
                    description: Some(
                        "Slave fast mode enable.",
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
            ],
        },
        FieldSet {
            name: "Dr",
            extends: None,
            description: Some(
                "data register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dr",
                    description: Some(
                        "Data register.",
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
            name: "Sr",
            extends: None,
            description: Some(
                "status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxne",
                    description: Some(
                        "Receive buffer not empty.",
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
                    name: "txe",
                    description: Some(
                        "Transmit buffer empty.",
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
                    name: "modf",
                    description: Some(
                        "Mode fault.",
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
                    name: "ovr",
                    description: Some(
                        "Overrun flag.",
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
                    name: "bsy",
                    description: Some(
                        "Busy flag.",
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
                    name: "frlvl",
                    description: Some(
                        "FIFO reception level.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Frlvl",
                    ),
                },
                Field {
                    name: "ftlvl",
                    description: Some(
                        "FIFO Transmission Level.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Ftlvl",
                    ),
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Bidimode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "UNIDIRECTIONAL",
                    description: Some(
                        "2-line unidirectional data mode selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BIDIRECTIONAL",
                    description: Some(
                        "1-line bidirectional data mode selected",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Bidioe",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RECEIVE",
                    description: Some(
                        "Output disabled (receive-only mode)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TRANSMIT",
                    description: Some(
                        "Output enabled (transmit-only mode)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Br",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "f_PCLK / 2",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "f_PCLK / 4",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "f_PCLK / 8",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "f_PCLK / 16",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DIV32",
                    description: Some(
                        "f_PCLK / 32",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV64",
                    description: Some(
                        "f_PCLK / 64",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV128",
                    description: Some(
                        "f_PCLK / 128",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV256",
                    description: Some(
                        "f_PCLK / 256",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Cpha",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "FIRSTEDGE",
                    description: Some(
                        "The first clock transition is the first data capture edge",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SECONDEDGE",
                    description: Some(
                        "The second clock transition is the first data capture edge",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cpol",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "IDLELOW",
                    description: Some(
                        "CK to 0 when idle",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "IDLEHIGH",
                    description: Some(
                        "CK to 1 when idle",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Frlvl",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "EMPTY",
                    description: Some(
                        "Rx FIFO Empty",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "QUARTER",
                    description: Some(
                        "Rx 1/4 FIFO",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HALF",
                    description: Some(
                        "Rx 1/2 FIFO",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "FULL",
                    description: Some(
                        "Rx FIFO full",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Frxth",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "HALF",
                    description: Some(
                        "RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "QUARTER",
                    description: Some(
                        "RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ftlvl",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "EMPTY",
                    description: Some(
                        "Tx FIFO Empty",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "QUARTER",
                    description: Some(
                        "Tx 1/4 FIFO",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HALF",
                    description: Some(
                        "Tx 1/2 FIFO",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "FULL",
                    description: Some(
                        "Tx FIFO full",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "LdmaRx",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "EVEN",
                    description: Some(
                        "Number of data to transfer for receive is even",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ODD",
                    description: Some(
                        "Number of data to transfer for receive is odd",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "LdmaTx",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "EVEN",
                    description: Some(
                        "Number of data to transfer for transmit is even",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ODD",
                    description: Some(
                        "Number of data to transfer for transmit is odd",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lsbfirst",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "MSBFIRST",
                    description: Some(
                        "Data is transmitted/received with the MSB first",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSBFIRST",
                    description: Some(
                        "Data is transmitted/received with the LSB first",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Mstr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SLAVE",
                    description: Some(
                        "Slave configuration",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MASTER",
                    description: Some(
                        "Master configuration",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rxonly",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "FULLDUPLEX",
                    description: Some(
                        "Full duplex (Transmit and receive)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "OUTPUTDISABLED",
                    description: Some(
                        "Output disabled (Receive-only mode)",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
                