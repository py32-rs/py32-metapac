pub static ALL_PERIPHERAL_VERSIONS: &[(&str, &[&str])] = &[
    ("adc", &["v1", "v2", ]),
    ("configbytes", &["f030", "f072", ]),
    ("dma", &["f030", "f072", ]),
    ("exti", &["v1", ]),
    ("flash", &["f030", "f072", ]),
    ("gpio", &["t020", "v1", ]),
    ("i2c", &["v1", ]),
    ("rcc", &["f030", "f072", ]),
    ("syscfg", &["f030", "f072", ]),
    ("timer", &["v1", ]),
    ("usart", &["v1", ]),
    ("usb", &["v1", ]),
];