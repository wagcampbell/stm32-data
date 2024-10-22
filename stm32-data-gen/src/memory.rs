use stm32_data_serde::chip::memory::{self, Settings};
use stm32_data_serde::chip::Memory;

use crate::util::RegexMap;

struct Mem {
    name: &'static str,
    address: u32,
    size: u32,
}

macro_rules! mem {
    ($($name:ident $addr:literal $size:literal),*) => {
        &[
            $(
                Mem {
                    name: stringify!($name),
                    address: $addr,
                    size: $size*1024,
                },
            )*
        ]
    };
}

#[rustfmt::skip]
static MEMS: RegexMap<&[Mem]> = RegexMap::new(&[
    // C0. TODO: check
    ("STM32C01..4",                  mem!(BANK_1 0x08000000 16, SRAM 0x20000000 6)),
    ("STM32C01..6",                  mem!(BANK_1 0x08000000 32, SRAM 0x20000000 6)),
    ("STM32C03..4",                  mem!(BANK_1 0x08000000 16, SRAM 0x20000000 12)),
    ("STM32C03..6",                  mem!(BANK_1 0x08000000 32, SRAM 0x20000000 12)),
    // F0. TODO: check
    ("STM32F0...C",                  mem!(BANK_1 0x08000000 256, SRAM 0x20000000 32)),
    ("STM32F0[35]..8",               mem!(BANK_1 0x08000000 64, SRAM 0x20000000 8)),
    ("STM32F0[47]..6",               mem!(BANK_1 0x08000000 32, SRAM 0x20000000 6)),
    ("STM32F03..4",                  mem!(BANK_1 0x08000000 16, SRAM 0x20000000 4)),
    ("STM32F03..6",                  mem!(BANK_1 0x08000000 32, SRAM 0x20000000 4)),
    ("STM32F04..4",                  mem!(BANK_1 0x08000000 16, SRAM 0x20000000 6)),
    ("STM32F05..4",                  mem!(BANK_1 0x08000000 16, SRAM 0x20000000 8)),
    ("STM32F05..6",                  mem!(BANK_1 0x08000000 32, SRAM 0x20000000 8)),
    ("STM32F07..8",                  mem!(BANK_1 0x08000000 64, SRAM 0x20000000 16)),
    ("STM32F07..B",                  mem!(BANK_1 0x08000000 128, SRAM 0x20000000 16)),
    ("STM32F09..B",                  mem!(BANK_1 0x08000000 128, SRAM 0x20000000 32)),
    // F1. TODO: check
    ("STM32F1.[12].6",               mem!(BANK_1 0x08000000 32, SRAM 0x20000000 6)),
    ("STM32F1.[12].8",               mem!(BANK_1 0x08000000 64, SRAM 0x20000000 10)),
    ("STM32F1.[12].B",               mem!(BANK_1 0x08000000 128, SRAM 0x20000000 16)),
    ("STM32F1.[57].B",               mem!(BANK_1 0x08000000 128, SRAM 0x20000000 64)),
    ("STM32F1.[57].C",               mem!(BANK_1 0x08000000 256, SRAM 0x20000000 64)),
    ("STM32F1.0.6",                  mem!(BANK_1 0x08000000 32, SRAM 0x20000000 4)),
    ("STM32F1.0.8",                  mem!(BANK_1 0x08000000 64, SRAM 0x20000000 8)),
    ("STM32F1.0.B",                  mem!(BANK_1 0x08000000 128, SRAM 0x20000000 8)),
    ("STM32F1.0.C",                  mem!(BANK_1 0x08000000 256, SRAM 0x20000000 24)),
    ("STM32F1.0.D",                  mem!(BANK_1 0x08000000 384, SRAM 0x20000000 32)),
    ("STM32F1.0.E",                  mem!(BANK_1 0x08000000 512, SRAM 0x20000000 32)),
    ("STM32F1.1.C",                  mem!(BANK_1 0x08000000 256, SRAM 0x20000000 32)),
    ("STM32F1.1.D",                  mem!(BANK_1 0x08000000 384, SRAM 0x20000000 48)),
    ("STM32F1.1.E",                  mem!(BANK_1 0x08000000 512, SRAM 0x20000000 48)),
    ("STM32F1.1.F",                  mem!(BANK_1 0x08000000 512, BANK_2 0x08080000 256, SRAM 0x20000000 80)),
    ("STM32F1.1.G",                  mem!(BANK_1 0x08000000 512, BANK_2 0x08080000 512, SRAM 0x20000000 80)),
    ("STM32F1.3.6",                  mem!(BANK_1 0x08000000 32, SRAM 0x20000000 10)),
    ("STM32F1.3.8",                  mem!(BANK_1 0x08000000 64, SRAM 0x20000000 20)),
    ("STM32F1.3.B",                  mem!(BANK_1 0x08000000 128, SRAM 0x20000000 20)),
    ("STM32F1.3.C",                  mem!(BANK_1 0x08000000 256, SRAM 0x20000000 48)),
    ("STM32F1.3.D",                  mem!(BANK_1 0x08000000 384, SRAM 0x20000000 64)),
    ("STM32F1.3.E",                  mem!(BANK_1 0x08000000 512, SRAM 0x20000000 64)),
    ("STM32F1.3.F",                  mem!(BANK_1 0x08000000 512, BANK_2 0x08080000 256, SRAM 0x20000000 96)),
    ("STM32F1.3.G",                  mem!(BANK_1 0x08000000 512, BANK_2 0x08080000 512, SRAM 0x20000000 96)),
    ("STM32F1.5.8",                  mem!(BANK_1 0x08000000 64, SRAM 0x20000000 64)),
    ("STM32F10[012].4",              mem!(BANK_1 0x08000000 16, SRAM 0x20000000 4)),
    ("STM32F103.4",                  mem!(BANK_1 0x08000000 16, SRAM 0x20000000 6)),
    // F2. TODO: check
    ("STM32F2...B",                  mem!(BANK_1 0x08000000 128, SRAM 0x20000000 64, SRAM2 0x2001c000 0)),
    ("STM32F2...E",                  mem!(BANK_1 0x08000000 512, SRAM 0x20000000 128, SRAM2 0x2001c000 0)),
    ("STM32F2...F",                  mem!(BANK_1 0x08000000 768, SRAM 0x20000000 128, SRAM2 0x2001c000 0)),
    ("STM32F2...G",                  mem!(BANK_1 0x08000000 1024, SRAM 0x20000000 128, SRAM2 0x2001c000 0)),
    ("STM32F2.5.C",                  mem!(BANK_1 0x08000000 256, SRAM 0x20000000 96, SRAM2 0x2001c000 0)),
    ("STM32F2.7.C",                  mem!(BANK_1 0x08000000 256, SRAM 0x20000000 128, SRAM2 0x2001c000 0)),
    // F3. TODO: check
    ("STM32F3...4",                  mem!(BANK_1 0x08000000 16, SRAM 0x20000000 12)),
    ("STM32F3...D",                  mem!(BANK_1 0x08000000 384, SRAM 0x20000000 64)),
    ("STM32F3...E",                  mem!(BANK_1 0x08000000 512, SRAM 0x20000000 64)),
    ("STM32F3.[12].6",               mem!(BANK_1 0x08000000 32, SRAM 0x20000000 16)),
    ("STM32F3.[34].6",               mem!(BANK_1 0x08000000 32, SRAM 0x20000000 12)),
    ("STM32F3([17]..8|0[12].8)",     mem!(BANK_1 0x08000000 64, SRAM 0x20000000 16)),
    ("STM32F3([23]..8|03.8)",        mem!(BANK_1 0x08000000 64, SRAM 0x20000000 12)),
    ("STM32F3[05]..C",               mem!(BANK_1 0x08000000 256, SRAM 0x20000000 40)),
    ("STM32F30..B",                  mem!(BANK_1 0x08000000 128, SRAM 0x20000000 32)),
    ("STM32F37..B",                  mem!(BANK_1 0x08000000 128, SRAM 0x20000000 24)),
    ("STM32F37..C",                  mem!(BANK_1 0x08000000 256, SRAM 0x20000000 32)),
    // F4. TODO: check
    ("STM32F4...8",                  mem!(BANK_1 0x08000000 64, SRAM 0x20000000 32)),
    ("STM32F4...D",                  mem!(BANK_1 0x08000000 384, SRAM 0x20000000 64)),
    ("STM32F4...H",                  mem!(BANK_1 0x08000000 1536, SRAM 0x20000000 320, SRAM2 0x20040000 0)),
    ("STM32F4.[567].E",              mem!(BANK_1 0x08000000 512, SRAM 0x20000000 128, SRAM2 0x2001c000 0)),
    ("STM32F4.1.C",                  mem!(BANK_1 0x08000000 256, SRAM 0x20000000 64)),
    ("STM32F4.1.E",                  mem!(BANK_1 0x08000000 512, SRAM 0x20000000 64)),
    ("STM32F4.2.E",                  mem!(BANK_1 0x08000000 512, SRAM 0x20000000 256)),
    ("STM32F4.6.C",                  mem!(BANK_1 0x08000000 256, SRAM 0x20000000 128, SRAM2 0x2001c000 0)),
    ("STM32F4(1[57].G|0..G)",        mem!(BANK_1 0x08000000 1024, SRAM 0x20000000 128, SRAM2 0x2001c000 0)),
    ("STM32F4[23]..G",               mem!(BANK_1 0x08000000 1024, SRAM 0x20000000 192, SRAM2 0x2001c000 0)),
    ("STM32F4[23]..I",               mem!(BANK_1 0x08000000 1024, BANK_2 0x08100000 1024, SRAM 0x20000000 192, SRAM2 0x2001c000 0)),
    ("STM32F4[67]..G",               mem!(BANK_1 0x08000000 1024, SRAM 0x20000000 320, SRAM2 0x20028000 0)),
    ("STM32F4[67]..I",               mem!(BANK_1 0x08000000 1024, BANK_2 0x08100000 1024, SRAM 0x20000000 320, SRAM2 0x20028000 0)),
    ("STM32F40..B",                  mem!(BANK_1 0x08000000 128, SRAM 0x20000000 64)),
    ("STM32F41..B",                  mem!(BANK_1 0x08000000 128, SRAM 0x20000000 32)),
    ("STM32F412.G",                  mem!(BANK_1 0x08000000 1024, SRAM 0x20000000 256)),
    ("STM32F413.G",                  mem!(BANK_1 0x08000000 1024, SRAM 0x20000000 320, SRAM2 0x20040000 0)),
    ("STM32F429.E",                  mem!(BANK_1 0x08000000 512, SRAM 0x20000000 192, SRAM2 0x2001c000 0)),
    ("STM32F469.E",                  mem!(BANK_1 0x08000000 512, SRAM 0x20000000 320, SRAM2 0x20028000 0)),
    // F7. TODO: check
    ("STM32F7...C",                  mem!(BANK_1 0x08000000 256, SRAM 0x20010000 192, SRAM2 0x2003c000 0)),
    ("STM32F7...I",                  mem!(BANK_1 0x08000000 2048, SRAM 0x20020000 384, SRAM2 0x2007c000 0)),
    ("STM32F7[23]..E",               mem!(BANK_1 0x08000000 512, SRAM 0x20010000 192, SRAM2 0x2003c000 0)),
    ("STM32F7[45]..G",               mem!(BANK_1 0x08000000 1024, SRAM 0x20010000 320, SRAM2 0x2004c000 0)),
    ("STM32F73..8",                  mem!(BANK_1 0x08000000 64, SRAM 0x20010000 192, SRAM2 0x2003c000 0)),
    ("STM32F74..E",                  mem!(BANK_1 0x08000000 512, SRAM 0x20010000 320, SRAM2 0x2004c000 0)),
    ("STM32F75..8",                  mem!(BANK_1 0x08000000 64, SRAM 0x20010000 320, SRAM2 0x2004c000 0)),
    ("STM32F76..G",                  mem!(BANK_1 0x08000000 1024, SRAM 0x20020000 384, SRAM2 0x2007c000 0)),
    // G0. TODO: check
    ("STM32G0...4",                  mem!(BANK_1 0x08000000 16, SRAM 0x20000000 8)),
    ("STM32G0...C",                  mem!(BANK_1 0x08000000 256, SRAM 0x20000000 128)),
    ("STM32G0...E",                  mem!(BANK_1 0x08000000 256, BANK_2 0x08040000 256, SRAM 0x20000000 128)),
    ("STM32G0[34]..8",               mem!(BANK_1 0x08000000 64, SRAM 0x20000000 8)),
    ("STM32G0(3..6|4.J6)",           mem!(BANK_1 0x08000000 32, SRAM 0x20000000 8)),
    ("STM32G04..6",                  mem!(BANK_1 0x08000000 64, SRAM 0x20000000 8)),
    ("STM32G0[56]..6",               mem!(BANK_1 0x08000000 32, SRAM 0x20000000 16)),
    ("STM32G0[56]..8",               mem!(BANK_1 0x08000000 64, SRAM 0x20000000 16)),
    ("STM32G0[78]..B",               mem!(BANK_1 0x08000000 128, SRAM 0x20000000 32)),
    ("STM32G07..6",                  mem!(BANK_1 0x08000000 32, SRAM 0x20000000 32)),
    ("STM32G07..8",                  mem!(BANK_1 0x08000000 64, SRAM 0x20000000 32)),
    ("STM32G0B..B",                  mem!(BANK_1 0x08000000 128, SRAM 0x20000000 128)),
    // G4. TODO: check
    ("STM32G4...6",                  mem!(BANK_1 0x08000000 32, SRAM 0x20000000 20, SRAM2 0x20004000 0)),
    ("STM32G4...8",                  mem!(BANK_1 0x08000000 64, SRAM 0x20000000 20, SRAM2 0x20004000 0)),
    ("STM32G4[34]..B",               mem!(BANK_1 0x08000000 128, SRAM 0x20000000 20, SRAM2 0x20004000 0)),
    ("STM32G4[78]..E",               mem!(BANK_1 0x08000000 512, SRAM 0x20000000 96, SRAM2 0x20014000 0)),
    ("STM32G4[9A]..E",               mem!(BANK_1 0x08000000 512, SRAM 0x20000000 32, SRAM2 0x20014000 0)),
    ("STM32G47..B",                  mem!(BANK_1 0x08000000 128, SRAM 0x20000000 96, SRAM2 0x20014000 0)),
    ("STM32G47..C",                  mem!(BANK_1 0x08000000 256, SRAM 0x20000000 96, SRAM2 0x20014000 0)),
    ("STM32G49..C",                  mem!(BANK_1 0x08000000 256, SRAM 0x20000000 32, SRAM2 0x20014000 0)),
    // H5.
    ("STM32H5...B",                  mem!(BANK_1 0x08000000 64, BANK_2 0x08010000 64, SRAM1 0x20000000 16, SRAM2 0x20004000 16)),
    ("STM32H5...C",                  mem!(BANK_1 0x08000000 128, BANK_2 0x08020000 128, SRAM1 0x20000000 128, SRAM2 0x20020000 80, SRAM3 0x20034000 64)),
    ("STM32H5...E",                  mem!(BANK_1 0x08000000 256, BANK_2 0x08040000 256, SRAM1 0x20000000 128, SRAM2 0x20020000 80, SRAM3 0x20034000 64)),
    ("STM32H5...G",                  mem!(BANK_1 0x08000000 512, BANK_2 0x08080000 512, SRAM1 0x20000000 256, SRAM2 0x20040000 64, SRAM3 0x20050000 320)),
    ("STM32H5...I",                  mem!(BANK_1 0x08000000 1024, BANK_2 0x08100000 1024, SRAM1 0x20000000 256, SRAM2 0x20040000 64, SRAM3 0x20050000 320)),
    // H7. TODO: check
    ("STM32H7...E",                  mem!(BANK_1 0x08000000 512, SRAM 0x24000000 128)),
    ("STM32H7[23]..G",               mem!(BANK_1 0x08000000 1024, SRAM 0x24000000 128)),
    ("STM32H7[45]..I",               mem!(BANK_1 0x08000000 1024, BANK_2 0x08100000 1024, SRAM 0x24000000 512)),
    ("STM32H7[AB]..I",               mem!(BANK_1 0x08000000 1024, BANK_2 0x08100000 1024, SRAM 0x24000000 1024)),
    ("STM32H73..B",                  mem!(BANK_1 0x08000000 128, SRAM 0x24000000 128)),
    ("STM32H74..G",                  mem!(BANK_1 0x08000000 1024, SRAM 0x24000000 512)),
    ("STM32H75..B",                  mem!(BANK_1 0x08000000 128, SRAM 0x24000000 512)),
    ("STM32H7A..G",                  mem!(BANK_1 0x08000000 1024, SRAM 0x24000000 1024)),
    ("STM32H7B..B",                  mem!(BANK_1 0x08000000 128, SRAM 0x24000000 1024)),
    // L0. TODO: check
    ("STM32L0...3",                  mem!(BANK_1 0x08000000 8, SRAM 0x20000000 2)),
    ("STM32L0...6",                  mem!(BANK_1 0x08000000 32, SRAM 0x20000000 8)),
    ("STM32L0...B",                  mem!(BANK_1 0x08000000 128, SRAM 0x20000000 20)),
    ("STM32L0...Z",                  mem!(BANK_1 0x08000000 192, SRAM 0x20000000 20)),
    ("STM32L0[12]..4",               mem!(BANK_1 0x08000000 16, SRAM 0x20000000 2)),
    ("STM32L0[156]..8",              mem!(BANK_1 0x08000000 64, SRAM 0x20000000 8)),
    ("STM32L0[34]..4",               mem!(BANK_1 0x08000000 16, SRAM 0x20000000 8)),
    ("STM32L0[78]..8",               mem!(BANK_1 0x08000000 64, SRAM 0x20000000 20)),
    // L1. TODO: check
    ("STM32L1...B",                  mem!(BANK_1 0x08000000 128, SRAM 0x20000000 10)),
    ("STM32L1...C..",                mem!(BANK_1 0x08000000 192, BANK_2 0x08030000 64, SRAM 0x20000000 32)),
    ("STM32L1...D..",                mem!(BANK_1 0x08000000 128, BANK_2 0x08040000 256, SRAM 0x20000000 80)),
    ("STM32L1...D",                  mem!(BANK_1 0x08000000 192, BANK_2 0x08030000 192, SRAM 0x20000000 48)),
    ("STM32L1...E",                  mem!(BANK_1 0x08000000 256, BANK_2 0x08040000 256, SRAM 0x20000000 80)),
    ("STM32L1(6.[RV]C|5.[CRUV]C)",   mem!(BANK_1 0x08000000 256, SRAM 0x20000000 32)),
    ("STM32L1[56].[QZ]C",            mem!(BANK_1 0x08000000 192, BANK_2 0x08030000 64, SRAM 0x20000000 32)),
    ("STM32L10..6..",                mem!(BANK_1 0x08000000 32, SRAM 0x20000000 10)),
    ("STM32L10..6",                  mem!(BANK_1 0x08000000 32, SRAM 0x20000000 4)),
    ("STM32L10..8..",                mem!(BANK_1 0x08000000 64, SRAM 0x20000000 10)),
    ("STM32L10..8",                  mem!(BANK_1 0x08000000 64, SRAM 0x20000000 8)),
    ("STM32L10..B..",                mem!(BANK_1 0x08000000 128, SRAM 0x20000000 10)),
    ("STM32L10..C",                  mem!(BANK_1 0x08000000 256, SRAM 0x20000000 16)),
    ("STM32L15..6..",                mem!(BANK_1 0x08000000 32, SRAM 0x20000000 16)),
    ("STM32L15..6",                  mem!(BANK_1 0x08000000 32, SRAM 0x20000000 10)),
    ("STM32L15..8..",                mem!(BANK_1 0x08000000 64, SRAM 0x20000000 16)),
    ("STM32L15..8",                  mem!(BANK_1 0x08000000 64, SRAM 0x20000000 10)),
    ("STM32L15..B..",                mem!(BANK_1 0x08000000 128, SRAM 0x20000000 16)),
    // L4. TODO: check
    ("STM32L4...8",                  mem!(BANK_1 0x08000000 64, SRAM 0x20000000 40)),
    ("STM32L4...I",                  mem!(BANK_1 0x08000000 2048, SRAM 0x20000000 192)),
    ("STM32L4[12]..B",               mem!(BANK_1 0x08000000 128, SRAM 0x20000000 40)),
    ("STM32L4[34]..C",               mem!(BANK_1 0x08000000 256, SRAM 0x20000000 48)),
    ("STM32L4[56]..E",               mem!(BANK_1 0x08000000 512, SRAM 0x20000000 128)),
    ("STM32L4[78]..G",               mem!(BANK_1 0x08000000 512, BANK_2 0x08080000 512, SRAM 0x20000000 96)),
    ("STM32L4[9A]..G",               mem!(BANK_1 0x08000000 512, BANK_2 0x08080000 512, SRAM 0x20000000 256)),
    ("STM32L43..B",                  mem!(BANK_1 0x08000000 128, SRAM 0x20000000 48)),
    ("STM32L45..C",                  mem!(BANK_1 0x08000000 256, SRAM 0x20000000 128)),
    ("STM32L47..C",                  mem!(BANK_1 0x08000000 256, SRAM 0x20000000 96)),
    ("STM32L47..E",                  mem!(BANK_1 0x08000000 512, SRAM 0x20000000 96)),
    ("STM32L49..E",                  mem!(BANK_1 0x08000000 512, SRAM 0x20000000 256)),
    // L4+. TODO: check
    ("STM32L4[PQR]..G",              mem!(BANK_1 0x08000000 1024, SRAM 0x20000000 192)),
    ("STM32L4P..E",                  mem!(BANK_1 0x08000000 512, SRAM 0x20000000 192)),
    // L5. TODO: check
    ("STM32L5...C",                  mem!(BANK_1 0x08000000 256, SRAM 0x20000000 256, SRAM2 0x20030000 0)),
    ("STM32L5...E",                  mem!(BANK_1 0x08000000 512, SRAM 0x20000000 256, SRAM2 0x20030000 0)),
    // U0
    ("STM32U031.4",                  mem!(BANK_1 0x08000000 16, SRAM 0x20000000 12)),
    ("STM32U031.6",                  mem!(BANK_1 0x08000000 32, SRAM 0x20000000 12)),
    ("STM32U031.8",                  mem!(BANK_1 0x08000000 64, SRAM 0x20000000 12)),
    ("STM32U0[78]3.8",               mem!(BANK_1 0x08000000 64, SRAM 0x20000000 40)),
    ("STM32U0[78]3.B",               mem!(BANK_1 0x08000000 128, SRAM 0x20000000 40)),
    ("STM32U0[78]3.C",               mem!(BANK_1 0x08000000 256, SRAM 0x20000000 40)),
    // U5
    ("STM32U5[34]..B",               mem!(BANK_1 0x08000000 64, BANK_2 0x08010000 64, SRAM 0x20000000 192, SRAM2 0x20030000 64)),
    ("STM32U5[34]..C",               mem!(BANK_1 0x08000000 128, BANK_2 0x08020000 128, SRAM 0x20000000 192, SRAM2 0x20030000 64)),
    ("STM32U5[43]..E",               mem!(BANK_1 0x08000000 256, BANK_2 0x08040000 256, SRAM 0x20000000 192, SRAM2 0x20030000 64)),
    ("STM32U5[78]..G",               mem!(BANK_1 0x08000000 512, BANK_2 0x08080000 512, SRAM 0x20000000 192, SRAM2 0x20030000 64, SRAM3 0x20040000 512)),
    ("STM32U5[78]..I",               mem!(BANK_1 0x08000000 1024, BANK_2 0x08100000 1024, SRAM 0x20000000 192, SRAM2 0x20030000 64, SRAM3 0x20040000 512)),
    ("STM32U5[9A]..I",               mem!(BANK_1 0x08000000 1024, BANK_2 0x08100000 1024, SRAM 0x20000000 768, SRAM2 0x200c0000 64, SRAM3 0x200d0000 832, SRAM5 0x201a0000 832)),
    ("STM32U5[9A]..J",               mem!(BANK_1 0x08000000 2048, BANK_2 0x08200000 2048, SRAM 0x20000000 768, SRAM2 0x200c0000 64, SRAM3 0x200d0000 832, SRAM5 0x201a0000 832)),
    ("STM32U5[FG]..I",               mem!(BANK_1 0x08000000 1024, BANK_2 0x08200000 1024, SRAM 0x20000000 768, SRAM2 0x200c0000 64, SRAM3 0x200d0000 832, SRAM5 0x201a0000 832, SRAM6 0x20270000 512)),
    ("STM32U5[FG]..J",               mem!(BANK_1 0x08000000 2048, BANK_2 0x08200000 2048, SRAM 0x20000000 768, SRAM2 0x200c0000 64, SRAM3 0x200d0000 832, SRAM5 0x201a0000 832, SRAM6 0x20270000 512)),
    // WB. TODO: check
    ("STM32WB...Y",                  mem!(BANK_1 0x08000000 640, SRAM 0x20000000 192)),
    ("STM32WB.(0C|5V)G",             mem!(BANK_1 0x08000000 1024, SRAM 0x20000000 128)),
    ("STM32WB.(5C|5R)G",             mem!(BANK_1 0x08000000 1024, SRAM 0x20000000 192)),
    ("STM32WB1..C",                  mem!(BANK_1 0x08000000 320, SRAM 0x20000000 12)),
    ("STM32WB3..C",                  mem!(BANK_1 0x08000000 256, SRAM 0x20000000 96)),
    ("STM32WB3..E",                  mem!(BANK_1 0x08000000 512, SRAM 0x20000000 96)),
    ("STM32WB5..C",                  mem!(BANK_1 0x08000000 256, SRAM 0x20000000 128)),
    ("STM32WB5.[CR]E",               mem!(BANK_1 0x08000000 512, SRAM 0x20000000 192)),
    ("STM32WB5.VE",                  mem!(BANK_1 0x08000000 512, SRAM 0x20000000 128)),
    // WBA. TODO: check
    ("STM32WBA...E",                 mem!(BANK_1 0x08000000 512, SRAM 0x20000000 96, SRAM2 0x20010000 0)),
    ("STM32WBA...G",                 mem!(BANK_1 0x08000000 1024, SRAM 0x20000000 128, SRAM2 0x20010000 0)),
    // WL. TODO: check
    ("STM32WL...8",                  mem!(BANK_1 0x08000000 64, SRAM 0x20000000 12, SRAM2 0x20008000 0)),
    ("STM32WL...B",                  mem!(BANK_1 0x08000000 128, SRAM 0x20000000 12, SRAM2 0x20008000 0)),
    ("STM32WL...C",                  mem!(BANK_1 0x08000000 256, SRAM 0x20000000 12, SRAM2 0x20008000 0)),
]);

struct FlashInfo {
    write_size: u32,
    erase_size: &'static [(u32, u32)],
    erase_value: u8,
}

#[rustfmt::skip]
#[allow(clippy::identity_op)]
static FLASH_INFO: RegexMap<FlashInfo> = RegexMap::new(&[
    ("STM32C0.*",               FlashInfo{ erase_value: 0xFF, write_size:  8, erase_size: &[(  2*1024, 0)] }),
    ("STM32F030.C",             FlashInfo{ erase_value: 0xFF, write_size:  4, erase_size: &[(  2*1024, 0)] }),
    ("STM32F070.6",             FlashInfo{ erase_value: 0xFF, write_size:  4, erase_size: &[(  1*1024, 0)] }),
    ("STM32F0[79].*",           FlashInfo{ erase_value: 0xFF, write_size:  4, erase_size: &[(  2*1024, 0)] }),
    ("STM32F0.*",               FlashInfo{ erase_value: 0xFF, write_size:  4, erase_size: &[(  1*1024, 0)] }),
    ("STM32F10[0123].[468B]",   FlashInfo{ erase_value: 0xFF, write_size:  4, erase_size: &[(  1*1024, 0)] }),
    ("STM32F1.*",               FlashInfo{ erase_value: 0xFF, write_size:  4, erase_size: &[(  2*1024, 0)] }),
    ("STM32F2.*",               FlashInfo{ erase_value: 0xFF, write_size:  4, erase_size: &[( 16*1024, 4), (64*1024, 1), ( 128*1024, 0)] }),
    ("STM32F3.*",               FlashInfo{ erase_value: 0xFF, write_size:  8, erase_size: &[(  2*1024, 0)] }),
    ("STM32F4.*",               FlashInfo{ erase_value: 0xFF, write_size:  4, erase_size: &[( 16*1024, 4), (64*1024, 1), ( 128*1024, 0)] }),
    ("STM32F7[4567].*",         FlashInfo{ erase_value: 0xFF, write_size: 16, erase_size: &[( 32*1024, 4), (128*1024, 1), ( 256*1024, 0)] }),
    ("STM32F7.*",               FlashInfo{ erase_value: 0xFF, write_size: 16, erase_size: &[( 16*1024, 4), (64*1024, 1), ( 128*1024, 0)] }),
    ("STM32G0.*",               FlashInfo{ erase_value: 0xFF, write_size:  8, erase_size: &[(  2*1024, 0)] }),
    ("STM32G4[78].*",           FlashInfo{ erase_value: 0xFF, write_size:  8, erase_size: &[(  4*1024, 0)] }),
    ("STM32G4.*",               FlashInfo{ erase_value: 0xFF, write_size:  8, erase_size: &[(  2*1024, 0)] }),
    ("STM32H5.*",               FlashInfo{ erase_value: 0xFF, write_size: 16, erase_size: &[(  8*1024, 0)] }),
    ("STM32H7[AB].*",           FlashInfo{ erase_value: 0xFF, write_size: 32, erase_size: &[(  8*1024, 0)] }),
    ("STM32H7.*",               FlashInfo{ erase_value: 0xFF, write_size: 32, erase_size: &[(128*1024, 0)] }),
    ("STM32L4[PQRS].*",         FlashInfo{ erase_value: 0xFF, write_size:  8, erase_size: &[(  8*1024, 0)] }),
    ("STM32L4.*",               FlashInfo{ erase_value: 0xFF, write_size:  8, erase_size: &[(  2*1024, 0)] }),
    ("STM32L0.*",               FlashInfo{ erase_value: 0x00, write_size:  4, erase_size: &[(     128, 0)] }),
    ("STM32L1.*",               FlashInfo{ erase_value: 0x00, write_size:  4, erase_size: &[(     256, 0)] }),
    ("STM32L5.*",               FlashInfo{ erase_value: 0xFF, write_size:  8, erase_size: &[(  4*1024, 0)] }),
    ("STM32U0.*",               FlashInfo{ erase_value: 0xFF, write_size:  8, erase_size: &[(  2*1024, 0)] }),
    ("STM32U5[78].*",           FlashInfo{ erase_value: 0xFF, write_size: 16, erase_size: &[(  8*1024, 0)] }),
    ("STM32U5.*",               FlashInfo{ erase_value: 0xFF, write_size: 16, erase_size: &[(  8*1024, 0)] }),
    ("STM32WBA.*",              FlashInfo{ erase_value: 0xFF, write_size: 16, erase_size: &[(  8*1024, 0)] }),
    ("STM32WB1.*",              FlashInfo{ erase_value: 0x00, write_size:  8, erase_size: &[(  2*1024, 0)] }),
    ("STM32WB.*",               FlashInfo{ erase_value: 0xFF, write_size:  8, erase_size: &[(  4*1024, 0)] }),
    ("STM32WL.*",               FlashInfo{ erase_value: 0xFF, write_size:  8, erase_size: &[(  2*1024, 0)] }),
    ("STM32.*",                 FlashInfo{ erase_value: 0xFF, write_size:  8, erase_size: &[(  2*1024, 0)] }),
]);

pub fn get(chip: &str) -> Vec<Memory> {
    let mems = *MEMS.must_get(chip);
    let flash = FLASH_INFO.must_get(chip);

    let mut res = Vec::new();

    for mem in mems {
        if mem.name.starts_with("BANK") {
            if flash.erase_size.len() == 1 || mem.size <= flash.erase_size[0].0 * flash.erase_size[0].1 {
                res.push(Memory {
                    name: mem.name.to_string(),
                    address: mem.address,
                    size: mem.size,
                    kind: memory::Kind::Flash,
                    settings: Some(Settings {
                        write_size: flash.write_size,
                        erase_size: flash.erase_size[0].0,
                        erase_value: flash.erase_value,
                    }),
                });
            } else {
                let mut offs = 0;
                for (i, &(erase_size, count)) in flash.erase_size.iter().enumerate() {
                    if offs >= mem.size {
                        break;
                    }
                    let left = mem.size - offs;
                    let mut size = left;
                    if i != flash.erase_size.len() - 1 {
                        size = size.min(erase_size * count);
                    }
                    #[allow(clippy::redundant_field_names)]
                    res.push(Memory {
                        name: format!("{}_REGION_{}", mem.name, i + 1),
                        address: mem.address + offs,
                        size: size,
                        kind: memory::Kind::Flash,
                        settings: Some(Settings {
                            write_size: flash.write_size,
                            erase_size: erase_size,
                            erase_value: flash.erase_value,
                        }),
                    });
                    offs += size;
                }
            }
        } else {
            let mut kind = memory::Kind::Ram;
            if mem.name.contains("FLASH") || mem.name.contains("AXIICP") {
                kind = memory::Kind::Flash;
            }
            res.push(Memory {
                name: mem.name.to_string(),
                address: mem.address,
                size: mem.size,
                kind,
                settings: None,
            });
        }
    }

    res.sort_by_key(|m| (m.address, m.name.clone()));

    res
}
