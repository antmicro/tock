// FIXME: CHECK IF THESE APPLY FOR nRF52-DK

use kernel::common::VolatileCell;

// seem to be OK perhaps add assertions for this
pub const RTC1_BASE: usize = 0x40011000;
#[repr(C, packed)]
pub struct RTC1 {
    pub tasks_start: VolatileCell<u32>,
    pub tasks_stop: VolatileCell<u32>,
    pub tasks_clear: VolatileCell<u32>,
    pub tasks_trigovrflw: VolatileCell<u32>,
    _reserved1: [u32; 60],
    pub events_tick: VolatileCell<u32>,
    pub events_ovrflw: VolatileCell<u32>,
    _reserved2: [u32; 14],
    pub events_compare: [VolatileCell<u32>; 4],
    _reserved3: [u32; 109],
    pub intenset: VolatileCell<u32>,
    pub intenclr: VolatileCell<u32>,
    _reserved4: [u32; 13],
    pub evten: VolatileCell<u32>,
    pub evtenset: VolatileCell<u32>,
    pub evtenclr: VolatileCell<u32>,
    _reserved5: [u32; 110],
    pub counter: VolatileCell<u32>,
    pub prescaler: VolatileCell<u32>,
    _reserved6: [u32; 13],
    pub cc: [VolatileCell<u32>; 4],
    _reserved7: [u32; 683],
    pub power: VolatileCell<u32>,    // this doesn't exist
}

// OK!!!
pub const GPIO_BASE: usize = 0x50000000;
#[repr(C, packed)]
pub struct GPIO {
    _reserved1: [u32; 321],
    pub out: VolatileCell<u32>,
    pub outset: VolatileCell<u32>,
    pub outclr: VolatileCell<u32>,
    pub in_: VolatileCell<u32>,
    pub dir: VolatileCell<u32>,
    pub dirset: VolatileCell<u32>,
    pub dirclr: VolatileCell<u32>,
    _reserved2: [u32; 120],
    pub pin_cnf: [VolatileCell<u32>; 32],
}

pub const TEMP_BASE: usize = 0x4000C000;
#[allow(non_snake_case)]
#[repr(C, packed)]
pub struct TEMP_REGS {
    pub START: VolatileCell<u32>, // 0x000 - 0x004
    pub STOP: VolatileCell<u32>, // 0x004 - 0x008
    pub _reserved1: [u32; 62], // 0x008 - 0x100
    pub DATARDY: VolatileCell<u32>, // 0x100 - 0x104
    pub _reserved2: [u32; 127], // 0x104 - 0x300
    pub INTEN: VolatileCell<u32>, // 0x300 - 0x304
    pub INTENSET: VolatileCell<u32>, // 0x304 - 0x308
    pub INTENCLR: VolatileCell<u32>, // 0x308 - 0x30c
    pub _reserved3: [u32; 127], // 0x30c - 0x508
    pub TEMP: VolatileCell<u32>, // 0x508 - 0x50c
}

pub const RNG_BASE: usize = 0x4000D000;
#[allow(non_snake_case)]
#[repr(C, packed)]
pub struct RNG_REGS {
    pub START: VolatileCell<u32>, // 0x000 - 0x004
    pub STOP: VolatileCell<u32>, // 0x004 - 0x008
    pub _reserved1: [u32; 62], // 0x008 - 0x100
    pub VALRDY: VolatileCell<u32>, // 0x100 - 0x104
    pub _reserved2: [u32; 63], // 0x104 - 0x200
    pub SHORTS: VolatileCell<u32>, // 0x200 - 0x204
    pub _reserved3: [u32; 63], // 0x204 - 0x300
    pub INTEN: VolatileCell<u32>, // 0x300 - 0x304
    pub INTENSET: VolatileCell<u32>, // 0x304 - 0x308
    pub INTENCLR: VolatileCell<u32>, // 0x308 - 0x30c
    pub _reserved4: [u32; 126], // 0x30c - 0x504
    pub CONFIG: VolatileCell<u32>, // 0x504 - 0x508
    pub VALUE: VolatileCell<u32>, // 0x508 - 0x50c
}

pub const AESECB_BASE: usize = 0x4000E000;
#[allow(non_snake_case)]
#[repr(C, packed)]
pub struct AESECB_REGS {
    pub STARTECB: VolatileCell<u32>, // 0x000 - 0x004
    pub STOPECB: VolatileCell<u32>, // 0x004 - 0x008
    pub _reserved1: [u32; 62], // 0x008 - 0x100
    pub ENDECB: VolatileCell<u32>, // 0x100 - 0x104
    pub ERRORECB: VolatileCell<u32>, // 0x104 - 0x108
    pub _reserved2: [u32; 127], // 0x108 - 0x304
    pub INTENSET: VolatileCell<u32>, // 0x304 - 0x308
    pub INTENCLR: VolatileCell<u32>, // 0x308 - 0x30c
    pub _reserved3: [u32; 126], // 0x30c - 0x504
    pub ECBDATAPTR: VolatileCell<u32>, // 0x504 - 0x508
}

pub const RADIO_BASE: usize = 0x40001000;
#[allow(non_snake_case)]
#[repr(C, packed)]
pub struct RADIO_REGS {
    pub TXEN: VolatileCell<u32>, // 0x000 ---> 0x004
    pub RXEN: VolatileCell<u32>, // 0x004 ---> 0x008
    pub START: VolatileCell<u32>, // 0x008 ---> 0x00c
    pub STOP: VolatileCell<u32>, // 0x00c ---> 0x010
    pub DISABLE: VolatileCell<u32>, // 0x010 ---> 0x014
    pub RSSISTART: VolatileCell<u32>, // 0x014 ---> 0x018
    pub RSSISTOP: VolatileCell<u32>, // 0x018 ---> 0x01c
    pub BCSTART: VolatileCell<u32>, // 0x01c ---> 0x020
    pub BCSTOP: VolatileCell<u32>, // 0x020 ---> 0x024
    _reserved1: [u32; 55], // 0x024 ---> 0x100
    pub READY: VolatileCell<u32>, // 0x100 ---> 0x104
    pub ADDRESS: VolatileCell<u32>, // 0x104 ---> 0x108
    pub PAYLOAD: VolatileCell<u32>, // 0x108 ---> 0x10c
    pub END: VolatileCell<u32>, // 0x10c ---> 0x110
    pub DISABLED: VolatileCell<u32>, // 0x110 ---> 0x114
    pub DEVMATCH: VolatileCell<u32>, // 0x114 ---> 0x118
    pub DEVMISS: VolatileCell<u32>, // 0x118 ---> 0x11c
    pub RSSIEND: VolatileCell<u32>, // 0x11c -->  0x120
    _reserved2: [u32; 2], // 0x120 ---> 0x128
    pub BCMATCH: VolatileCell<u32>, // 0x128 ---> 0x12c
    _reserved3: [u32; 53], // 0x12c ---> 0x200
    pub SHORTS: VolatileCell<u32>, // 0x200 ---> 0x204
    _reserved4: [u32; 64], // 0x204 ---> 0x304
    pub INTENSET: VolatileCell<u32>, // 0x304 ---> 0x308
    pub INTENCLR: VolatileCell<u32>, // 0x308 ---> 0x30c
    _reserved5: [u32; 61], // 0x30c ---> 0x400
    pub CRCSTATUS: VolatileCell<u32>, // 0x400 - 0x404
    _reserved6: [u32; 1], // 0x404 - 0x408
    pub RXMATCH: VolatileCell<u32>, // 0x408 - 0x40c
    pub RXCRC: VolatileCell<u32>, // 0x40c - 0x410
    pub DAI: VolatileCell<u32>, // 0x410 - 0x414
    _reserved7: [u32; 60], // 0x414 - 0x504
    pub PACKETPTR: VolatileCell<u32>, // 0x504 - 0x508
    pub FREQEUNCY: VolatileCell<u32>, // 0x508 - 0x50c
    pub TXPOWER: VolatileCell<u32>, // 0x50c - 0x510
    pub MODE: VolatileCell<u32>, // 0x510 - 0x514
    pub PCNF0: VolatileCell<u32>, // 0x514 - 0x518
    pub PCNF1: VolatileCell<u32>, // 0x518 - 0x51c
    pub BASE0: VolatileCell<u32>, // 0x51c - 0x520
    pub BASE1: VolatileCell<u32>, // 0x520 - 0x524
    pub PREFIX0: VolatileCell<u32>, // 0x524 - 0x528
    pub PREFIX1: VolatileCell<u32>, // 0x528 - 0x52c
    pub TXADDRESS: VolatileCell<u32>, // 0x52c - 0x530
    pub RXADDRESSES: VolatileCell<u32>, // 0x530 - 0x534
    pub CRCCNF: VolatileCell<u32>, // 0x534 - 0x538
    pub CRCPOLY: VolatileCell<u32>, // 0x538 - 0x53c
    pub CRCINIT: VolatileCell<u32>, // 0x53c - 0x540
    pub TEST: VolatileCell<u32>, // 0x540 - 0x544
    pub TIFS: VolatileCell<u32>, // 0x544 - 0x548
    pub RSSISAMPLE: VolatileCell<u32>, // 0x548 - 0x54c
    _reserved8: [u32; 1], // 0x54c - 0x550
    pub STATE: VolatileCell<u32>, // 0x550 - 0x554
    pub DATAWHITEIV: VolatileCell<u32>, // 0x554 - 0x558
    _reserved9: [u32; 2], // 0x558 - 0x560
    pub BCC: VolatileCell<u32>, // 0x560 - 0x564
    _reserved10: [u32; 39], // 0x560 - 0x600
    pub DAB0: VolatileCell<u32>, // 0x600 - 0x604
    pub DAB1: VolatileCell<u32>, // 0x604 - 0x608
    pub DAB2: VolatileCell<u32>, // 0x608 - 0x60c
    pub DAB3: VolatileCell<u32>, // 0x60c - 0x610
    pub DAB4: VolatileCell<u32>, // 0x610 - 0x614
    pub DAB5: VolatileCell<u32>, // 0x614 - 0x618
    pub DAB6: VolatileCell<u32>, // 0x618 - 0x61c
    pub DAB7: VolatileCell<u32>, // 0x61c - 0x620
    pub DAP0: VolatileCell<u32>, // 0x620 - 0x624
    pub DAP1: VolatileCell<u32>, // 0x624 - 0x628
    pub DAP2: VolatileCell<u32>, // 0x628 - 0x62c
    pub DAP3: VolatileCell<u32>, // 0x62c - 0x630
    pub DAP4: VolatileCell<u32>, // 0x630 - 0x634
    pub DAP5: VolatileCell<u32>, // 0x634 - 0x638
    pub DAP6: VolatileCell<u32>, // 0x638 - 0x63c
    pub DAP7: VolatileCell<u32>, // 0x63c - 0x640
    pub DACNF: VolatileCell<u32>, // 0x640 - 0x644
    _reserved11: [u32; 56], // 0x644 - 0x724
    pub OVERRIDE0: VolatileCell<u32>, // 0x724 - 0x728
    pub OVERRIDE1: VolatileCell<u32>, // 0x728 - 0x72c
    pub OVERRIDE2: VolatileCell<u32>, // 0x72c - 0x730
    pub OVERRIDE3: VolatileCell<u32>, // 0x730 - 0x734
    pub OVERRIDE4: VolatileCell<u32>, // 0x734 - 0x738
    _reserved12: [u32; 561], // 0x738 - 0x724
    pub POWER: VolatileCell<u32>, // 0xFFC - 0x1000
}
