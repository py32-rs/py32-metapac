
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Usb",
            extends: None,
            description: Some(
                "USB control and status registers for managing USB operations.",
            ),
            items: &[
                BlockItem {
                    name: "addr",
                    description: Some(
                        "Function address of the USB device.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Addr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "power",
                    description: Some(
                        "USB power management register.",
                    ),
                    array: None,
                    byte_offset: 0x1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Power",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_usb",
                    description: Some(
                        "USB interrupt status register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "IntUsb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_out1",
                    description: Some(
                        "Interrupt status for OUT endpoint 1.",
                    ),
                    array: None,
                    byte_offset: 0x5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "IntOut1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_in1",
                    description: Some(
                        "Interrupt status for IN endpoint 1.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "IntIn1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_usbe",
                    description: Some(
                        "USB interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "IntUsb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_out1e",
                    description: Some(
                        "Interrupt enable for OUT endpoint 1.",
                    ),
                    array: None,
                    byte_offset: 0x9,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "IntOut1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_in1e",
                    description: Some(
                        "Interrupt enable for IN endpoint 1.",
                    ),
                    array: None,
                    byte_offset: 0xa,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "IntIn1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "frame",
                    description: Some(
                        "USB frame number and endpoint index.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Frame",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "index",
                    description: Some(
                        "Selected endpoint index.",
                    ),
                    array: None,
                    byte_offset: 0xe,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Index",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ep0_csr",
                    description: Some(
                        "Endpoint 0 control and status register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Ep0Csr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ep0_count",
                    description: Some(
                        "Data count for endpoint 0.",
                    ),
                    array: None,
                    byte_offset: 0x11,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Ep0Count",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "in_csr2",
                    description: Some(
                        "Control and status register for IN endpoints.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "InCsr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "in_csr1",
                    description: Some(
                        "Control and status register for IN endpoints.",
                    ),
                    array: None,
                    byte_offset: 0x15,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "InCsr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "max_pkt_in",
                    description: Some(
                        "Maximum packet size for IN endpoints.",
                    ),
                    array: None,
                    byte_offset: 0x16,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "MaxPkt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "out_csr2",
                    description: Some(
                        "Control and status register for OUT endpoints.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "OutCsr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "out_csr1",
                    description: Some(
                        "Control and status register for OUT endpoints.",
                    ),
                    array: None,
                    byte_offset: 0x19,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "OutCsr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "max_pkt_out",
                    description: Some(
                        "Maximum packet size for OUT endpoints.",
                    ),
                    array: None,
                    byte_offset: 0x1a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "MaxPkt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "out_count",
                    description: Some(
                        "Data count for OUT endpoints.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "OutCount",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fifo",
                    description: Some(
                        "FIFO for endpoints.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 6,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Fifo",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Addr",
            extends: None,
            description: Some(
                "USB Address Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "addr",
                    description: Some(
                        "Function Address.",
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
                    name: "update",
                    description: Some(
                        "Address update flag. Set when a new address is written, cleared after transfer ends.",
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
            ],
        },
        FieldSet {
            name: "Ep0Count",
            extends: None,
            description: Some(
                "USB Endpoint 0 Data Count Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "count",
                    description: Some(
                        "Length of received data in IN packet.",
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
            ],
        },
        FieldSet {
            name: "Ep0Csr",
            extends: None,
            description: Some(
                "USB Endpoint 0 Control and Status Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "out_pkt_rdy",
                    description: Some(
                        "Indicates that an OUT packet has been received.",
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
                    name: "in_pkt_rdy",
                    description: Some(
                        "Indicates that an IN packet is ready for transmission.",
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
                    name: "sent_stall",
                    description: Some(
                        "Indicates that a STALL handshake was sent.",
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
                    name: "data_end",
                    description: Some(
                        "Indicates the end of data stage in control transfer.",
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
                    name: "setup_end",
                    description: Some(
                        "Indicates the end of a setup stage in control transfer.",
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
                    name: "send_stall",
                    description: Some(
                        "Sends a STALL handshake.",
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
                    name: "serviced_out_pkt_rdy",
                    description: Some(
                        "Clears OutPktRdy flag.",
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
                    name: "serviced_setup_end",
                    description: Some(
                        "Clears SetupEnd flag.",
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
            ],
        },
        FieldSet {
            name: "Fifo",
            extends: None,
            description: Some(
                "USB Endpoint FIFO Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "FIFO data for each endpoint.",
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
            ],
        },
        FieldSet {
            name: "Frame",
            extends: None,
            description: Some(
                "USB Frame Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "framenum",
                    description: Some(
                        "Frame number of the last received frame.",
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
            ],
        },
        FieldSet {
            name: "InCsr1",
            extends: None,
            description: Some(
                "USB IN Endpoint Control and Status Register 1.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "in_pkt_rdy",
                    description: Some(
                        "Indicates that an IN packet is ready to be sent.",
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
                    name: "fifo_not_empty",
                    description: Some(
                        "Indicates that the FIFO contains data.",
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
                    name: "underrun",
                    description: Some(
                        "Indicates that an underrun condition occurred.",
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
                    name: "flush_fifo",
                    description: Some(
                        "Flushes the FIFO content.",
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
                    name: "send_stall",
                    description: Some(
                        "Sends a STALL handshake for the IN endpoint.",
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
                    name: "sent_stall",
                    description: Some(
                        "Indicates that a STALL handshake was sent.",
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
                    name: "clr_data_tog",
                    description: Some(
                        "Clears the endpoint's data toggle bit.",
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
            ],
        },
        FieldSet {
            name: "InCsr2",
            extends: None,
            description: Some(
                "IN Endpoint Control Register 2.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "frc_data_tog",
                    description: Some(
                        "Force Data Toggle. Forces toggle even without ACK and clears FIFO.",
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
                    name: "dmae",
                    description: Some(
                        "DMA request enable for IN endpoint.",
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
                    name: "mode",
                    description: Some(
                        "Endpoint mode. 1, IN endpoint, 0, OUT endpoint.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Mode",
                    ),
                },
                Field {
                    name: "iso",
                    description: Some(
                        "Enable ISO transmission. 1, ISO mode, 0, Bulk or Interrupt mode.",
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
                    name: "auto_set",
                    description: Some(
                        "When set, automatically sets InPktRdy when the maximum packet size is written to the FIFO.",
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
            ],
        },
        FieldSet {
            name: "Index",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "index",
                    description: Some(
                        "Endpoint selection index.",
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
            name: "IntIn1",
            extends: None,
            description: Some(
                "USB IN Endpoint Interrupt Status Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ep0",
                    description: Some(
                        "Endpoint 0 interrupt.",
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
                    name: "epin",
                    description: Some(
                        "Endpoint n+1 IN interrupt.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "IntOut1",
            extends: None,
            description: Some(
                "USB OUT Endpoint Interrupt Status Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "epout",
                    description: Some(
                        "Endpoint n+1 OUT interrupt.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "IntUsb",
            extends: None,
            description: Some(
                "USB Interrupt Status Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "suspend",
                    description: Some(
                        "Suspend signal detected on USB bus.",
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
                    name: "resume",
                    description: Some(
                        "Resume signal detected on USB bus.",
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
                    name: "reset",
                    description: Some(
                        "Reset signal detected on USB bus.",
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
                    name: "sof",
                    description: Some(
                        "Start-of-frame interrupt.",
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
            name: "MaxPkt",
            extends: None,
            description: Some(
                "USB Endpoint Maximum Packet Size Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "max_pkt_size",
                    description: Some(
                        "Maximum packet size for the endpoint.",
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
            ],
        },
        FieldSet {
            name: "OutCount",
            extends: None,
            description: Some(
                "USB OUT Endpoint Data Count Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "count",
                    description: Some(
                        "Length of received data in OUT packet.",
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
            ],
        },
        FieldSet {
            name: "OutCsr1",
            extends: None,
            description: Some(
                "USB OUT Endpoint Control and Status Register 1.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "out_pkt_rdy",
                    description: Some(
                        "Indicates that an OUT packet has been received.",
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
                    name: "fifo_full",
                    description: Some(
                        "Indicates that the FIFO is full.",
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
                    name: "overrun",
                    description: Some(
                        "Indicates that an overrun condition occurred.",
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
                    name: "data_error",
                    description: Some(
                        "Indicates that a data error occurred.",
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
                    name: "flush_fifo",
                    description: Some(
                        "Flushes the FIFO content.",
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
                    name: "send_stall",
                    description: Some(
                        "Sends a STALL handshake for the OUT endpoint.",
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
                    name: "sent_stall",
                    description: Some(
                        "Indicates that a STALL handshake was sent.",
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
                    name: "clr_data_tog",
                    description: Some(
                        "Clears the endpoint's data toggle bit.",
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
            ],
        },
        FieldSet {
            name: "OutCsr2",
            extends: None,
            description: Some(
                "USB OUT Endpoint Control and Status Register 2.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "dma_mode",
                    description: Some(
                        "DMA mode for the OUT endpoint.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dmamode",
                    ),
                },
                Field {
                    name: "dmae",
                    description: Some(
                        "Enables DMA requests for the OUT endpoint.",
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
                    name: "iso",
                    description: Some(
                        "Indicates if the endpoint is configured for ISO transfer.",
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
                    name: "auto_clear",
                    description: Some(
                        "Automatically clears OutPktRdy after data is read from the FIFO.",
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
            ],
        },
        FieldSet {
            name: "Power",
            extends: None,
            description: Some(
                "USB Power Management Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "enable_suspend",
                    description: Some(
                        "Enable suspend functionality.",
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
                    name: "suspend_mode",
                    description: Some(
                        "Indicates if the USB device is in suspend mode.",
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
                    name: "resume",
                    description: Some(
                        "Resumes the USB device from suspend mode when set by software.",
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
                    name: "reset",
                    description: Some(
                        "Indicates if there is a reset signal on the USB bus.",
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
                    name: "iso_update",
                    description: Some(
                        "Forces the USB controller to wait for a SOF before sending a data packet.",
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
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Dmamode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DMAANDIT",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "DMAORIT",
                    description: None,
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Mode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "OUT",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "IN",
                    description: None,
                    value: 1,
                },
            ],
        },
    ],
};
                