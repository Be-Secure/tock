// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! FlexSPI Configuration Block (FCB)
//!
//! The FCB holds command sequences and configurations necessary to boot
//! an iMXRT10xx processor from FLASH over SPI.
//!
//! The array was auto-generated using the [`imxrt-boot-gen` crate](https://github.com/imxrt-rs/imxrt-boot-gen).
//! We copied the array of magic numbers here, rather than incorporating
//! that build-time dependency in tock.
//!
//! The array is only referenced in the linker script.

pub type FCB = [u8; 512];

#[link_section = ".fcb"]
#[no_mangle]
#[used]
static FLEXSPI_CONFIGURATION_BLOCK: FCB = [
    0x46, // 0x000 Tag 'FCFB'
    0x43, // 0x001
    0x46, // 0x002
    0x42, // 0x003
    0x00, // 0x004 Version 'bugfix'
    0x00, // 0x005 Version 'minor'
    0x01, // 0x006 Version 'major
    0x56, // 0x007 Version 'V'
    0x00, // 0x008 RESERVED
    0x00, // 0x009 RESERVED
    0x00, // 0x00A RESERVED
    0x00, // 0x00B RESERVED
    0x01, // 0x00C readSampleClkSrc
    0x01, // 0x00D csHoldTime
    0x02, // 0x00E csSetupTime
    0x00, // 0x00F columnAddressWidth
    0x00, // 0x010 deviceModeCfgEnable
    0x00, // 0x011 RESERVED
    0x00, // 0x012
    0x00, // 0x013 waitTimeCfgCommands
    0x00, // 0x014
    0x00, // 0x015
    0x00, // 0x016
    0x00, // 0x017
    0x00, // 0x018
    0x00, // 0x019
    0x00, // 0x01A
    0x00, // 0x01B
    0x00, // 0x01C
    0x00, // 0x01D RESERVED
    0x00, // 0x01E RESERVED
    0x00, // 0x01F RESERVED
    0x00, // 0x020
    0x00, // 0x021
    0x00, // 0x022
    0x00, // 0x023
    0x00, // 0x024
    0x00, // 0x025
    0x00, // 0x026
    0x00, // 0x027
    0x00, // 0x028
    0x00, // 0x029
    0x00, // 0x02A
    0x00, // 0x02B
    0x00, // 0x02C RESERVED
    0x00, // 0x02D RESERVED
    0x00, // 0x02E RESERVED
    0x00, // 0x02F RESERVED
    0x00, // 0x030
    0x00, // 0x031
    0x00, // 0x032
    0x00, // 0x033
    0x00, // 0x034
    0x00, // 0x035
    0x00, // 0x036
    0x00, // 0x037
    0x00, // 0x038
    0x00, // 0x039
    0x00, // 0x03A
    0x00, // 0x03B
    0x00, // 0x03C RESERVED
    0x00, // 0x03D RESERVED
    0x00, // 0x03E RESERVED
    0x00, // 0x03F RESERVED
    0x00, // 0x040
    0x00, // 0x041
    0x00, // 0x042
    0x00, // 0x043
    0x01, // 0x044 deviceType
    0x04, // 0x045 sflashPadType
    0x03, // 0x046 serialClkFreq
    0x00, // 0x047
    0x00, // 0x048 RESERVED
    0x00, // 0x049 RESERVED
    0x00, // 0x04A RESERVED
    0x00, // 0x04B RESERVED
    0x00, // 0x04C RESERVED
    0x00, // 0x04D RESERVED
    0x00, // 0x04E RESERVED
    0x00, // 0x04F RESERVED
    0x00, // 0x050 sflashA1Size
    0x00, // 0x051
    0x20, // 0x052
    0x00, // 0x053
    0x00, // 0x054 sflashA2Size
    0x00, // 0x055
    0x00, // 0x056
    0x00, // 0x057
    0x00, // 0x058 sflashB1Size
    0x00, // 0x059
    0x00, // 0x05A
    0x00, // 0x05B
    0x00, // 0x05C sflashB2Size
    0x00, // 0x05D
    0x00, // 0x05E
    0x00, // 0x05F
    0x00, // 0x060
    0x00, // 0x061
    0x00, // 0x062
    0x00, // 0x063
    0x00, // 0x064
    0x00, // 0x065
    0x00, // 0x066
    0x00, // 0x067
    0x00, // 0x068
    0x00, // 0x069
    0x00, // 0x06A
    0x00, // 0x06B
    0x00, // 0x06C
    0x00, // 0x06D
    0x00, // 0x06E
    0x00, // 0x06F
    0x00, // 0x070
    0x00, // 0x071
    0x00, // 0x072
    0x00, // 0x073
    0x00, // 0x074
    0x00, // 0x075
    0x00, // 0x076
    0x00, // 0x077
    0x00, // 0x078
    0x00, // 0x079
    0x00, // 0x07A
    0x00, // 0x07B
    0x00, // 0x07C
    0x00, // 0x07D
    0x00, // 0x07E
    0x00, // 0x07F
    0xEB, // 0x080 (LUT[0]) READ: OPCODE=CMD_SDR, PADS=SINGLE, OPERAND=0xEB
    0x04, // 0x081
    0x18, // 0x082 (LUT[0]) READ: OPCODE=RADDR_SDR, PADS=QUAD, OPERAND=0x18
    0x0A, // 0x083
    0x06, // 0x084 (LUT[0]) READ: OPCODE=DUMMY_SDR, PADS=QUAD, OPERAND=0x6
    0x32, // 0x085
    0x04, // 0x086 (LUT[0]) READ: OPCODE=READ_SDR, PADS=QUAD, OPERAND=0x4
    0x26, // 0x087
    0x00, // 0x088 (LUT[0]) READ: STOP
    0x00, // 0x089
    0x00, // 0x08A (LUT[0]) READ: STOP
    0x00, // 0x08B
    0x00, // 0x08C (LUT[0]) READ: STOP
    0x00, // 0x08D
    0x00, // 0x08E (LUT[0]) READ: STOP
    0x00, // 0x08F
    0x05, // 0x090 (LUT[1]) READ_STATUS: OPCODE=CMD_SDR, PADS=SINGLE, OPERAND=0x5
    0x04, // 0x091
    0x04, // 0x092 (LUT[1]) READ_STATUS: OPCODE=READ_SDR, PADS=SINGLE, OPERAND=0x4
    0x24, // 0x093
    0x00, // 0x094 (LUT[1]) READ_STATUS: STOP
    0x00, // 0x095
    0x00, // 0x096 (LUT[1]) READ_STATUS: STOP
    0x00, // 0x097
    0x00, // 0x098 (LUT[1]) READ_STATUS: STOP
    0x00, // 0x099
    0x00, // 0x09A (LUT[1]) READ_STATUS: STOP
    0x00, // 0x09B
    0x00, // 0x09C (LUT[1]) READ_STATUS: STOP
    0x00, // 0x09D
    0x00, // 0x09E (LUT[1]) READ_STATUS: STOP
    0x00, // 0x09F
    0x00, // 0x0A0 (LUT[2])
    0x00, // 0x0A1
    0x00, // 0x0A2 (LUT[2])
    0x00, // 0x0A3
    0x00, // 0x0A4 (LUT[2])
    0x00, // 0x0A5
    0x00, // 0x0A6 (LUT[2])
    0x00, // 0x0A7
    0x00, // 0x0A8 (LUT[2])
    0x00, // 0x0A9
    0x00, // 0x0AA (LUT[2])
    0x00, // 0x0AB
    0x00, // 0x0AC (LUT[2])
    0x00, // 0x0AD
    0x00, // 0x0AE (LUT[2])
    0x00, // 0x0AF
    0x06, // 0x0B0 (LUT[3]) WRITE_ENABLE: OPCODE=CMD_SDR, PADS=SINGLE, OPERAND=0x6
    0x04, // 0x0B1
    0x00, // 0x0B2 (LUT[3]) WRITE_ENABLE: STOP
    0x00, // 0x0B3
    0x00, // 0x0B4 (LUT[3]) WRITE_ENABLE: STOP
    0x00, // 0x0B5
    0x00, // 0x0B6 (LUT[3]) WRITE_ENABLE: STOP
    0x00, // 0x0B7
    0x00, // 0x0B8 (LUT[3]) WRITE_ENABLE: STOP
    0x00, // 0x0B9
    0x00, // 0x0BA (LUT[3]) WRITE_ENABLE: STOP
    0x00, // 0x0BB
    0x00, // 0x0BC (LUT[3]) WRITE_ENABLE: STOP
    0x00, // 0x0BD
    0x00, // 0x0BE (LUT[3]) WRITE_ENABLE: STOP
    0x00, // 0x0BF
    0x00, // 0x0C0 (LUT[4])
    0x00, // 0x0C1
    0x00, // 0x0C2 (LUT[4])
    0x00, // 0x0C3
    0x00, // 0x0C4 (LUT[4])
    0x00, // 0x0C5
    0x00, // 0x0C6 (LUT[4])
    0x00, // 0x0C7
    0x00, // 0x0C8 (LUT[4])
    0x00, // 0x0C9
    0x00, // 0x0CA (LUT[4])
    0x00, // 0x0CB
    0x00, // 0x0CC (LUT[4])
    0x00, // 0x0CD
    0x00, // 0x0CE (LUT[4])
    0x00, // 0x0CF
    0x20, // 0x0D0 (LUT[5]) ERASE_SECTOR: OPCODE=CMD_SDR, PADS=SINGLE, OPERAND=0x20
    0x04, // 0x0D1
    0x18, // 0x0D2 (LUT[5]) ERASE_SECTOR: OPCODE=RADDR_SDR, PADS=SINGLE, OPERAND=0x18
    0x08, // 0x0D3
    0x00, // 0x0D4 (LUT[5]) ERASE_SECTOR: STOP
    0x00, // 0x0D5
    0x00, // 0x0D6 (LUT[5]) ERASE_SECTOR: STOP
    0x00, // 0x0D7
    0x00, // 0x0D8 (LUT[5]) ERASE_SECTOR: STOP
    0x00, // 0x0D9
    0x00, // 0x0DA (LUT[5]) ERASE_SECTOR: STOP
    0x00, // 0x0DB
    0x00, // 0x0DC (LUT[5]) ERASE_SECTOR: STOP
    0x00, // 0x0DD
    0x00, // 0x0DE (LUT[5]) ERASE_SECTOR: STOP
    0x00, // 0x0DF
    0x00, // 0x0E0 (LUT[6])
    0x00, // 0x0E1
    0x00, // 0x0E2 (LUT[6])
    0x00, // 0x0E3
    0x00, // 0x0E4 (LUT[6])
    0x00, // 0x0E5
    0x00, // 0x0E6 (LUT[6])
    0x00, // 0x0E7
    0x00, // 0x0E8 (LUT[6])
    0x00, // 0x0E9
    0x00, // 0x0EA (LUT[6])
    0x00, // 0x0EB
    0x00, // 0x0EC (LUT[6])
    0x00, // 0x0ED
    0x00, // 0x0EE (LUT[6])
    0x00, // 0x0EF
    0x00, // 0x0F0 (LUT[7])
    0x00, // 0x0F1
    0x00, // 0x0F2 (LUT[7])
    0x00, // 0x0F3
    0x00, // 0x0F4 (LUT[7])
    0x00, // 0x0F5
    0x00, // 0x0F6 (LUT[7])
    0x00, // 0x0F7
    0x00, // 0x0F8 (LUT[7])
    0x00, // 0x0F9
    0x00, // 0x0FA (LUT[7])
    0x00, // 0x0FB
    0x00, // 0x0FC (LUT[7])
    0x00, // 0x0FD
    0x00, // 0x0FE (LUT[7])
    0x00, // 0x0FF
    0x00, // 0x100 (LUT[8])
    0x00, // 0x101
    0x00, // 0x102 (LUT[8])
    0x00, // 0x103
    0x00, // 0x104 (LUT[8])
    0x00, // 0x105
    0x00, // 0x106 (LUT[8])
    0x00, // 0x107
    0x00, // 0x108 (LUT[8])
    0x00, // 0x109
    0x00, // 0x10A (LUT[8])
    0x00, // 0x10B
    0x00, // 0x10C (LUT[8])
    0x00, // 0x10D
    0x00, // 0x10E (LUT[8])
    0x00, // 0x10F
    0x02, // 0x110 (LUT[9]) PAGE_PROGRAM: OPCODE=CMD_SDR, PADS=SINGLE, OPERAND=0x2
    0x04, // 0x111
    0x18, // 0x112 (LUT[9]) PAGE_PROGRAM: OPCODE=RADDR_SDR, PADS=SINGLE, OPERAND=0x18
    0x08, // 0x113
    0x04, // 0x114 (LUT[9]) PAGE_PROGRAM: OPCODE=WRITE_SDR, PADS=SINGLE, OPERAND=0x4
    0x20, // 0x115
    0x00, // 0x116 (LUT[9]) PAGE_PROGRAM: STOP
    0x00, // 0x117
    0x00, // 0x118 (LUT[9]) PAGE_PROGRAM: STOP
    0x00, // 0x119
    0x00, // 0x11A (LUT[9]) PAGE_PROGRAM: STOP
    0x00, // 0x11B
    0x00, // 0x11C (LUT[9]) PAGE_PROGRAM: STOP
    0x00, // 0x11D
    0x00, // 0x11E (LUT[9]) PAGE_PROGRAM: STOP
    0x00, // 0x11F
    0x00, // 0x120 (LUT[10])
    0x00, // 0x121
    0x00, // 0x122 (LUT[10])
    0x00, // 0x123
    0x00, // 0x124 (LUT[10])
    0x00, // 0x125
    0x00, // 0x126 (LUT[10])
    0x00, // 0x127
    0x00, // 0x128 (LUT[10])
    0x00, // 0x129
    0x00, // 0x12A (LUT[10])
    0x00, // 0x12B
    0x00, // 0x12C (LUT[10])
    0x00, // 0x12D
    0x00, // 0x12E (LUT[10])
    0x00, // 0x12F
    0x60, // 0x130 (LUT[11]) CHIP_ERASE: OPCODE=CMD_SDR, PADS=SINGLE, OPERAND=0x60
    0x04, // 0x131
    0x00, // 0x132 (LUT[11]) CHIP_ERASE: STOP
    0x00, // 0x133
    0x00, // 0x134 (LUT[11]) CHIP_ERASE: STOP
    0x00, // 0x135
    0x00, // 0x136 (LUT[11]) CHIP_ERASE: STOP
    0x00, // 0x137
    0x00, // 0x138 (LUT[11]) CHIP_ERASE: STOP
    0x00, // 0x139
    0x00, // 0x13A (LUT[11]) CHIP_ERASE: STOP
    0x00, // 0x13B
    0x00, // 0x13C (LUT[11]) CHIP_ERASE: STOP
    0x00, // 0x13D
    0x00, // 0x13E (LUT[11]) CHIP_ERASE: STOP
    0x00, // 0x13F
    0x00, // 0x140 (LUT[12])
    0x00, // 0x141
    0x00, // 0x142 (LUT[12])
    0x00, // 0x143
    0x00, // 0x144 (LUT[12])
    0x00, // 0x145
    0x00, // 0x146 (LUT[12])
    0x00, // 0x147
    0x00, // 0x148 (LUT[12])
    0x00, // 0x149
    0x00, // 0x14A (LUT[12])
    0x00, // 0x14B
    0x00, // 0x14C (LUT[12])
    0x00, // 0x14D
    0x00, // 0x14E (LUT[12])
    0x00, // 0x14F
    0x00, // 0x150 (LUT[13])
    0x00, // 0x151
    0x00, // 0x152 (LUT[13])
    0x00, // 0x153
    0x00, // 0x154 (LUT[13])
    0x00, // 0x155
    0x00, // 0x156 (LUT[13])
    0x00, // 0x157
    0x00, // 0x158 (LUT[13])
    0x00, // 0x159
    0x00, // 0x15A (LUT[13])
    0x00, // 0x15B
    0x00, // 0x15C (LUT[13])
    0x00, // 0x15D
    0x00, // 0x15E (LUT[13])
    0x00, // 0x15F
    0x00, // 0x160 (LUT[14])
    0x00, // 0x161
    0x00, // 0x162 (LUT[14])
    0x00, // 0x163
    0x00, // 0x164 (LUT[14])
    0x00, // 0x165
    0x00, // 0x166 (LUT[14])
    0x00, // 0x167
    0x00, // 0x168 (LUT[14])
    0x00, // 0x169
    0x00, // 0x16A (LUT[14])
    0x00, // 0x16B
    0x00, // 0x16C (LUT[14])
    0x00, // 0x16D
    0x00, // 0x16E (LUT[14])
    0x00, // 0x16F
    0x00, // 0x170 (LUT[15]) DUMMY: STOP
    0x00, // 0x171
    0x00, // 0x172 (LUT[15]) DUMMY: STOP
    0x00, // 0x173
    0x00, // 0x174 (LUT[15]) DUMMY: STOP
    0x00, // 0x175
    0x00, // 0x176 (LUT[15]) DUMMY: STOP
    0x00, // 0x177
    0x00, // 0x178 (LUT[15]) DUMMY: STOP
    0x00, // 0x179
    0x00, // 0x17A (LUT[15]) DUMMY: STOP
    0x00, // 0x17B
    0x00, // 0x17C (LUT[15]) DUMMY: STOP
    0x00, // 0x17D
    0x00, // 0x17E (LUT[15]) DUMMY: STOP
    0x00, // 0x17F
    0x00, // 0x180
    0x00, // 0x181
    0x00, // 0x182
    0x00, // 0x183
    0x00, // 0x184
    0x00, // 0x185
    0x00, // 0x186
    0x00, // 0x187
    0x00, // 0x188
    0x00, // 0x189
    0x00, // 0x18A
    0x00, // 0x18B
    0x00, // 0x18C
    0x00, // 0x18D
    0x00, // 0x18E
    0x00, // 0x18F
    0x00, // 0x190
    0x00, // 0x191
    0x00, // 0x192
    0x00, // 0x193
    0x00, // 0x194
    0x00, // 0x195
    0x00, // 0x196
    0x00, // 0x197
    0x00, // 0x198
    0x00, // 0x199
    0x00, // 0x19A
    0x00, // 0x19B
    0x00, // 0x19C
    0x00, // 0x19D
    0x00, // 0x19E
    0x00, // 0x19F
    0x00, // 0x1A0
    0x00, // 0x1A1
    0x00, // 0x1A2
    0x00, // 0x1A3
    0x00, // 0x1A4
    0x00, // 0x1A5
    0x00, // 0x1A6
    0x00, // 0x1A7
    0x00, // 0x1A8
    0x00, // 0x1A9
    0x00, // 0x1AA
    0x00, // 0x1AB
    0x00, // 0x1AC
    0x00, // 0x1AD
    0x00, // 0x1AE
    0x00, // 0x1AF
    0x00, // 0x1B0 RESERVED
    0x00, // 0x1B1 RESERVED
    0x00, // 0x1B2 RESERVED
    0x00, // 0x1B3 RESERVED
    0x00, // 0x1B4 RESERVED
    0x00, // 0x1B5 RESERVED
    0x00, // 0x1B6 RESERVED
    0x00, // 0x1B7 RESERVED
    0x00, // 0x1B8 RESERVED
    0x00, // 0x1B9 RESERVED
    0x00, // 0x1BA RESERVED
    0x00, // 0x1BB RESERVED
    0x00, // 0x1BC RESERVED
    0x00, // 0x1BD RESERVED
    0x00, // 0x1BE RESERVED
    0x00, // 0x1BF RESERVED
    0x00, // 0x1C0 pageSize
    0x01, // 0x1C1
    0x00, // 0x1C2
    0x00, // 0x1C3
    0x00, // 0x1C4 sectorSize
    0x10, // 0x1C5
    0x00, // 0x1C6
    0x00, // 0x1C7
    0x01, // 0x1C8 ipCmdSerialClkFreq
    0x00, // 0x1C9
    0x00, // 0x1CA
    0x00, // 0x1CB
    0x00, // 0x1CC RESERVED
    0x00, // 0x1CD RESERVED
    0x00, // 0x1CE RESERVED
    0x00, // 0x1CF RESERVED
    0x00, // 0x1D0 RESERVED
    0x00, // 0x1D1 RESERVED
    0x00, // 0x1D2 RESERVED
    0x00, // 0x1D3 RESERVED
    0x00, // 0x1D4 RESERVED
    0x00, // 0x1D5 RESERVED
    0x00, // 0x1D6 RESERVED
    0x00, // 0x1D7 RESERVED
    0x00, // 0x1D8 RESERVED
    0x00, // 0x1D9 RESERVED
    0x00, // 0x1DA RESERVED
    0x00, // 0x1DB RESERVED
    0x00, // 0x1DC RESERVED
    0x00, // 0x1DD RESERVED
    0x00, // 0x1DE RESERVED
    0x00, // 0x1DF RESERVED
    0x00, // 0x1E0 RESERVED
    0x00, // 0x1E1 RESERVED
    0x00, // 0x1E2 RESERVED
    0x00, // 0x1E3 RESERVED
    0x00, // 0x1E4 RESERVED
    0x00, // 0x1E5 RESERVED
    0x00, // 0x1E6 RESERVED
    0x00, // 0x1E7 RESERVED
    0x00, // 0x1E8 RESERVED
    0x00, // 0x1E9 RESERVED
    0x00, // 0x1EA RESERVED
    0x00, // 0x1EB RESERVED
    0x00, // 0x1EC RESERVED
    0x00, // 0x1ED RESERVED
    0x00, // 0x1EE RESERVED
    0x00, // 0x1EF RESERVED
    0x00, // 0x1F0 RESERVED
    0x00, // 0x1F1 RESERVED
    0x00, // 0x1F2 RESERVED
    0x00, // 0x1F3 RESERVED
    0x00, // 0x1F4 RESERVED
    0x00, // 0x1F5 RESERVED
    0x00, // 0x1F6 RESERVED
    0x00, // 0x1F7 RESERVED
    0x00, // 0x1F8 RESERVED
    0x00, // 0x1F9 RESERVED
    0x00, // 0x1FA RESERVED
    0x00, // 0x1FB RESERVED
    0x00, // 0x1FC RESERVED
    0x00, // 0x1FD RESERVED
    0x00, // 0x1FE RESERVED
    0x00, // 0x1FF RESERVED
];
