type DarwinSizeT = u64;
#[repr(C)]
#[derive(Copy, Clone)]
struct SHA3Context {
    u: SHA3ContextU0,
    n_rate: u32,
    n_loaded: u32,
    ix_mask: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
union SHA3ContextU0 {
    s: [u64; 25],
    x: [u8; 1600],
}
extern "C" fn keccak_f1600_step(p: &mut SHA3Context) -> () {
    unsafe {
        let mut i: i32 = 0;
        let mut b0: u64 = 0 as u64;
        let mut b1: u64 = 0 as u64;
        let mut b2: u64 = 0 as u64;
        let mut b3: u64 = 0 as u64;
        let mut b4: u64 = 0 as u64;
        let mut c0: u64 = 0 as u64;
        let mut c1: u64 = 0 as u64;
        let mut c2: u64 = 0 as u64;
        let mut c3: u64 = 0 as u64;
        let mut c4: u64 = 0 as u64;
        let mut d0: u64 = 0 as u64;
        let mut d1: u64 = 0 as u64;
        let mut d2: u64 = 0 as u64;
        let mut d3: u64 = 0 as u64;
        let mut d4: u64 = 0 as u64;
        {
            i = 0;
            '__b0: loop {
                if !(i < 24) { break '__b0; }
                '__c0: loop {
                    c0 =
                        (*p).u.s[0 as usize] ^ (*p).u.s[5 as usize] ^
                                    (*p).u.s[10 as usize] ^ (*p).u.s[15 as usize] ^
                            (*p).u.s[20 as usize];
                    c1 =
                        (*p).u.s[1 as usize] ^ (*p).u.s[6 as usize] ^
                                    (*p).u.s[11 as usize] ^ (*p).u.s[16 as usize] ^
                            (*p).u.s[21 as usize];
                    c2 =
                        (*p).u.s[2 as usize] ^ (*p).u.s[7 as usize] ^
                                    (*p).u.s[12 as usize] ^ (*p).u.s[17 as usize] ^
                            (*p).u.s[22 as usize];
                    c3 =
                        (*p).u.s[3 as usize] ^ (*p).u.s[8 as usize] ^
                                    (*p).u.s[13 as usize] ^ (*p).u.s[18 as usize] ^
                            (*p).u.s[23 as usize];
                    c4 =
                        (*p).u.s[4 as usize] ^ (*p).u.s[9 as usize] ^
                                    (*p).u.s[14 as usize] ^ (*p).u.s[19 as usize] ^
                            (*p).u.s[24 as usize];
                    d0 = c4 ^ (c1 << 1 | c1 >> 64 - 1);
                    d1 = c0 ^ (c2 << 1 | c2 >> 64 - 1);
                    d2 = c1 ^ (c3 << 1 | c3 >> 64 - 1);
                    d3 = c2 ^ (c4 << 1 | c4 >> 64 - 1);
                    d4 = c3 ^ (c0 << 1 | c0 >> 64 - 1);
                    b0 = (*p).u.s[0 as usize] ^ d0;
                    b1 =
                        ((*p).u.s[6 as usize] ^ d1) << 44 |
                            ((*p).u.s[6 as usize] ^ d1) >> 64 - 44;
                    b2 =
                        ((*p).u.s[12 as usize] ^ d2) << 43 |
                            ((*p).u.s[12 as usize] ^ d2) >> 64 - 43;
                    b3 =
                        ((*p).u.s[18 as usize] ^ d3) << 21 |
                            ((*p).u.s[18 as usize] ^ d3) >> 64 - 21;
                    b4 =
                        ((*p).u.s[24 as usize] ^ d4) << 14 |
                            ((*p).u.s[24 as usize] ^ d4) >> 64 - 14;
                    (*p).u.s[0 as usize] = b0 ^ !b1 & b2;
                    (*p).u.s[0 as usize] ^= rc_1[i as usize] as u64;
                    (*p).u.s[6 as usize] = b1 ^ !b2 & b3;
                    (*p).u.s[12 as usize] = b2 ^ !b3 & b4;
                    (*p).u.s[18 as usize] = b3 ^ !b4 & b0;
                    (*p).u.s[24 as usize] = b4 ^ !b0 & b1;
                    b2 =
                        ((*p).u.s[10 as usize] ^ d0) << 3 |
                            ((*p).u.s[10 as usize] ^ d0) >> 64 - 3;
                    b3 =
                        ((*p).u.s[16 as usize] ^ d1) << 45 |
                            ((*p).u.s[16 as usize] ^ d1) >> 64 - 45;
                    b4 =
                        ((*p).u.s[22 as usize] ^ d2) << 61 |
                            ((*p).u.s[22 as usize] ^ d2) >> 64 - 61;
                    b0 =
                        ((*p).u.s[3 as usize] ^ d3) << 28 |
                            ((*p).u.s[3 as usize] ^ d3) >> 64 - 28;
                    b1 =
                        ((*p).u.s[9 as usize] ^ d4) << 20 |
                            ((*p).u.s[9 as usize] ^ d4) >> 64 - 20;
                    (*p).u.s[10 as usize] = b0 ^ !b1 & b2;
                    (*p).u.s[16 as usize] = b1 ^ !b2 & b3;
                    (*p).u.s[22 as usize] = b2 ^ !b3 & b4;
                    (*p).u.s[3 as usize] = b3 ^ !b4 & b0;
                    (*p).u.s[9 as usize] = b4 ^ !b0 & b1;
                    b4 =
                        ((*p).u.s[20 as usize] ^ d0) << 18 |
                            ((*p).u.s[20 as usize] ^ d0) >> 64 - 18;
                    b0 =
                        ((*p).u.s[1 as usize] ^ d1) << 1 |
                            ((*p).u.s[1 as usize] ^ d1) >> 64 - 1;
                    b1 =
                        ((*p).u.s[7 as usize] ^ d2) << 6 |
                            ((*p).u.s[7 as usize] ^ d2) >> 64 - 6;
                    b2 =
                        ((*p).u.s[13 as usize] ^ d3) << 25 |
                            ((*p).u.s[13 as usize] ^ d3) >> 64 - 25;
                    b3 =
                        ((*p).u.s[19 as usize] ^ d4) << 8 |
                            ((*p).u.s[19 as usize] ^ d4) >> 64 - 8;
                    (*p).u.s[20 as usize] = b0 ^ !b1 & b2;
                    (*p).u.s[1 as usize] = b1 ^ !b2 & b3;
                    (*p).u.s[7 as usize] = b2 ^ !b3 & b4;
                    (*p).u.s[13 as usize] = b3 ^ !b4 & b0;
                    (*p).u.s[19 as usize] = b4 ^ !b0 & b1;
                    b1 =
                        ((*p).u.s[5 as usize] ^ d0) << 36 |
                            ((*p).u.s[5 as usize] ^ d0) >> 64 - 36;
                    b2 =
                        ((*p).u.s[11 as usize] ^ d1) << 10 |
                            ((*p).u.s[11 as usize] ^ d1) >> 64 - 10;
                    b3 =
                        ((*p).u.s[17 as usize] ^ d2) << 15 |
                            ((*p).u.s[17 as usize] ^ d2) >> 64 - 15;
                    b4 =
                        ((*p).u.s[23 as usize] ^ d3) << 56 |
                            ((*p).u.s[23 as usize] ^ d3) >> 64 - 56;
                    b0 =
                        ((*p).u.s[4 as usize] ^ d4) << 27 |
                            ((*p).u.s[4 as usize] ^ d4) >> 64 - 27;
                    (*p).u.s[5 as usize] = b0 ^ !b1 & b2;
                    (*p).u.s[11 as usize] = b1 ^ !b2 & b3;
                    (*p).u.s[17 as usize] = b2 ^ !b3 & b4;
                    (*p).u.s[23 as usize] = b3 ^ !b4 & b0;
                    (*p).u.s[4 as usize] = b4 ^ !b0 & b1;
                    b3 =
                        ((*p).u.s[15 as usize] ^ d0) << 41 |
                            ((*p).u.s[15 as usize] ^ d0) >> 64 - 41;
                    b4 =
                        ((*p).u.s[21 as usize] ^ d1) << 2 |
                            ((*p).u.s[21 as usize] ^ d1) >> 64 - 2;
                    b0 =
                        ((*p).u.s[2 as usize] ^ d2) << 62 |
                            ((*p).u.s[2 as usize] ^ d2) >> 64 - 62;
                    b1 =
                        ((*p).u.s[8 as usize] ^ d3) << 55 |
                            ((*p).u.s[8 as usize] ^ d3) >> 64 - 55;
                    b2 =
                        ((*p).u.s[14 as usize] ^ d4) << 39 |
                            ((*p).u.s[14 as usize] ^ d4) >> 64 - 39;
                    (*p).u.s[15 as usize] = b0 ^ !b1 & b2;
                    (*p).u.s[21 as usize] = b1 ^ !b2 & b3;
                    (*p).u.s[2 as usize] = b2 ^ !b3 & b4;
                    (*p).u.s[8 as usize] = b3 ^ !b4 & b0;
                    (*p).u.s[14 as usize] = b4 ^ !b0 & b1;
                    c0 =
                        (*p).u.s[0 as usize] ^ (*p).u.s[10 as usize] ^
                                    (*p).u.s[20 as usize] ^ (*p).u.s[5 as usize] ^
                            (*p).u.s[15 as usize];
                    c1 =
                        (*p).u.s[6 as usize] ^ (*p).u.s[16 as usize] ^
                                    (*p).u.s[1 as usize] ^ (*p).u.s[11 as usize] ^
                            (*p).u.s[21 as usize];
                    c2 =
                        (*p).u.s[12 as usize] ^ (*p).u.s[22 as usize] ^
                                    (*p).u.s[7 as usize] ^ (*p).u.s[17 as usize] ^
                            (*p).u.s[2 as usize];
                    c3 =
                        (*p).u.s[18 as usize] ^ (*p).u.s[3 as usize] ^
                                    (*p).u.s[13 as usize] ^ (*p).u.s[23 as usize] ^
                            (*p).u.s[8 as usize];
                    c4 =
                        (*p).u.s[24 as usize] ^ (*p).u.s[9 as usize] ^
                                    (*p).u.s[19 as usize] ^ (*p).u.s[4 as usize] ^
                            (*p).u.s[14 as usize];
                    d0 = c4 ^ (c1 << 1 | c1 >> 64 - 1);
                    d1 = c0 ^ (c2 << 1 | c2 >> 64 - 1);
                    d2 = c1 ^ (c3 << 1 | c3 >> 64 - 1);
                    d3 = c2 ^ (c4 << 1 | c4 >> 64 - 1);
                    d4 = c3 ^ (c0 << 1 | c0 >> 64 - 1);
                    b0 = (*p).u.s[0 as usize] ^ d0;
                    b1 =
                        ((*p).u.s[16 as usize] ^ d1) << 44 |
                            ((*p).u.s[16 as usize] ^ d1) >> 64 - 44;
                    b2 =
                        ((*p).u.s[7 as usize] ^ d2) << 43 |
                            ((*p).u.s[7 as usize] ^ d2) >> 64 - 43;
                    b3 =
                        ((*p).u.s[23 as usize] ^ d3) << 21 |
                            ((*p).u.s[23 as usize] ^ d3) >> 64 - 21;
                    b4 =
                        ((*p).u.s[14 as usize] ^ d4) << 14 |
                            ((*p).u.s[14 as usize] ^ d4) >> 64 - 14;
                    (*p).u.s[0 as usize] = b0 ^ !b1 & b2;
                    (*p).u.s[0 as usize] ^= rc_1[(i + 1) as usize] as u64;
                    (*p).u.s[16 as usize] = b1 ^ !b2 & b3;
                    (*p).u.s[7 as usize] = b2 ^ !b3 & b4;
                    (*p).u.s[23 as usize] = b3 ^ !b4 & b0;
                    (*p).u.s[14 as usize] = b4 ^ !b0 & b1;
                    b2 =
                        ((*p).u.s[20 as usize] ^ d0) << 3 |
                            ((*p).u.s[20 as usize] ^ d0) >> 64 - 3;
                    b3 =
                        ((*p).u.s[11 as usize] ^ d1) << 45 |
                            ((*p).u.s[11 as usize] ^ d1) >> 64 - 45;
                    b4 =
                        ((*p).u.s[2 as usize] ^ d2) << 61 |
                            ((*p).u.s[2 as usize] ^ d2) >> 64 - 61;
                    b0 =
                        ((*p).u.s[18 as usize] ^ d3) << 28 |
                            ((*p).u.s[18 as usize] ^ d3) >> 64 - 28;
                    b1 =
                        ((*p).u.s[9 as usize] ^ d4) << 20 |
                            ((*p).u.s[9 as usize] ^ d4) >> 64 - 20;
                    (*p).u.s[20 as usize] = b0 ^ !b1 & b2;
                    (*p).u.s[11 as usize] = b1 ^ !b2 & b3;
                    (*p).u.s[2 as usize] = b2 ^ !b3 & b4;
                    (*p).u.s[18 as usize] = b3 ^ !b4 & b0;
                    (*p).u.s[9 as usize] = b4 ^ !b0 & b1;
                    b4 =
                        ((*p).u.s[15 as usize] ^ d0) << 18 |
                            ((*p).u.s[15 as usize] ^ d0) >> 64 - 18;
                    b0 =
                        ((*p).u.s[6 as usize] ^ d1) << 1 |
                            ((*p).u.s[6 as usize] ^ d1) >> 64 - 1;
                    b1 =
                        ((*p).u.s[22 as usize] ^ d2) << 6 |
                            ((*p).u.s[22 as usize] ^ d2) >> 64 - 6;
                    b2 =
                        ((*p).u.s[13 as usize] ^ d3) << 25 |
                            ((*p).u.s[13 as usize] ^ d3) >> 64 - 25;
                    b3 =
                        ((*p).u.s[4 as usize] ^ d4) << 8 |
                            ((*p).u.s[4 as usize] ^ d4) >> 64 - 8;
                    (*p).u.s[15 as usize] = b0 ^ !b1 & b2;
                    (*p).u.s[6 as usize] = b1 ^ !b2 & b3;
                    (*p).u.s[22 as usize] = b2 ^ !b3 & b4;
                    (*p).u.s[13 as usize] = b3 ^ !b4 & b0;
                    (*p).u.s[4 as usize] = b4 ^ !b0 & b1;
                    b1 =
                        ((*p).u.s[10 as usize] ^ d0) << 36 |
                            ((*p).u.s[10 as usize] ^ d0) >> 64 - 36;
                    b2 =
                        ((*p).u.s[1 as usize] ^ d1) << 10 |
                            ((*p).u.s[1 as usize] ^ d1) >> 64 - 10;
                    b3 =
                        ((*p).u.s[17 as usize] ^ d2) << 15 |
                            ((*p).u.s[17 as usize] ^ d2) >> 64 - 15;
                    b4 =
                        ((*p).u.s[8 as usize] ^ d3) << 56 |
                            ((*p).u.s[8 as usize] ^ d3) >> 64 - 56;
                    b0 =
                        ((*p).u.s[24 as usize] ^ d4) << 27 |
                            ((*p).u.s[24 as usize] ^ d4) >> 64 - 27;
                    (*p).u.s[10 as usize] = b0 ^ !b1 & b2;
                    (*p).u.s[1 as usize] = b1 ^ !b2 & b3;
                    (*p).u.s[17 as usize] = b2 ^ !b3 & b4;
                    (*p).u.s[8 as usize] = b3 ^ !b4 & b0;
                    (*p).u.s[24 as usize] = b4 ^ !b0 & b1;
                    b3 =
                        ((*p).u.s[5 as usize] ^ d0) << 41 |
                            ((*p).u.s[5 as usize] ^ d0) >> 64 - 41;
                    b4 =
                        ((*p).u.s[21 as usize] ^ d1) << 2 |
                            ((*p).u.s[21 as usize] ^ d1) >> 64 - 2;
                    b0 =
                        ((*p).u.s[12 as usize] ^ d2) << 62 |
                            ((*p).u.s[12 as usize] ^ d2) >> 64 - 62;
                    b1 =
                        ((*p).u.s[3 as usize] ^ d3) << 55 |
                            ((*p).u.s[3 as usize] ^ d3) >> 64 - 55;
                    b2 =
                        ((*p).u.s[19 as usize] ^ d4) << 39 |
                            ((*p).u.s[19 as usize] ^ d4) >> 64 - 39;
                    (*p).u.s[5 as usize] = b0 ^ !b1 & b2;
                    (*p).u.s[21 as usize] = b1 ^ !b2 & b3;
                    (*p).u.s[12 as usize] = b2 ^ !b3 & b4;
                    (*p).u.s[3 as usize] = b3 ^ !b4 & b0;
                    (*p).u.s[19 as usize] = b4 ^ !b0 & b1;
                    c0 =
                        (*p).u.s[0 as usize] ^ (*p).u.s[20 as usize] ^
                                    (*p).u.s[15 as usize] ^ (*p).u.s[10 as usize] ^
                            (*p).u.s[5 as usize];
                    c1 =
                        (*p).u.s[16 as usize] ^ (*p).u.s[11 as usize] ^
                                    (*p).u.s[6 as usize] ^ (*p).u.s[1 as usize] ^
                            (*p).u.s[21 as usize];
                    c2 =
                        (*p).u.s[7 as usize] ^ (*p).u.s[2 as usize] ^
                                    (*p).u.s[22 as usize] ^ (*p).u.s[17 as usize] ^
                            (*p).u.s[12 as usize];
                    c3 =
                        (*p).u.s[23 as usize] ^ (*p).u.s[18 as usize] ^
                                    (*p).u.s[13 as usize] ^ (*p).u.s[8 as usize] ^
                            (*p).u.s[3 as usize];
                    c4 =
                        (*p).u.s[14 as usize] ^ (*p).u.s[9 as usize] ^
                                    (*p).u.s[4 as usize] ^ (*p).u.s[24 as usize] ^
                            (*p).u.s[19 as usize];
                    d0 = c4 ^ (c1 << 1 | c1 >> 64 - 1);
                    d1 = c0 ^ (c2 << 1 | c2 >> 64 - 1);
                    d2 = c1 ^ (c3 << 1 | c3 >> 64 - 1);
                    d3 = c2 ^ (c4 << 1 | c4 >> 64 - 1);
                    d4 = c3 ^ (c0 << 1 | c0 >> 64 - 1);
                    b0 = (*p).u.s[0 as usize] ^ d0;
                    b1 =
                        ((*p).u.s[11 as usize] ^ d1) << 44 |
                            ((*p).u.s[11 as usize] ^ d1) >> 64 - 44;
                    b2 =
                        ((*p).u.s[22 as usize] ^ d2) << 43 |
                            ((*p).u.s[22 as usize] ^ d2) >> 64 - 43;
                    b3 =
                        ((*p).u.s[8 as usize] ^ d3) << 21 |
                            ((*p).u.s[8 as usize] ^ d3) >> 64 - 21;
                    b4 =
                        ((*p).u.s[19 as usize] ^ d4) << 14 |
                            ((*p).u.s[19 as usize] ^ d4) >> 64 - 14;
                    (*p).u.s[0 as usize] = b0 ^ !b1 & b2;
                    (*p).u.s[0 as usize] ^= rc_1[(i + 2) as usize] as u64;
                    (*p).u.s[11 as usize] = b1 ^ !b2 & b3;
                    (*p).u.s[22 as usize] = b2 ^ !b3 & b4;
                    (*p).u.s[8 as usize] = b3 ^ !b4 & b0;
                    (*p).u.s[19 as usize] = b4 ^ !b0 & b1;
                    b2 =
                        ((*p).u.s[15 as usize] ^ d0) << 3 |
                            ((*p).u.s[15 as usize] ^ d0) >> 64 - 3;
                    b3 =
                        ((*p).u.s[1 as usize] ^ d1) << 45 |
                            ((*p).u.s[1 as usize] ^ d1) >> 64 - 45;
                    b4 =
                        ((*p).u.s[12 as usize] ^ d2) << 61 |
                            ((*p).u.s[12 as usize] ^ d2) >> 64 - 61;
                    b0 =
                        ((*p).u.s[23 as usize] ^ d3) << 28 |
                            ((*p).u.s[23 as usize] ^ d3) >> 64 - 28;
                    b1 =
                        ((*p).u.s[9 as usize] ^ d4) << 20 |
                            ((*p).u.s[9 as usize] ^ d4) >> 64 - 20;
                    (*p).u.s[15 as usize] = b0 ^ !b1 & b2;
                    (*p).u.s[1 as usize] = b1 ^ !b2 & b3;
                    (*p).u.s[12 as usize] = b2 ^ !b3 & b4;
                    (*p).u.s[23 as usize] = b3 ^ !b4 & b0;
                    (*p).u.s[9 as usize] = b4 ^ !b0 & b1;
                    b4 =
                        ((*p).u.s[5 as usize] ^ d0) << 18 |
                            ((*p).u.s[5 as usize] ^ d0) >> 64 - 18;
                    b0 =
                        ((*p).u.s[16 as usize] ^ d1) << 1 |
                            ((*p).u.s[16 as usize] ^ d1) >> 64 - 1;
                    b1 =
                        ((*p).u.s[2 as usize] ^ d2) << 6 |
                            ((*p).u.s[2 as usize] ^ d2) >> 64 - 6;
                    b2 =
                        ((*p).u.s[13 as usize] ^ d3) << 25 |
                            ((*p).u.s[13 as usize] ^ d3) >> 64 - 25;
                    b3 =
                        ((*p).u.s[24 as usize] ^ d4) << 8 |
                            ((*p).u.s[24 as usize] ^ d4) >> 64 - 8;
                    (*p).u.s[5 as usize] = b0 ^ !b1 & b2;
                    (*p).u.s[16 as usize] = b1 ^ !b2 & b3;
                    (*p).u.s[2 as usize] = b2 ^ !b3 & b4;
                    (*p).u.s[13 as usize] = b3 ^ !b4 & b0;
                    (*p).u.s[24 as usize] = b4 ^ !b0 & b1;
                    b1 =
                        ((*p).u.s[20 as usize] ^ d0) << 36 |
                            ((*p).u.s[20 as usize] ^ d0) >> 64 - 36;
                    b2 =
                        ((*p).u.s[6 as usize] ^ d1) << 10 |
                            ((*p).u.s[6 as usize] ^ d1) >> 64 - 10;
                    b3 =
                        ((*p).u.s[17 as usize] ^ d2) << 15 |
                            ((*p).u.s[17 as usize] ^ d2) >> 64 - 15;
                    b4 =
                        ((*p).u.s[3 as usize] ^ d3) << 56 |
                            ((*p).u.s[3 as usize] ^ d3) >> 64 - 56;
                    b0 =
                        ((*p).u.s[14 as usize] ^ d4) << 27 |
                            ((*p).u.s[14 as usize] ^ d4) >> 64 - 27;
                    (*p).u.s[20 as usize] = b0 ^ !b1 & b2;
                    (*p).u.s[6 as usize] = b1 ^ !b2 & b3;
                    (*p).u.s[17 as usize] = b2 ^ !b3 & b4;
                    (*p).u.s[3 as usize] = b3 ^ !b4 & b0;
                    (*p).u.s[14 as usize] = b4 ^ !b0 & b1;
                    b3 =
                        ((*p).u.s[10 as usize] ^ d0) << 41 |
                            ((*p).u.s[10 as usize] ^ d0) >> 64 - 41;
                    b4 =
                        ((*p).u.s[21 as usize] ^ d1) << 2 |
                            ((*p).u.s[21 as usize] ^ d1) >> 64 - 2;
                    b0 =
                        ((*p).u.s[7 as usize] ^ d2) << 62 |
                            ((*p).u.s[7 as usize] ^ d2) >> 64 - 62;
                    b1 =
                        ((*p).u.s[18 as usize] ^ d3) << 55 |
                            ((*p).u.s[18 as usize] ^ d3) >> 64 - 55;
                    b2 =
                        ((*p).u.s[4 as usize] ^ d4) << 39 |
                            ((*p).u.s[4 as usize] ^ d4) >> 64 - 39;
                    (*p).u.s[10 as usize] = b0 ^ !b1 & b2;
                    (*p).u.s[21 as usize] = b1 ^ !b2 & b3;
                    (*p).u.s[7 as usize] = b2 ^ !b3 & b4;
                    (*p).u.s[18 as usize] = b3 ^ !b4 & b0;
                    (*p).u.s[4 as usize] = b4 ^ !b0 & b1;
                    c0 =
                        (*p).u.s[0 as usize] ^ (*p).u.s[15 as usize] ^
                                    (*p).u.s[5 as usize] ^ (*p).u.s[20 as usize] ^
                            (*p).u.s[10 as usize];
                    c1 =
                        (*p).u.s[11 as usize] ^ (*p).u.s[1 as usize] ^
                                    (*p).u.s[16 as usize] ^ (*p).u.s[6 as usize] ^
                            (*p).u.s[21 as usize];
                    c2 =
                        (*p).u.s[22 as usize] ^ (*p).u.s[12 as usize] ^
                                    (*p).u.s[2 as usize] ^ (*p).u.s[17 as usize] ^
                            (*p).u.s[7 as usize];
                    c3 =
                        (*p).u.s[8 as usize] ^ (*p).u.s[23 as usize] ^
                                    (*p).u.s[13 as usize] ^ (*p).u.s[3 as usize] ^
                            (*p).u.s[18 as usize];
                    c4 =
                        (*p).u.s[19 as usize] ^ (*p).u.s[9 as usize] ^
                                    (*p).u.s[24 as usize] ^ (*p).u.s[14 as usize] ^
                            (*p).u.s[4 as usize];
                    d0 = c4 ^ (c1 << 1 | c1 >> 64 - 1);
                    d1 = c0 ^ (c2 << 1 | c2 >> 64 - 1);
                    d2 = c1 ^ (c3 << 1 | c3 >> 64 - 1);
                    d3 = c2 ^ (c4 << 1 | c4 >> 64 - 1);
                    d4 = c3 ^ (c0 << 1 | c0 >> 64 - 1);
                    b0 = (*p).u.s[0 as usize] ^ d0;
                    b1 =
                        ((*p).u.s[1 as usize] ^ d1) << 44 |
                            ((*p).u.s[1 as usize] ^ d1) >> 64 - 44;
                    b2 =
                        ((*p).u.s[2 as usize] ^ d2) << 43 |
                            ((*p).u.s[2 as usize] ^ d2) >> 64 - 43;
                    b3 =
                        ((*p).u.s[3 as usize] ^ d3) << 21 |
                            ((*p).u.s[3 as usize] ^ d3) >> 64 - 21;
                    b4 =
                        ((*p).u.s[4 as usize] ^ d4) << 14 |
                            ((*p).u.s[4 as usize] ^ d4) >> 64 - 14;
                    (*p).u.s[0 as usize] = b0 ^ !b1 & b2;
                    (*p).u.s[0 as usize] ^= rc_1[(i + 3) as usize] as u64;
                    (*p).u.s[1 as usize] = b1 ^ !b2 & b3;
                    (*p).u.s[2 as usize] = b2 ^ !b3 & b4;
                    (*p).u.s[3 as usize] = b3 ^ !b4 & b0;
                    (*p).u.s[4 as usize] = b4 ^ !b0 & b1;
                    b2 =
                        ((*p).u.s[5 as usize] ^ d0) << 3 |
                            ((*p).u.s[5 as usize] ^ d0) >> 64 - 3;
                    b3 =
                        ((*p).u.s[6 as usize] ^ d1) << 45 |
                            ((*p).u.s[6 as usize] ^ d1) >> 64 - 45;
                    b4 =
                        ((*p).u.s[7 as usize] ^ d2) << 61 |
                            ((*p).u.s[7 as usize] ^ d2) >> 64 - 61;
                    b0 =
                        ((*p).u.s[8 as usize] ^ d3) << 28 |
                            ((*p).u.s[8 as usize] ^ d3) >> 64 - 28;
                    b1 =
                        ((*p).u.s[9 as usize] ^ d4) << 20 |
                            ((*p).u.s[9 as usize] ^ d4) >> 64 - 20;
                    (*p).u.s[5 as usize] = b0 ^ !b1 & b2;
                    (*p).u.s[6 as usize] = b1 ^ !b2 & b3;
                    (*p).u.s[7 as usize] = b2 ^ !b3 & b4;
                    (*p).u.s[8 as usize] = b3 ^ !b4 & b0;
                    (*p).u.s[9 as usize] = b4 ^ !b0 & b1;
                    b4 =
                        ((*p).u.s[10 as usize] ^ d0) << 18 |
                            ((*p).u.s[10 as usize] ^ d0) >> 64 - 18;
                    b0 =
                        ((*p).u.s[11 as usize] ^ d1) << 1 |
                            ((*p).u.s[11 as usize] ^ d1) >> 64 - 1;
                    b1 =
                        ((*p).u.s[12 as usize] ^ d2) << 6 |
                            ((*p).u.s[12 as usize] ^ d2) >> 64 - 6;
                    b2 =
                        ((*p).u.s[13 as usize] ^ d3) << 25 |
                            ((*p).u.s[13 as usize] ^ d3) >> 64 - 25;
                    b3 =
                        ((*p).u.s[14 as usize] ^ d4) << 8 |
                            ((*p).u.s[14 as usize] ^ d4) >> 64 - 8;
                    (*p).u.s[10 as usize] = b0 ^ !b1 & b2;
                    (*p).u.s[11 as usize] = b1 ^ !b2 & b3;
                    (*p).u.s[12 as usize] = b2 ^ !b3 & b4;
                    (*p).u.s[13 as usize] = b3 ^ !b4 & b0;
                    (*p).u.s[14 as usize] = b4 ^ !b0 & b1;
                    b1 =
                        ((*p).u.s[15 as usize] ^ d0) << 36 |
                            ((*p).u.s[15 as usize] ^ d0) >> 64 - 36;
                    b2 =
                        ((*p).u.s[16 as usize] ^ d1) << 10 |
                            ((*p).u.s[16 as usize] ^ d1) >> 64 - 10;
                    b3 =
                        ((*p).u.s[17 as usize] ^ d2) << 15 |
                            ((*p).u.s[17 as usize] ^ d2) >> 64 - 15;
                    b4 =
                        ((*p).u.s[18 as usize] ^ d3) << 56 |
                            ((*p).u.s[18 as usize] ^ d3) >> 64 - 56;
                    b0 =
                        ((*p).u.s[19 as usize] ^ d4) << 27 |
                            ((*p).u.s[19 as usize] ^ d4) >> 64 - 27;
                    (*p).u.s[15 as usize] = b0 ^ !b1 & b2;
                    (*p).u.s[16 as usize] = b1 ^ !b2 & b3;
                    (*p).u.s[17 as usize] = b2 ^ !b3 & b4;
                    (*p).u.s[18 as usize] = b3 ^ !b4 & b0;
                    (*p).u.s[19 as usize] = b4 ^ !b0 & b1;
                    b3 =
                        ((*p).u.s[20 as usize] ^ d0) << 41 |
                            ((*p).u.s[20 as usize] ^ d0) >> 64 - 41;
                    b4 =
                        ((*p).u.s[21 as usize] ^ d1) << 2 |
                            ((*p).u.s[21 as usize] ^ d1) >> 64 - 2;
                    b0 =
                        ((*p).u.s[22 as usize] ^ d2) << 62 |
                            ((*p).u.s[22 as usize] ^ d2) >> 64 - 62;
                    b1 =
                        ((*p).u.s[23 as usize] ^ d3) << 55 |
                            ((*p).u.s[23 as usize] ^ d3) >> 64 - 55;
                    b2 =
                        ((*p).u.s[24 as usize] ^ d4) << 39 |
                            ((*p).u.s[24 as usize] ^ d4) >> 64 - 39;
                    (*p).u.s[20 as usize] = b0 ^ !b1 & b2;
                    (*p).u.s[21 as usize] = b1 ^ !b2 & b3;
                    (*p).u.s[22 as usize] = b2 ^ !b3 & b4;
                    (*p).u.s[23 as usize] = b3 ^ !b4 & b0;
                    (*p).u.s[24 as usize] = b4 ^ !b0 & b1;
                    break '__c0;
                }
                i += 4;
            }
        }
    }
}
extern "C" fn sha3_init(p: *mut SHA3Context, i_size_1: i32) -> () {
    unsafe {
        unsafe {
            memset(p as *mut (), 0,
                core::mem::size_of::<SHA3Context>() as u64)
        };
        if i_size_1 >= 128 && i_size_1 <= 512 {
            unsafe {
                (*p).n_rate = ((1600 - (i_size_1 + 31 & !31) * 2) / 8) as u32
            };
        } else { unsafe { (*p).n_rate = ((1600 - 2 * 256) / 8) as u32 }; }
        {
            if 1 == unsafe { *(&raw mut one_1 as *mut u8) } as i32 {
                unsafe { (*p).ix_mask = 0 as u32 };
            } else { unsafe { (*p).ix_mask = 7 as u32 }; }
        }
    }
}
extern "C" fn sha3_update(p: *mut SHA3Context, a_data_1: &[u8]) -> () {
    unsafe {
        let mut i: u32 = 0 as u32;
        {
            '__b1: loop {
                if !(i < a_data_1.len() as u32) { break '__b1; }
                '__c1: loop {
                    unsafe {
                        (*p).u.x[(unsafe { (*p).n_loaded } ^
                                        unsafe { (*p).ix_mask }) as usize] ^=
                            a_data_1[i as usize] as i32 as u8
                    };
                    {
                        let __p = unsafe { &mut (*p).n_loaded };
                        let __t = *__p;
                        *__p += 1;
                        __t
                    };
                    if unsafe { (*p).n_loaded } == unsafe { (*p).n_rate } {
                        keccak_f1600_step(unsafe { &mut *p });
                        unsafe { (*p).n_loaded = 0 as u32 };
                    }
                    break '__c1;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}
extern "C" fn sha3_final(p: *mut SHA3Context) -> *mut u8 {
    unsafe {
        let mut i: u32 = 0 as u32;
        if unsafe { (*p).n_loaded } == unsafe { (*p).n_rate } - 1 as u32 {
            let c1: u8 = 134 as u8;
            sha3_update(p,
                unsafe {
                    let __p = &c1 as *const u8;
                    if __p.is_null() {
                        &[]
                    } else { core::slice::from_raw_parts(__p, 1 as usize) }
                });
        } else {
            let c2: u8 = 6 as u8;
            let c3: u8 = 128 as u8;
            sha3_update(p,
                unsafe {
                    let __p = &c2 as *const u8;
                    if __p.is_null() {
                        &[]
                    } else { core::slice::from_raw_parts(__p, 1 as usize) }
                });
            unsafe { (*p).n_loaded = unsafe { (*p).n_rate } - 1 as u32 };
            sha3_update(p,
                unsafe {
                    let __p = &c3 as *const u8;
                    if __p.is_null() {
                        &[]
                    } else { core::slice::from_raw_parts(__p, 1 as usize) }
                });
        }
        {
            i = 0 as u32;
            '__b2: loop {
                if !(i < unsafe { (*p).n_rate }) { break '__b2; }
                '__c2: loop {
                    unsafe {
                        (*p).u.x[(i + unsafe { (*p).n_rate }) as usize] =
                            unsafe { (*p).u.x[(i ^ unsafe { (*p).ix_mask }) as usize] }
                    };
                    break '__c2;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return unsafe { &mut (*p).u.x[unsafe { (*p).n_rate } as usize] };
    }
}
extern "C" fn digest_to_base16(mut digest: *const u8, mut z_buf_1: *mut i8,
    n_byte_1: i32) -> () {
    let mut ix: i32 = 0;
    {
        ix = 0;
        '__b3: loop {
            if !(ix < n_byte_1) { break '__b3; }
            '__c3: loop {
                unsafe {
                    *{
                                let __p = &mut z_buf_1;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            } =
                        z_encode[(unsafe { *digest } as i32 >> 4 & 15) as usize] as
                            i8
                };
                unsafe {
                    *{
                                let __p = &mut z_buf_1;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            } =
                        z_encode[(unsafe {
                                                *{
                                                        let __p = &mut digest;
                                                        let __t = *__p;
                                                        *__p = unsafe { (*__p).offset(1) };
                                                        __t
                                                    }
                                            } as i32 & 15) as usize] as i8
                };
                break '__c3;
            }
            { let __p = &mut ix; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { *z_buf_1 = '\u{0}' as i32 as i8 };
}
extern "C" fn sha3sum_file(z_filename_1: *const i8, i_size_1: i32,
    p_cksum_1: *mut i8) -> i32 {
    let mut in_: *mut FILE = core::ptr::null_mut();
    let mut ctx: SHA3Context = unsafe { core::mem::zeroed() };
    let mut z_buf: [i8; 10240] = [0; 10240];
    in_ =
        unsafe {
            fopen(z_filename_1, c"rb".as_ptr() as *mut i8 as *const i8)
        };
    if in_ == core::ptr::null_mut() { return 1; }
    sha3_init(&mut ctx, i_size_1);
    {
        '__b4: loop {
            '__c4: loop {
                let n: i32 =
                    unsafe {
                            fread(&raw mut z_buf[0 as usize] as *mut i8 as *mut (),
                                1 as u64, core::mem::size_of::<[i8; 10240]>() as u64, in_)
                        } as i32;
                if n <= 0 { break '__b4; }
                sha3_update(&mut ctx,
                    unsafe {
                        let __p =
                            &raw mut z_buf[0 as usize] as *mut u8 as *const u8;
                        if __p.is_null() {
                            &[]
                        } else {
                            core::slice::from_raw_parts(__p, n as u32 as usize)
                        }
                    });
                break '__c4;
            }
        }
    }
    unsafe { fclose(in_) };
    digest_to_base16(sha3_final(&mut ctx) as *const u8, p_cksum_1,
        i_size_1 / 8);
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct SHA1Context {
    state: [u32; 5],
    count: [u32; 2],
    buffer: [u8; 64],
}
extern "C" fn sha1_transform(state: *mut u32, buffer: *const u8) -> () {
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
        if 1 == unsafe { *(&raw mut one_2 as *mut u8) } as i32 {
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
extern "C" fn sha1_init(context: &mut SHA1Context) -> () {
    (*context).state[0 as usize] = 1732584193 as u32;
    (*context).state[1 as usize] = 4023233417u32;
    (*context).state[2 as usize] = 2562383102u32;
    (*context).state[3 as usize] = 271733878 as u32;
    (*context).state[4 as usize] = 3285377520u32;
    (*context).count[0 as usize] =
        {
            (*context).count[1 as usize] = 0 as u32;
            (*context).count[1 as usize]
        };
}
extern "C" fn sha1_update(context: &mut SHA1Context, data: *const u8,
    len: u32) -> () {
    let mut i: u32 = 0 as u32;
    let mut j: u32 = 0 as u32;
    j = (*context).count[0 as usize];
    if {
                (*context).count[0 as usize] += len << 3;
                (*context).count[0 as usize]
            } < j {
        (*context).count[1 as usize] += (len >> 29) + 1 as u32;
    }
    j = j >> 3 & 63 as u32;
    if j + len > 63 as u32 {
        {
            let _ =
                unsafe {
                    memcpy(&raw mut (*context).buffer[j as usize] as *mut (),
                        data as *const (), { i = 64 as u32 - j; i } as u64)
                };
        };
        sha1_transform(&raw mut (*context).state[0 as usize] as *mut u32,
            &raw mut (*context).buffer[0 as usize] as *mut u8 as *const u8);
        {
            '__b5: loop {
                if !((i + 63 as u32) < len) { break '__b5; }
                '__c5: loop {
                    sha1_transform(&raw mut (*context).state[0 as usize] as
                            *mut u32, unsafe { &*data.add(i as usize) });
                    break '__c5;
                }
                i += 64 as u32;
            }
        }
        j = 0 as u32;
    } else { i = 0 as u32; }
    {
        let _ =
            unsafe {
                memcpy(&raw mut (*context).buffer[j as usize] as *mut (),
                    unsafe { &raw const *data.add(i as usize) } as *const (),
                    (len - i) as u64)
            };
    };
}
extern "C" fn sha1_final(digest: *mut u8, context: *mut SHA1Context) -> () {
    let mut i: u32 = 0 as u32;
    let mut finalcount: [u8; 8] = [0; 8];
    {
        i = 0 as u32;
        '__b6: loop {
            if !(i < 8 as u32) { break '__b6; }
            '__c6: loop {
                finalcount[i as usize] =
                    (unsafe {
                                    (*context).count[if i >= 4 as u32 { 0 } else { 1 } as usize]
                                } >> (3 as u32 - (i & 3 as u32)) * 8 as u32 & 255 as u32) as
                        u8;
                break '__c6;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    sha1_update(unsafe { &mut *context }, b"\x80\x00".as_ptr() as *const u8,
        1 as u32);
    while unsafe { (*context).count[0 as usize] } & 504 as u32 != 448 as u32 {
        sha1_update(unsafe { &mut *context },
            b"\x00\x00".as_ptr() as *const u8, 1 as u32);
    }
    sha1_update(unsafe { &mut *context },
        &raw mut finalcount[0 as usize] as *mut u8 as *const u8, 8 as u32);
    if !(digest).is_null() {
        {
            i = 0 as u32;
            '__b8: loop {
                if !(i < 20 as u32) { break '__b8; }
                '__c8: loop {
                    unsafe {
                        *digest.add(i as usize) =
                            (unsafe { (*context).state[(i >> 2) as usize] } >>
                                        (3 as u32 - (i & 3 as u32)) * 8 as u32 & 255 as u32) as u8
                    };
                    break '__c8;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}
extern "C" fn sha1sum_file(z_filename_1: *const i8, p_cksum_1: *mut i8)
    -> i32 {
    let mut in_: *mut FILE = core::ptr::null_mut();
    let mut ctx: SHA1Context = unsafe { core::mem::zeroed() };
    let mut z_result: [u8; 20] = [0; 20];
    let mut z_buf: [i8; 10240] = [0; 10240];
    in_ =
        unsafe {
            fopen(z_filename_1, c"rb".as_ptr() as *mut i8 as *const i8)
        };
    if in_ == core::ptr::null_mut() { return 1; }
    sha1_init(&mut ctx);
    {
        '__b9: loop {
            '__c9: loop {
                let n: i32 =
                    unsafe {
                            fread(&raw mut z_buf[0 as usize] as *mut i8 as *mut (),
                                1 as u64, core::mem::size_of::<[i8; 10240]>() as u64, in_)
                        } as i32;
                if n <= 0 { break '__b9; }
                sha1_update(&mut ctx,
                    &raw mut z_buf[0 as usize] as *mut u8 as *const u8,
                    n as u32);
                break '__c9;
            }
        }
    }
    unsafe { fclose(in_) };
    sha1_final(&raw mut z_result[0 as usize] as *mut u8, &mut ctx);
    digest_to_base16(&raw mut z_result[0 as usize] as *mut u8 as *const u8,
        p_cksum_1, 20);
    return 0;
}
extern "C" fn usage(argv0: *const i8) -> () {
    unsafe {
        unsafe {
            fprintf(__stderrp,
                c"Usage: %s manifest\nOptions:\n   -v  Diagnostic output\n".as_ptr()
                        as *mut i8 as *const i8, argv0)
        };
        unsafe { exit(1) };
    }
}
extern "C" fn next_token(mut z: *mut i8) -> *mut i8 {
    while unsafe { *z } != 0 &&
            (unsafe { isspace(unsafe { *z } as i32) } == 0) as i32 != 0 {
        {
            let __p = &mut z;
            let __t = *__p;
            *__p = unsafe { (*__p).offset(1) };
            __t
        };
    }
    if unsafe { *z } as i32 == 0 { return z; }
    unsafe { *z = 0 as i8 };
    return unsafe { &mut *z.offset(1 as isize) };
}
extern "C" fn __main_inner(argc: i32, argv: *const *mut i8)
    -> Result<(), i32> {
    unsafe {
        let mut z_manifest: *const i8 = core::ptr::null();
        let mut i: i32 = 0;
        let mut b_verbose: i32 = 0;
        let mut in_: *mut FILE = core::ptr::null_mut();
        let mut all_valid: i32 = 1;
        let mut rc: i32 = 0;
        let mut ctx: SHA3Context = unsafe { core::mem::zeroed() };
        let mut z_date: [i8; 50] = [0; 50];
        let mut z_hash: [i8; 100] = [0; 100];
        let mut z_line: [i8; 20000] = [0; 20000];
        {
            i = 1;
            '__b11: loop {
                if !(i < argc) { break '__b11; }
                '__c11: loop {
                    let mut z: *const i8 =
                        unsafe { *argv.offset(i as isize) } as *const i8;
                    if unsafe { *z.offset(0 as isize) } as i32 == '-' as i32 {
                        if unsafe { *z.offset(1 as isize) } as i32 == '-' as i32 {
                            {
                                let __p = &mut z;
                                let __t = *__p;
                                *__p = unsafe { (*__p).offset(1) };
                                __t
                            };
                        }
                        if unsafe {
                                    strcmp(z, c"-v".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            b_verbose = 1;
                        } else {
                            unsafe {
                                fprintf(__stderrp,
                                    c"unknown option \"%s\"".as_ptr() as *mut i8 as *const i8,
                                    unsafe { *argv.offset(i as isize) })
                            };
                            unsafe { exit(1) };
                        }
                    } else if z_manifest != core::ptr::null() {
                        usage(unsafe { *argv.offset(0 as isize) } as *const i8);
                    } else { z_manifest = z; }
                    break '__c11;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if z_manifest == core::ptr::null() {
            usage(unsafe { *argv.offset(0 as isize) } as *const i8);
        }
        z_date[0 as usize] = 0 as i8;
        in_ =
            unsafe {
                fopen(z_manifest, c"rb".as_ptr() as *mut i8 as *const i8)
            };
        if in_ == core::ptr::null_mut() {
            unsafe {
                fprintf(__stderrp,
                    c"cannot open \"%s\" for reading\n".as_ptr() as *mut i8 as
                        *const i8, z_manifest)
            };
            unsafe { exit(1) };
        }
        sha3_init(&mut ctx, 256);
        while !(unsafe {
                            fgets(&raw mut z_line[0 as usize] as *mut i8,
                                core::mem::size_of::<[i8; 20000]>() as i32, in_)
                        }).is_null() {
            if unsafe {
                        strncmp(&raw mut z_line[0 as usize] as *mut i8 as *const i8,
                            c"# Remove this line".as_ptr() as *mut i8 as *const i8,
                            18 as u64)
                    } != 0 {
                sha3_update(&mut ctx,
                    unsafe {
                        let __p =
                            &raw mut z_line[0 as usize] as *mut u8 as *const u8;
                        if __p.is_null() {
                            &[]
                        } else {
                            core::slice::from_raw_parts(__p,
                                unsafe {
                                            strlen(&raw mut z_line[0 as usize] as *mut i8 as *const i8)
                                        } as u32 as usize)
                        }
                    });
            }
            if unsafe {
                        strncmp(&raw mut z_line[0 as usize] as *mut i8 as *const i8,
                            c"D 20".as_ptr() as *mut i8 as *const i8, 4 as u64)
                    } == 0 {
                unsafe {
                    memcpy(&raw mut z_date[0 as usize] as *mut i8 as *mut (),
                        &raw mut z_line[2 as usize] as *const (), 10 as u64)
                };
                z_date[10 as usize] = ' ' as i32 as i8;
                unsafe {
                    memcpy(&raw mut z_date[11 as usize] as *mut (),
                        &raw mut z_line[13 as usize] as *const (), 8 as u64)
                };
                z_date[19 as usize] = 0 as i8;
                continue;
            }
            if unsafe {
                        strncmp(&raw mut z_line[0 as usize] as *mut i8 as *const i8,
                            c"F ".as_ptr() as *mut i8 as *const i8, 2 as u64)
                    } == 0 {
                let z_filename: *mut i8 = &mut z_line[2 as usize];
                let z_m_hash: *mut i8 = next_token(z_filename);
                next_token(z_m_hash);
                if unsafe { strlen(z_m_hash as *const i8) } == 40 as u64 {
                    rc =
                        sha1sum_file(z_filename as *const i8,
                            &raw mut z_hash[0 as usize] as *mut i8);
                } else {
                    rc =
                        sha3sum_file(z_filename as *const i8, 256,
                            &raw mut z_hash[0 as usize] as *mut i8);
                }
                if rc != 0 {
                    all_valid = 0;
                    if b_verbose != 0 {
                        unsafe {
                            printf(c"hash failed: %s\n".as_ptr() as *mut i8 as
                                    *const i8, z_filename)
                        };
                    }
                } else if unsafe {
                            strcmp(&raw mut z_hash[0 as usize] as *mut i8 as *const i8,
                                z_m_hash as *const i8)
                        } != 0 {
                    all_valid = 0;
                    if b_verbose != 0 {
                        unsafe {
                            printf(c"wrong hash: %s\n".as_ptr() as *mut i8 as *const i8,
                                z_filename)
                        };
                        unsafe {
                            printf(c"... expected: %s\n".as_ptr() as *mut i8 as
                                    *const i8, z_m_hash)
                        };
                        unsafe {
                            printf(c"... got:      %s\n".as_ptr() as *mut i8 as
                                    *const i8, &raw mut z_hash[0 as usize] as *mut i8)
                        };
                    }
                }
            }
        }
        unsafe { fclose(in_) };
        digest_to_base16(sha3_final(&mut ctx) as *const u8,
            &raw mut z_hash[0 as usize] as *mut i8, 256 / 8);
        if (all_valid == 0) as i32 != 0 {
            unsafe {
                printf(c"%s %.60salt1\n".as_ptr() as *mut i8 as *const i8,
                    &raw mut z_date[0 as usize] as *mut i8,
                    &raw mut z_hash[0 as usize] as *mut i8)
            };
        } else {
            unsafe {
                printf(c"%s %s\n".as_ptr() as *mut i8 as *const i8,
                    &raw mut z_date[0 as usize] as *mut i8,
                    &raw mut z_hash[0 as usize] as *mut i8)
            };
        }
        return Ok(());
    }
}
static rc_1: [u64; 24] =
    [1, 32898, 9223372036854808714u64, 9223372039002292224u64, 32907,
            2147483649u64, 9223372039002292353u64, 9223372036854808585u64,
            138, 136, 2147516425u64, 2147483658u64, 2147516555u64,
            9223372036854775947u64, 9223372036854808713u64,
            9223372036854808579u64, 9223372036854808578u64,
            9223372036854775936u64, 32778, 9223372039002259466u64,
            9223372039002292353u64, 9223372036854808704u64, 2147483649u64,
            9223372039002292232u64];
static mut one_1: u32 = 1 as u32;
static z_encode: [i8; 17] =
    [48 as i8, 49 as i8, 50 as i8, 51 as i8, 52 as i8, 53 as i8, 54 as i8,
            55 as i8, 56 as i8, 57 as i8, 97 as i8, 98 as i8, 99 as i8,
            100 as i8, 101 as i8, 102 as i8, 0 as i8];
static mut one_2: i32 = 1;
#[unsafe(no_mangle)]
pub extern "C" fn main(argc: i32, argv: *const *mut i8) -> i32 {
    let __r: Result<(), i32> = __main_inner(argc, argv);
    if __r.is_ok() { return 0; }
    return __r.unwrap_err();
}
extern "C" {
    fn __transpiler_isa(child: i32, ancestor: i32)
    -> bool;
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    fn fread(__ptr: *mut (), __size: u64, __nitems: u64, __stream: *mut FILE)
    -> u64;
    fn fclose(_: *mut FILE)
    -> i32;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn exit(_: i32)
    -> ();
    fn isspace(_c: i32)
    -> i32;
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn fgets(_: *mut i8, __size: i32, _: *mut FILE)
    -> *mut i8;
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64)
    -> i32;
    fn strlen(__s: *const i8)
    -> u64;
    fn printf(_: *const i8, ...)
    -> i32;
    static mut __stderrp: *mut FILE;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}
type FILE = SFILE;