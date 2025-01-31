extern crate stream_cipher;
extern crate block_cipher_trait;
extern crate salsa20_family;

use block_cipher_trait::generic_array::GenericArray;
use salsa20_family::Salsa20;
use stream_cipher::NewStreamCipher;
use stream_cipher::StreamCipher;
use stream_cipher::SyncStreamCipherSeek;

#[cfg(test)]
const KEY_BYTES: usize = 32;

#[cfg(test)]
const IV_BYTES: usize = 8;

#[cfg(test)]
const KEY0: [u8; KEY_BYTES] =
    [ 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00 ];

#[cfg(test)]
const KEY1: [u8; KEY_BYTES] =
    [ 0x80, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00 ];

#[cfg(test)]
const KEY_LONG: [u8; KEY_BYTES] =
    [ 1, 2, 3, 4,
      5, 6, 7, 8,
      9, 10, 11, 12,
      13, 14, 15, 16,
      17, 18, 19, 20,
      21, 22, 23, 24,
      25, 26, 27, 28,
      29, 30, 31, 32 ];

#[cfg(test)]
const IV0: [u8; IV_BYTES] = [0; IV_BYTES];

#[cfg(test)]
const IV1: [u8; IV_BYTES] =
    [ 0x80, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00 ];

#[cfg(test)]
const IVHI: [u8; IV_BYTES] =
    [ 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x01 ];

#[cfg(test)]
const IV_LONG: [u8; IV_BYTES] =
    [ 3, 1, 4, 1,
      5, 9, 2, 6 ];

#[cfg(test)]
const EXPECTED_KEY1_IV0: [u8; 64] = [
    0xe3, 0xbe, 0x8f, 0xdd,
    0x8b, 0xec, 0xa2, 0xe3,
    0xea, 0x8e, 0xf9, 0x47,
    0x5b, 0x29, 0xa6, 0xe7,
    0x00, 0x39, 0x51, 0xe1,
    0x09, 0x7a, 0x5c, 0x38,
    0xd2, 0x3b, 0x7a, 0x5f,
    0xad, 0x9f, 0x68, 0x44,
    0xb2, 0x2c, 0x97, 0x55,
    0x9e, 0x27, 0x23, 0xc7,
    0xcb, 0xbd, 0x3f, 0xe4,
    0xfc, 0x8d, 0x9a, 0x07,
    0x44, 0x65, 0x2a, 0x83,
    0xe7, 0x2a, 0x9c, 0x46,
    0x18, 0x76, 0xaf, 0x4d,
    0x7e, 0xf1, 0xa1, 0x17
];

#[cfg(test)]
const EXPECTED_KEY0_IV1: [u8; 64] = [
    0x2a, 0xba, 0x3d, 0xc4,
    0x5b, 0x49, 0x47, 0x00,
    0x7b, 0x14, 0xc8, 0x51,
    0xcd, 0x69, 0x44, 0x56,
    0xb3, 0x03, 0xad, 0x59,
    0xa4, 0x65, 0x66, 0x28,
    0x03, 0x00, 0x67, 0x05,
    0x67, 0x3d, 0x6c, 0x3e,
    0x29, 0xf1, 0xd3, 0x51,
    0x0d, 0xfc, 0x04, 0x05,
    0x46, 0x3c, 0x03, 0x41,
    0x4e, 0x0e, 0x07, 0xe3,
    0x59, 0xf1, 0xf1, 0x81,
    0x6c, 0x68, 0xb2, 0x43,
    0x4a, 0x19, 0xd3, 0xee,
    0xe0, 0x46, 0x48, 0x73
];

#[cfg(test)]
const EXPECTED_KEY0_IVHI: [u8; 64] = [
    0xb4, 0x7f, 0x96, 0xaa,
    0x96, 0x78, 0x61, 0x35,
    0x29, 0x7a, 0x3c, 0x4e,
    0xc5, 0x6a, 0x61, 0x3d,
    0x0b, 0x80, 0x09, 0x53,
    0x24, 0xff, 0x43, 0x23,
    0x9d, 0x68, 0x4c, 0x57,
    0xff, 0xe4, 0x2e, 0x1c,
    0x44, 0xf3, 0xcc, 0x01,
    0x16, 0x13, 0xdb, 0x6c,
    0xdc, 0x88, 0x09, 0x99,
    0xa1, 0xe6, 0x5a, 0xed,
    0x12, 0x87, 0xfc, 0xb1,
    0x1c, 0x83, 0x9c, 0x37,
    0x12, 0x07, 0x65, 0xaf,
    0xa7, 0x3e, 0x50, 0x75
];

#[cfg(test)]
const EXPECTED_LONG: [u8; 256] = [
    0x6e, 0xbc, 0xbd, 0xbf,
    0x76, 0xfc, 0xcc, 0x64,
    0xab, 0x05, 0x54, 0x2b,
    0xee, 0x8a, 0x67, 0xcb,
    0xc2, 0x8f, 0xa2, 0xe1,
    0x41, 0xfb, 0xef, 0xbb,
    0x3a, 0x2f, 0x9b, 0x22,
    0x19, 0x09, 0xc8, 0xd7,
    0xd4, 0x29, 0x52, 0x58,
    0xcb, 0x53, 0x97, 0x70,
    0xdd, 0x24, 0xd7, 0xac,
    0x34, 0x43, 0x76, 0x9f,
    0xfa, 0x27, 0xa5, 0x0e,
    0x60, 0x64, 0x42, 0x64,
    0xdc, 0x8b, 0x6b, 0x61,
    0x26, 0x83, 0x37, 0x2e,
    0x08, 0x5d, 0x0a, 0x12,
    0xbf, 0x24, 0x0b, 0x18,
    0x9c, 0xe2, 0xb7, 0x82,
    0x89, 0x86, 0x2b, 0x56,
    0xfd, 0xc9, 0xfc, 0xff,
    0xc3, 0x3b, 0xef, 0x93,
    0x25, 0xa2, 0xe8, 0x1b,
    0x98, 0xfb, 0x3f, 0xb9,
    0xaa, 0x04, 0xcf, 0x43,
    0x46, 0x15, 0xce, 0xff,
    0xeb, 0x98, 0x5c, 0x1c,
    0xb0, 0x8d, 0x84, 0x40,
    0xe9, 0x0b, 0x1d, 0x56,
    0xdd, 0xea, 0xea, 0x16,
    0xd9, 0xe1, 0x5a, 0xff,
    0xff, 0x1f, 0x69, 0x8c,
    0x48, 0x3c, 0x7a, 0x46,
    0x6a, 0xf1, 0xfe, 0x06,
    0x25, 0x74, 0xad, 0xfd,
    0x2b, 0x06, 0xa6, 0x2b,
    0x4d, 0x98, 0x44, 0x07,
    0x19, 0xea, 0x77, 0x63,
    0x85, 0xc4, 0x70, 0x34,
    0x9a, 0x7e, 0xd6, 0x96,
    0x95, 0x83, 0x46, 0x3e,
    0xd5, 0xd2, 0x6b, 0x8f,
    0xef, 0xcc, 0xb2, 0x05,
    0xda, 0x0f, 0x5b, 0xfa,
    0x98, 0xc7, 0x78, 0x12,
    0xfe, 0x75, 0x6b, 0x09,
    0xea, 0xcc, 0x28, 0x2a,
    0xa4, 0x2f, 0x4b, 0xaf,
    0xa7, 0x96, 0x33, 0x18,
    0x90, 0x46, 0xe2, 0xb2,
    0x0f, 0x35, 0xb3, 0xe0,
    0xe5, 0x4a, 0xa3, 0xb9,
    0x29, 0xe2, 0x3c, 0x0f,
    0x47, 0xdc, 0x7b, 0xcd,
    0x4f, 0x92, 0x8b, 0x2a,
    0x97, 0x64, 0xbe, 0x7d,
    0x4b, 0x8a, 0x50, 0xf9,
    0x80, 0xa5, 0x0b, 0x35,
    0xad, 0x80, 0x87, 0x37,
    0x5e, 0x0c, 0x55, 0x6e,
    0xcb, 0xe6, 0xa7, 0x16,
    0x1e, 0x86, 0x53, 0xce,
    0x93, 0x91, 0xe1, 0xe6,
    0x71, 0x0e, 0xd4, 0xf1
];

#[test]
fn salsa20_KEY1_IV0() {
    let mut cipher = Salsa20::new(&GenericArray::from(KEY1),
                                  &GenericArray::from(IV0));
    let mut buf = [0; 64];

    cipher.encrypt(&mut buf);

    for i in 0 .. 64 {
        assert_eq!(buf[i], EXPECTED_KEY1_IV0[i])
    }
}

#[test]
fn salsa20_KEY0_IV1() {
    let mut cipher = Salsa20::new(&GenericArray::from(KEY0),
                                  &GenericArray::from(IV1));
    let mut buf = [0; 64];

    cipher.encrypt(&mut buf);

    for i in 0 .. 64 {
        assert_eq!(buf[i], EXPECTED_KEY0_IV1[i])
    }
}

#[test]
fn salsa20_KEY0_IVHI() {
    let mut cipher = Salsa20::new(&GenericArray::from(KEY0),
                                  &GenericArray::from(IVHI));
    let mut buf = [0; 64];

    cipher.encrypt(&mut buf);

    for i in 0 .. 64 {
        assert_eq!(buf[i], EXPECTED_KEY0_IVHI[i])
    }
}

#[test]
fn salsa20_LONG() {
    let mut cipher = Salsa20::new(&GenericArray::from(KEY_LONG),
                                  &GenericArray::from(IV_LONG));
    let mut buf = [0; 256];

    cipher.encrypt(&mut buf);

    for i in 0 .. 256 {
        assert_eq!(buf[i], EXPECTED_LONG[i])
    }
}

#[test]
fn salsa20_offsets() {
    for idx in 0 .. 256 {
        for middle in idx .. 256 {
            for last in middle .. 256 {
                let mut cipher = Salsa20::new(&GenericArray::from(KEY_LONG),
                                              &GenericArray::from(IV_LONG));
                let mut buf = [0; 256];

                cipher.seek(idx as u64);
                cipher.encrypt(&mut buf[idx .. middle]);
                cipher.encrypt(&mut buf[middle .. last]);

                for k in idx .. last {
                    assert_eq!(buf[k], EXPECTED_LONG[k])
                }
            }
        }
    }
}
