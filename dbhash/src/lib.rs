#![feature(c_variadic)]
#![allow(unused_imports, dead_code)]
mod sqlite3_h;
pub(crate) use crate::sqlite3_h::*;
type DarwinSizeT = u64;
#[repr(C)]
#[derive(Copy, Clone)]
struct SHA1Context {
    state: [u32; 5],
    count: [u32; 2],
    buffer: [u8; 64],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct GlobalVars {
    z_argv0: *const i8,
    f_debug: u32,
    db: *mut Sqlite3,
    cx: SHA1Context,
}
#[unsafe(no_mangle)]
pub static mut g: GlobalVars = unsafe { core::mem::zeroed() };
#[unsafe(no_mangle)]
pub extern "C" fn sha1_transform(state: *mut u32, buffer: *const u8) -> () {
    unsafe {
        let mut qq: [u32; 5] = [0; 5];
        let mut block: [u32; 16] = [0; 16];
        unsafe {
            memcpy(&raw mut block[0 as usize] as *mut u32 as *mut (),
                buffer as *const (), 64 as u64)
        };
        unsafe {
            memcpy(&raw mut qq[0 as usize] as *mut u32 as *mut (),
                state as *const (),
                5 as u64 * core::mem::size_of::<u32>() as u64)
        };
        if 1 == unsafe { *(&raw mut one as *mut u8) } as i32 {
            qq[4 as usize] +=
                (qq[1 as usize] & (qq[2 as usize] ^ qq[3 as usize]) ^
                                qq[3 as usize]) +
                            {
                                block[0 as usize] =
                                    (block[0 as usize] << 32 - 8 | block[0 as usize] >> 8) &
                                            4278255360u32 |
                                        (block[0 as usize] << 8 | block[0 as usize] >> 32 - 8) &
                                            16711935 as u32;
                                block[0 as usize]
                            } + 1518500249 as u32 +
                    (qq[0 as usize] << 5 | qq[0 as usize] >> 32 - 5);
            qq[1 as usize] = qq[1 as usize] << 32 - 2 | qq[1 as usize] >> 2;
            qq[3 as usize] +=
                (qq[0 as usize] & (qq[1 as usize] ^ qq[2 as usize]) ^
                                qq[2 as usize]) +
                            {
                                block[1 as usize] =
                                    (block[1 as usize] << 32 - 8 | block[1 as usize] >> 8) &
                                            4278255360u32 |
                                        (block[1 as usize] << 8 | block[1 as usize] >> 32 - 8) &
                                            16711935 as u32;
                                block[1 as usize]
                            } + 1518500249 as u32 +
                    (qq[4 as usize] << 5 | qq[4 as usize] >> 32 - 5);
            qq[0 as usize] = qq[0 as usize] << 32 - 2 | qq[0 as usize] >> 2;
            qq[2 as usize] +=
                (qq[4 as usize] & (qq[0 as usize] ^ qq[1 as usize]) ^
                                qq[1 as usize]) +
                            {
                                block[2 as usize] =
                                    (block[2 as usize] << 32 - 8 | block[2 as usize] >> 8) &
                                            4278255360u32 |
                                        (block[2 as usize] << 8 | block[2 as usize] >> 32 - 8) &
                                            16711935 as u32;
                                block[2 as usize]
                            } + 1518500249 as u32 +
                    (qq[3 as usize] << 5 | qq[3 as usize] >> 32 - 5);
            qq[4 as usize] = qq[4 as usize] << 32 - 2 | qq[4 as usize] >> 2;
            qq[1 as usize] +=
                (qq[3 as usize] & (qq[4 as usize] ^ qq[0 as usize]) ^
                                qq[0 as usize]) +
                            {
                                block[3 as usize] =
                                    (block[3 as usize] << 32 - 8 | block[3 as usize] >> 8) &
                                            4278255360u32 |
                                        (block[3 as usize] << 8 | block[3 as usize] >> 32 - 8) &
                                            16711935 as u32;
                                block[3 as usize]
                            } + 1518500249 as u32 +
                    (qq[2 as usize] << 5 | qq[2 as usize] >> 32 - 5);
            qq[3 as usize] = qq[3 as usize] << 32 - 2 | qq[3 as usize] >> 2;
            qq[0 as usize] +=
                (qq[2 as usize] & (qq[3 as usize] ^ qq[4 as usize]) ^
                                qq[4 as usize]) +
                            {
                                block[4 as usize] =
                                    (block[4 as usize] << 32 - 8 | block[4 as usize] >> 8) &
                                            4278255360u32 |
                                        (block[4 as usize] << 8 | block[4 as usize] >> 32 - 8) &
                                            16711935 as u32;
                                block[4 as usize]
                            } + 1518500249 as u32 +
                    (qq[1 as usize] << 5 | qq[1 as usize] >> 32 - 5);
            qq[2 as usize] = qq[2 as usize] << 32 - 2 | qq[2 as usize] >> 2;
            qq[4 as usize] +=
                (qq[1 as usize] & (qq[2 as usize] ^ qq[3 as usize]) ^
                                qq[3 as usize]) +
                            {
                                block[5 as usize] =
                                    (block[5 as usize] << 32 - 8 | block[5 as usize] >> 8) &
                                            4278255360u32 |
                                        (block[5 as usize] << 8 | block[5 as usize] >> 32 - 8) &
                                            16711935 as u32;
                                block[5 as usize]
                            } + 1518500249 as u32 +
                    (qq[0 as usize] << 5 | qq[0 as usize] >> 32 - 5);
            qq[1 as usize] = qq[1 as usize] << 32 - 2 | qq[1 as usize] >> 2;
            qq[3 as usize] +=
                (qq[0 as usize] & (qq[1 as usize] ^ qq[2 as usize]) ^
                                qq[2 as usize]) +
                            {
                                block[6 as usize] =
                                    (block[6 as usize] << 32 - 8 | block[6 as usize] >> 8) &
                                            4278255360u32 |
                                        (block[6 as usize] << 8 | block[6 as usize] >> 32 - 8) &
                                            16711935 as u32;
                                block[6 as usize]
                            } + 1518500249 as u32 +
                    (qq[4 as usize] << 5 | qq[4 as usize] >> 32 - 5);
            qq[0 as usize] = qq[0 as usize] << 32 - 2 | qq[0 as usize] >> 2;
            qq[2 as usize] +=
                (qq[4 as usize] & (qq[0 as usize] ^ qq[1 as usize]) ^
                                qq[1 as usize]) +
                            {
                                block[7 as usize] =
                                    (block[7 as usize] << 32 - 8 | block[7 as usize] >> 8) &
                                            4278255360u32 |
                                        (block[7 as usize] << 8 | block[7 as usize] >> 32 - 8) &
                                            16711935 as u32;
                                block[7 as usize]
                            } + 1518500249 as u32 +
                    (qq[3 as usize] << 5 | qq[3 as usize] >> 32 - 5);
            qq[4 as usize] = qq[4 as usize] << 32 - 2 | qq[4 as usize] >> 2;
            qq[1 as usize] +=
                (qq[3 as usize] & (qq[4 as usize] ^ qq[0 as usize]) ^
                                qq[0 as usize]) +
                            {
                                block[8 as usize] =
                                    (block[8 as usize] << 32 - 8 | block[8 as usize] >> 8) &
                                            4278255360u32 |
                                        (block[8 as usize] << 8 | block[8 as usize] >> 32 - 8) &
                                            16711935 as u32;
                                block[8 as usize]
                            } + 1518500249 as u32 +
                    (qq[2 as usize] << 5 | qq[2 as usize] >> 32 - 5);
            qq[3 as usize] = qq[3 as usize] << 32 - 2 | qq[3 as usize] >> 2;
            qq[0 as usize] +=
                (qq[2 as usize] & (qq[3 as usize] ^ qq[4 as usize]) ^
                                qq[4 as usize]) +
                            {
                                block[9 as usize] =
                                    (block[9 as usize] << 32 - 8 | block[9 as usize] >> 8) &
                                            4278255360u32 |
                                        (block[9 as usize] << 8 | block[9 as usize] >> 32 - 8) &
                                            16711935 as u32;
                                block[9 as usize]
                            } + 1518500249 as u32 +
                    (qq[1 as usize] << 5 | qq[1 as usize] >> 32 - 5);
            qq[2 as usize] = qq[2 as usize] << 32 - 2 | qq[2 as usize] >> 2;
            qq[4 as usize] +=
                (qq[1 as usize] & (qq[2 as usize] ^ qq[3 as usize]) ^
                                qq[3 as usize]) +
                            {
                                block[10 as usize] =
                                    (block[10 as usize] << 32 - 8 | block[10 as usize] >> 8) &
                                            4278255360u32 |
                                        (block[10 as usize] << 8 | block[10 as usize] >> 32 - 8) &
                                            16711935 as u32;
                                block[10 as usize]
                            } + 1518500249 as u32 +
                    (qq[0 as usize] << 5 | qq[0 as usize] >> 32 - 5);
            qq[1 as usize] = qq[1 as usize] << 32 - 2 | qq[1 as usize] >> 2;
            qq[3 as usize] +=
                (qq[0 as usize] & (qq[1 as usize] ^ qq[2 as usize]) ^
                                qq[2 as usize]) +
                            {
                                block[11 as usize] =
                                    (block[11 as usize] << 32 - 8 | block[11 as usize] >> 8) &
                                            4278255360u32 |
                                        (block[11 as usize] << 8 | block[11 as usize] >> 32 - 8) &
                                            16711935 as u32;
                                block[11 as usize]
                            } + 1518500249 as u32 +
                    (qq[4 as usize] << 5 | qq[4 as usize] >> 32 - 5);
            qq[0 as usize] = qq[0 as usize] << 32 - 2 | qq[0 as usize] >> 2;
            qq[2 as usize] +=
                (qq[4 as usize] & (qq[0 as usize] ^ qq[1 as usize]) ^
                                qq[1 as usize]) +
                            {
                                block[12 as usize] =
                                    (block[12 as usize] << 32 - 8 | block[12 as usize] >> 8) &
                                            4278255360u32 |
                                        (block[12 as usize] << 8 | block[12 as usize] >> 32 - 8) &
                                            16711935 as u32;
                                block[12 as usize]
                            } + 1518500249 as u32 +
                    (qq[3 as usize] << 5 | qq[3 as usize] >> 32 - 5);
            qq[4 as usize] = qq[4 as usize] << 32 - 2 | qq[4 as usize] >> 2;
            qq[1 as usize] +=
                (qq[3 as usize] & (qq[4 as usize] ^ qq[0 as usize]) ^
                                qq[0 as usize]) +
                            {
                                block[13 as usize] =
                                    (block[13 as usize] << 32 - 8 | block[13 as usize] >> 8) &
                                            4278255360u32 |
                                        (block[13 as usize] << 8 | block[13 as usize] >> 32 - 8) &
                                            16711935 as u32;
                                block[13 as usize]
                            } + 1518500249 as u32 +
                    (qq[2 as usize] << 5 | qq[2 as usize] >> 32 - 5);
            qq[3 as usize] = qq[3 as usize] << 32 - 2 | qq[3 as usize] >> 2;
            qq[0 as usize] +=
                (qq[2 as usize] & (qq[3 as usize] ^ qq[4 as usize]) ^
                                qq[4 as usize]) +
                            {
                                block[14 as usize] =
                                    (block[14 as usize] << 32 - 8 | block[14 as usize] >> 8) &
                                            4278255360u32 |
                                        (block[14 as usize] << 8 | block[14 as usize] >> 32 - 8) &
                                            16711935 as u32;
                                block[14 as usize]
                            } + 1518500249 as u32 +
                    (qq[1 as usize] << 5 | qq[1 as usize] >> 32 - 5);
            qq[2 as usize] = qq[2 as usize] << 32 - 2 | qq[2 as usize] >> 2;
            qq[4 as usize] +=
                (qq[1 as usize] & (qq[2 as usize] ^ qq[3 as usize]) ^
                                qq[3 as usize]) +
                            {
                                block[15 as usize] =
                                    (block[15 as usize] << 32 - 8 | block[15 as usize] >> 8) &
                                            4278255360u32 |
                                        (block[15 as usize] << 8 | block[15 as usize] >> 32 - 8) &
                                            16711935 as u32;
                                block[15 as usize]
                            } + 1518500249 as u32 +
                    (qq[0 as usize] << 5 | qq[0 as usize] >> 32 - 5);
            qq[1 as usize] = qq[1 as usize] << 32 - 2 | qq[1 as usize] >> 2;
        } else {
            qq[4 as usize] +=
                (qq[1 as usize] & (qq[2 as usize] ^ qq[3 as usize]) ^
                                qq[3 as usize]) + block[0 as usize] + 1518500249 as u32 +
                    (qq[0 as usize] << 5 | qq[0 as usize] >> 32 - 5);
            qq[1 as usize] = qq[1 as usize] << 32 - 2 | qq[1 as usize] >> 2;
            qq[3 as usize] +=
                (qq[0 as usize] & (qq[1 as usize] ^ qq[2 as usize]) ^
                                qq[2 as usize]) + block[1 as usize] + 1518500249 as u32 +
                    (qq[4 as usize] << 5 | qq[4 as usize] >> 32 - 5);
            qq[0 as usize] = qq[0 as usize] << 32 - 2 | qq[0 as usize] >> 2;
            qq[2 as usize] +=
                (qq[4 as usize] & (qq[0 as usize] ^ qq[1 as usize]) ^
                                qq[1 as usize]) + block[2 as usize] + 1518500249 as u32 +
                    (qq[3 as usize] << 5 | qq[3 as usize] >> 32 - 5);
            qq[4 as usize] = qq[4 as usize] << 32 - 2 | qq[4 as usize] >> 2;
            qq[1 as usize] +=
                (qq[3 as usize] & (qq[4 as usize] ^ qq[0 as usize]) ^
                                qq[0 as usize]) + block[3 as usize] + 1518500249 as u32 +
                    (qq[2 as usize] << 5 | qq[2 as usize] >> 32 - 5);
            qq[3 as usize] = qq[3 as usize] << 32 - 2 | qq[3 as usize] >> 2;
            qq[0 as usize] +=
                (qq[2 as usize] & (qq[3 as usize] ^ qq[4 as usize]) ^
                                qq[4 as usize]) + block[4 as usize] + 1518500249 as u32 +
                    (qq[1 as usize] << 5 | qq[1 as usize] >> 32 - 5);
            qq[2 as usize] = qq[2 as usize] << 32 - 2 | qq[2 as usize] >> 2;
            qq[4 as usize] +=
                (qq[1 as usize] & (qq[2 as usize] ^ qq[3 as usize]) ^
                                qq[3 as usize]) + block[5 as usize] + 1518500249 as u32 +
                    (qq[0 as usize] << 5 | qq[0 as usize] >> 32 - 5);
            qq[1 as usize] = qq[1 as usize] << 32 - 2 | qq[1 as usize] >> 2;
            qq[3 as usize] +=
                (qq[0 as usize] & (qq[1 as usize] ^ qq[2 as usize]) ^
                                qq[2 as usize]) + block[6 as usize] + 1518500249 as u32 +
                    (qq[4 as usize] << 5 | qq[4 as usize] >> 32 - 5);
            qq[0 as usize] = qq[0 as usize] << 32 - 2 | qq[0 as usize] >> 2;
            qq[2 as usize] +=
                (qq[4 as usize] & (qq[0 as usize] ^ qq[1 as usize]) ^
                                qq[1 as usize]) + block[7 as usize] + 1518500249 as u32 +
                    (qq[3 as usize] << 5 | qq[3 as usize] >> 32 - 5);
            qq[4 as usize] = qq[4 as usize] << 32 - 2 | qq[4 as usize] >> 2;
            qq[1 as usize] +=
                (qq[3 as usize] & (qq[4 as usize] ^ qq[0 as usize]) ^
                                qq[0 as usize]) + block[8 as usize] + 1518500249 as u32 +
                    (qq[2 as usize] << 5 | qq[2 as usize] >> 32 - 5);
            qq[3 as usize] = qq[3 as usize] << 32 - 2 | qq[3 as usize] >> 2;
            qq[0 as usize] +=
                (qq[2 as usize] & (qq[3 as usize] ^ qq[4 as usize]) ^
                                qq[4 as usize]) + block[9 as usize] + 1518500249 as u32 +
                    (qq[1 as usize] << 5 | qq[1 as usize] >> 32 - 5);
            qq[2 as usize] = qq[2 as usize] << 32 - 2 | qq[2 as usize] >> 2;
            qq[4 as usize] +=
                (qq[1 as usize] & (qq[2 as usize] ^ qq[3 as usize]) ^
                                qq[3 as usize]) + block[10 as usize] + 1518500249 as u32 +
                    (qq[0 as usize] << 5 | qq[0 as usize] >> 32 - 5);
            qq[1 as usize] = qq[1 as usize] << 32 - 2 | qq[1 as usize] >> 2;
            qq[3 as usize] +=
                (qq[0 as usize] & (qq[1 as usize] ^ qq[2 as usize]) ^
                                qq[2 as usize]) + block[11 as usize] + 1518500249 as u32 +
                    (qq[4 as usize] << 5 | qq[4 as usize] >> 32 - 5);
            qq[0 as usize] = qq[0 as usize] << 32 - 2 | qq[0 as usize] >> 2;
            qq[2 as usize] +=
                (qq[4 as usize] & (qq[0 as usize] ^ qq[1 as usize]) ^
                                qq[1 as usize]) + block[12 as usize] + 1518500249 as u32 +
                    (qq[3 as usize] << 5 | qq[3 as usize] >> 32 - 5);
            qq[4 as usize] = qq[4 as usize] << 32 - 2 | qq[4 as usize] >> 2;
            qq[1 as usize] +=
                (qq[3 as usize] & (qq[4 as usize] ^ qq[0 as usize]) ^
                                qq[0 as usize]) + block[13 as usize] + 1518500249 as u32 +
                    (qq[2 as usize] << 5 | qq[2 as usize] >> 32 - 5);
            qq[3 as usize] = qq[3 as usize] << 32 - 2 | qq[3 as usize] >> 2;
            qq[0 as usize] +=
                (qq[2 as usize] & (qq[3 as usize] ^ qq[4 as usize]) ^
                                qq[4 as usize]) + block[14 as usize] + 1518500249 as u32 +
                    (qq[1 as usize] << 5 | qq[1 as usize] >> 32 - 5);
            qq[2 as usize] = qq[2 as usize] << 32 - 2 | qq[2 as usize] >> 2;
            qq[4 as usize] +=
                (qq[1 as usize] & (qq[2 as usize] ^ qq[3 as usize]) ^
                                qq[3 as usize]) + block[15 as usize] + 1518500249 as u32 +
                    (qq[0 as usize] << 5 | qq[0 as usize] >> 32 - 5);
            qq[1 as usize] = qq[1 as usize] << 32 - 2 | qq[1 as usize] >> 2;
        }
        qq[3 as usize] +=
            (qq[0 as usize] & (qq[1 as usize] ^ qq[2 as usize]) ^
                            qq[2 as usize]) +
                        {
                            block[(16 & 15) as usize] =
                                (block[(16 + 13 & 15) as usize] ^
                                                    block[(16 + 8 & 15) as usize] ^
                                                block[(16 + 2 & 15) as usize] ^ block[(16 & 15) as usize])
                                        << 1 |
                                    (block[(16 + 13 & 15) as usize] ^
                                                    block[(16 + 8 & 15) as usize] ^
                                                block[(16 + 2 & 15) as usize] ^ block[(16 & 15) as usize])
                                        >> 32 - 1;
                            block[(16 & 15) as usize]
                        } + 1518500249 as u32 +
                (qq[4 as usize] << 5 | qq[4 as usize] >> 32 - 5);
        qq[0 as usize] = qq[0 as usize] << 32 - 2 | qq[0 as usize] >> 2;
        qq[2 as usize] +=
            (qq[4 as usize] & (qq[0 as usize] ^ qq[1 as usize]) ^
                            qq[1 as usize]) +
                        {
                            block[(17 & 15) as usize] =
                                (block[(17 + 13 & 15) as usize] ^
                                                    block[(17 + 8 & 15) as usize] ^
                                                block[(17 + 2 & 15) as usize] ^ block[(17 & 15) as usize])
                                        << 1 |
                                    (block[(17 + 13 & 15) as usize] ^
                                                    block[(17 + 8 & 15) as usize] ^
                                                block[(17 + 2 & 15) as usize] ^ block[(17 & 15) as usize])
                                        >> 32 - 1;
                            block[(17 & 15) as usize]
                        } + 1518500249 as u32 +
                (qq[3 as usize] << 5 | qq[3 as usize] >> 32 - 5);
        qq[4 as usize] = qq[4 as usize] << 32 - 2 | qq[4 as usize] >> 2;
        qq[1 as usize] +=
            (qq[3 as usize] & (qq[4 as usize] ^ qq[0 as usize]) ^
                            qq[0 as usize]) +
                        {
                            block[(18 & 15) as usize] =
                                (block[(18 + 13 & 15) as usize] ^
                                                    block[(18 + 8 & 15) as usize] ^
                                                block[(18 + 2 & 15) as usize] ^ block[(18 & 15) as usize])
                                        << 1 |
                                    (block[(18 + 13 & 15) as usize] ^
                                                    block[(18 + 8 & 15) as usize] ^
                                                block[(18 + 2 & 15) as usize] ^ block[(18 & 15) as usize])
                                        >> 32 - 1;
                            block[(18 & 15) as usize]
                        } + 1518500249 as u32 +
                (qq[2 as usize] << 5 | qq[2 as usize] >> 32 - 5);
        qq[3 as usize] = qq[3 as usize] << 32 - 2 | qq[3 as usize] >> 2;
        qq[0 as usize] +=
            (qq[2 as usize] & (qq[3 as usize] ^ qq[4 as usize]) ^
                            qq[4 as usize]) +
                        {
                            block[(19 & 15) as usize] =
                                (block[(19 + 13 & 15) as usize] ^
                                                    block[(19 + 8 & 15) as usize] ^
                                                block[(19 + 2 & 15) as usize] ^ block[(19 & 15) as usize])
                                        << 1 |
                                    (block[(19 + 13 & 15) as usize] ^
                                                    block[(19 + 8 & 15) as usize] ^
                                                block[(19 + 2 & 15) as usize] ^ block[(19 & 15) as usize])
                                        >> 32 - 1;
                            block[(19 & 15) as usize]
                        } + 1518500249 as u32 +
                (qq[1 as usize] << 5 | qq[1 as usize] >> 32 - 5);
        qq[2 as usize] = qq[2 as usize] << 32 - 2 | qq[2 as usize] >> 2;
        qq[4 as usize] +=
            (qq[1 as usize] ^ qq[2 as usize] ^ qq[3 as usize]) +
                        {
                            block[(20 & 15) as usize] =
                                (block[(20 + 13 & 15) as usize] ^
                                                    block[(20 + 8 & 15) as usize] ^
                                                block[(20 + 2 & 15) as usize] ^ block[(20 & 15) as usize])
                                        << 1 |
                                    (block[(20 + 13 & 15) as usize] ^
                                                    block[(20 + 8 & 15) as usize] ^
                                                block[(20 + 2 & 15) as usize] ^ block[(20 & 15) as usize])
                                        >> 32 - 1;
                            block[(20 & 15) as usize]
                        } + 1859775393 as u32 +
                (qq[0 as usize] << 5 | qq[0 as usize] >> 32 - 5);
        qq[1 as usize] = qq[1 as usize] << 32 - 2 | qq[1 as usize] >> 2;
        qq[3 as usize] +=
            (qq[0 as usize] ^ qq[1 as usize] ^ qq[2 as usize]) +
                        {
                            block[(21 & 15) as usize] =
                                (block[(21 + 13 & 15) as usize] ^
                                                    block[(21 + 8 & 15) as usize] ^
                                                block[(21 + 2 & 15) as usize] ^ block[(21 & 15) as usize])
                                        << 1 |
                                    (block[(21 + 13 & 15) as usize] ^
                                                    block[(21 + 8 & 15) as usize] ^
                                                block[(21 + 2 & 15) as usize] ^ block[(21 & 15) as usize])
                                        >> 32 - 1;
                            block[(21 & 15) as usize]
                        } + 1859775393 as u32 +
                (qq[4 as usize] << 5 | qq[4 as usize] >> 32 - 5);
        qq[0 as usize] = qq[0 as usize] << 32 - 2 | qq[0 as usize] >> 2;
        qq[2 as usize] +=
            (qq[4 as usize] ^ qq[0 as usize] ^ qq[1 as usize]) +
                        {
                            block[(22 & 15) as usize] =
                                (block[(22 + 13 & 15) as usize] ^
                                                    block[(22 + 8 & 15) as usize] ^
                                                block[(22 + 2 & 15) as usize] ^ block[(22 & 15) as usize])
                                        << 1 |
                                    (block[(22 + 13 & 15) as usize] ^
                                                    block[(22 + 8 & 15) as usize] ^
                                                block[(22 + 2 & 15) as usize] ^ block[(22 & 15) as usize])
                                        >> 32 - 1;
                            block[(22 & 15) as usize]
                        } + 1859775393 as u32 +
                (qq[3 as usize] << 5 | qq[3 as usize] >> 32 - 5);
        qq[4 as usize] = qq[4 as usize] << 32 - 2 | qq[4 as usize] >> 2;
        qq[1 as usize] +=
            (qq[3 as usize] ^ qq[4 as usize] ^ qq[0 as usize]) +
                        {
                            block[(23 & 15) as usize] =
                                (block[(23 + 13 & 15) as usize] ^
                                                    block[(23 + 8 & 15) as usize] ^
                                                block[(23 + 2 & 15) as usize] ^ block[(23 & 15) as usize])
                                        << 1 |
                                    (block[(23 + 13 & 15) as usize] ^
                                                    block[(23 + 8 & 15) as usize] ^
                                                block[(23 + 2 & 15) as usize] ^ block[(23 & 15) as usize])
                                        >> 32 - 1;
                            block[(23 & 15) as usize]
                        } + 1859775393 as u32 +
                (qq[2 as usize] << 5 | qq[2 as usize] >> 32 - 5);
        qq[3 as usize] = qq[3 as usize] << 32 - 2 | qq[3 as usize] >> 2;
        qq[0 as usize] +=
            (qq[2 as usize] ^ qq[3 as usize] ^ qq[4 as usize]) +
                        {
                            block[(24 & 15) as usize] =
                                (block[(24 + 13 & 15) as usize] ^
                                                    block[(24 + 8 & 15) as usize] ^
                                                block[(24 + 2 & 15) as usize] ^ block[(24 & 15) as usize])
                                        << 1 |
                                    (block[(24 + 13 & 15) as usize] ^
                                                    block[(24 + 8 & 15) as usize] ^
                                                block[(24 + 2 & 15) as usize] ^ block[(24 & 15) as usize])
                                        >> 32 - 1;
                            block[(24 & 15) as usize]
                        } + 1859775393 as u32 +
                (qq[1 as usize] << 5 | qq[1 as usize] >> 32 - 5);
        qq[2 as usize] = qq[2 as usize] << 32 - 2 | qq[2 as usize] >> 2;
        qq[4 as usize] +=
            (qq[1 as usize] ^ qq[2 as usize] ^ qq[3 as usize]) +
                        {
                            block[(25 & 15) as usize] =
                                (block[(25 + 13 & 15) as usize] ^
                                                    block[(25 + 8 & 15) as usize] ^
                                                block[(25 + 2 & 15) as usize] ^ block[(25 & 15) as usize])
                                        << 1 |
                                    (block[(25 + 13 & 15) as usize] ^
                                                    block[(25 + 8 & 15) as usize] ^
                                                block[(25 + 2 & 15) as usize] ^ block[(25 & 15) as usize])
                                        >> 32 - 1;
                            block[(25 & 15) as usize]
                        } + 1859775393 as u32 +
                (qq[0 as usize] << 5 | qq[0 as usize] >> 32 - 5);
        qq[1 as usize] = qq[1 as usize] << 32 - 2 | qq[1 as usize] >> 2;
        qq[3 as usize] +=
            (qq[0 as usize] ^ qq[1 as usize] ^ qq[2 as usize]) +
                        {
                            block[(26 & 15) as usize] =
                                (block[(26 + 13 & 15) as usize] ^
                                                    block[(26 + 8 & 15) as usize] ^
                                                block[(26 + 2 & 15) as usize] ^ block[(26 & 15) as usize])
                                        << 1 |
                                    (block[(26 + 13 & 15) as usize] ^
                                                    block[(26 + 8 & 15) as usize] ^
                                                block[(26 + 2 & 15) as usize] ^ block[(26 & 15) as usize])
                                        >> 32 - 1;
                            block[(26 & 15) as usize]
                        } + 1859775393 as u32 +
                (qq[4 as usize] << 5 | qq[4 as usize] >> 32 - 5);
        qq[0 as usize] = qq[0 as usize] << 32 - 2 | qq[0 as usize] >> 2;
        qq[2 as usize] +=
            (qq[4 as usize] ^ qq[0 as usize] ^ qq[1 as usize]) +
                        {
                            block[(27 & 15) as usize] =
                                (block[(27 + 13 & 15) as usize] ^
                                                    block[(27 + 8 & 15) as usize] ^
                                                block[(27 + 2 & 15) as usize] ^ block[(27 & 15) as usize])
                                        << 1 |
                                    (block[(27 + 13 & 15) as usize] ^
                                                    block[(27 + 8 & 15) as usize] ^
                                                block[(27 + 2 & 15) as usize] ^ block[(27 & 15) as usize])
                                        >> 32 - 1;
                            block[(27 & 15) as usize]
                        } + 1859775393 as u32 +
                (qq[3 as usize] << 5 | qq[3 as usize] >> 32 - 5);
        qq[4 as usize] = qq[4 as usize] << 32 - 2 | qq[4 as usize] >> 2;
        qq[1 as usize] +=
            (qq[3 as usize] ^ qq[4 as usize] ^ qq[0 as usize]) +
                        {
                            block[(28 & 15) as usize] =
                                (block[(28 + 13 & 15) as usize] ^
                                                    block[(28 + 8 & 15) as usize] ^
                                                block[(28 + 2 & 15) as usize] ^ block[(28 & 15) as usize])
                                        << 1 |
                                    (block[(28 + 13 & 15) as usize] ^
                                                    block[(28 + 8 & 15) as usize] ^
                                                block[(28 + 2 & 15) as usize] ^ block[(28 & 15) as usize])
                                        >> 32 - 1;
                            block[(28 & 15) as usize]
                        } + 1859775393 as u32 +
                (qq[2 as usize] << 5 | qq[2 as usize] >> 32 - 5);
        qq[3 as usize] = qq[3 as usize] << 32 - 2 | qq[3 as usize] >> 2;
        qq[0 as usize] +=
            (qq[2 as usize] ^ qq[3 as usize] ^ qq[4 as usize]) +
                        {
                            block[(29 & 15) as usize] =
                                (block[(29 + 13 & 15) as usize] ^
                                                    block[(29 + 8 & 15) as usize] ^
                                                block[(29 + 2 & 15) as usize] ^ block[(29 & 15) as usize])
                                        << 1 |
                                    (block[(29 + 13 & 15) as usize] ^
                                                    block[(29 + 8 & 15) as usize] ^
                                                block[(29 + 2 & 15) as usize] ^ block[(29 & 15) as usize])
                                        >> 32 - 1;
                            block[(29 & 15) as usize]
                        } + 1859775393 as u32 +
                (qq[1 as usize] << 5 | qq[1 as usize] >> 32 - 5);
        qq[2 as usize] = qq[2 as usize] << 32 - 2 | qq[2 as usize] >> 2;
        qq[4 as usize] +=
            (qq[1 as usize] ^ qq[2 as usize] ^ qq[3 as usize]) +
                        {
                            block[(30 & 15) as usize] =
                                (block[(30 + 13 & 15) as usize] ^
                                                    block[(30 + 8 & 15) as usize] ^
                                                block[(30 + 2 & 15) as usize] ^ block[(30 & 15) as usize])
                                        << 1 |
                                    (block[(30 + 13 & 15) as usize] ^
                                                    block[(30 + 8 & 15) as usize] ^
                                                block[(30 + 2 & 15) as usize] ^ block[(30 & 15) as usize])
                                        >> 32 - 1;
                            block[(30 & 15) as usize]
                        } + 1859775393 as u32 +
                (qq[0 as usize] << 5 | qq[0 as usize] >> 32 - 5);
        qq[1 as usize] = qq[1 as usize] << 32 - 2 | qq[1 as usize] >> 2;
        qq[3 as usize] +=
            (qq[0 as usize] ^ qq[1 as usize] ^ qq[2 as usize]) +
                        {
                            block[(31 & 15) as usize] =
                                (block[(31 + 13 & 15) as usize] ^
                                                    block[(31 + 8 & 15) as usize] ^
                                                block[(31 + 2 & 15) as usize] ^ block[(31 & 15) as usize])
                                        << 1 |
                                    (block[(31 + 13 & 15) as usize] ^
                                                    block[(31 + 8 & 15) as usize] ^
                                                block[(31 + 2 & 15) as usize] ^ block[(31 & 15) as usize])
                                        >> 32 - 1;
                            block[(31 & 15) as usize]
                        } + 1859775393 as u32 +
                (qq[4 as usize] << 5 | qq[4 as usize] >> 32 - 5);
        qq[0 as usize] = qq[0 as usize] << 32 - 2 | qq[0 as usize] >> 2;
        qq[2 as usize] +=
            (qq[4 as usize] ^ qq[0 as usize] ^ qq[1 as usize]) +
                        {
                            block[(32 & 15) as usize] =
                                (block[(32 + 13 & 15) as usize] ^
                                                    block[(32 + 8 & 15) as usize] ^
                                                block[(32 + 2 & 15) as usize] ^ block[(32 & 15) as usize])
                                        << 1 |
                                    (block[(32 + 13 & 15) as usize] ^
                                                    block[(32 + 8 & 15) as usize] ^
                                                block[(32 + 2 & 15) as usize] ^ block[(32 & 15) as usize])
                                        >> 32 - 1;
                            block[(32 & 15) as usize]
                        } + 1859775393 as u32 +
                (qq[3 as usize] << 5 | qq[3 as usize] >> 32 - 5);
        qq[4 as usize] = qq[4 as usize] << 32 - 2 | qq[4 as usize] >> 2;
        qq[1 as usize] +=
            (qq[3 as usize] ^ qq[4 as usize] ^ qq[0 as usize]) +
                        {
                            block[(33 & 15) as usize] =
                                (block[(33 + 13 & 15) as usize] ^
                                                    block[(33 + 8 & 15) as usize] ^
                                                block[(33 + 2 & 15) as usize] ^ block[(33 & 15) as usize])
                                        << 1 |
                                    (block[(33 + 13 & 15) as usize] ^
                                                    block[(33 + 8 & 15) as usize] ^
                                                block[(33 + 2 & 15) as usize] ^ block[(33 & 15) as usize])
                                        >> 32 - 1;
                            block[(33 & 15) as usize]
                        } + 1859775393 as u32 +
                (qq[2 as usize] << 5 | qq[2 as usize] >> 32 - 5);
        qq[3 as usize] = qq[3 as usize] << 32 - 2 | qq[3 as usize] >> 2;
        qq[0 as usize] +=
            (qq[2 as usize] ^ qq[3 as usize] ^ qq[4 as usize]) +
                        {
                            block[(34 & 15) as usize] =
                                (block[(34 + 13 & 15) as usize] ^
                                                    block[(34 + 8 & 15) as usize] ^
                                                block[(34 + 2 & 15) as usize] ^ block[(34 & 15) as usize])
                                        << 1 |
                                    (block[(34 + 13 & 15) as usize] ^
                                                    block[(34 + 8 & 15) as usize] ^
                                                block[(34 + 2 & 15) as usize] ^ block[(34 & 15) as usize])
                                        >> 32 - 1;
                            block[(34 & 15) as usize]
                        } + 1859775393 as u32 +
                (qq[1 as usize] << 5 | qq[1 as usize] >> 32 - 5);
        qq[2 as usize] = qq[2 as usize] << 32 - 2 | qq[2 as usize] >> 2;
        qq[4 as usize] +=
            (qq[1 as usize] ^ qq[2 as usize] ^ qq[3 as usize]) +
                        {
                            block[(35 & 15) as usize] =
                                (block[(35 + 13 & 15) as usize] ^
                                                    block[(35 + 8 & 15) as usize] ^
                                                block[(35 + 2 & 15) as usize] ^ block[(35 & 15) as usize])
                                        << 1 |
                                    (block[(35 + 13 & 15) as usize] ^
                                                    block[(35 + 8 & 15) as usize] ^
                                                block[(35 + 2 & 15) as usize] ^ block[(35 & 15) as usize])
                                        >> 32 - 1;
                            block[(35 & 15) as usize]
                        } + 1859775393 as u32 +
                (qq[0 as usize] << 5 | qq[0 as usize] >> 32 - 5);
        qq[1 as usize] = qq[1 as usize] << 32 - 2 | qq[1 as usize] >> 2;
        qq[3 as usize] +=
            (qq[0 as usize] ^ qq[1 as usize] ^ qq[2 as usize]) +
                        {
                            block[(36 & 15) as usize] =
                                (block[(36 + 13 & 15) as usize] ^
                                                    block[(36 + 8 & 15) as usize] ^
                                                block[(36 + 2 & 15) as usize] ^ block[(36 & 15) as usize])
                                        << 1 |
                                    (block[(36 + 13 & 15) as usize] ^
                                                    block[(36 + 8 & 15) as usize] ^
                                                block[(36 + 2 & 15) as usize] ^ block[(36 & 15) as usize])
                                        >> 32 - 1;
                            block[(36 & 15) as usize]
                        } + 1859775393 as u32 +
                (qq[4 as usize] << 5 | qq[4 as usize] >> 32 - 5);
        qq[0 as usize] = qq[0 as usize] << 32 - 2 | qq[0 as usize] >> 2;
        qq[2 as usize] +=
            (qq[4 as usize] ^ qq[0 as usize] ^ qq[1 as usize]) +
                        {
                            block[(37 & 15) as usize] =
                                (block[(37 + 13 & 15) as usize] ^
                                                    block[(37 + 8 & 15) as usize] ^
                                                block[(37 + 2 & 15) as usize] ^ block[(37 & 15) as usize])
                                        << 1 |
                                    (block[(37 + 13 & 15) as usize] ^
                                                    block[(37 + 8 & 15) as usize] ^
                                                block[(37 + 2 & 15) as usize] ^ block[(37 & 15) as usize])
                                        >> 32 - 1;
                            block[(37 & 15) as usize]
                        } + 1859775393 as u32 +
                (qq[3 as usize] << 5 | qq[3 as usize] >> 32 - 5);
        qq[4 as usize] = qq[4 as usize] << 32 - 2 | qq[4 as usize] >> 2;
        qq[1 as usize] +=
            (qq[3 as usize] ^ qq[4 as usize] ^ qq[0 as usize]) +
                        {
                            block[(38 & 15) as usize] =
                                (block[(38 + 13 & 15) as usize] ^
                                                    block[(38 + 8 & 15) as usize] ^
                                                block[(38 + 2 & 15) as usize] ^ block[(38 & 15) as usize])
                                        << 1 |
                                    (block[(38 + 13 & 15) as usize] ^
                                                    block[(38 + 8 & 15) as usize] ^
                                                block[(38 + 2 & 15) as usize] ^ block[(38 & 15) as usize])
                                        >> 32 - 1;
                            block[(38 & 15) as usize]
                        } + 1859775393 as u32 +
                (qq[2 as usize] << 5 | qq[2 as usize] >> 32 - 5);
        qq[3 as usize] = qq[3 as usize] << 32 - 2 | qq[3 as usize] >> 2;
        qq[0 as usize] +=
            (qq[2 as usize] ^ qq[3 as usize] ^ qq[4 as usize]) +
                        {
                            block[(39 & 15) as usize] =
                                (block[(39 + 13 & 15) as usize] ^
                                                    block[(39 + 8 & 15) as usize] ^
                                                block[(39 + 2 & 15) as usize] ^ block[(39 & 15) as usize])
                                        << 1 |
                                    (block[(39 + 13 & 15) as usize] ^
                                                    block[(39 + 8 & 15) as usize] ^
                                                block[(39 + 2 & 15) as usize] ^ block[(39 & 15) as usize])
                                        >> 32 - 1;
                            block[(39 & 15) as usize]
                        } + 1859775393 as u32 +
                (qq[1 as usize] << 5 | qq[1 as usize] >> 32 - 5);
        qq[2 as usize] = qq[2 as usize] << 32 - 2 | qq[2 as usize] >> 2;
        qq[4 as usize] +=
            ((qq[1 as usize] | qq[2 as usize]) & qq[3 as usize] |
                            qq[1 as usize] & qq[2 as usize]) +
                        {
                            block[(40 & 15) as usize] =
                                (block[(40 + 13 & 15) as usize] ^
                                                    block[(40 + 8 & 15) as usize] ^
                                                block[(40 + 2 & 15) as usize] ^ block[(40 & 15) as usize])
                                        << 1 |
                                    (block[(40 + 13 & 15) as usize] ^
                                                    block[(40 + 8 & 15) as usize] ^
                                                block[(40 + 2 & 15) as usize] ^ block[(40 & 15) as usize])
                                        >> 32 - 1;
                            block[(40 & 15) as usize]
                        } + 2400959708u32 +
                (qq[0 as usize] << 5 | qq[0 as usize] >> 32 - 5);
        qq[1 as usize] = qq[1 as usize] << 32 - 2 | qq[1 as usize] >> 2;
        qq[3 as usize] +=
            ((qq[0 as usize] | qq[1 as usize]) & qq[2 as usize] |
                            qq[0 as usize] & qq[1 as usize]) +
                        {
                            block[(41 & 15) as usize] =
                                (block[(41 + 13 & 15) as usize] ^
                                                    block[(41 + 8 & 15) as usize] ^
                                                block[(41 + 2 & 15) as usize] ^ block[(41 & 15) as usize])
                                        << 1 |
                                    (block[(41 + 13 & 15) as usize] ^
                                                    block[(41 + 8 & 15) as usize] ^
                                                block[(41 + 2 & 15) as usize] ^ block[(41 & 15) as usize])
                                        >> 32 - 1;
                            block[(41 & 15) as usize]
                        } + 2400959708u32 +
                (qq[4 as usize] << 5 | qq[4 as usize] >> 32 - 5);
        qq[0 as usize] = qq[0 as usize] << 32 - 2 | qq[0 as usize] >> 2;
        qq[2 as usize] +=
            ((qq[4 as usize] | qq[0 as usize]) & qq[1 as usize] |
                            qq[4 as usize] & qq[0 as usize]) +
                        {
                            block[(42 & 15) as usize] =
                                (block[(42 + 13 & 15) as usize] ^
                                                    block[(42 + 8 & 15) as usize] ^
                                                block[(42 + 2 & 15) as usize] ^ block[(42 & 15) as usize])
                                        << 1 |
                                    (block[(42 + 13 & 15) as usize] ^
                                                    block[(42 + 8 & 15) as usize] ^
                                                block[(42 + 2 & 15) as usize] ^ block[(42 & 15) as usize])
                                        >> 32 - 1;
                            block[(42 & 15) as usize]
                        } + 2400959708u32 +
                (qq[3 as usize] << 5 | qq[3 as usize] >> 32 - 5);
        qq[4 as usize] = qq[4 as usize] << 32 - 2 | qq[4 as usize] >> 2;
        qq[1 as usize] +=
            ((qq[3 as usize] | qq[4 as usize]) & qq[0 as usize] |
                            qq[3 as usize] & qq[4 as usize]) +
                        {
                            block[(43 & 15) as usize] =
                                (block[(43 + 13 & 15) as usize] ^
                                                    block[(43 + 8 & 15) as usize] ^
                                                block[(43 + 2 & 15) as usize] ^ block[(43 & 15) as usize])
                                        << 1 |
                                    (block[(43 + 13 & 15) as usize] ^
                                                    block[(43 + 8 & 15) as usize] ^
                                                block[(43 + 2 & 15) as usize] ^ block[(43 & 15) as usize])
                                        >> 32 - 1;
                            block[(43 & 15) as usize]
                        } + 2400959708u32 +
                (qq[2 as usize] << 5 | qq[2 as usize] >> 32 - 5);
        qq[3 as usize] = qq[3 as usize] << 32 - 2 | qq[3 as usize] >> 2;
        qq[0 as usize] +=
            ((qq[2 as usize] | qq[3 as usize]) & qq[4 as usize] |
                            qq[2 as usize] & qq[3 as usize]) +
                        {
                            block[(44 & 15) as usize] =
                                (block[(44 + 13 & 15) as usize] ^
                                                    block[(44 + 8 & 15) as usize] ^
                                                block[(44 + 2 & 15) as usize] ^ block[(44 & 15) as usize])
                                        << 1 |
                                    (block[(44 + 13 & 15) as usize] ^
                                                    block[(44 + 8 & 15) as usize] ^
                                                block[(44 + 2 & 15) as usize] ^ block[(44 & 15) as usize])
                                        >> 32 - 1;
                            block[(44 & 15) as usize]
                        } + 2400959708u32 +
                (qq[1 as usize] << 5 | qq[1 as usize] >> 32 - 5);
        qq[2 as usize] = qq[2 as usize] << 32 - 2 | qq[2 as usize] >> 2;
        qq[4 as usize] +=
            ((qq[1 as usize] | qq[2 as usize]) & qq[3 as usize] |
                            qq[1 as usize] & qq[2 as usize]) +
                        {
                            block[(45 & 15) as usize] =
                                (block[(45 + 13 & 15) as usize] ^
                                                    block[(45 + 8 & 15) as usize] ^
                                                block[(45 + 2 & 15) as usize] ^ block[(45 & 15) as usize])
                                        << 1 |
                                    (block[(45 + 13 & 15) as usize] ^
                                                    block[(45 + 8 & 15) as usize] ^
                                                block[(45 + 2 & 15) as usize] ^ block[(45 & 15) as usize])
                                        >> 32 - 1;
                            block[(45 & 15) as usize]
                        } + 2400959708u32 +
                (qq[0 as usize] << 5 | qq[0 as usize] >> 32 - 5);
        qq[1 as usize] = qq[1 as usize] << 32 - 2 | qq[1 as usize] >> 2;
        qq[3 as usize] +=
            ((qq[0 as usize] | qq[1 as usize]) & qq[2 as usize] |
                            qq[0 as usize] & qq[1 as usize]) +
                        {
                            block[(46 & 15) as usize] =
                                (block[(46 + 13 & 15) as usize] ^
                                                    block[(46 + 8 & 15) as usize] ^
                                                block[(46 + 2 & 15) as usize] ^ block[(46 & 15) as usize])
                                        << 1 |
                                    (block[(46 + 13 & 15) as usize] ^
                                                    block[(46 + 8 & 15) as usize] ^
                                                block[(46 + 2 & 15) as usize] ^ block[(46 & 15) as usize])
                                        >> 32 - 1;
                            block[(46 & 15) as usize]
                        } + 2400959708u32 +
                (qq[4 as usize] << 5 | qq[4 as usize] >> 32 - 5);
        qq[0 as usize] = qq[0 as usize] << 32 - 2 | qq[0 as usize] >> 2;
        qq[2 as usize] +=
            ((qq[4 as usize] | qq[0 as usize]) & qq[1 as usize] |
                            qq[4 as usize] & qq[0 as usize]) +
                        {
                            block[(47 & 15) as usize] =
                                (block[(47 + 13 & 15) as usize] ^
                                                    block[(47 + 8 & 15) as usize] ^
                                                block[(47 + 2 & 15) as usize] ^ block[(47 & 15) as usize])
                                        << 1 |
                                    (block[(47 + 13 & 15) as usize] ^
                                                    block[(47 + 8 & 15) as usize] ^
                                                block[(47 + 2 & 15) as usize] ^ block[(47 & 15) as usize])
                                        >> 32 - 1;
                            block[(47 & 15) as usize]
                        } + 2400959708u32 +
                (qq[3 as usize] << 5 | qq[3 as usize] >> 32 - 5);
        qq[4 as usize] = qq[4 as usize] << 32 - 2 | qq[4 as usize] >> 2;
        qq[1 as usize] +=
            ((qq[3 as usize] | qq[4 as usize]) & qq[0 as usize] |
                            qq[3 as usize] & qq[4 as usize]) +
                        {
                            block[(48 & 15) as usize] =
                                (block[(48 + 13 & 15) as usize] ^
                                                    block[(48 + 8 & 15) as usize] ^
                                                block[(48 + 2 & 15) as usize] ^ block[(48 & 15) as usize])
                                        << 1 |
                                    (block[(48 + 13 & 15) as usize] ^
                                                    block[(48 + 8 & 15) as usize] ^
                                                block[(48 + 2 & 15) as usize] ^ block[(48 & 15) as usize])
                                        >> 32 - 1;
                            block[(48 & 15) as usize]
                        } + 2400959708u32 +
                (qq[2 as usize] << 5 | qq[2 as usize] >> 32 - 5);
        qq[3 as usize] = qq[3 as usize] << 32 - 2 | qq[3 as usize] >> 2;
        qq[0 as usize] +=
            ((qq[2 as usize] | qq[3 as usize]) & qq[4 as usize] |
                            qq[2 as usize] & qq[3 as usize]) +
                        {
                            block[(49 & 15) as usize] =
                                (block[(49 + 13 & 15) as usize] ^
                                                    block[(49 + 8 & 15) as usize] ^
                                                block[(49 + 2 & 15) as usize] ^ block[(49 & 15) as usize])
                                        << 1 |
                                    (block[(49 + 13 & 15) as usize] ^
                                                    block[(49 + 8 & 15) as usize] ^
                                                block[(49 + 2 & 15) as usize] ^ block[(49 & 15) as usize])
                                        >> 32 - 1;
                            block[(49 & 15) as usize]
                        } + 2400959708u32 +
                (qq[1 as usize] << 5 | qq[1 as usize] >> 32 - 5);
        qq[2 as usize] = qq[2 as usize] << 32 - 2 | qq[2 as usize] >> 2;
        qq[4 as usize] +=
            ((qq[1 as usize] | qq[2 as usize]) & qq[3 as usize] |
                            qq[1 as usize] & qq[2 as usize]) +
                        {
                            block[(50 & 15) as usize] =
                                (block[(50 + 13 & 15) as usize] ^
                                                    block[(50 + 8 & 15) as usize] ^
                                                block[(50 + 2 & 15) as usize] ^ block[(50 & 15) as usize])
                                        << 1 |
                                    (block[(50 + 13 & 15) as usize] ^
                                                    block[(50 + 8 & 15) as usize] ^
                                                block[(50 + 2 & 15) as usize] ^ block[(50 & 15) as usize])
                                        >> 32 - 1;
                            block[(50 & 15) as usize]
                        } + 2400959708u32 +
                (qq[0 as usize] << 5 | qq[0 as usize] >> 32 - 5);
        qq[1 as usize] = qq[1 as usize] << 32 - 2 | qq[1 as usize] >> 2;
        qq[3 as usize] +=
            ((qq[0 as usize] | qq[1 as usize]) & qq[2 as usize] |
                            qq[0 as usize] & qq[1 as usize]) +
                        {
                            block[(51 & 15) as usize] =
                                (block[(51 + 13 & 15) as usize] ^
                                                    block[(51 + 8 & 15) as usize] ^
                                                block[(51 + 2 & 15) as usize] ^ block[(51 & 15) as usize])
                                        << 1 |
                                    (block[(51 + 13 & 15) as usize] ^
                                                    block[(51 + 8 & 15) as usize] ^
                                                block[(51 + 2 & 15) as usize] ^ block[(51 & 15) as usize])
                                        >> 32 - 1;
                            block[(51 & 15) as usize]
                        } + 2400959708u32 +
                (qq[4 as usize] << 5 | qq[4 as usize] >> 32 - 5);
        qq[0 as usize] = qq[0 as usize] << 32 - 2 | qq[0 as usize] >> 2;
        qq[2 as usize] +=
            ((qq[4 as usize] | qq[0 as usize]) & qq[1 as usize] |
                            qq[4 as usize] & qq[0 as usize]) +
                        {
                            block[(52 & 15) as usize] =
                                (block[(52 + 13 & 15) as usize] ^
                                                    block[(52 + 8 & 15) as usize] ^
                                                block[(52 + 2 & 15) as usize] ^ block[(52 & 15) as usize])
                                        << 1 |
                                    (block[(52 + 13 & 15) as usize] ^
                                                    block[(52 + 8 & 15) as usize] ^
                                                block[(52 + 2 & 15) as usize] ^ block[(52 & 15) as usize])
                                        >> 32 - 1;
                            block[(52 & 15) as usize]
                        } + 2400959708u32 +
                (qq[3 as usize] << 5 | qq[3 as usize] >> 32 - 5);
        qq[4 as usize] = qq[4 as usize] << 32 - 2 | qq[4 as usize] >> 2;
        qq[1 as usize] +=
            ((qq[3 as usize] | qq[4 as usize]) & qq[0 as usize] |
                            qq[3 as usize] & qq[4 as usize]) +
                        {
                            block[(53 & 15) as usize] =
                                (block[(53 + 13 & 15) as usize] ^
                                                    block[(53 + 8 & 15) as usize] ^
                                                block[(53 + 2 & 15) as usize] ^ block[(53 & 15) as usize])
                                        << 1 |
                                    (block[(53 + 13 & 15) as usize] ^
                                                    block[(53 + 8 & 15) as usize] ^
                                                block[(53 + 2 & 15) as usize] ^ block[(53 & 15) as usize])
                                        >> 32 - 1;
                            block[(53 & 15) as usize]
                        } + 2400959708u32 +
                (qq[2 as usize] << 5 | qq[2 as usize] >> 32 - 5);
        qq[3 as usize] = qq[3 as usize] << 32 - 2 | qq[3 as usize] >> 2;
        qq[0 as usize] +=
            ((qq[2 as usize] | qq[3 as usize]) & qq[4 as usize] |
                            qq[2 as usize] & qq[3 as usize]) +
                        {
                            block[(54 & 15) as usize] =
                                (block[(54 + 13 & 15) as usize] ^
                                                    block[(54 + 8 & 15) as usize] ^
                                                block[(54 + 2 & 15) as usize] ^ block[(54 & 15) as usize])
                                        << 1 |
                                    (block[(54 + 13 & 15) as usize] ^
                                                    block[(54 + 8 & 15) as usize] ^
                                                block[(54 + 2 & 15) as usize] ^ block[(54 & 15) as usize])
                                        >> 32 - 1;
                            block[(54 & 15) as usize]
                        } + 2400959708u32 +
                (qq[1 as usize] << 5 | qq[1 as usize] >> 32 - 5);
        qq[2 as usize] = qq[2 as usize] << 32 - 2 | qq[2 as usize] >> 2;
        qq[4 as usize] +=
            ((qq[1 as usize] | qq[2 as usize]) & qq[3 as usize] |
                            qq[1 as usize] & qq[2 as usize]) +
                        {
                            block[(55 & 15) as usize] =
                                (block[(55 + 13 & 15) as usize] ^
                                                    block[(55 + 8 & 15) as usize] ^
                                                block[(55 + 2 & 15) as usize] ^ block[(55 & 15) as usize])
                                        << 1 |
                                    (block[(55 + 13 & 15) as usize] ^
                                                    block[(55 + 8 & 15) as usize] ^
                                                block[(55 + 2 & 15) as usize] ^ block[(55 & 15) as usize])
                                        >> 32 - 1;
                            block[(55 & 15) as usize]
                        } + 2400959708u32 +
                (qq[0 as usize] << 5 | qq[0 as usize] >> 32 - 5);
        qq[1 as usize] = qq[1 as usize] << 32 - 2 | qq[1 as usize] >> 2;
        qq[3 as usize] +=
            ((qq[0 as usize] | qq[1 as usize]) & qq[2 as usize] |
                            qq[0 as usize] & qq[1 as usize]) +
                        {
                            block[(56 & 15) as usize] =
                                (block[(56 + 13 & 15) as usize] ^
                                                    block[(56 + 8 & 15) as usize] ^
                                                block[(56 + 2 & 15) as usize] ^ block[(56 & 15) as usize])
                                        << 1 |
                                    (block[(56 + 13 & 15) as usize] ^
                                                    block[(56 + 8 & 15) as usize] ^
                                                block[(56 + 2 & 15) as usize] ^ block[(56 & 15) as usize])
                                        >> 32 - 1;
                            block[(56 & 15) as usize]
                        } + 2400959708u32 +
                (qq[4 as usize] << 5 | qq[4 as usize] >> 32 - 5);
        qq[0 as usize] = qq[0 as usize] << 32 - 2 | qq[0 as usize] >> 2;
        qq[2 as usize] +=
            ((qq[4 as usize] | qq[0 as usize]) & qq[1 as usize] |
                            qq[4 as usize] & qq[0 as usize]) +
                        {
                            block[(57 & 15) as usize] =
                                (block[(57 + 13 & 15) as usize] ^
                                                    block[(57 + 8 & 15) as usize] ^
                                                block[(57 + 2 & 15) as usize] ^ block[(57 & 15) as usize])
                                        << 1 |
                                    (block[(57 + 13 & 15) as usize] ^
                                                    block[(57 + 8 & 15) as usize] ^
                                                block[(57 + 2 & 15) as usize] ^ block[(57 & 15) as usize])
                                        >> 32 - 1;
                            block[(57 & 15) as usize]
                        } + 2400959708u32 +
                (qq[3 as usize] << 5 | qq[3 as usize] >> 32 - 5);
        qq[4 as usize] = qq[4 as usize] << 32 - 2 | qq[4 as usize] >> 2;
        qq[1 as usize] +=
            ((qq[3 as usize] | qq[4 as usize]) & qq[0 as usize] |
                            qq[3 as usize] & qq[4 as usize]) +
                        {
                            block[(58 & 15) as usize] =
                                (block[(58 + 13 & 15) as usize] ^
                                                    block[(58 + 8 & 15) as usize] ^
                                                block[(58 + 2 & 15) as usize] ^ block[(58 & 15) as usize])
                                        << 1 |
                                    (block[(58 + 13 & 15) as usize] ^
                                                    block[(58 + 8 & 15) as usize] ^
                                                block[(58 + 2 & 15) as usize] ^ block[(58 & 15) as usize])
                                        >> 32 - 1;
                            block[(58 & 15) as usize]
                        } + 2400959708u32 +
                (qq[2 as usize] << 5 | qq[2 as usize] >> 32 - 5);
        qq[3 as usize] = qq[3 as usize] << 32 - 2 | qq[3 as usize] >> 2;
        qq[0 as usize] +=
            ((qq[2 as usize] | qq[3 as usize]) & qq[4 as usize] |
                            qq[2 as usize] & qq[3 as usize]) +
                        {
                            block[(59 & 15) as usize] =
                                (block[(59 + 13 & 15) as usize] ^
                                                    block[(59 + 8 & 15) as usize] ^
                                                block[(59 + 2 & 15) as usize] ^ block[(59 & 15) as usize])
                                        << 1 |
                                    (block[(59 + 13 & 15) as usize] ^
                                                    block[(59 + 8 & 15) as usize] ^
                                                block[(59 + 2 & 15) as usize] ^ block[(59 & 15) as usize])
                                        >> 32 - 1;
                            block[(59 & 15) as usize]
                        } + 2400959708u32 +
                (qq[1 as usize] << 5 | qq[1 as usize] >> 32 - 5);
        qq[2 as usize] = qq[2 as usize] << 32 - 2 | qq[2 as usize] >> 2;
        qq[4 as usize] +=
            (qq[1 as usize] ^ qq[2 as usize] ^ qq[3 as usize]) +
                        {
                            block[(60 & 15) as usize] =
                                (block[(60 + 13 & 15) as usize] ^
                                                    block[(60 + 8 & 15) as usize] ^
                                                block[(60 + 2 & 15) as usize] ^ block[(60 & 15) as usize])
                                        << 1 |
                                    (block[(60 + 13 & 15) as usize] ^
                                                    block[(60 + 8 & 15) as usize] ^
                                                block[(60 + 2 & 15) as usize] ^ block[(60 & 15) as usize])
                                        >> 32 - 1;
                            block[(60 & 15) as usize]
                        } + 3395469782u32 +
                (qq[0 as usize] << 5 | qq[0 as usize] >> 32 - 5);
        qq[1 as usize] = qq[1 as usize] << 32 - 2 | qq[1 as usize] >> 2;
        qq[3 as usize] +=
            (qq[0 as usize] ^ qq[1 as usize] ^ qq[2 as usize]) +
                        {
                            block[(61 & 15) as usize] =
                                (block[(61 + 13 & 15) as usize] ^
                                                    block[(61 + 8 & 15) as usize] ^
                                                block[(61 + 2 & 15) as usize] ^ block[(61 & 15) as usize])
                                        << 1 |
                                    (block[(61 + 13 & 15) as usize] ^
                                                    block[(61 + 8 & 15) as usize] ^
                                                block[(61 + 2 & 15) as usize] ^ block[(61 & 15) as usize])
                                        >> 32 - 1;
                            block[(61 & 15) as usize]
                        } + 3395469782u32 +
                (qq[4 as usize] << 5 | qq[4 as usize] >> 32 - 5);
        qq[0 as usize] = qq[0 as usize] << 32 - 2 | qq[0 as usize] >> 2;
        qq[2 as usize] +=
            (qq[4 as usize] ^ qq[0 as usize] ^ qq[1 as usize]) +
                        {
                            block[(62 & 15) as usize] =
                                (block[(62 + 13 & 15) as usize] ^
                                                    block[(62 + 8 & 15) as usize] ^
                                                block[(62 + 2 & 15) as usize] ^ block[(62 & 15) as usize])
                                        << 1 |
                                    (block[(62 + 13 & 15) as usize] ^
                                                    block[(62 + 8 & 15) as usize] ^
                                                block[(62 + 2 & 15) as usize] ^ block[(62 & 15) as usize])
                                        >> 32 - 1;
                            block[(62 & 15) as usize]
                        } + 3395469782u32 +
                (qq[3 as usize] << 5 | qq[3 as usize] >> 32 - 5);
        qq[4 as usize] = qq[4 as usize] << 32 - 2 | qq[4 as usize] >> 2;
        qq[1 as usize] +=
            (qq[3 as usize] ^ qq[4 as usize] ^ qq[0 as usize]) +
                        {
                            block[(63 & 15) as usize] =
                                (block[(63 + 13 & 15) as usize] ^
                                                    block[(63 + 8 & 15) as usize] ^
                                                block[(63 + 2 & 15) as usize] ^ block[(63 & 15) as usize])
                                        << 1 |
                                    (block[(63 + 13 & 15) as usize] ^
                                                    block[(63 + 8 & 15) as usize] ^
                                                block[(63 + 2 & 15) as usize] ^ block[(63 & 15) as usize])
                                        >> 32 - 1;
                            block[(63 & 15) as usize]
                        } + 3395469782u32 +
                (qq[2 as usize] << 5 | qq[2 as usize] >> 32 - 5);
        qq[3 as usize] = qq[3 as usize] << 32 - 2 | qq[3 as usize] >> 2;
        qq[0 as usize] +=
            (qq[2 as usize] ^ qq[3 as usize] ^ qq[4 as usize]) +
                        {
                            block[(64 & 15) as usize] =
                                (block[(64 + 13 & 15) as usize] ^
                                                    block[(64 + 8 & 15) as usize] ^
                                                block[(64 + 2 & 15) as usize] ^ block[(64 & 15) as usize])
                                        << 1 |
                                    (block[(64 + 13 & 15) as usize] ^
                                                    block[(64 + 8 & 15) as usize] ^
                                                block[(64 + 2 & 15) as usize] ^ block[(64 & 15) as usize])
                                        >> 32 - 1;
                            block[(64 & 15) as usize]
                        } + 3395469782u32 +
                (qq[1 as usize] << 5 | qq[1 as usize] >> 32 - 5);
        qq[2 as usize] = qq[2 as usize] << 32 - 2 | qq[2 as usize] >> 2;
        qq[4 as usize] +=
            (qq[1 as usize] ^ qq[2 as usize] ^ qq[3 as usize]) +
                        {
                            block[(65 & 15) as usize] =
                                (block[(65 + 13 & 15) as usize] ^
                                                    block[(65 + 8 & 15) as usize] ^
                                                block[(65 + 2 & 15) as usize] ^ block[(65 & 15) as usize])
                                        << 1 |
                                    (block[(65 + 13 & 15) as usize] ^
                                                    block[(65 + 8 & 15) as usize] ^
                                                block[(65 + 2 & 15) as usize] ^ block[(65 & 15) as usize])
                                        >> 32 - 1;
                            block[(65 & 15) as usize]
                        } + 3395469782u32 +
                (qq[0 as usize] << 5 | qq[0 as usize] >> 32 - 5);
        qq[1 as usize] = qq[1 as usize] << 32 - 2 | qq[1 as usize] >> 2;
        qq[3 as usize] +=
            (qq[0 as usize] ^ qq[1 as usize] ^ qq[2 as usize]) +
                        {
                            block[(66 & 15) as usize] =
                                (block[(66 + 13 & 15) as usize] ^
                                                    block[(66 + 8 & 15) as usize] ^
                                                block[(66 + 2 & 15) as usize] ^ block[(66 & 15) as usize])
                                        << 1 |
                                    (block[(66 + 13 & 15) as usize] ^
                                                    block[(66 + 8 & 15) as usize] ^
                                                block[(66 + 2 & 15) as usize] ^ block[(66 & 15) as usize])
                                        >> 32 - 1;
                            block[(66 & 15) as usize]
                        } + 3395469782u32 +
                (qq[4 as usize] << 5 | qq[4 as usize] >> 32 - 5);
        qq[0 as usize] = qq[0 as usize] << 32 - 2 | qq[0 as usize] >> 2;
        qq[2 as usize] +=
            (qq[4 as usize] ^ qq[0 as usize] ^ qq[1 as usize]) +
                        {
                            block[(67 & 15) as usize] =
                                (block[(67 + 13 & 15) as usize] ^
                                                    block[(67 + 8 & 15) as usize] ^
                                                block[(67 + 2 & 15) as usize] ^ block[(67 & 15) as usize])
                                        << 1 |
                                    (block[(67 + 13 & 15) as usize] ^
                                                    block[(67 + 8 & 15) as usize] ^
                                                block[(67 + 2 & 15) as usize] ^ block[(67 & 15) as usize])
                                        >> 32 - 1;
                            block[(67 & 15) as usize]
                        } + 3395469782u32 +
                (qq[3 as usize] << 5 | qq[3 as usize] >> 32 - 5);
        qq[4 as usize] = qq[4 as usize] << 32 - 2 | qq[4 as usize] >> 2;
        qq[1 as usize] +=
            (qq[3 as usize] ^ qq[4 as usize] ^ qq[0 as usize]) +
                        {
                            block[(68 & 15) as usize] =
                                (block[(68 + 13 & 15) as usize] ^
                                                    block[(68 + 8 & 15) as usize] ^
                                                block[(68 + 2 & 15) as usize] ^ block[(68 & 15) as usize])
                                        << 1 |
                                    (block[(68 + 13 & 15) as usize] ^
                                                    block[(68 + 8 & 15) as usize] ^
                                                block[(68 + 2 & 15) as usize] ^ block[(68 & 15) as usize])
                                        >> 32 - 1;
                            block[(68 & 15) as usize]
                        } + 3395469782u32 +
                (qq[2 as usize] << 5 | qq[2 as usize] >> 32 - 5);
        qq[3 as usize] = qq[3 as usize] << 32 - 2 | qq[3 as usize] >> 2;
        qq[0 as usize] +=
            (qq[2 as usize] ^ qq[3 as usize] ^ qq[4 as usize]) +
                        {
                            block[(69 & 15) as usize] =
                                (block[(69 + 13 & 15) as usize] ^
                                                    block[(69 + 8 & 15) as usize] ^
                                                block[(69 + 2 & 15) as usize] ^ block[(69 & 15) as usize])
                                        << 1 |
                                    (block[(69 + 13 & 15) as usize] ^
                                                    block[(69 + 8 & 15) as usize] ^
                                                block[(69 + 2 & 15) as usize] ^ block[(69 & 15) as usize])
                                        >> 32 - 1;
                            block[(69 & 15) as usize]
                        } + 3395469782u32 +
                (qq[1 as usize] << 5 | qq[1 as usize] >> 32 - 5);
        qq[2 as usize] = qq[2 as usize] << 32 - 2 | qq[2 as usize] >> 2;
        qq[4 as usize] +=
            (qq[1 as usize] ^ qq[2 as usize] ^ qq[3 as usize]) +
                        {
                            block[(70 & 15) as usize] =
                                (block[(70 + 13 & 15) as usize] ^
                                                    block[(70 + 8 & 15) as usize] ^
                                                block[(70 + 2 & 15) as usize] ^ block[(70 & 15) as usize])
                                        << 1 |
                                    (block[(70 + 13 & 15) as usize] ^
                                                    block[(70 + 8 & 15) as usize] ^
                                                block[(70 + 2 & 15) as usize] ^ block[(70 & 15) as usize])
                                        >> 32 - 1;
                            block[(70 & 15) as usize]
                        } + 3395469782u32 +
                (qq[0 as usize] << 5 | qq[0 as usize] >> 32 - 5);
        qq[1 as usize] = qq[1 as usize] << 32 - 2 | qq[1 as usize] >> 2;
        qq[3 as usize] +=
            (qq[0 as usize] ^ qq[1 as usize] ^ qq[2 as usize]) +
                        {
                            block[(71 & 15) as usize] =
                                (block[(71 + 13 & 15) as usize] ^
                                                    block[(71 + 8 & 15) as usize] ^
                                                block[(71 + 2 & 15) as usize] ^ block[(71 & 15) as usize])
                                        << 1 |
                                    (block[(71 + 13 & 15) as usize] ^
                                                    block[(71 + 8 & 15) as usize] ^
                                                block[(71 + 2 & 15) as usize] ^ block[(71 & 15) as usize])
                                        >> 32 - 1;
                            block[(71 & 15) as usize]
                        } + 3395469782u32 +
                (qq[4 as usize] << 5 | qq[4 as usize] >> 32 - 5);
        qq[0 as usize] = qq[0 as usize] << 32 - 2 | qq[0 as usize] >> 2;
        qq[2 as usize] +=
            (qq[4 as usize] ^ qq[0 as usize] ^ qq[1 as usize]) +
                        {
                            block[(72 & 15) as usize] =
                                (block[(72 + 13 & 15) as usize] ^
                                                    block[(72 + 8 & 15) as usize] ^
                                                block[(72 + 2 & 15) as usize] ^ block[(72 & 15) as usize])
                                        << 1 |
                                    (block[(72 + 13 & 15) as usize] ^
                                                    block[(72 + 8 & 15) as usize] ^
                                                block[(72 + 2 & 15) as usize] ^ block[(72 & 15) as usize])
                                        >> 32 - 1;
                            block[(72 & 15) as usize]
                        } + 3395469782u32 +
                (qq[3 as usize] << 5 | qq[3 as usize] >> 32 - 5);
        qq[4 as usize] = qq[4 as usize] << 32 - 2 | qq[4 as usize] >> 2;
        qq[1 as usize] +=
            (qq[3 as usize] ^ qq[4 as usize] ^ qq[0 as usize]) +
                        {
                            block[(73 & 15) as usize] =
                                (block[(73 + 13 & 15) as usize] ^
                                                    block[(73 + 8 & 15) as usize] ^
                                                block[(73 + 2 & 15) as usize] ^ block[(73 & 15) as usize])
                                        << 1 |
                                    (block[(73 + 13 & 15) as usize] ^
                                                    block[(73 + 8 & 15) as usize] ^
                                                block[(73 + 2 & 15) as usize] ^ block[(73 & 15) as usize])
                                        >> 32 - 1;
                            block[(73 & 15) as usize]
                        } + 3395469782u32 +
                (qq[2 as usize] << 5 | qq[2 as usize] >> 32 - 5);
        qq[3 as usize] = qq[3 as usize] << 32 - 2 | qq[3 as usize] >> 2;
        qq[0 as usize] +=
            (qq[2 as usize] ^ qq[3 as usize] ^ qq[4 as usize]) +
                        {
                            block[(74 & 15) as usize] =
                                (block[(74 + 13 & 15) as usize] ^
                                                    block[(74 + 8 & 15) as usize] ^
                                                block[(74 + 2 & 15) as usize] ^ block[(74 & 15) as usize])
                                        << 1 |
                                    (block[(74 + 13 & 15) as usize] ^
                                                    block[(74 + 8 & 15) as usize] ^
                                                block[(74 + 2 & 15) as usize] ^ block[(74 & 15) as usize])
                                        >> 32 - 1;
                            block[(74 & 15) as usize]
                        } + 3395469782u32 +
                (qq[1 as usize] << 5 | qq[1 as usize] >> 32 - 5);
        qq[2 as usize] = qq[2 as usize] << 32 - 2 | qq[2 as usize] >> 2;
        qq[4 as usize] +=
            (qq[1 as usize] ^ qq[2 as usize] ^ qq[3 as usize]) +
                        {
                            block[(75 & 15) as usize] =
                                (block[(75 + 13 & 15) as usize] ^
                                                    block[(75 + 8 & 15) as usize] ^
                                                block[(75 + 2 & 15) as usize] ^ block[(75 & 15) as usize])
                                        << 1 |
                                    (block[(75 + 13 & 15) as usize] ^
                                                    block[(75 + 8 & 15) as usize] ^
                                                block[(75 + 2 & 15) as usize] ^ block[(75 & 15) as usize])
                                        >> 32 - 1;
                            block[(75 & 15) as usize]
                        } + 3395469782u32 +
                (qq[0 as usize] << 5 | qq[0 as usize] >> 32 - 5);
        qq[1 as usize] = qq[1 as usize] << 32 - 2 | qq[1 as usize] >> 2;
        qq[3 as usize] +=
            (qq[0 as usize] ^ qq[1 as usize] ^ qq[2 as usize]) +
                        {
                            block[(76 & 15) as usize] =
                                (block[(76 + 13 & 15) as usize] ^
                                                    block[(76 + 8 & 15) as usize] ^
                                                block[(76 + 2 & 15) as usize] ^ block[(76 & 15) as usize])
                                        << 1 |
                                    (block[(76 + 13 & 15) as usize] ^
                                                    block[(76 + 8 & 15) as usize] ^
                                                block[(76 + 2 & 15) as usize] ^ block[(76 & 15) as usize])
                                        >> 32 - 1;
                            block[(76 & 15) as usize]
                        } + 3395469782u32 +
                (qq[4 as usize] << 5 | qq[4 as usize] >> 32 - 5);
        qq[0 as usize] = qq[0 as usize] << 32 - 2 | qq[0 as usize] >> 2;
        qq[2 as usize] +=
            (qq[4 as usize] ^ qq[0 as usize] ^ qq[1 as usize]) +
                        {
                            block[(77 & 15) as usize] =
                                (block[(77 + 13 & 15) as usize] ^
                                                    block[(77 + 8 & 15) as usize] ^
                                                block[(77 + 2 & 15) as usize] ^ block[(77 & 15) as usize])
                                        << 1 |
                                    (block[(77 + 13 & 15) as usize] ^
                                                    block[(77 + 8 & 15) as usize] ^
                                                block[(77 + 2 & 15) as usize] ^ block[(77 & 15) as usize])
                                        >> 32 - 1;
                            block[(77 & 15) as usize]
                        } + 3395469782u32 +
                (qq[3 as usize] << 5 | qq[3 as usize] >> 32 - 5);
        qq[4 as usize] = qq[4 as usize] << 32 - 2 | qq[4 as usize] >> 2;
        qq[1 as usize] +=
            (qq[3 as usize] ^ qq[4 as usize] ^ qq[0 as usize]) +
                        {
                            block[(78 & 15) as usize] =
                                (block[(78 + 13 & 15) as usize] ^
                                                    block[(78 + 8 & 15) as usize] ^
                                                block[(78 + 2 & 15) as usize] ^ block[(78 & 15) as usize])
                                        << 1 |
                                    (block[(78 + 13 & 15) as usize] ^
                                                    block[(78 + 8 & 15) as usize] ^
                                                block[(78 + 2 & 15) as usize] ^ block[(78 & 15) as usize])
                                        >> 32 - 1;
                            block[(78 & 15) as usize]
                        } + 3395469782u32 +
                (qq[2 as usize] << 5 | qq[2 as usize] >> 32 - 5);
        qq[3 as usize] = qq[3 as usize] << 32 - 2 | qq[3 as usize] >> 2;
        qq[0 as usize] +=
            (qq[2 as usize] ^ qq[3 as usize] ^ qq[4 as usize]) +
                        {
                            block[(79 & 15) as usize] =
                                (block[(79 + 13 & 15) as usize] ^
                                                    block[(79 + 8 & 15) as usize] ^
                                                block[(79 + 2 & 15) as usize] ^ block[(79 & 15) as usize])
                                        << 1 |
                                    (block[(79 + 13 & 15) as usize] ^
                                                    block[(79 + 8 & 15) as usize] ^
                                                block[(79 + 2 & 15) as usize] ^ block[(79 & 15) as usize])
                                        >> 32 - 1;
                            block[(79 & 15) as usize]
                        } + 3395469782u32 +
                (qq[1 as usize] << 5 | qq[1 as usize] >> 32 - 5);
        qq[2 as usize] = qq[2 as usize] << 32 - 2 | qq[2 as usize] >> 2;
        unsafe { *state.offset(0 as isize) += qq[0 as usize] };
        unsafe { *state.offset(1 as isize) += qq[1 as usize] };
        unsafe { *state.offset(2 as isize) += qq[2 as usize] };
        unsafe { *state.offset(3 as isize) += qq[3 as usize] };
        unsafe { *state.offset(4 as isize) += qq[4 as usize] };
    }
}
extern "C" fn hash_init() -> () {
    unsafe {
        g.cx.state[0 as usize] = 1732584193 as u32;
        g.cx.state[1 as usize] = 4023233417u32;
        g.cx.state[2 as usize] = 2562383102u32;
        g.cx.state[3 as usize] = 271733878 as u32;
        g.cx.state[4 as usize] = 3285377520u32;
        g.cx.count[0 as usize] =
            { g.cx.count[1 as usize] = 0 as u32; g.cx.count[1 as usize] };
    }
}
extern "C" fn hash_step(data: *const u8, len: u32) -> () {
    unsafe {
        let mut i: u32 = 0 as u32;
        let mut j: u32 = 0 as u32;
        j = g.cx.count[0 as usize];
        if { g.cx.count[0 as usize] += len << 3; g.cx.count[0 as usize] } < j
            {
            g.cx.count[1 as usize] += (len >> 29) + 1 as u32;
        }
        j = j >> 3 & 63 as u32;
        if j + len > 63 as u32 {
            {
                let _ =
                    unsafe {
                        memcpy(&raw mut g.cx.buffer[j as usize] as *mut (),
                            data as *const (), { i = 64 as u32 - j; i } as u64)
                    };
            };
            sha1_transform(&raw mut g.cx.state[0 as usize] as *mut u32,
                &raw mut g.cx.buffer[0 as usize] as *mut u8 as *const u8);
            {
                '__b0: loop {
                    if !((i + 63 as u32) < len) { break '__b0; }
                    '__c0: loop {
                        sha1_transform(&raw mut g.cx.state[0 as usize] as *mut u32,
                            unsafe { &*data.add(i as usize) });
                        break '__c0;
                    }
                    i += 64 as u32;
                }
            }
            j = 0 as u32;
        } else { i = 0 as u32; }
        {
            let _ =
                unsafe {
                    memcpy(&raw mut g.cx.buffer[j as usize] as *mut (),
                        unsafe { &raw const *data.add(i as usize) } as *const (),
                        (len - i) as u64)
                };
        };
    }
}
extern "C" fn hash_finish(z_name_1: *const i8) -> () {
    unsafe {
        let mut i: u32 = 0 as u32;
        let mut finalcount: [u8; 8] = [0; 8];
        let mut digest: [u8; 20] = [0; 20];
        let mut z_out: [i8; 41] = [0; 41];
        {
            i = 0 as u32;
            '__b1: loop {
                if !(i < 8 as u32) { break '__b1; }
                '__c1: loop {
                    finalcount[i as usize] =
                        (g.cx.count[if i >= 4 as u32 { 0 } else { 1 } as usize] >>
                                    (3 as u32 - (i & 3 as u32)) * 8 as u32 & 255 as u32) as u8;
                    break '__c1;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        hash_step(b"\x80\x00".as_ptr() as *const u8, 1 as u32);
        while g.cx.count[0 as usize] & 504 as u32 != 448 as u32 {
            hash_step(b"\x00\x00".as_ptr() as *const u8, 1 as u32);
        }
        hash_step(&raw mut finalcount[0 as usize] as *mut u8 as *const u8,
            8 as u32);
        {
            i = 0 as u32;
            '__b3: loop {
                if !(i < 20 as u32) { break '__b3; }
                '__c3: loop {
                    digest[i as usize] =
                        (g.cx.state[(i >> 2) as usize] >>
                                    (3 as u32 - (i & 3 as u32)) * 8 as u32 & 255 as u32) as u8;
                    break '__c3;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        {
            i = 0 as u32;
            '__b4: loop {
                if !(i < 20 as u32) { break '__b4; }
                '__c4: loop {
                    z_out[(i * 2 as u32) as usize] =
                        z_encode[(digest[i as usize] as i32 >> 4 & 15) as usize] as
                            i8;
                    z_out[(i * 2 as u32 + 1 as u32) as usize] =
                        z_encode[(digest[i as usize] as i32 & 15) as usize] as i8;
                    break '__c4;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        z_out[(i * 2 as u32) as usize] = 0 as i8;
        unsafe {
            printf(c"%s %s\n".as_ptr() as *mut i8 as *const i8,
                &raw mut z_out[0 as usize] as *mut i8, z_name_1)
        };
    }
}
unsafe extern "C" fn cmdline_error(z_format_1: *const i8, mut __va0: ...)
    -> () {
    unsafe {
        let mut ap: *mut i8 = core::ptr::null_mut();
        unsafe {
            fprintf(__stderrp, c"%s: ".as_ptr() as *mut i8 as *const i8,
                g.z_argv0)
        };
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        unsafe { vfprintf(__stderrp, z_format_1, ap) };
        ();
        unsafe {
            fprintf(__stderrp,
                c"\n\"%s --help\" for more help\n".as_ptr() as *mut i8 as
                    *const i8, g.z_argv0)
        };
        unsafe { exit(1) };
    }
}
unsafe extern "C" fn runtime_error(z_format_1: *const i8, mut __va0: ...)
    -> () {
    unsafe {
        let mut ap: *mut i8 = core::ptr::null_mut();
        unsafe {
            fprintf(__stderrp, c"%s: ".as_ptr() as *mut i8 as *const i8,
                g.z_argv0)
        };
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        unsafe { vfprintf(__stderrp, z_format_1, ap) };
        ();
        eprintln!("");
        unsafe { exit(1) };
    }
}
extern "C" fn db_vprepare(z_format_1: *const i8, ap: *mut i8)
    -> *mut Sqlite3Stmt {
    unsafe {
        let mut z_sql: *mut i8 = core::ptr::null_mut();
        let mut rc: i32 = 0;
        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
        z_sql = unsafe { sqlite3_vmprintf(z_format_1, ap) };
        if z_sql == core::ptr::null_mut() {
            unsafe {
                runtime_error(c"out of memory".as_ptr() as *mut i8 as
                        *const i8)
            };
        }
        rc =
            unsafe {
                sqlite3_prepare_v2(g.db, z_sql as *const i8, -1, &mut p_stmt,
                    core::ptr::null_mut())
            };
        if rc != 0 {
            unsafe {
                runtime_error(c"SQL statement error: %s\n\"%s\"".as_ptr() as
                            *mut i8 as *const i8, unsafe { sqlite3_errmsg(g.db) },
                    z_sql)
            };
        }
        unsafe { sqlite3_free(z_sql as *mut ()) };
        return p_stmt;
    }
}
unsafe extern "C" fn db_prepare(z_format_1: *const i8, mut __va0: ...)
    -> *mut Sqlite3Stmt {
    let mut ap: *mut i8 = core::ptr::null_mut();
    let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
    unsafe { ap = core::mem::transmute_copy(&__va0) };
    p_stmt = db_vprepare(z_format_1, ap);
    ();
    return p_stmt;
}
unsafe extern "C" fn hash_one_query(z_format_1: *const i8, mut __va0: ...)
    -> () {
    unsafe {
        let mut ap: *mut i8 = core::ptr::null_mut();
        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
        let mut n_col: i32 = 0;
        let mut i: i32 = 0;
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        p_stmt = db_vprepare(z_format_1, ap);
        ();
        n_col = unsafe { sqlite3_column_count(p_stmt) };
        while 100 == unsafe { sqlite3_step(p_stmt) } {
            {
                i = 0;
                '__b6: loop {
                    if !(i < n_col) { break '__b6; }
                    '__c6: loop {
                        '__s7:
                            {
                            match unsafe { sqlite3_column_type(p_stmt, i) } {
                                5 => {
                                    {
                                        hash_step(c"0".as_ptr() as *const u8, 1 as u32);
                                        if g.f_debug & 1 as u32 != 0 { eprintln!("NULL"); }
                                        break '__s7;
                                    }
                                    {
                                        let mut u: Sqlite3Uint64 = 0 as Sqlite3Uint64;
                                        let mut j: i32 = 0;
                                        let mut x: [u8; 8] = [0; 8];
                                        let mut v: Sqlite3Int64 =
                                            unsafe { sqlite3_column_int64(p_stmt, i) };
                                        unsafe {
                                            memcpy(&raw mut u as *mut (), &raw mut v as *const (),
                                                8 as u64)
                                        };
                                        {
                                            j = 7;
                                            '__b8: loop {
                                                if !(j >= 0) { break '__b8; }
                                                '__c8: loop {
                                                    x[j as usize] = (u & 255 as Sqlite3Uint64) as u8;
                                                    u >>= 8 as Sqlite3Uint64;
                                                    break '__c8;
                                                }
                                                { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                                            }
                                        }
                                        hash_step(c"1".as_ptr() as *const u8, 1 as u32);
                                        hash_step(&raw mut x[0 as usize] as *mut u8 as *const u8,
                                            8 as u32);
                                        if g.f_debug & 1 as u32 != 0 {
                                            unsafe {
                                                fprintf(__stderrp,
                                                    c"INT %s\n".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { sqlite3_column_text(p_stmt, i) })
                                            };
                                        }
                                        break '__s7;
                                    }
                                    {
                                        let mut u: Sqlite3Uint64 = 0 as Sqlite3Uint64;
                                        let mut j: i32 = 0;
                                        let mut x: [u8; 8] = [0; 8];
                                        let mut r: f64 =
                                            unsafe { sqlite3_column_double(p_stmt, i) };
                                        unsafe {
                                            memcpy(&raw mut u as *mut (), &raw mut r as *const (),
                                                8 as u64)
                                        };
                                        {
                                            j = 7;
                                            '__b9: loop {
                                                if !(j >= 0) { break '__b9; }
                                                '__c9: loop {
                                                    x[j as usize] = (u & 255 as Sqlite3Uint64) as u8;
                                                    u >>= 8 as Sqlite3Uint64;
                                                    break '__c9;
                                                }
                                                { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                                            }
                                        }
                                        hash_step(c"2".as_ptr() as *const u8, 1 as u32);
                                        hash_step(&raw mut x[0 as usize] as *mut u8 as *const u8,
                                            8 as u32);
                                        if g.f_debug & 1 as u32 != 0 {
                                            unsafe {
                                                fprintf(__stderrp,
                                                    c"FLOAT %s\n".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { sqlite3_column_text(p_stmt, i) })
                                            };
                                        }
                                        break '__s7;
                                    }
                                    {
                                        let n: i32 = unsafe { sqlite3_column_bytes(p_stmt, i) };
                                        let z: *const u8 =
                                            unsafe { sqlite3_column_text(p_stmt, i) };
                                        hash_step(c"3".as_ptr() as *const u8, 1 as u32);
                                        hash_step(z, n as u32);
                                        if g.f_debug & 1 as u32 != 0 {
                                            unsafe {
                                                fprintf(__stderrp,
                                                    c"TEXT \'%s\'\n".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { sqlite3_column_text(p_stmt, i) })
                                            };
                                        }
                                        break '__s7;
                                    }
                                    {
                                        let n: i32 = unsafe { sqlite3_column_bytes(p_stmt, i) };
                                        let z: *const u8 =
                                            unsafe { sqlite3_column_blob(p_stmt, i) } as *const u8;
                                        hash_step(c"4".as_ptr() as *const u8, 1 as u32);
                                        hash_step(z, n as u32);
                                        if g.f_debug & 1 as u32 != 0 {
                                            eprintln!("BLOB ({} bytes)", n as i32);
                                        }
                                        break '__s7;
                                    }
                                }
                                1 => {
                                    {
                                        let mut u: Sqlite3Uint64 = 0 as Sqlite3Uint64;
                                        let mut j: i32 = 0;
                                        let mut x: [u8; 8] = [0; 8];
                                        let mut v: Sqlite3Int64 =
                                            unsafe { sqlite3_column_int64(p_stmt, i) };
                                        unsafe {
                                            memcpy(&raw mut u as *mut (), &raw mut v as *const (),
                                                8 as u64)
                                        };
                                        {
                                            j = 7;
                                            '__b8: loop {
                                                if !(j >= 0) { break '__b8; }
                                                '__c8: loop {
                                                    x[j as usize] = (u & 255 as Sqlite3Uint64) as u8;
                                                    u >>= 8 as Sqlite3Uint64;
                                                    break '__c8;
                                                }
                                                { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                                            }
                                        }
                                        hash_step(c"1".as_ptr() as *const u8, 1 as u32);
                                        hash_step(&raw mut x[0 as usize] as *mut u8 as *const u8,
                                            8 as u32);
                                        if g.f_debug & 1 as u32 != 0 {
                                            unsafe {
                                                fprintf(__stderrp,
                                                    c"INT %s\n".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { sqlite3_column_text(p_stmt, i) })
                                            };
                                        }
                                        break '__s7;
                                    }
                                    {
                                        let mut u: Sqlite3Uint64 = 0 as Sqlite3Uint64;
                                        let mut j: i32 = 0;
                                        let mut x: [u8; 8] = [0; 8];
                                        let mut r: f64 =
                                            unsafe { sqlite3_column_double(p_stmt, i) };
                                        unsafe {
                                            memcpy(&raw mut u as *mut (), &raw mut r as *const (),
                                                8 as u64)
                                        };
                                        {
                                            j = 7;
                                            '__b9: loop {
                                                if !(j >= 0) { break '__b9; }
                                                '__c9: loop {
                                                    x[j as usize] = (u & 255 as Sqlite3Uint64) as u8;
                                                    u >>= 8 as Sqlite3Uint64;
                                                    break '__c9;
                                                }
                                                { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                                            }
                                        }
                                        hash_step(c"2".as_ptr() as *const u8, 1 as u32);
                                        hash_step(&raw mut x[0 as usize] as *mut u8 as *const u8,
                                            8 as u32);
                                        if g.f_debug & 1 as u32 != 0 {
                                            unsafe {
                                                fprintf(__stderrp,
                                                    c"FLOAT %s\n".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { sqlite3_column_text(p_stmt, i) })
                                            };
                                        }
                                        break '__s7;
                                    }
                                    {
                                        let n: i32 = unsafe { sqlite3_column_bytes(p_stmt, i) };
                                        let z: *const u8 =
                                            unsafe { sqlite3_column_text(p_stmt, i) };
                                        hash_step(c"3".as_ptr() as *const u8, 1 as u32);
                                        hash_step(z, n as u32);
                                        if g.f_debug & 1 as u32 != 0 {
                                            unsafe {
                                                fprintf(__stderrp,
                                                    c"TEXT \'%s\'\n".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { sqlite3_column_text(p_stmt, i) })
                                            };
                                        }
                                        break '__s7;
                                    }
                                    {
                                        let n: i32 = unsafe { sqlite3_column_bytes(p_stmt, i) };
                                        let z: *const u8 =
                                            unsafe { sqlite3_column_blob(p_stmt, i) } as *const u8;
                                        hash_step(c"4".as_ptr() as *const u8, 1 as u32);
                                        hash_step(z, n as u32);
                                        if g.f_debug & 1 as u32 != 0 {
                                            eprintln!("BLOB ({} bytes)", n as i32);
                                        }
                                        break '__s7;
                                    }
                                }
                                2 => {
                                    {
                                        let mut u: Sqlite3Uint64 = 0 as Sqlite3Uint64;
                                        let mut j: i32 = 0;
                                        let mut x: [u8; 8] = [0; 8];
                                        let mut r: f64 =
                                            unsafe { sqlite3_column_double(p_stmt, i) };
                                        unsafe {
                                            memcpy(&raw mut u as *mut (), &raw mut r as *const (),
                                                8 as u64)
                                        };
                                        {
                                            j = 7;
                                            '__b9: loop {
                                                if !(j >= 0) { break '__b9; }
                                                '__c9: loop {
                                                    x[j as usize] = (u & 255 as Sqlite3Uint64) as u8;
                                                    u >>= 8 as Sqlite3Uint64;
                                                    break '__c9;
                                                }
                                                { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                                            }
                                        }
                                        hash_step(c"2".as_ptr() as *const u8, 1 as u32);
                                        hash_step(&raw mut x[0 as usize] as *mut u8 as *const u8,
                                            8 as u32);
                                        if g.f_debug & 1 as u32 != 0 {
                                            unsafe {
                                                fprintf(__stderrp,
                                                    c"FLOAT %s\n".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { sqlite3_column_text(p_stmt, i) })
                                            };
                                        }
                                        break '__s7;
                                    }
                                    {
                                        let n: i32 = unsafe { sqlite3_column_bytes(p_stmt, i) };
                                        let z: *const u8 =
                                            unsafe { sqlite3_column_text(p_stmt, i) };
                                        hash_step(c"3".as_ptr() as *const u8, 1 as u32);
                                        hash_step(z, n as u32);
                                        if g.f_debug & 1 as u32 != 0 {
                                            unsafe {
                                                fprintf(__stderrp,
                                                    c"TEXT \'%s\'\n".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { sqlite3_column_text(p_stmt, i) })
                                            };
                                        }
                                        break '__s7;
                                    }
                                    {
                                        let n: i32 = unsafe { sqlite3_column_bytes(p_stmt, i) };
                                        let z: *const u8 =
                                            unsafe { sqlite3_column_blob(p_stmt, i) } as *const u8;
                                        hash_step(c"4".as_ptr() as *const u8, 1 as u32);
                                        hash_step(z, n as u32);
                                        if g.f_debug & 1 as u32 != 0 {
                                            eprintln!("BLOB ({} bytes)", n as i32);
                                        }
                                        break '__s7;
                                    }
                                }
                                3 => {
                                    {
                                        let n: i32 = unsafe { sqlite3_column_bytes(p_stmt, i) };
                                        let z: *const u8 =
                                            unsafe { sqlite3_column_text(p_stmt, i) };
                                        hash_step(c"3".as_ptr() as *const u8, 1 as u32);
                                        hash_step(z, n as u32);
                                        if g.f_debug & 1 as u32 != 0 {
                                            unsafe {
                                                fprintf(__stderrp,
                                                    c"TEXT \'%s\'\n".as_ptr() as *mut i8 as *const i8,
                                                    unsafe { sqlite3_column_text(p_stmt, i) })
                                            };
                                        }
                                        break '__s7;
                                    }
                                    {
                                        let n: i32 = unsafe { sqlite3_column_bytes(p_stmt, i) };
                                        let z: *const u8 =
                                            unsafe { sqlite3_column_blob(p_stmt, i) } as *const u8;
                                        hash_step(c"4".as_ptr() as *const u8, 1 as u32);
                                        hash_step(z, n as u32);
                                        if g.f_debug & 1 as u32 != 0 {
                                            eprintln!("BLOB ({} bytes)", n as i32);
                                        }
                                        break '__s7;
                                    }
                                }
                                4 => {
                                    {
                                        let n: i32 = unsafe { sqlite3_column_bytes(p_stmt, i) };
                                        let z: *const u8 =
                                            unsafe { sqlite3_column_blob(p_stmt, i) } as *const u8;
                                        hash_step(c"4".as_ptr() as *const u8, 1 as u32);
                                        hash_step(z, n as u32);
                                        if g.f_debug & 1 as u32 != 0 {
                                            eprintln!("BLOB ({} bytes)", n as i32);
                                        }
                                        break '__s7;
                                    }
                                }
                                _ => {}
                            }
                        }
                        break '__c6;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        unsafe { sqlite3_finalize(p_stmt) };
    }
}
extern "C" fn show_help() -> () {
    unsafe {
        unsafe {
            printf(c"Usage: %s [options] FILE ...\n".as_ptr() as *mut i8 as
                    *const i8, g.z_argv0)
        };
        unsafe {
            printf(c"Compute a SHA1 hash on the content of database FILE.  System tables such as\nsqlite_stat1, sqlite_stat4, and sqlite_sequence are omitted from the hash.\nOptions:\n   --debug N           Set debugging flags to N (experts only)\n   --like PATTERN      Only hash tables whose name is LIKE the pattern\n   --schema-only       Only hash the schema - omit table content\n   --without-schema    Only hash table content - omit the schema\n".as_ptr()
                        as *mut i8 as *const i8)
        };
    }
}
extern "C" fn __main_inner(argc: i32, argv: *mut *mut i8) -> Result<(), i32> {
    unsafe {
        let mut z_db: *const i8 = core::ptr::null();
        let mut i: i32 = 0;
        let mut rc: i32 = 0;
        let mut z_err_msg: *mut i8 = core::ptr::null_mut();
        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
        let mut z_like: *const i8 = core::ptr::null();
        let mut omit_schema: i32 = 0;
        let mut omit_content: i32 = 0;
        let mut n_file: i32 = 0;
        g.z_argv0 = unsafe { *argv.offset(0 as isize) } as *const i8;
        unsafe { sqlite3_config(1) };
        {
            i = 1;
            '__b10: loop {
                if !(i < argc) { break '__b10; }
                '__c10: loop {
                    let mut z: *const i8 =
                        unsafe { *argv.offset(i as isize) } as *const i8;
                    if unsafe { *z.offset(0 as isize) } as i32 == '-' as i32 {
                        {
                            let __p = &mut z;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        };
                        if unsafe { *z.offset(0 as isize) } as i32 == '-' as i32 {
                            {
                                let __p = &mut z;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                        }
                        if unsafe {
                                    strcmp(z, c"debug".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            if i == argc - 1 {
                                unsafe {
                                    cmdline_error(c"missing argument to %s".as_ptr() as *mut i8
                                            as *const i8, unsafe { *argv.offset(i as isize) })
                                };
                            }
                            g.f_debug =
                                unsafe {
                                        strtol(unsafe {
                                                    *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                                } as *const i8, core::ptr::null_mut(), 0)
                                    } as u32;
                        } else if unsafe {
                                    strcmp(z, c"help".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            show_help();
                            return Ok(());
                        } else if unsafe {
                                    strcmp(z, c"like".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            if i == argc - 1 {
                                unsafe {
                                    cmdline_error(c"missing argument to %s".as_ptr() as *mut i8
                                            as *const i8, unsafe { *argv.offset(i as isize) })
                                };
                            }
                            if z_like != core::ptr::null() {
                                unsafe {
                                    cmdline_error(c"only one --like allowed".as_ptr() as *mut i8
                                            as *const i8)
                                };
                            }
                            z_like =
                                unsafe {
                                        *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                    } as *const i8;
                        } else if unsafe {
                                    strcmp(z, c"schema-only".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            omit_content = 1;
                        } else if unsafe {
                                    strcmp(z,
                                        c"without-schema".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            omit_schema = 1;
                        } else {
                            unsafe {
                                cmdline_error(c"unknown option: %s".as_ptr() as *mut i8 as
                                        *const i8, unsafe { *argv.offset(i as isize) })
                            };
                        }
                    } else {
                        { let __p = &mut n_file; let __t = *__p; *__p += 1; __t };
                        if n_file < i {
                            unsafe {
                                *argv.offset(n_file as isize) =
                                    unsafe { *argv.offset(i as isize) }
                            };
                        }
                    }
                    break '__c10;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if n_file == 0 {
            unsafe {
                cmdline_error(c"no input files specified - nothing to do".as_ptr()
                            as *mut i8 as *const i8)
            };
        }
        if omit_schema != 0 && omit_content != 0 {
            unsafe {
                cmdline_error(c"only one of --without-schema and --omit-schema allowed".as_ptr()
                            as *mut i8 as *const i8)
            };
        }
        if z_like == core::ptr::null() {
            z_like = c"%".as_ptr() as *mut i8 as *const i8;
        }
        {
            i = 1;
            '__b11: loop {
                if !(i <= n_file) { break '__b11; }
                '__c11: loop {
                    z_db = unsafe { *argv.offset(i as isize) } as *const i8;
                    rc =
                        unsafe {
                            sqlite3_open_v2(z_db, &mut g.db, open_flags,
                                core::ptr::null())
                        };
                    if rc != 0 {
                        unsafe {
                            fprintf(__stderrp,
                                c"cannot open database file \'%s\'\n".as_ptr() as *mut i8 as
                                    *const i8, z_db)
                        };
                        break '__c11;
                    }
                    rc =
                        unsafe {
                            sqlite3_exec(g.db,
                                c"SELECT * FROM sqlite_schema".as_ptr() as *mut i8 as
                                    *const i8, None, core::ptr::null_mut(), &mut z_err_msg)
                        };
                    if rc != 0 || !(z_err_msg).is_null() {
                        unsafe { sqlite3_close(g.db) };
                        g.db = core::ptr::null_mut();
                        unsafe {
                            fprintf(__stderrp,
                                c"\'%s\' is not a valid SQLite database\n".as_ptr() as
                                        *mut i8 as *const i8, z_db)
                        };
                        break '__c11;
                    }
                    hash_init();
                    if (omit_content == 0) as i32 != 0 {
                        p_stmt =
                            unsafe {
                                db_prepare(c"SELECT name FROM sqlite_schema\n WHERE type=\'table\' AND sql NOT LIKE \'CREATE VIRTUAL%%\'\n   AND name NOT LIKE \'sqlite_%%\'\n   AND name LIKE \'%q\'\n ORDER BY name COLLATE nocase;\n".as_ptr()
                                            as *mut i8 as *const i8, z_like)
                            };
                        while 100 == unsafe { sqlite3_step(p_stmt) } {
                            unsafe {
                                hash_one_query(c"SELECT * FROM \"%w\"".as_ptr() as *mut i8
                                        as *const i8, unsafe { sqlite3_column_text(p_stmt, 0) })
                            };
                        }
                        unsafe { sqlite3_finalize(p_stmt) };
                    }
                    if (omit_schema == 0) as i32 != 0 {
                        unsafe {
                            hash_one_query(c"SELECT type, name, tbl_name, sql FROM sqlite_schema\n WHERE tbl_name LIKE \'%q\'\n ORDER BY name COLLATE nocase;\n".as_ptr()
                                        as *mut i8 as *const i8, z_like)
                        };
                    }
                    hash_finish(z_db);
                    unsafe { sqlite3_close(g.db) };
                    break '__c11;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return Ok(());
    }
}
static mut one: i32 = 1;
static z_encode: [i8; 17] =
    [48 as i8, 49 as i8, 50 as i8, 51 as i8, 52 as i8, 53 as i8, 54 as i8,
            55 as i8, 56 as i8, 57 as i8, 97 as i8, 98 as i8, 99 as i8,
            100 as i8, 101 as i8, 102 as i8, 0 as i8];
static open_flags: i32 = (2 | 64) as i32;
#[unsafe(no_mangle)]
pub extern "C" fn main(argc: i32, argv: *mut *mut i8) -> i32 {
    let __r: Result<(), i32> = __main_inner(argc, argv);
    if __r.is_ok() { return 0; }
    return __r.unwrap_err();
}
extern "C" {
    fn __transpiler_isa(child: i32, ancestor: i32)
    -> bool;
    static sqlite3_version: [i8; 0];
    fn sqlite3_libversion()
    -> *const i8;
    fn sqlite3_sourceid()
    -> *const i8;
    fn sqlite3_libversion_number()
    -> i32;
    fn sqlite3_compileoption_used(z_opt_name_1: *const i8)
    -> i32;
    fn sqlite3_compileoption_get(n_1: i32)
    -> *const i8;
    fn sqlite3_threadsafe()
    -> i32;
    fn sqlite3_close(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_close_v2(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_exec(_: *mut Sqlite3, sql: *const i8,
    callback:
        Option<unsafe extern "C" fn(*mut (), i32, *mut *mut i8, *mut *mut i8)
            -> i32>, _: *mut (), errmsg: *mut *mut i8)
    -> i32;
    fn sqlite3_initialize()
    -> i32;
    fn sqlite3_shutdown()
    -> i32;
    fn sqlite3_os_init()
    -> i32;
    fn sqlite3_os_end()
    -> i32;
    fn sqlite3_config(_: i32, ...)
    -> i32;
    fn sqlite3_db_config(_: *mut Sqlite3, op: i32, ...)
    -> i32;
    fn sqlite3_extended_result_codes(_: *mut Sqlite3, onoff: i32)
    -> i32;
    fn sqlite3_last_insert_rowid(_: *mut Sqlite3)
    -> Sqlite3Int64;
    fn sqlite3_set_last_insert_rowid(_: *mut Sqlite3, _: Sqlite3Int64)
    -> ();
    fn sqlite3_changes(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_changes64(_: *mut Sqlite3)
    -> Sqlite3Int64;
    fn sqlite3_total_changes(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_total_changes64(_: *mut Sqlite3)
    -> Sqlite3Int64;
    fn sqlite3_interrupt(_: *mut Sqlite3)
    -> ();
    fn sqlite3_is_interrupted(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_complete(sql: *const i8)
    -> i32;
    fn sqlite3_complete16(sql: *const ())
    -> i32;
    fn sqlite3_incomplete(sql: *const i8)
    -> Sqlite3Int64;
    fn sqlite3_busy_handler(_: *mut Sqlite3,
    _: Option<unsafe extern "C" fn(*mut (), i32) -> i32>, _: *mut ())
    -> i32;
    fn sqlite3_busy_timeout(_: *mut Sqlite3, ms: i32)
    -> i32;
    fn sqlite3_setlk_timeout(_: *mut Sqlite3, ms: i32, flags: i32)
    -> i32;
    fn sqlite3_get_table(db: *mut Sqlite3, z_sql_1: *const i8,
    paz_result_1: *mut *mut *mut i8, pn_row_1: *mut i32,
    pn_column_1: *mut i32, pz_errmsg_1: *mut *mut i8)
    -> i32;
    fn sqlite3_free_table(result: *mut *mut i8)
    -> ();
    fn sqlite3_mprintf(_: *const i8, ...)
    -> *mut i8;
    fn sqlite3_vmprintf(_: *const i8, _: *mut i8)
    -> *mut i8;
    fn sqlite3_snprintf(_: i32, _: *mut i8, _: *const i8, ...)
    -> *mut i8;
    fn sqlite3_vsnprintf(_: i32, _: *mut i8, _: *const i8, _: *mut i8)
    -> *mut i8;
    fn sqlite3_malloc(_: i32)
    -> *mut ();
    fn sqlite3_malloc64(_: Sqlite3Uint64)
    -> *mut ();
    fn sqlite3_realloc(_: *mut (), _: i32)
    -> *mut ();
    fn sqlite3_realloc64(_: *mut (), _: Sqlite3Uint64)
    -> *mut ();
    fn sqlite3_free(_: *mut ())
    -> ();
    fn sqlite3_msize(_: *mut ())
    -> Sqlite3Uint64;
    fn sqlite3_memory_used()
    -> Sqlite3Int64;
    fn sqlite3_memory_highwater(reset_flag_1: i32)
    -> Sqlite3Int64;
    fn sqlite3_randomness(n_1: i32, p_1: *mut ())
    -> ();
    fn sqlite3_set_authorizer(_: *mut Sqlite3,
    x_auth_1:
        Option<unsafe extern "C" fn(*mut (), i32, *const i8, *const i8,
            *const i8, *const i8) -> i32>, p_user_data_1: *mut ())
    -> i32;
    fn sqlite3_trace(_: *mut Sqlite3,
    x_trace_1: Option<unsafe extern "C" fn(*mut (), *const i8) -> ()>,
    _: *mut ())
    -> *mut ();
    fn sqlite3_profile(_: *mut Sqlite3,
    x_profile_1: Option<unsafe extern "C" fn(*mut (), *const i8, u64) -> ()>,
    _: *mut ())
    -> *mut ();
    fn sqlite3_trace_v2(_: *mut Sqlite3, u_mask_1: u32,
    x_callback_1:
        Option<unsafe extern "C" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
    p_ctx_1: *mut ())
    -> i32;
    fn sqlite3_progress_handler(_: *mut Sqlite3, _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> i32>, _: *mut ())
    -> ();
    fn sqlite3_open(filename: *const i8, pp_db_1: *mut *mut Sqlite3)
    -> i32;
    fn sqlite3_open16(filename: *const (), pp_db_1: *mut *mut Sqlite3)
    -> i32;
    fn sqlite3_open_v2(filename: *const i8, pp_db_1: *mut *mut Sqlite3,
    flags: i32, z_vfs_1: *const i8)
    -> i32;
    fn sqlite3_uri_parameter(z: Sqlite3Filename, z_param_1: *const i8)
    -> *const i8;
    fn sqlite3_uri_boolean(z: Sqlite3Filename, z_param_1: *const i8,
    b_default_1: i32)
    -> i32;
    fn sqlite3_uri_int64(_: Sqlite3Filename, _: *const i8, _: Sqlite3Int64)
    -> Sqlite3Int64;
    fn sqlite3_uri_key(z: Sqlite3Filename, n_1: i32)
    -> *const i8;
    fn sqlite3_filename_database(_: Sqlite3Filename)
    -> *const i8;
    fn sqlite3_filename_journal(_: Sqlite3Filename)
    -> *const i8;
    fn sqlite3_filename_wal(_: Sqlite3Filename)
    -> *const i8;
    fn sqlite3_database_file_object(_: *const i8)
    -> *mut Sqlite3File;
    fn sqlite3_create_filename(z_database_1: *const i8,
    z_journal_1: *const i8, z_wal_1: *const i8, n_param_1: i32,
    az_param_1: *mut *const i8)
    -> Sqlite3Filename;
    fn sqlite3_free_filename(_: Sqlite3Filename)
    -> ();
    fn sqlite3_errcode(db: *mut Sqlite3)
    -> i32;
    fn sqlite3_extended_errcode(db: *mut Sqlite3)
    -> i32;
    fn sqlite3_errmsg(_: *mut Sqlite3)
    -> *const i8;
    fn sqlite3_errmsg16(_: *mut Sqlite3)
    -> *const ();
    fn sqlite3_errstr(_: i32)
    -> *const i8;
    fn sqlite3_error_offset(db: *mut Sqlite3)
    -> i32;
    fn sqlite3_set_errmsg(db: *mut Sqlite3, errcode: i32,
    z_err_msg_1: *const i8)
    -> i32;
    fn sqlite3_limit(_: *mut Sqlite3, id: i32, new_val_1: i32)
    -> i32;
    fn sqlite3_prepare(db: *mut Sqlite3, z_sql_1: *const i8, n_byte_1: i32,
    pp_stmt_1: *mut *mut Sqlite3Stmt, pz_tail_1: *mut *const i8)
    -> i32;
    fn sqlite3_prepare_v2(db: *mut Sqlite3, z_sql_1: *const i8, n_byte_1: i32,
    pp_stmt_1: *mut *mut Sqlite3Stmt, pz_tail_1: *mut *const i8)
    -> i32;
    fn sqlite3_prepare_v3(db: *mut Sqlite3, z_sql_1: *const i8, n_byte_1: i32,
    prep_flags_1: u32, pp_stmt_1: *mut *mut Sqlite3Stmt,
    pz_tail_1: *mut *const i8)
    -> i32;
    fn sqlite3_prepare16(db: *mut Sqlite3, z_sql_1: *const (), n_byte_1: i32,
    pp_stmt_1: *mut *mut Sqlite3Stmt, pz_tail_1: *mut *const ())
    -> i32;
    fn sqlite3_prepare16_v2(db: *mut Sqlite3, z_sql_1: *const (),
    n_byte_1: i32, pp_stmt_1: *mut *mut Sqlite3Stmt,
    pz_tail_1: *mut *const ())
    -> i32;
    fn sqlite3_prepare16_v3(db: *mut Sqlite3, z_sql_1: *const (),
    n_byte_1: i32, prep_flags_1: u32, pp_stmt_1: *mut *mut Sqlite3Stmt,
    pz_tail_1: *mut *const ())
    -> i32;
    fn sqlite3_sql(p_stmt_1: *mut Sqlite3Stmt)
    -> *const i8;
    fn sqlite3_expanded_sql(p_stmt_1: *mut Sqlite3Stmt)
    -> *mut i8;
    fn sqlite3_stmt_readonly(p_stmt_1: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_stmt_isexplain(p_stmt_1: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_stmt_explain(p_stmt_1: *mut Sqlite3Stmt, e_mode_1: i32)
    -> i32;
    fn sqlite3_stmt_busy(_: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_bind_blob(_: *mut Sqlite3Stmt, _: i32, _: *const (), n: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_bind_blob64(_: *mut Sqlite3Stmt, _: i32, _: *const (),
    _: Sqlite3Uint64, _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_bind_double(_: *mut Sqlite3Stmt, _: i32, _: f64)
    -> i32;
    fn sqlite3_bind_int(_: *mut Sqlite3Stmt, _: i32, _: i32)
    -> i32;
    fn sqlite3_bind_int64(_: *mut Sqlite3Stmt, _: i32, _: Sqlite3Int64)
    -> i32;
    fn sqlite3_bind_null(_: *mut Sqlite3Stmt, _: i32)
    -> i32;
    fn sqlite3_bind_text(_: *mut Sqlite3Stmt, _: i32, _: *const i8, _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_bind_text16(_: *mut Sqlite3Stmt, _: i32, _: *const (), _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_bind_text64(_: *mut Sqlite3Stmt, _: i32, _: *const i8,
    _: Sqlite3Uint64, _: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    encoding: u8)
    -> i32;
    fn sqlite3_bind_value(_: *mut Sqlite3Stmt, _: i32, _: *const Sqlite3Value)
    -> i32;
    fn sqlite3_bind_pointer(_: *mut Sqlite3Stmt, _: i32, _: *mut (),
    _: *const i8, _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_bind_zeroblob(_: *mut Sqlite3Stmt, _: i32, n: i32)
    -> i32;
    fn sqlite3_bind_zeroblob64(_: *mut Sqlite3Stmt, _: i32, _: Sqlite3Uint64)
    -> i32;
    fn sqlite3_bind_parameter_count(_: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_bind_parameter_name(_: *mut Sqlite3Stmt, _: i32)
    -> *const i8;
    fn sqlite3_bind_parameter_index(_: *mut Sqlite3Stmt, z_name_1: *const i8)
    -> i32;
    fn sqlite3_clear_bindings(_: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_column_count(p_stmt_1: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_column_name(_: *mut Sqlite3Stmt, n_1: i32)
    -> *const i8;
    fn sqlite3_column_name16(_: *mut Sqlite3Stmt, n_1: i32)
    -> *const ();
    fn sqlite3_column_database_name(_: *mut Sqlite3Stmt, _: i32)
    -> *const i8;
    fn sqlite3_column_database_name16(_: *mut Sqlite3Stmt, _: i32)
    -> *const ();
    fn sqlite3_column_table_name(_: *mut Sqlite3Stmt, _: i32)
    -> *const i8;
    fn sqlite3_column_table_name16(_: *mut Sqlite3Stmt, _: i32)
    -> *const ();
    fn sqlite3_column_origin_name(_: *mut Sqlite3Stmt, _: i32)
    -> *const i8;
    fn sqlite3_column_origin_name16(_: *mut Sqlite3Stmt, _: i32)
    -> *const ();
    fn sqlite3_column_decltype(_: *mut Sqlite3Stmt, _: i32)
    -> *const i8;
    fn sqlite3_column_decltype16(_: *mut Sqlite3Stmt, _: i32)
    -> *const ();
    fn sqlite3_step(_: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_data_count(p_stmt_1: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_column_blob(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> *const ();
    fn sqlite3_column_double(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> f64;
    fn sqlite3_column_int(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> i32;
    fn sqlite3_column_int64(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> Sqlite3Int64;
    fn sqlite3_column_text(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> *const u8;
    fn sqlite3_column_text16(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> *const ();
    fn sqlite3_column_value(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> *mut Sqlite3Value;
    fn sqlite3_column_bytes(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> i32;
    fn sqlite3_column_bytes16(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> i32;
    fn sqlite3_column_type(_: *mut Sqlite3Stmt, i_col_1: i32)
    -> i32;
    fn sqlite3_finalize(p_stmt_1: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_reset(p_stmt_1: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_create_function(db: *mut Sqlite3, z_function_name_1: *const i8,
    n_arg_1: i32, e_text_rep_1: i32, p_app_1: *mut (),
    x_func_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_step_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_final_1: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>)
    -> i32;
    fn sqlite3_create_function16(db: *mut Sqlite3,
    z_function_name_1: *const (), n_arg_1: i32, e_text_rep_1: i32,
    p_app_1: *mut (),
    x_func_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_step_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_final_1: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>)
    -> i32;
    fn sqlite3_create_function_v2(db: *mut Sqlite3,
    z_function_name_1: *const i8, n_arg_1: i32, e_text_rep_1: i32,
    p_app_1: *mut (),
    x_func_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_step_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_final_1: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>,
    x_destroy_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_create_window_function(db: *mut Sqlite3,
    z_function_name_1: *const i8, n_arg_1: i32, e_text_rep_1: i32,
    p_app_1: *mut (),
    x_step_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_final_1: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>,
    x_value_1: Option<unsafe extern "C" fn(*mut Sqlite3Context) -> ()>,
    x_inverse_1:
        Option<unsafe extern "C" fn(*mut Sqlite3Context, i32,
            *mut *mut Sqlite3Value) -> ()>,
    x_destroy_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_aggregate_count(_: *mut Sqlite3Context)
    -> i32;
    fn sqlite3_expired(_: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_transfer_bindings(_: *mut Sqlite3Stmt, _: *mut Sqlite3Stmt)
    -> i32;
    fn sqlite3_global_recover()
    -> i32;
    fn sqlite3_thread_cleanup()
    -> ();
    fn sqlite3_memory_alarm(_:
        Option<unsafe extern "C" fn(*mut (), i64, i32) -> ()>, _: *mut (),
    _: Sqlite3Int64)
    -> i32;
    fn sqlite3_value_blob(_: *mut Sqlite3Value)
    -> *const ();
    fn sqlite3_value_double(_: *mut Sqlite3Value)
    -> f64;
    fn sqlite3_value_int(_: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_value_int64(_: *mut Sqlite3Value)
    -> Sqlite3Int64;
    fn sqlite3_value_pointer(_: *mut Sqlite3Value, _: *const i8)
    -> *mut ();
    fn sqlite3_value_text(_: *mut Sqlite3Value)
    -> *const u8;
    fn sqlite3_value_text16(_: *mut Sqlite3Value)
    -> *const ();
    fn sqlite3_value_text16le(_: *mut Sqlite3Value)
    -> *const ();
    fn sqlite3_value_text16be(_: *mut Sqlite3Value)
    -> *const ();
    fn sqlite3_value_bytes(_: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_value_bytes16(_: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_value_type(_: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_value_numeric_type(_: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_value_nochange(_: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_value_frombind(_: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_value_encoding(_: *mut Sqlite3Value)
    -> i32;
    fn sqlite3_value_subtype(_: *mut Sqlite3Value)
    -> u32;
    fn sqlite3_value_dup(_: *const Sqlite3Value)
    -> *mut Sqlite3Value;
    fn sqlite3_value_free(_: *mut Sqlite3Value)
    -> ();
    fn sqlite3_aggregate_context(_: *mut Sqlite3Context, n_bytes_1: i32)
    -> *mut ();
    fn sqlite3_user_data(_: *mut Sqlite3Context)
    -> *mut ();
    fn sqlite3_context_db_handle(_: *mut Sqlite3Context)
    -> *mut Sqlite3;
    fn sqlite3_get_auxdata(_: *mut Sqlite3Context, n_1: i32)
    -> *mut ();
    fn sqlite3_set_auxdata(_: *mut Sqlite3Context, n_1: i32, _: *mut (),
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_get_clientdata(_: *mut Sqlite3, _: *const i8)
    -> *mut ();
    fn sqlite3_set_clientdata(_: *mut Sqlite3, _: *const i8, _: *mut (),
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_result_blob(_: *mut Sqlite3Context, _: *const (), _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_blob64(_: *mut Sqlite3Context, _: *const (),
    _: Sqlite3Uint64, _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_double(_: *mut Sqlite3Context, _: f64)
    -> ();
    fn sqlite3_result_error(_: *mut Sqlite3Context, _: *const i8, _: i32)
    -> ();
    fn sqlite3_result_error16(_: *mut Sqlite3Context, _: *const (), _: i32)
    -> ();
    fn sqlite3_result_error_toobig(_: *mut Sqlite3Context)
    -> ();
    fn sqlite3_result_error_nomem(_: *mut Sqlite3Context)
    -> ();
    fn sqlite3_result_error_code(_: *mut Sqlite3Context, _: i32)
    -> ();
    fn sqlite3_result_int(_: *mut Sqlite3Context, _: i32)
    -> ();
    fn sqlite3_result_int64(_: *mut Sqlite3Context, _: Sqlite3Int64)
    -> ();
    fn sqlite3_result_null(_: *mut Sqlite3Context)
    -> ();
    fn sqlite3_result_text(_: *mut Sqlite3Context, _: *const i8, _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_text64(_: *mut Sqlite3Context, z: *const i8,
    n: Sqlite3Uint64, _: Option<unsafe extern "C" fn(*mut ()) -> ()>,
    encoding: u8)
    -> ();
    fn sqlite3_result_text16(_: *mut Sqlite3Context, _: *const (), _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_text16le(_: *mut Sqlite3Context, _: *const (), _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_text16be(_: *mut Sqlite3Context, _: *const (), _: i32,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_value(_: *mut Sqlite3Context, _: *mut Sqlite3Value)
    -> ();
    fn sqlite3_result_pointer(_: *mut Sqlite3Context, _: *mut (),
    _: *const i8, _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> ();
    fn sqlite3_result_zeroblob(_: *mut Sqlite3Context, n: i32)
    -> ();
    fn sqlite3_result_zeroblob64(_: *mut Sqlite3Context, n: Sqlite3Uint64)
    -> i32;
    fn sqlite3_result_subtype(_: *mut Sqlite3Context, _: u32)
    -> ();
    fn sqlite3_create_collation(_: *mut Sqlite3, z_name_1: *const i8,
    e_text_rep_1: i32, p_arg_1: *mut (),
    x_compare_1:
        Option<unsafe extern "C" fn(*mut (), i32, *const (), i32, *const ())
            -> i32>)
    -> i32;
    fn sqlite3_create_collation_v2(_: *mut Sqlite3, z_name_1: *const i8,
    e_text_rep_1: i32, p_arg_1: *mut (),
    x_compare_1:
        Option<unsafe extern "C" fn(*mut (), i32, *const (), i32, *const ())
            -> i32>, x_destroy_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_create_collation16(_: *mut Sqlite3, z_name_1: *const (),
    e_text_rep_1: i32, p_arg_1: *mut (),
    x_compare_1:
        Option<unsafe extern "C" fn(*mut (), i32, *const (), i32, *const ())
            -> i32>)
    -> i32;
    fn sqlite3_collation_needed(_: *mut Sqlite3, _: *mut (),
    _:
        Option<unsafe extern "C" fn(*mut (), *mut Sqlite3, i32, *const i8)
            -> ()>)
    -> i32;
    fn sqlite3_collation_needed16(_: *mut Sqlite3, _: *mut (),
    _:
        Option<unsafe extern "C" fn(*mut (), *mut Sqlite3, i32, *const ())
            -> ()>)
    -> i32;
    fn sqlite3_sleep(_: i32)
    -> i32;
    static mut sqlite3_temp_directory: *mut i8;
    static mut sqlite3_data_directory: *mut i8;
    fn sqlite3_win32_set_directory(type__1: u64, z_value_1: *mut ())
    -> i32;
    fn sqlite3_win32_set_directory8(type__1: u64, z_value_1: *const i8)
    -> i32;
    fn sqlite3_win32_set_directory16(type__1: u64, z_value_1: *const ())
    -> i32;
    fn sqlite3_get_autocommit(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_db_handle(_: *mut Sqlite3Stmt)
    -> *mut Sqlite3;
    fn sqlite3_db_name(db: *mut Sqlite3, n_1: i32)
    -> *const i8;
    fn sqlite3_db_filename(db: *mut Sqlite3, z_db_name_1: *const i8)
    -> Sqlite3Filename;
    fn sqlite3_db_readonly(db: *mut Sqlite3, z_db_name_1: *const i8)
    -> i32;
    fn sqlite3_txn_state(_: *mut Sqlite3, z_schema_1: *const i8)
    -> i32;
    fn sqlite3_next_stmt(p_db_1: *mut Sqlite3, p_stmt_1: *mut Sqlite3Stmt)
    -> *mut Sqlite3Stmt;
    fn sqlite3_commit_hook(_: *mut Sqlite3,
    _: Option<unsafe extern "C" fn(*mut ()) -> i32>, _: *mut ())
    -> *mut ();
    fn sqlite3_rollback_hook(_: *mut Sqlite3,
    _: Option<unsafe extern "C" fn(*mut ()) -> ()>, _: *mut ())
    -> *mut ();
    fn sqlite3_autovacuum_pages(db: *mut Sqlite3,
    _: Option<unsafe extern "C" fn(*mut (), *const i8, u32, u32, u32) -> u32>,
    _: *mut (), _: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_update_hook(_: *mut Sqlite3,
    _:
        Option<unsafe extern "C" fn(*mut (), i32, *const i8, *const i8, i64)
            -> ()>, _: *mut ())
    -> *mut ();
    fn sqlite3_enable_shared_cache(_: i32)
    -> i32;
    fn sqlite3_release_memory(_: i32)
    -> i32;
    fn sqlite3_db_release_memory(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_soft_heap_limit64(n_1: Sqlite3Int64)
    -> Sqlite3Int64;
    fn sqlite3_hard_heap_limit64(n_1: Sqlite3Int64)
    -> Sqlite3Int64;
    fn sqlite3_soft_heap_limit(n_1: i32)
    -> ();
    fn sqlite3_table_column_metadata(db: *mut Sqlite3, z_db_name_1: *const i8,
    z_table_name_1: *const i8, z_column_name_1: *const i8,
    pz_data_type_1: *mut *const i8, pz_coll_seq_1: *mut *const i8,
    p_not_null_1: *mut i32, p_primary_key_1: *mut i32, p_autoinc_1: *mut i32)
    -> i32;
    fn sqlite3_load_extension(db: *mut Sqlite3, z_file_1: *const i8,
    z_proc_1: *const i8, pz_err_msg_1: *mut *mut i8)
    -> i32;
    fn sqlite3_enable_load_extension(db: *mut Sqlite3, onoff: i32)
    -> i32;
    fn sqlite3_auto_extension(x_entry_point_1:
        Option<unsafe extern "C" fn() -> ()>)
    -> i32;
    fn sqlite3_cancel_auto_extension(x_entry_point_1:
        Option<unsafe extern "C" fn() -> ()>)
    -> i32;
    fn sqlite3_reset_auto_extension()
    -> ();
    fn sqlite3_create_module(db: *mut Sqlite3, z_name_1: *const i8,
    p: *const Sqlite3Module, p_client_data_1: *mut ())
    -> i32;
    fn sqlite3_create_module_v2(db: *mut Sqlite3, z_name_1: *const i8,
    p: *const Sqlite3Module, p_client_data_1: *mut (),
    x_destroy_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_drop_modules(db: *mut Sqlite3, az_keep_1: *mut *const i8)
    -> i32;
    fn sqlite3_declare_vtab(_: *mut Sqlite3, z_sql_1: *const i8)
    -> i32;
    fn sqlite3_overload_function(_: *mut Sqlite3, z_func_name_1: *const i8,
    n_arg_1: i32)
    -> i32;
    fn sqlite3_blob_open(_: *mut Sqlite3, z_db_1: *const i8,
    z_table_1: *const i8, z_column_1: *const i8, i_row_1: Sqlite3Int64,
    flags: i32, pp_blob_1: *mut *mut Sqlite3Blob)
    -> i32;
    fn sqlite3_blob_reopen(_: *mut Sqlite3Blob, _: Sqlite3Int64)
    -> i32;
    fn sqlite3_blob_close(_: *mut Sqlite3Blob)
    -> i32;
    fn sqlite3_blob_bytes(_: *mut Sqlite3Blob)
    -> i32;
    fn sqlite3_blob_read(_: *mut Sqlite3Blob, z_1: *mut (), n_1: i32,
    i_offset_1: i32)
    -> i32;
    fn sqlite3_blob_write(_: *mut Sqlite3Blob, z: *const (), n: i32,
    i_offset_1: i32)
    -> i32;
    fn sqlite3_vfs_find(z_vfs_name_1: *const i8)
    -> *mut Sqlite3Vfs;
    fn sqlite3_vfs_register(_: *mut Sqlite3Vfs, make_dflt_1: i32)
    -> i32;
    fn sqlite3_vfs_unregister(_: *mut Sqlite3Vfs)
    -> i32;
    fn sqlite3_mutex_alloc(_: i32)
    -> *mut Sqlite3Mutex;
    fn sqlite3_mutex_free(_: *mut Sqlite3Mutex)
    -> ();
    fn sqlite3_mutex_enter(_: *mut Sqlite3Mutex)
    -> ();
    fn sqlite3_mutex_try(_: *mut Sqlite3Mutex)
    -> i32;
    fn sqlite3_mutex_leave(_: *mut Sqlite3Mutex)
    -> ();
    fn sqlite3_mutex_held(_: *mut Sqlite3Mutex)
    -> i32;
    fn sqlite3_mutex_notheld(_: *mut Sqlite3Mutex)
    -> i32;
    fn sqlite3_db_mutex(_: *mut Sqlite3)
    -> *mut Sqlite3Mutex;
    fn sqlite3_file_control(_: *mut Sqlite3, z_db_name_1: *const i8, op: i32,
    _: *mut ())
    -> i32;
    fn sqlite3_test_control(op: i32, ...)
    -> i32;
    fn sqlite3_keyword_count()
    -> i32;
    fn sqlite3_keyword_name(_: i32, _: *mut *const i8, _: *mut i32)
    -> i32;
    fn sqlite3_keyword_check(_: *const i8, _: i32)
    -> i32;
    fn sqlite3_str_new(_: *mut Sqlite3)
    -> *mut Sqlite3Str;
    fn sqlite3_str_finish(_: *mut Sqlite3Str)
    -> *mut i8;
    fn sqlite3_str_free(_: *mut Sqlite3Str)
    -> ();
    fn sqlite3_result_str(_: *mut Sqlite3Context, _: *mut Sqlite3Str, _: i32)
    -> ();
    fn sqlite3_str_appendf(_: *mut Sqlite3Str, z_format_1: *const i8, ...)
    -> ();
    fn sqlite3_str_vappendf(_: *mut Sqlite3Str, z_format_1: *const i8,
    _: *mut i8)
    -> ();
    fn sqlite3_str_append(_: *mut Sqlite3Str, z_in_1: *const i8, n_1: i32)
    -> ();
    fn sqlite3_str_appendall(_: *mut Sqlite3Str, z_in_1: *const i8)
    -> ();
    fn sqlite3_str_appendchar(_: *mut Sqlite3Str, n_1: i32, c_1: i8)
    -> ();
    fn sqlite3_str_reset(_: *mut Sqlite3Str)
    -> ();
    fn sqlite3_str_truncate(_: *mut Sqlite3Str, n_1: i32)
    -> ();
    fn sqlite3_str_errcode(_: *mut Sqlite3Str)
    -> i32;
    fn sqlite3_str_length(_: *mut Sqlite3Str)
    -> i32;
    fn sqlite3_str_value(_: *mut Sqlite3Str)
    -> *mut i8;
    fn sqlite3_status(op: i32, p_current_1: *mut i32, p_highwater_1: *mut i32,
    reset_flag_1: i32)
    -> i32;
    fn sqlite3_status64(op: i32, p_current_1: *mut Sqlite3Int64,
    p_highwater_1: *mut Sqlite3Int64, reset_flag_1: i32)
    -> i32;
    fn sqlite3_db_status(_: *mut Sqlite3, op: i32, p_cur_1: *mut i32,
    p_hiwtr_1: *mut i32, reset_flg_1: i32)
    -> i32;
    fn sqlite3_db_status64(_: *mut Sqlite3, _: i32, _: *mut Sqlite3Int64,
    _: *mut Sqlite3Int64, _: i32)
    -> i32;
    fn sqlite3_stmt_status(_: *mut Sqlite3Stmt, op: i32, reset_flg_1: i32)
    -> i32;
    fn sqlite3_backup_init(p_dest_1: *mut Sqlite3, z_dest_name_1: *const i8,
    p_source_1: *mut Sqlite3, z_source_name_1: *const i8)
    -> *mut Sqlite3Backup;
    fn sqlite3_backup_step(p: *mut Sqlite3Backup, n_page_1: i32)
    -> i32;
    fn sqlite3_backup_finish(p: *mut Sqlite3Backup)
    -> i32;
    fn sqlite3_backup_remaining(p: *mut Sqlite3Backup)
    -> i32;
    fn sqlite3_backup_pagecount(p: *mut Sqlite3Backup)
    -> i32;
    fn sqlite3_unlock_notify(p_blocked_1: *mut Sqlite3,
    x_notify_1: Option<unsafe extern "C" fn(*mut *mut (), i32) -> ()>,
    p_notify_arg_1: *mut ())
    -> i32;
    fn sqlite3_stricmp(_: *const i8, _: *const i8)
    -> i32;
    fn sqlite3_strnicmp(_: *const i8, _: *const i8, _: i32)
    -> i32;
    fn sqlite3_strglob(z_glob_1: *const i8, z_str_1: *const i8)
    -> i32;
    fn sqlite3_strlike(z_glob_1: *const i8, z_str_1: *const i8, c_esc_1: u32)
    -> i32;
    fn sqlite3_log(i_err_code_1: i32, z_format_1: *const i8, ...)
    -> ();
    fn sqlite3_wal_hook(_: *mut Sqlite3,
    _:
        Option<unsafe extern "C" fn(*mut (), *mut Sqlite3, *const i8, i32)
            -> i32>, _: *mut ())
    -> *mut ();
    fn sqlite3_wal_autocheckpoint(db: *mut Sqlite3, n_1: i32)
    -> i32;
    fn sqlite3_wal_checkpoint(db: *mut Sqlite3, z_db_1: *const i8)
    -> i32;
    fn sqlite3_wal_checkpoint_v2(db: *mut Sqlite3, z_db_1: *const i8,
    e_mode_1: i32, pn_log_1: *mut i32, pn_ckpt_1: *mut i32)
    -> i32;
    fn sqlite3_vtab_config(_: *mut Sqlite3, op: i32, ...)
    -> i32;
    fn sqlite3_vtab_on_conflict(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_vtab_nochange(_: *mut Sqlite3Context)
    -> i32;
    fn sqlite3_vtab_collation(_: *mut Sqlite3IndexInfo, _: i32)
    -> *const i8;
    fn sqlite3_vtab_distinct(_: *mut Sqlite3IndexInfo)
    -> i32;
    fn sqlite3_vtab_in(_: *mut Sqlite3IndexInfo, i_cons_1: i32,
    b_handle_1: i32)
    -> i32;
    fn sqlite3_vtab_in_first(p_val_1: *mut Sqlite3Value,
    pp_out_1: *mut *mut Sqlite3Value)
    -> i32;
    fn sqlite3_vtab_in_next(p_val_1: *mut Sqlite3Value,
    pp_out_1: *mut *mut Sqlite3Value)
    -> i32;
    fn sqlite3_vtab_rhs_value(_: *mut Sqlite3IndexInfo, _: i32,
    pp_val_1: *mut *mut Sqlite3Value)
    -> i32;
    fn sqlite3_stmt_scanstatus(p_stmt_1: *mut Sqlite3Stmt, idx: i32,
    i_scan_status_op_1: i32, p_out_1: *mut ())
    -> i32;
    fn sqlite3_stmt_scanstatus_v2(p_stmt_1: *mut Sqlite3Stmt, idx: i32,
    i_scan_status_op_1: i32, flags: i32, p_out_1: *mut ())
    -> i32;
    fn sqlite3_stmt_scanstatus_reset(_: *mut Sqlite3Stmt)
    -> ();
    fn sqlite3_db_cacheflush(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_system_errno(_: *mut Sqlite3)
    -> i32;
    fn sqlite3_snapshot_get(db: *mut Sqlite3, z_schema_1: *const i8,
    pp_snapshot_1: *mut *mut Sqlite3Snapshot)
    -> i32;
    fn sqlite3_snapshot_open(db: *mut Sqlite3, z_schema_1: *const i8,
    p_snapshot_1: *mut Sqlite3Snapshot)
    -> i32;
    fn sqlite3_snapshot_free(_: *mut Sqlite3Snapshot)
    -> ();
    fn sqlite3_snapshot_cmp(p1: *mut Sqlite3Snapshot,
    p2: *mut Sqlite3Snapshot)
    -> i32;
    fn sqlite3_snapshot_recover(db: *mut Sqlite3, z_db_1: *const i8)
    -> i32;
    fn sqlite3_serialize(db: *mut Sqlite3, z_schema_1: *const i8,
    pi_size_1: *mut Sqlite3Int64, m_flags_1: u32)
    -> *mut u8;
    fn sqlite3_deserialize(db: *mut Sqlite3, z_schema_1: *const i8,
    p_data_1: *mut u8, sz_db_1: Sqlite3Int64, sz_buf_1: Sqlite3Int64,
    m_flags_1: u32)
    -> i32;
    fn sqlite3_carray_bind_v2(p_stmt_1: *mut Sqlite3Stmt, i: i32,
    a_data_1: *mut (), n_data_1: i32, m_flags_1: i32,
    x_del_1: Option<unsafe extern "C" fn(*mut ()) -> ()>, p_del_1: *mut ())
    -> i32;
    fn sqlite3_carray_bind(p_stmt_1: *mut Sqlite3Stmt, i: i32,
    a_data_1: *mut (), n_data_1: i32, m_flags_1: i32,
    x_del_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn sqlite3_rtree_geometry_callback(db: *mut Sqlite3, z_geom_1: *const i8,
    x_geom_1:
        Option<unsafe extern "C" fn(*mut Sqlite3RtreeGeometry, i32, *mut f64,
            *mut i32) -> i32>, p_context_1: *mut ())
    -> i32;
    fn sqlite3_rtree_query_callback(db: *mut Sqlite3,
    z_query_func_1: *const i8,
    x_query_func_1:
        Option<unsafe extern "C" fn(*mut Sqlite3RtreeQueryInfo) -> i32>,
    p_context_1: *mut (),
    x_destructor_1: Option<unsafe extern "C" fn(*mut ()) -> ()>)
    -> i32;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn printf(_: *const i8, ...)
    -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn vfprintf(_: *mut FILE, _: *const i8, _: *mut i8)
    -> i32;
    fn exit(_: i32)
    -> ();
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn strtol(__str: *const i8, __endptr: *mut *mut i8, __base: i32)
    -> i64;
    static mut __stderrp: *mut FILE;
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
}
#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}
type FILE = SFILE;