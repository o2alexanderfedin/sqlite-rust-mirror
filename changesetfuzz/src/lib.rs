#![allow(unused_imports, dead_code)]

mod sqlite3_h;
pub(crate) use crate::sqlite3_h::*;

type DarwinSizeT = u64;

extern "C" fn usage(argv0: *const i8) -> () {
    unsafe {
        unsafe {
            fprintf(__stderrp,
                c"Usage: %s FILENAME ?SEED N?\n".as_ptr() as *mut i8 as
                    *const i8, argv0)
        };
        unsafe { exit(1) };
    }
}

extern "C" fn fuzz_read_file(z_filename_1: *const i8, p_sz_1: &mut i32,
    pp_buf_1: &mut *mut ()) -> () {
    unsafe {
        let mut f: *mut FILE = core::ptr::null_mut();
        let mut sz: Sqlite3Int64 = 0 as Sqlite3Int64;
        let mut p_buf: *mut () = core::ptr::null_mut();
        f =
            unsafe {
                fopen(z_filename_1, c"rb".as_ptr() as *mut i8 as *const i8)
            };
        if f == core::ptr::null_mut() {
            unsafe {
                fprintf(__stderrp,
                    c"cannot open \"%s\" for reading\n".as_ptr() as *mut i8 as
                        *const i8, z_filename_1)
            };
            unsafe { exit(1) };
        }
        unsafe { fseek(f, 0 as i64, 2) };
        sz = unsafe { ftell(f) } as Sqlite3Int64;
        unsafe { rewind(f) };
        p_buf =
            unsafe {
                sqlite3_malloc64(if sz != 0 { sz } else { 1 as Sqlite3Int64 }
                        as Sqlite3Uint64)
            };
        if p_buf == core::ptr::null_mut() {
            unsafe {
                fprintf(__stderrp,
                    c"cannot allocate %d to hold content of \"%s\"\n".as_ptr()
                            as *mut i8 as *const i8, sz as i32, z_filename_1)
            };
            unsafe { exit(1) };
        }
        if sz > 0 as i64 {
            if unsafe { fread(p_buf, sz as u64, 1 as u64, f) } != 1 as u64 {
                unsafe {
                    fprintf(__stderrp,
                        c"cannot read all %d bytes of \"%s\"\n".as_ptr() as *mut i8
                            as *const i8, sz as i32, z_filename_1)
                };
                unsafe { exit(1) };
            }
            unsafe { fclose(f) };
        }
        *p_sz_1 = sz as i32;
        *pp_buf_1 = p_buf;
    }
}

extern "C" fn fuzz_write_file(z_filename_1: *const i8, p_buf_1: &mut [u8])
    -> () {
    unsafe {
        let mut f: *mut FILE = core::ptr::null_mut();
        f =
            unsafe {
                fopen(z_filename_1, c"wb".as_ptr() as *mut i8 as *const i8)
            };
        if f == core::ptr::null_mut() {
            unsafe {
                fprintf(__stderrp,
                    c"cannot open \"%s\" for writing\n".as_ptr() as *mut i8 as
                        *const i8, z_filename_1)
            };
            unsafe { exit(1) };
        }
        if unsafe {
                    fwrite(p_buf_1.as_ptr() as *const (),
                        p_buf_1.len() as i32 as u64, 1 as u64, f)
                } != 1 as u64 {
            unsafe {
                fprintf(__stderrp,
                    c"cannot write to \"%s\"\n".as_ptr() as *mut i8 as
                        *const i8, z_filename_1)
            };
            unsafe { exit(1) };
        }
        unsafe { fclose(f) };
    }
}

extern "C" fn fuzz_corrupt() -> i32 { return 11; }

#[repr(C)]
#[derive(Copy, Clone)]
struct Sqlite3PrngType {
    i: u8,
    j: u8,
    s: [u8; 256],
}

static mut sqlite3_prng: Sqlite3PrngType =
    Sqlite3PrngType {
        i: 175 as u8,
        j: 40 as u8,
        s: [113 as u8, 245 as u8, 180 as u8, 110 as u8, 128 as u8, 171 as u8,
                29 as u8, 184 as u8, 251 as u8, 183 as u8, 73 as u8,
                191 as u8, 255 as u8, 114 as u8, 45 as u8, 20 as u8,
                121 as u8, 9 as u8, 227 as u8, 120 as u8, 118 as u8,
                176 as u8, 44 as u8, 10 as u8, 142 as u8, 35 as u8, 238 as u8,
                223 as u8, 224 as u8, 154 as u8, 47 as u8, 103 as u8,
                225 as u8, 190 as u8, 14 as u8, 167 as u8, 8 as u8, 151 as u8,
                235 as u8, 119 as u8, 120 as u8, 186 as u8, 157 as u8,
                202 as u8, 73 as u8, 76 as u8, 96 as u8, 154 as u8, 246 as u8,
                189 as u8, 218 as u8, 127 as u8, 188 as u8, 72 as u8,
                88 as u8, 82 as u8, 229 as u8, 205 as u8, 131 as u8,
                114 as u8, 35 as u8, 82 as u8, 255 as u8, 109 as u8,
                239 as u8, 15 as u8, 130 as u8, 41 as u8, 160 as u8,
                131 as u8, 63 as u8, 125 as u8, 164 as u8, 136 as u8,
                49 as u8, 231 as u8, 136 as u8, 146 as u8, 59 as u8,
                155 as u8, 59 as u8, 44 as u8, 194 as u8, 76 as u8, 113 as u8,
                162 as u8, 176 as u8, 234 as u8, 54 as u8, 208 as u8, 0 as u8,
                241 as u8, 211 as u8, 57 as u8, 23 as u8, 93 as u8, 42 as u8,
                122 as u8, 228 as u8, 173 as u8, 225 as u8, 100 as u8,
                206 as u8, 15 as u8, 156 as u8, 217 as u8, 245 as u8,
                237 as u8, 176 as u8, 34 as u8, 94 as u8, 98 as u8, 151 as u8,
                2 as u8, 163 as u8, 140 as u8, 103 as u8, 128 as u8,
                252 as u8, 136 as u8, 20 as u8, 11 as u8, 21 as u8, 16 as u8,
                15 as u8, 199 as u8, 64 as u8, 212 as u8, 241 as u8,
                249 as u8, 14 as u8, 26 as u8, 206 as u8, 185 as u8, 30 as u8,
                161 as u8, 114 as u8, 142 as u8, 215 as u8, 120 as u8,
                57 as u8, 205 as u8, 244 as u8, 93 as u8, 42 as u8, 89 as u8,
                38 as u8, 52 as u8, 242 as u8, 115 as u8, 11 as u8, 160 as u8,
                2 as u8, 81 as u8, 44 as u8, 3 as u8, 163 as u8, 167 as u8,
                67 as u8, 19 as u8, 232 as u8, 152 as u8, 43 as u8, 210 as u8,
                83 as u8, 248 as u8, 238 as u8, 145 as u8, 125 as u8,
                231 as u8, 227 as u8, 218 as u8, 213 as u8, 187 as u8,
                192 as u8, 146 as u8, 157 as u8, 152 as u8, 1 as u8, 44 as u8,
                249 as u8, 185 as u8, 160 as u8, 235 as u8, 207 as u8,
                50 as u8, 250 as u8, 1 as u8, 73 as u8, 165 as u8, 29 as u8,
                154 as u8, 118 as u8, 134 as u8, 63 as u8, 64 as u8,
                212 as u8, 137 as u8, 143 as u8, 156 as u8, 226 as u8,
                227 as u8, 17 as u8, 49 as u8, 55 as u8, 178 as u8, 73 as u8,
                40 as u8, 53 as u8, 192 as u8, 153 as u8, 182 as u8,
                208 as u8, 188 as u8, 102 as u8, 53 as u8, 247 as u8,
                131 as u8, 91 as u8, 215 as u8, 55 as u8, 26 as u8, 43 as u8,
                24 as u8, 166 as u8, 255 as u8, 141 as u8, 124 as u8,
                129 as u8, 168 as u8, 252 as u8, 158 as u8, 196 as u8,
                236 as u8, 128 as u8, 208 as u8, 152 as u8, 167 as u8,
                118 as u8, 204 as u8, 156 as u8, 47 as u8, 123 as u8,
                255 as u8, 142 as u8, 14 as u8, 187 as u8, 144 as u8,
                174 as u8, 19 as u8, 6 as u8, 245 as u8, 28 as u8, 78 as u8,
                82 as u8, 247 as u8],
    };

extern "C" fn fuzz_random_byte() -> u8 {
    unsafe {
        let mut t: u8 = 0 as u8;
        { let __p = &mut sqlite3_prng.i; let __t = *__p; *__p += 1; __t };
        t = sqlite3_prng.s[sqlite3_prng.i as usize];
        sqlite3_prng.j += t as i32 as u8;
        sqlite3_prng.s[sqlite3_prng.i as usize] =
            sqlite3_prng.s[sqlite3_prng.j as usize];
        sqlite3_prng.s[sqlite3_prng.j as usize] = t;
        t += sqlite3_prng.s[sqlite3_prng.i as usize] as i32 as u8;
        return sqlite3_prng.s[t as usize];
    }
}

extern "C" fn fuzz_random_blob(mut z_buf_1: &mut [u8]) -> () {
    let mut i: i32 = 0;
    {
        i = 0;
        '__b0: loop {
            if !(i < z_buf_1.len() as i32) { break '__b0; }
            '__c0: loop {
                z_buf_1[i as usize] = fuzz_random_byte();
                break '__c0;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

extern "C" fn fuzz_random_int(n_range_1: u32) -> u32 {
    let mut ret: u32 = 0 as u32;
    if !(n_range_1 > 0 as u32) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fuzzRandomInt".as_ptr() as *const i8,
                c"changesetfuzz.c".as_ptr() as *mut i8 as *const i8, 273,
                c"nRange>0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    fuzz_random_blob(unsafe {
            let __p = &raw mut ret as *mut u8 as *mut u8;
            if __p.is_null() {
                &mut []
            } else {
                core::slice::from_raw_parts_mut(__p,
                    core::mem::size_of::<u32>() as usize)
            }
        });
    return ret % n_range_1;
}

extern "C" fn fuzz_random_u64() -> u64 {
    let mut ret: u64 = 0 as u64;
    fuzz_random_blob(unsafe {
            let __p = &raw mut ret as *mut u8 as *mut u8;
            if __p.is_null() {
                &mut []
            } else {
                core::slice::from_raw_parts_mut(__p,
                    core::mem::size_of::<u64>() as usize)
            }
        });
    return ret;
}

extern "C" fn fuzz_random_seed(i_seed_1: u32) -> () {
    unsafe {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b1: loop {
                if !(i < 256) { break '__b1; }
                '__c1: loop {
                    sqlite3_prng.s[i as usize] ^=
                        (i_seed_1 >> 24 & 255 as u32) as u8;
                    sqlite3_prng.s[(i + 1) as usize] ^=
                        (i_seed_1 >> 16 & 255 as u32) as u8;
                    sqlite3_prng.s[(i + 2) as usize] ^=
                        (i_seed_1 >> 8 & 255 as u32) as u8;
                    sqlite3_prng.s[(i + 3) as usize] ^=
                        (i_seed_1 >> 0 & 255 as u32) as u8;
                    break '__c1;
                }
                i += 4;
            }
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FuzzChangeset {
    b_patchset: i32,
    ap_group: *mut *mut FuzzChangesetGroup,
    n_group: i32,
    ap_val: *mut *mut u8,
    n_val: i32,
    n_change: i32,
    n_update: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FuzzChangesetGroup {
    z_tab: *const i8,
    n_col: i32,
    a_pk: *mut u8,
    a_change: *mut u8,
    sz_change: i32,
    n_change: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FuzzChange {
    e_type: i32,
    i_change: i32,
    i_group: i32,
    i_delete: i32,
    p_sub1: *mut u8,
    p_sub2: *mut u8,
    a_sub: [u8; 128],
    i_current: i32,
}

extern "C" fn fuzz_malloc(n_byte_1: Sqlite3Int64) -> *mut () {
    let p_ret: *mut () =
        unsafe { sqlite3_malloc64(n_byte_1 as Sqlite3Uint64) };
    if !(p_ret).is_null() { unsafe { memset(p_ret, 0, n_byte_1 as u64) }; }
    return p_ret;
}

extern "C" fn fuzz_free(p: *mut ()) -> () { unsafe { sqlite3_free(p) }; }

extern "C" fn fuzz_get_varint(p: *const u8, pn_val_1: &mut i32) -> i32 {
    let mut i: i32 = 0;
    let mut n_val: Sqlite3Uint64 = 0 as Sqlite3Uint64;
    {
        i = 0;
        '__b2: loop {
            if !(i < 9) { break '__b2; }
            '__c2: loop {
                n_val =
                    (n_val << 7) +
                        (unsafe { *p.offset(i as isize) } as i32 & 127) as
                            Sqlite3Uint64;
                if unsafe { *p.offset(i as isize) } as i32 & 128 == 0 {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    break '__b2;
                }
                break '__c2;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    *pn_val_1 = n_val as i32;
    return i;
}

extern "C" fn fuzz_put_varint(p: *mut u8, n_val_1: i32) -> i32 {
    if !(n_val_1 > 0 && n_val_1 < 2097152) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fuzzPutVarint".as_ptr() as *const i8,
                c"changesetfuzz.c".as_ptr() as *mut i8 as *const i8, 386,
                c"nVal>0 && nVal<2097152".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    if n_val_1 < 128 {
        unsafe { *p.offset(0 as isize) = n_val_1 as u8 };
        return 1;
    }
    if n_val_1 < 16384 {
        unsafe { *p.offset(0 as isize) = (n_val_1 >> 7 & 127 | 128) as u8 };
        unsafe { *p.offset(1 as isize) = (n_val_1 & 127) as u8 };
        return 2;
    }
    unsafe { *p.offset(0 as isize) = (n_val_1 >> 14 & 127 | 128) as u8 };
    unsafe { *p.offset(1 as isize) = (n_val_1 >> 7 & 127 | 128) as u8 };
    unsafe { *p.offset(2 as isize) = (n_val_1 & 127) as u8 };
    return 3;
}

extern "C" fn fuzz_get_i64(a_rec_1: *const u8) -> i64 {
    return (((unsafe { *a_rec_1.offset(0 as isize) } as u64) << 56) +
                                        ((unsafe { *a_rec_1.offset(1 as isize) } as u64) << 48) +
                                    ((unsafe { *a_rec_1.offset(2 as isize) } as u64) << 40) +
                                ((unsafe { *a_rec_1.offset(3 as isize) } as u64) << 32) +
                            ((unsafe { *a_rec_1.offset(4 as isize) } as u64) << 24) +
                        ((unsafe { *a_rec_1.offset(5 as isize) } as u64) << 16) +
                    ((unsafe { *a_rec_1.offset(6 as isize) } as u64) << 8) +
                ((unsafe { *a_rec_1.offset(7 as isize) } as u64) << 0)) as
            i64;
}

extern "C" fn fuzz_put_u64(a_rec_1: *mut u8, i_val_1: u64) -> () {
    unsafe {
        *a_rec_1.offset(0 as isize) = (i_val_1 >> 56 & 255 as u64) as u8
    };
    unsafe {
        *a_rec_1.offset(1 as isize) = (i_val_1 >> 48 & 255 as u64) as u8
    };
    unsafe {
        *a_rec_1.offset(2 as isize) = (i_val_1 >> 40 & 255 as u64) as u8
    };
    unsafe {
        *a_rec_1.offset(3 as isize) = (i_val_1 >> 32 & 255 as u64) as u8
    };
    unsafe {
        *a_rec_1.offset(4 as isize) = (i_val_1 >> 24 & 255 as u64) as u8
    };
    unsafe {
        *a_rec_1.offset(5 as isize) = (i_val_1 >> 16 & 255 as u64) as u8
    };
    unsafe {
        *a_rec_1.offset(6 as isize) = (i_val_1 >> 8 & 255 as u64) as u8
    };
    unsafe { *a_rec_1.offset(7 as isize) = (i_val_1 & 255 as u64) as u8 };
}

extern "C" fn fuzz_parse_header(p_parse_1: &FuzzChangeset,
    pp_hdr_1: &mut *mut u8, p_end_1: *mut u8,
    pp_grp_1: &mut *mut FuzzChangesetGroup) -> i32 {
    let mut rc: i32 = 0;
    let mut p_grp: *mut FuzzChangesetGroup = core::ptr::null_mut();
    let c_hdr: u8 =
        if (*p_parse_1).b_patchset != 0 { 'P' as i32 } else { 'T' as i32 } as
            u8;
    if !(p_end_1 > *pp_hdr_1) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fuzzParseHeader".as_ptr() as *const i8,
                c"changesetfuzz.c".as_ptr() as *mut i8 as *const i8, 449,
                c"pEnd>(*ppHdr)".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    p_grp =
        fuzz_malloc(core::mem::size_of::<FuzzChangesetGroup>() as
                    Sqlite3Int64) as *mut FuzzChangesetGroup;
    if (p_grp).is_null() as i32 != 0 {
        rc = 7;
    } else {
        let mut p: *mut u8 = *pp_hdr_1;
        if unsafe { *p.offset(0 as isize) } as i32 != c_hdr as i32 {
            rc = fuzz_corrupt();
        } else {
            {
                let __p = &mut p;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
            {
                let __n =
                    fuzz_get_varint(p as *const u8,
                        unsafe { &mut (*p_grp).n_col });
                let __p = &mut p;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            unsafe { (*p_grp).a_pk = p };
            {
                let __n = unsafe { (*p_grp).n_col };
                let __p = &mut p;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            unsafe { (*p_grp).z_tab = p as *const i8 };
            p =
                unsafe {
                    p.add((unsafe { strlen(p as *const i8) } + 1 as u64) as
                            usize)
                };
            if p >= p_end_1 { rc = fuzz_corrupt(); }
        }
        *pp_hdr_1 = p;
    }
    if rc != 0 { fuzz_free(p_grp as *mut ()); p_grp = core::ptr::null_mut(); }
    *pp_grp_1 = p_grp;
    return rc;
}

extern "C" fn fuzz_change_size(p: *mut u8, p_sz_1: &mut i32) -> i32 {
    let e_type: u8 = unsafe { *p.offset(0 as isize) };
    '__s3:
        {
        match e_type {
            0 => { *p_sz_1 = 1; }
            5 => { *p_sz_1 = 1; }
            1 => { *p_sz_1 = 9; }
            2 => { *p_sz_1 = 9; }
            3 => {
                {
                    let mut n_txt: i32 = 0;
                    let mut sz: i32 = 0;
                    sz =
                        fuzz_get_varint(unsafe { &raw mut *p.offset(1 as isize) } as
                                *const u8, &mut n_txt);
                    *p_sz_1 = 1 + sz + n_txt;
                    break '__s3;
                }
                return fuzz_corrupt();
            }
            4 => {
                {
                    let mut n_txt: i32 = 0;
                    let mut sz: i32 = 0;
                    sz =
                        fuzz_get_varint(unsafe { &raw mut *p.offset(1 as isize) } as
                                *const u8, &mut n_txt);
                    *p_sz_1 = 1 + sz + n_txt;
                    break '__s3;
                }
                return fuzz_corrupt();
            }
            _ => { return fuzz_corrupt(); }
        }
    }
    return 0;
}

extern "C" fn fuzz_parse_record(pp_rec_1: &mut *mut u8, p_end_1: *mut u8,
    p_parse_1: &mut FuzzChangeset, b_pk_only_1: i32) -> i32 {
    let mut rc: i32 = 0;
    let p_grp: *const FuzzChangesetGroup =
        unsafe {
                *(*p_parse_1).ap_group.offset(((*p_parse_1).n_group - 1) as
                            isize)
            } as *const FuzzChangesetGroup;
    let mut i: i32 = 0;
    let mut p: *mut u8 = *pp_rec_1;
    {
        i = 0;
        '__b4: loop {
            if !(rc == 0 && i < unsafe { (*p_grp).n_col }) { break '__b4; }
            '__c4: loop {
                if b_pk_only_1 == 0 ||
                        unsafe { *unsafe { (*p_grp).a_pk.offset(i as isize) } } != 0
                    {
                    let mut sz: i32 = 0;
                    if p >= p_end_1 { break '__b4; }
                    if (*p_parse_1).n_val & (*p_parse_1).n_val - 1 == 0 {
                        let n_new: i32 =
                            if (*p_parse_1).n_val != 0 {
                                (*p_parse_1).n_val * 2
                            } else { 4 };
                        let ap_new: *mut *mut u8 =
                            unsafe {
                                    sqlite3_realloc((*p_parse_1).ap_val as *mut (),
                                        (n_new as u64 * core::mem::size_of::<*mut u8>() as u64) as
                                            i32)
                                } as *mut *mut u8;
                        if ap_new == core::ptr::null_mut() { return 7; }
                        (*p_parse_1).ap_val = ap_new;
                    }
                    unsafe {
                        *(*p_parse_1).ap_val.offset({
                                            let __p = &mut (*p_parse_1).n_val;
                                            let __t = *__p;
                                            *__p += 1;
                                            __t
                                        } as isize) = p
                    };
                    rc = fuzz_change_size(p, &mut sz);
                    {
                        let __n = sz;
                        let __p = &mut p;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                }
                break '__c4;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if rc == 0 && i < unsafe { (*p_grp).n_col } { rc = fuzz_corrupt(); }
    *pp_rec_1 = p;
    return rc;
}

extern "C" fn fuzz_parse_changes(pp_data_1: &mut *mut u8, p_end_1: *mut u8,
    p_parse_1: *mut FuzzChangeset) -> i32 {
    let c_hdr: u8 =
        if unsafe { (*p_parse_1).b_patchset } != 0 {
                'P' as i32
            } else { 'T' as i32 } as u8;
    let p_grp: *mut FuzzChangesetGroup =
        unsafe {
            *unsafe {
                    (*p_parse_1).ap_group.offset((unsafe {
                                    (*p_parse_1).n_group
                                } - 1) as isize)
                }
        };
    let mut rc: i32 = 0;
    let mut p: *mut u8 = *pp_data_1;
    unsafe { (*p_grp).a_change = p };
    while rc == 0 && p < p_end_1 &&
            unsafe { *p.offset(0 as isize) } as i32 != c_hdr as i32 {
        let e_op: u8 = unsafe { *p.offset(0 as isize) };
        let b_indirect: u8 = unsafe { *p.offset(1 as isize) };
        {
            let __n = 2;
            let __p = &mut p;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
        if e_op as i32 == 23 {
            {
                let __p = unsafe { &mut (*p_parse_1).n_update };
                let __t = *__p;
                *__p += 1;
                __t
            };
            if unsafe { (*p_parse_1).b_patchset } == 0 {
                rc =
                    fuzz_parse_record(&mut p, p_end_1,
                        unsafe { &mut *p_parse_1 }, 0);
            }
        } else if e_op as i32 != 18 && e_op as i32 != 9 {
            rc = fuzz_corrupt();
        }
        if rc == 0 {
            let b_pk_only: i32 =
                (e_op as i32 == 9 && unsafe { (*p_parse_1).b_patchset } != 0)
                    as i32;
            rc =
                fuzz_parse_record(&mut p, p_end_1, unsafe { &mut *p_parse_1 },
                    b_pk_only);
        }
        {
            let __p = unsafe { &mut (*p_grp).n_change };
            let __t = *__p;
            *__p += 1;
            __t
        };
        {
            let __p = unsafe { &mut (*p_parse_1).n_change };
            let __t = *__p;
            *__p += 1;
            __t
        };
    }
    unsafe {
        (*p_grp).sz_change =
            unsafe { p.offset_from(unsafe { (*p_grp).a_change }) } as i64 as
                i32
    };
    *pp_data_1 = p;
    return rc;
}

extern "C" fn fuzz_parse_changeset(p_changeset_1: *mut u8, n_changeset_1: i32,
    p_parse_1: *mut FuzzChangeset) -> i32 {
    let p_end: *mut u8 =
        unsafe { &mut *p_changeset_1.offset(n_changeset_1 as isize) };
    let mut p: *mut u8 = p_changeset_1;
    let mut rc: i32 = 0;
    unsafe {
        memset(p_parse_1 as *mut (), 0,
            core::mem::size_of::<FuzzChangeset>() as u64)
    };
    if n_changeset_1 > 0 {
        unsafe {
            (*p_parse_1).b_patchset =
                (unsafe { *p_changeset_1.offset(0 as isize) } as i32 ==
                        'P' as i32) as i32
        };
    }
    while rc == 0 && p < p_end {
        let mut p_grp: *mut FuzzChangesetGroup = core::ptr::null_mut();
        rc =
            fuzz_parse_header(unsafe { &*p_parse_1 }, &mut p, p_end,
                &mut p_grp);
        if !((rc == 0) as i32 == (p_grp != core::ptr::null_mut()) as i32) as
                        i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"fuzzParseChangeset".as_ptr() as *const i8,
                    c"changesetfuzz.c".as_ptr() as *mut i8 as *const i8, 630,
                    c"(rc==SQLITE_OK)==(pGrp!=0)".as_ptr() as *mut i8 as
                        *const i8)
            }
        } else { { let _ = 0; } };
        if rc == 0 {
            let ap_new: *mut *mut FuzzChangesetGroup =
                unsafe {
                        sqlite3_realloc64(unsafe { (*p_parse_1).ap_group } as
                                *mut (),
                            core::mem::size_of::<*mut FuzzChangesetGroup>() as u64 *
                                (unsafe { (*p_parse_1).n_group } + 1) as u64)
                    } as *mut *mut FuzzChangesetGroup;
            if ap_new == core::ptr::null_mut() {
                rc = 7;
            } else {
                unsafe {
                    *ap_new.offset(unsafe { (*p_parse_1).n_group } as isize) =
                        p_grp
                };
                unsafe { (*p_parse_1).ap_group = ap_new };
                {
                    let __p = unsafe { &mut (*p_parse_1).n_group };
                    let __t = *__p;
                    *__p += 1;
                    __t
                };
            }
            rc = fuzz_parse_changes(&mut p, p_end, p_parse_1);
        }
    }
    return rc;
}

extern "C" fn fuzz_print_record(p_grp_1: &FuzzChangesetGroup,
    pp_rec_1: &mut *mut u8, b_pk_only_1: i32) -> i32 {
    let rc: i32 = 0;
    let mut p: *mut u8 = *pp_rec_1;
    let mut i: i32 = 0;
    let mut z_pre: *const i8 = c" (".as_ptr() as *mut i8 as *const i8;
    {
        i = 0;
        '__b7: loop {
            if !(i < (*p_grp_1).n_col) { break '__b7; }
            '__c7: loop {
                if b_pk_only_1 == 0 ||
                        unsafe { *(*p_grp_1).a_pk.offset(i as isize) } != 0 {
                    let e_type: u8 =
                        unsafe {
                            *{
                                        let __p = &mut p;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    }.offset(0 as isize)
                        };
                    '__s8:
                        {
                        match e_type {
                            0 => {
                                unsafe {
                                    printf(c"%sn/a".as_ptr() as *mut i8 as *const i8, z_pre)
                                };
                            }
                            1 => {
                                {
                                    let mut i_val: Sqlite3Int64 = 0 as Sqlite3Int64;
                                    i_val = fuzz_get_i64(p as *const u8);
                                    unsafe {
                                        printf(c"%s%lld".as_ptr() as *mut i8 as *const i8, z_pre,
                                            i_val)
                                    };
                                    {
                                        let __n = 8;
                                        let __p = &mut p;
                                        *__p = unsafe { (*__p).offset(__n as isize) };
                                    };
                                    break '__s8;
                                }
                                {
                                    let mut i_val_1: Sqlite3Int64 = 0 as Sqlite3Int64;
                                    let mut f_val: f64 = 0.0;
                                    i_val_1 = fuzz_get_i64(p as *const u8);
                                    unsafe {
                                        memcpy(&raw mut f_val as *mut (),
                                            &raw mut i_val_1 as *const (), 8 as u64)
                                    };
                                    unsafe {
                                        printf(c"%s%f".as_ptr() as *mut i8 as *const i8, z_pre,
                                            f_val)
                                    };
                                    {
                                        let __n = 8;
                                        let __p = &mut p;
                                        *__p = unsafe { (*__p).offset(__n as isize) };
                                    };
                                    break '__s8;
                                }
                                {
                                    let mut n_txt: i32 = 0;
                                    {
                                        let __n = fuzz_get_varint(p as *const u8, &mut n_txt);
                                        let __p = &mut p;
                                        *__p = unsafe { (*__p).offset(__n as isize) };
                                    };
                                    unsafe {
                                        printf(c"%s%s".as_ptr() as *mut i8 as *const i8, z_pre,
                                            if e_type as i32 == 3 {
                                                c"\'".as_ptr() as *mut i8
                                            } else { c"X\'".as_ptr() as *mut i8 })
                                    };
                                    {
                                        i = 0;
                                        '__b9: loop {
                                            if !(i < n_txt) { break '__b9; }
                                            '__c9: loop {
                                                if e_type as i32 == 3 {
                                                    unsafe {
                                                        printf(c"%c".as_ptr() as *mut i8 as *const i8,
                                                            unsafe { *p.offset(i as isize) } as i32)
                                                    };
                                                } else {
                                                    let a_hex: [i8; 16] =
                                                        ['0' as i32 as i8, '1' as i32 as i8, '2' as i32 as i8,
                                                                '3' as i32 as i8, '4' as i32 as i8, '5' as i32 as i8,
                                                                '6' as i32 as i8, '7' as i32 as i8, '8' as i32 as i8,
                                                                '9' as i32 as i8, 'A' as i32 as i8, 'B' as i32 as i8,
                                                                'C' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8,
                                                                'F' as i32 as i8];
                                                    unsafe {
                                                        printf(c"%c".as_ptr() as *mut i8 as *const i8,
                                                            a_hex[(unsafe { *p.offset(i as isize) } as i32 >> 4) as
                                                                        usize] as i32)
                                                    };
                                                    unsafe {
                                                        printf(c"%c".as_ptr() as *mut i8 as *const i8,
                                                            a_hex[(unsafe { *p.offset(i as isize) } as i32 & 15) as
                                                                        usize] as i32)
                                                    };
                                                }
                                                break '__c9;
                                            }
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                        }
                                    }
                                    unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                                    {
                                        let __n = n_txt;
                                        let __p = &mut p;
                                        *__p = unsafe { (*__p).offset(__n as isize) };
                                    };
                                    break '__s8;
                                }
                                unsafe {
                                    printf(c"%sNULL".as_ptr() as *mut i8 as *const i8, z_pre)
                                };
                            }
                            2 => {
                                {
                                    let mut i_val_1: Sqlite3Int64 = 0 as Sqlite3Int64;
                                    let mut f_val: f64 = 0.0;
                                    i_val_1 = fuzz_get_i64(p as *const u8);
                                    unsafe {
                                        memcpy(&raw mut f_val as *mut (),
                                            &raw mut i_val_1 as *const (), 8 as u64)
                                    };
                                    unsafe {
                                        printf(c"%s%f".as_ptr() as *mut i8 as *const i8, z_pre,
                                            f_val)
                                    };
                                    {
                                        let __n = 8;
                                        let __p = &mut p;
                                        *__p = unsafe { (*__p).offset(__n as isize) };
                                    };
                                    break '__s8;
                                }
                                {
                                    let mut n_txt: i32 = 0;
                                    {
                                        let __n = fuzz_get_varint(p as *const u8, &mut n_txt);
                                        let __p = &mut p;
                                        *__p = unsafe { (*__p).offset(__n as isize) };
                                    };
                                    unsafe {
                                        printf(c"%s%s".as_ptr() as *mut i8 as *const i8, z_pre,
                                            if e_type as i32 == 3 {
                                                c"\'".as_ptr() as *mut i8
                                            } else { c"X\'".as_ptr() as *mut i8 })
                                    };
                                    {
                                        i = 0;
                                        '__b9: loop {
                                            if !(i < n_txt) { break '__b9; }
                                            '__c9: loop {
                                                if e_type as i32 == 3 {
                                                    unsafe {
                                                        printf(c"%c".as_ptr() as *mut i8 as *const i8,
                                                            unsafe { *p.offset(i as isize) } as i32)
                                                    };
                                                } else {
                                                    let a_hex: [i8; 16] =
                                                        ['0' as i32 as i8, '1' as i32 as i8, '2' as i32 as i8,
                                                                '3' as i32 as i8, '4' as i32 as i8, '5' as i32 as i8,
                                                                '6' as i32 as i8, '7' as i32 as i8, '8' as i32 as i8,
                                                                '9' as i32 as i8, 'A' as i32 as i8, 'B' as i32 as i8,
                                                                'C' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8,
                                                                'F' as i32 as i8];
                                                    unsafe {
                                                        printf(c"%c".as_ptr() as *mut i8 as *const i8,
                                                            a_hex[(unsafe { *p.offset(i as isize) } as i32 >> 4) as
                                                                        usize] as i32)
                                                    };
                                                    unsafe {
                                                        printf(c"%c".as_ptr() as *mut i8 as *const i8,
                                                            a_hex[(unsafe { *p.offset(i as isize) } as i32 & 15) as
                                                                        usize] as i32)
                                                    };
                                                }
                                                break '__c9;
                                            }
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                        }
                                    }
                                    unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                                    {
                                        let __n = n_txt;
                                        let __p = &mut p;
                                        *__p = unsafe { (*__p).offset(__n as isize) };
                                    };
                                    break '__s8;
                                }
                                unsafe {
                                    printf(c"%sNULL".as_ptr() as *mut i8 as *const i8, z_pre)
                                };
                            }
                            3 => {
                                {
                                    let mut n_txt: i32 = 0;
                                    {
                                        let __n = fuzz_get_varint(p as *const u8, &mut n_txt);
                                        let __p = &mut p;
                                        *__p = unsafe { (*__p).offset(__n as isize) };
                                    };
                                    unsafe {
                                        printf(c"%s%s".as_ptr() as *mut i8 as *const i8, z_pre,
                                            if e_type as i32 == 3 {
                                                c"\'".as_ptr() as *mut i8
                                            } else { c"X\'".as_ptr() as *mut i8 })
                                    };
                                    {
                                        i = 0;
                                        '__b9: loop {
                                            if !(i < n_txt) { break '__b9; }
                                            '__c9: loop {
                                                if e_type as i32 == 3 {
                                                    unsafe {
                                                        printf(c"%c".as_ptr() as *mut i8 as *const i8,
                                                            unsafe { *p.offset(i as isize) } as i32)
                                                    };
                                                } else {
                                                    let a_hex: [i8; 16] =
                                                        ['0' as i32 as i8, '1' as i32 as i8, '2' as i32 as i8,
                                                                '3' as i32 as i8, '4' as i32 as i8, '5' as i32 as i8,
                                                                '6' as i32 as i8, '7' as i32 as i8, '8' as i32 as i8,
                                                                '9' as i32 as i8, 'A' as i32 as i8, 'B' as i32 as i8,
                                                                'C' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8,
                                                                'F' as i32 as i8];
                                                    unsafe {
                                                        printf(c"%c".as_ptr() as *mut i8 as *const i8,
                                                            a_hex[(unsafe { *p.offset(i as isize) } as i32 >> 4) as
                                                                        usize] as i32)
                                                    };
                                                    unsafe {
                                                        printf(c"%c".as_ptr() as *mut i8 as *const i8,
                                                            a_hex[(unsafe { *p.offset(i as isize) } as i32 & 15) as
                                                                        usize] as i32)
                                                    };
                                                }
                                                break '__c9;
                                            }
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                        }
                                    }
                                    unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                                    {
                                        let __n = n_txt;
                                        let __p = &mut p;
                                        *__p = unsafe { (*__p).offset(__n as isize) };
                                    };
                                    break '__s8;
                                }
                                unsafe {
                                    printf(c"%sNULL".as_ptr() as *mut i8 as *const i8, z_pre)
                                };
                            }
                            4 => {
                                {
                                    let mut n_txt: i32 = 0;
                                    {
                                        let __n = fuzz_get_varint(p as *const u8, &mut n_txt);
                                        let __p = &mut p;
                                        *__p = unsafe { (*__p).offset(__n as isize) };
                                    };
                                    unsafe {
                                        printf(c"%s%s".as_ptr() as *mut i8 as *const i8, z_pre,
                                            if e_type as i32 == 3 {
                                                c"\'".as_ptr() as *mut i8
                                            } else { c"X\'".as_ptr() as *mut i8 })
                                    };
                                    {
                                        i = 0;
                                        '__b9: loop {
                                            if !(i < n_txt) { break '__b9; }
                                            '__c9: loop {
                                                if e_type as i32 == 3 {
                                                    unsafe {
                                                        printf(c"%c".as_ptr() as *mut i8 as *const i8,
                                                            unsafe { *p.offset(i as isize) } as i32)
                                                    };
                                                } else {
                                                    let a_hex: [i8; 16] =
                                                        ['0' as i32 as i8, '1' as i32 as i8, '2' as i32 as i8,
                                                                '3' as i32 as i8, '4' as i32 as i8, '5' as i32 as i8,
                                                                '6' as i32 as i8, '7' as i32 as i8, '8' as i32 as i8,
                                                                '9' as i32 as i8, 'A' as i32 as i8, 'B' as i32 as i8,
                                                                'C' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8,
                                                                'F' as i32 as i8];
                                                    unsafe {
                                                        printf(c"%c".as_ptr() as *mut i8 as *const i8,
                                                            a_hex[(unsafe { *p.offset(i as isize) } as i32 >> 4) as
                                                                        usize] as i32)
                                                    };
                                                    unsafe {
                                                        printf(c"%c".as_ptr() as *mut i8 as *const i8,
                                                            a_hex[(unsafe { *p.offset(i as isize) } as i32 & 15) as
                                                                        usize] as i32)
                                                    };
                                                }
                                                break '__c9;
                                            }
                                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                        }
                                    }
                                    unsafe { printf(c"\'".as_ptr() as *mut i8 as *const i8) };
                                    {
                                        let __n = n_txt;
                                        let __p = &mut p;
                                        *__p = unsafe { (*__p).offset(__n as isize) };
                                    };
                                    break '__s8;
                                }
                                unsafe {
                                    printf(c"%sNULL".as_ptr() as *mut i8 as *const i8, z_pre)
                                };
                            }
                            5 => {
                                unsafe {
                                    printf(c"%sNULL".as_ptr() as *mut i8 as *const i8, z_pre)
                                };
                            }
                            _ => {}
                        }
                    }
                    z_pre = c", ".as_ptr() as *mut i8 as *const i8;
                }
                break '__c7;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { printf(c")".as_ptr() as *mut i8 as *const i8) };
    *pp_rec_1 = p;
    return rc;
}

extern "C" fn fuzz_print_group(p_parse_1: *const FuzzChangeset,
    p_grp_1: *mut FuzzChangesetGroup) -> () {
    let mut i: i32 = 0;
    let mut p: *mut u8 = core::ptr::null_mut();
    unsafe {
        printf(c"TABLE:  %s nCol=%d aPK=".as_ptr() as *mut i8 as *const i8,
            unsafe { (*p_grp_1).z_tab }, unsafe { (*p_grp_1).n_col })
    };
    {
        i = 0;
        '__b10: loop {
            if !(i < unsafe { (*p_grp_1).n_col }) { break '__b10; }
            '__c10: loop {
                unsafe {
                    printf(c"%d".as_ptr() as *mut i8 as *const i8,
                        unsafe { *unsafe { (*p_grp_1).a_pk.offset(i as isize) } } as
                            i32)
                };
                break '__c10;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
    p = unsafe { (*p_grp_1).a_change };
    {
        i = 0;
        '__b11: loop {
            if !(i < unsafe { (*p_grp_1).n_change }) { break '__b11; }
            '__c11: loop {
                let e_type: u8 = unsafe { *p.offset(0 as isize) };
                let b_indirect: u8 = unsafe { *p.offset(1 as isize) };
                unsafe {
                    printf(c"%s (ind=%d):".as_ptr() as *mut i8 as *const i8,
                        if e_type as i32 == 18 {
                            c"INSERT".as_ptr() as *mut i8
                        } else {
                            if e_type as i32 == 9 {
                                c"DELETE".as_ptr() as *mut i8
                            } else { c"UPDATE".as_ptr() as *mut i8 }
                        }, b_indirect as i32)
                };
                {
                    let __n = 2;
                    let __p = &mut p;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                if unsafe { (*p_parse_1).b_patchset } == 0 &&
                        e_type as i32 == 23 {
                    fuzz_print_record(unsafe { &*p_grp_1 }, &mut p, 0);
                }
                fuzz_print_record(unsafe { &*p_grp_1 }, &mut p,
                    (e_type as i32 == 9 &&
                            unsafe { (*p_parse_1).b_patchset } != 0) as i32);
                unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
                break '__c11;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
}

extern "C" fn fuzz_select_change(p_parse_1: &FuzzChangeset,
    p_change_1: *mut FuzzChange) -> i32 {
    let mut i_sub: i32 = 0;
    unsafe {
        memset(p_change_1 as *mut (), 0,
            core::mem::size_of::<FuzzChange>() as u64)
    };
    unsafe {
        (*p_change_1).e_type = (fuzz_random_int(14 as u32) + 1 as u32) as i32
    };
    if !(unsafe { (*p_change_1).e_type } == 1 ||
                                                                            unsafe { (*p_change_1).e_type } == 2 ||
                                                                        unsafe { (*p_change_1).e_type } == 3 ||
                                                                    unsafe { (*p_change_1).e_type } == 4 ||
                                                                unsafe { (*p_change_1).e_type } == 5 ||
                                                            unsafe { (*p_change_1).e_type } == 6 ||
                                                        unsafe { (*p_change_1).e_type } == 7 ||
                                                    unsafe { (*p_change_1).e_type } == 8 ||
                                                unsafe { (*p_change_1).e_type } == 9 ||
                                            unsafe { (*p_change_1).e_type } == 10 ||
                                        unsafe { (*p_change_1).e_type } == 11 ||
                                    unsafe { (*p_change_1).e_type } == 12 ||
                                unsafe { (*p_change_1).e_type } == 13 ||
                            unsafe { (*p_change_1).e_type } == 14) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"fuzzSelectChange".as_ptr() as *const i8,
                c"changesetfuzz.c".as_ptr() as *mut i8 as *const i8, 795,
                c"pChange->eType==FUZZ_VALUE_SUB || pChange->eType==FUZZ_VALUE_MOD || pChange->eType==FUZZ_VALUE_RND || pChange->eType==FUZZ_CHANGE_DUP || pChange->eType==FUZZ_CHANGE_DEL || pChange->eType==FUZZ_CHANGE_TYPE || pChange->eType==FUZZ_CHANGE_FIELD || pChange->eType==FUZZ_CHANGE_INDIRECT || pChange->eType==FUZZ_GROUP_DUP || pChange->eType==FUZZ_GROUP_DEL || pChange->eType==FUZZ_GROUP_SWAP || pChange->eType==FUZZ_COLUMN_ADD || pChange->eType==FUZZ_COLUMN_ADDPK || pChange->eType==FUZZ_COLUMN_DEL".as_ptr()
                        as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    unsafe {
        (*p_change_1).i_group =
            fuzz_random_int((*p_parse_1).n_group as u32) as i32
    };
    unsafe {
        (*p_change_1).i_change =
            fuzz_random_int((*p_parse_1).n_change as u32) as i32
    };
    if unsafe { (*p_change_1).e_type } == 7 {
        if (*p_parse_1).n_update == 0 { return -1; }
        unsafe {
            (*p_change_1).i_change =
                fuzz_random_int((*p_parse_1).n_update as u32) as i32
        };
    }
    unsafe { (*p_change_1).i_delete = -1 };
    if unsafe { (*p_change_1).e_type } == 14 {
        let p_grp: *const FuzzChangesetGroup =
            unsafe {
                    *(*p_parse_1).ap_group.offset(unsafe {
                                    (*p_change_1).i_group
                                } as isize)
                } as *const FuzzChangesetGroup;
        let mut i: i32 = 0;
        unsafe {
            (*p_change_1).i_delete =
                fuzz_random_int(unsafe { (*p_grp).n_col } as u32) as i32
        };
        {
            i = unsafe { (*p_grp).n_col } - 1;
            '__b12: loop {
                if !(i >= 0) { break '__b12; }
                '__c12: loop {
                    if unsafe { *unsafe { (*p_grp).a_pk.offset(i as isize) } }
                                != 0 && unsafe { (*p_change_1).i_delete } != i {
                        break '__b12;
                    }
                    break '__c12;
                }
                { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
            }
        }
        if i < 0 { return -1; }
    }
    if unsafe { (*p_change_1).e_type } == 11 {
        let mut p_grp_1: *mut FuzzChangesetGroup = core::ptr::null_mut();
        let mut i_grp: i32 = unsafe { (*p_change_1).i_group };
        if (*p_parse_1).n_group == 1 { return -1; }
        while i_grp == unsafe { (*p_change_1).i_group } {
            i_grp = fuzz_random_int((*p_parse_1).n_group as u32) as i32;
        }
        p_grp_1 =
            unsafe {
                *(*p_parse_1).ap_group.offset(unsafe { (*p_change_1).i_group }
                            as isize)
            };
        unsafe {
            *(*p_parse_1).ap_group.offset(unsafe { (*p_change_1).i_group } as
                            isize) =
                unsafe { *(*p_parse_1).ap_group.offset(i_grp as isize) }
        };
        unsafe { *(*p_parse_1).ap_group.offset(i_grp as isize) = p_grp_1 };
    }
    if unsafe { (*p_change_1).e_type } == 1 ||
                unsafe { (*p_change_1).e_type } == 2 ||
            unsafe { (*p_change_1).e_type } == 3 {
        i_sub = fuzz_random_int((*p_parse_1).n_val as u32) as i32;
        unsafe {
            (*p_change_1).p_sub1 =
                unsafe { *(*p_parse_1).ap_val.offset(i_sub as isize) }
        };
        if unsafe { (*p_change_1).e_type } == 1 {
            i_sub = fuzz_random_int((*p_parse_1).n_val as u32) as i32;
            unsafe {
                (*p_change_1).p_sub2 =
                    unsafe { *(*p_parse_1).ap_val.offset(i_sub as isize) }
            };
        } else {
            unsafe {
                (*p_change_1).p_sub2 =
                    unsafe { &raw mut (*p_change_1).a_sub[0 as usize] } as
                        *mut u8
            };
        }
        if unsafe { (*p_change_1).e_type } == 3 {
            unsafe {
                (*p_change_1).a_sub[0 as usize] =
                    (fuzz_random_int(5 as u32) + 1 as u32) as u8
            };
            '__s14:
                {
                match unsafe { (*p_change_1).a_sub[0 as usize] } {
                    1 => {
                        {
                            let i_val: u64 = fuzz_random_u64();
                            fuzz_put_u64(unsafe {
                                    &mut (*p_change_1).a_sub[1 as usize]
                                }, i_val);
                            break '__s14;
                        }
                        {
                            let mut i_val1: u64 = fuzz_random_u64();
                            let i_val2: u64 = fuzz_random_u64();
                            let mut d: f64 = i_val1 as f64 / i_val2 as f64;
                            unsafe {
                                memcpy(&raw mut i_val1 as *mut (), &raw mut d as *const (),
                                    core::mem::size_of::<u64>() as u64)
                            };
                            fuzz_put_u64(unsafe {
                                    &mut (*p_change_1).a_sub[1 as usize]
                                }, i_val1);
                            break '__s14;
                        }
                        {
                            let n_byte: i32 = fuzz_random_int(48 as u32) as i32;
                            unsafe { (*p_change_1).a_sub[1 as usize] = n_byte as u8 };
                            fuzz_random_blob(unsafe {
                                    let __p =
                                        unsafe { &mut (*p_change_1).a_sub[2 as usize] } as *mut u8;
                                    if __p.is_null() {
                                        &mut []
                                    } else {
                                        core::slice::from_raw_parts_mut(__p, n_byte as usize)
                                    }
                                });
                            if unsafe { (*p_change_1).a_sub[0 as usize] } as i32 == 3 {
                                let mut i: i32 = 0;
                                {
                                    i = 0;
                                    '__b15: loop {
                                        if !(i < n_byte) { break '__b15; }
                                        '__c15: loop {
                                            unsafe {
                                                (*p_change_1).a_sub[(2 + i) as usize] &= 127 as u8
                                            };
                                            break '__c15;
                                        }
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                            }
                            break '__s14;
                        }
                    }
                    2 => {
                        {
                            let mut i_val1: u64 = fuzz_random_u64();
                            let i_val2: u64 = fuzz_random_u64();
                            let mut d: f64 = i_val1 as f64 / i_val2 as f64;
                            unsafe {
                                memcpy(&raw mut i_val1 as *mut (), &raw mut d as *const (),
                                    core::mem::size_of::<u64>() as u64)
                            };
                            fuzz_put_u64(unsafe {
                                    &mut (*p_change_1).a_sub[1 as usize]
                                }, i_val1);
                            break '__s14;
                        }
                        {
                            let n_byte: i32 = fuzz_random_int(48 as u32) as i32;
                            unsafe { (*p_change_1).a_sub[1 as usize] = n_byte as u8 };
                            fuzz_random_blob(unsafe {
                                    let __p =
                                        unsafe { &mut (*p_change_1).a_sub[2 as usize] } as *mut u8;
                                    if __p.is_null() {
                                        &mut []
                                    } else {
                                        core::slice::from_raw_parts_mut(__p, n_byte as usize)
                                    }
                                });
                            if unsafe { (*p_change_1).a_sub[0 as usize] } as i32 == 3 {
                                let mut i: i32 = 0;
                                {
                                    i = 0;
                                    '__b15: loop {
                                        if !(i < n_byte) { break '__b15; }
                                        '__c15: loop {
                                            unsafe {
                                                (*p_change_1).a_sub[(2 + i) as usize] &= 127 as u8
                                            };
                                            break '__c15;
                                        }
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                            }
                            break '__s14;
                        }
                    }
                    3 => {
                        {
                            let n_byte: i32 = fuzz_random_int(48 as u32) as i32;
                            unsafe { (*p_change_1).a_sub[1 as usize] = n_byte as u8 };
                            fuzz_random_blob(unsafe {
                                    let __p =
                                        unsafe { &mut (*p_change_1).a_sub[2 as usize] } as *mut u8;
                                    if __p.is_null() {
                                        &mut []
                                    } else {
                                        core::slice::from_raw_parts_mut(__p, n_byte as usize)
                                    }
                                });
                            if unsafe { (*p_change_1).a_sub[0 as usize] } as i32 == 3 {
                                let mut i: i32 = 0;
                                {
                                    i = 0;
                                    '__b15: loop {
                                        if !(i < n_byte) { break '__b15; }
                                        '__c15: loop {
                                            unsafe {
                                                (*p_change_1).a_sub[(2 + i) as usize] &= 127 as u8
                                            };
                                            break '__c15;
                                        }
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                            }
                            break '__s14;
                        }
                    }
                    4 => {
                        {
                            let n_byte: i32 = fuzz_random_int(48 as u32) as i32;
                            unsafe { (*p_change_1).a_sub[1 as usize] = n_byte as u8 };
                            fuzz_random_blob(unsafe {
                                    let __p =
                                        unsafe { &mut (*p_change_1).a_sub[2 as usize] } as *mut u8;
                                    if __p.is_null() {
                                        &mut []
                                    } else {
                                        core::slice::from_raw_parts_mut(__p, n_byte as usize)
                                    }
                                });
                            if unsafe { (*p_change_1).a_sub[0 as usize] } as i32 == 3 {
                                let mut i: i32 = 0;
                                {
                                    i = 0;
                                    '__b15: loop {
                                        if !(i < n_byte) { break '__b15; }
                                        '__c15: loop {
                                            unsafe {
                                                (*p_change_1).a_sub[(2 + i) as usize] &= 127 as u8
                                            };
                                            break '__c15;
                                        }
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                            }
                            break '__s14;
                        }
                    }
                    _ => {}
                }
            }
        }
        if unsafe { (*p_change_1).e_type } == 2 {
            let mut sz: i32 = 0;
            let mut i_mod: i32 = -1;
            fuzz_change_size(unsafe { (*p_change_1).p_sub1 }, &mut sz);
            unsafe {
                memcpy(unsafe { &raw mut (*p_change_1).a_sub[0 as usize] } as
                            *mut u8 as *mut (),
                    unsafe { (*p_change_1).p_sub1 } as *const (), sz as u64)
            };
            '__s16:
                {
                match unsafe { (*p_change_1).a_sub[0 as usize] } {
                    1 => {
                        i_mod = (fuzz_random_int(8 as u32) + 1 as u32) as i32;
                    }
                    2 => {
                        i_mod = (fuzz_random_int(8 as u32) + 1 as u32) as i32;
                    }
                    3 => {
                        {
                            let mut n_byte_1: i32 = 0;
                            let i_first: i32 =
                                1 +
                                    fuzz_get_varint(unsafe {
                                                &raw mut (*p_change_1).a_sub[1 as usize]
                                            } as *const u8, &mut n_byte_1);
                            if n_byte_1 > 0 {
                                i_mod =
                                    (fuzz_random_int(n_byte_1 as u32) + i_first as u32) as i32;
                            }
                            break '__s16;
                        }
                    }
                    4 => {
                        {
                            let mut n_byte_1: i32 = 0;
                            let i_first: i32 =
                                1 +
                                    fuzz_get_varint(unsafe {
                                                &raw mut (*p_change_1).a_sub[1 as usize]
                                            } as *const u8, &mut n_byte_1);
                            if n_byte_1 > 0 {
                                i_mod =
                                    (fuzz_random_int(n_byte_1 as u32) + i_first as u32) as i32;
                            }
                            break '__s16;
                        }
                    }
                    _ => {}
                }
            }
            if i_mod >= 0 {
                let mask: u8 =
                    (1 <<
                            fuzz_random_int((8 -
                                        (unsafe { (*p_change_1).a_sub[0 as usize] } as i32 == 3) as
                                            i32) as u32)) as u8;
                unsafe {
                    (*p_change_1).a_sub[i_mod as usize] ^= mask as i32 as u8
                };
            }
        }
    }
    return 0;
}

extern "C" fn fuzz_copy_change(p_parse_1: &FuzzChangeset, i_grp_1: i32,
    p_fuzz_1: &mut FuzzChange, pp: &mut *mut u8, pp_out_1: &mut *mut u8)
    -> i32 {
    let b_ps: i32 = (*p_parse_1).b_patchset;
    let p_grp: *const FuzzChangesetGroup =
        unsafe { *(*p_parse_1).ap_group.offset(i_grp_1 as isize) } as
            *const FuzzChangesetGroup;
    let mut p: *mut u8 = *pp;
    let mut p_out: *mut u8 = *pp_out_1;
    let e_type: u8 =
        unsafe {
            *{
                        let __p = &mut p;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    }.offset(0 as isize)
        };
    let mut i_rec: i32 = 0;
    let n_rec: i32 =
        if e_type as i32 == 23 && (b_ps == 0) as i32 != 0 { 2 } else { 1 };
    let mut i_undef: i32 = -1;
    let mut n_update: i32 = 0;
    let mut e_new: u8 = e_type;
    if (*p_fuzz_1).i_current == (*p_fuzz_1).i_change &&
            (*p_fuzz_1).e_type == 6 {
        '__s17:
            {
            match e_type {
                18 => { e_new = 9 as u8; }
                9 => { e_new = 23 as u8; }
                23 => { e_new = 18 as u8; }
                _ => {}
            }
        }
    }
    if (*p_fuzz_1).i_current == (*p_fuzz_1).i_change &&
                (*p_fuzz_1).e_type == 7 && e_type as i32 == 23 {
        let mut sz: i32 = 0;
        let mut i: i32 = 0;
        let mut n_def: i32 = 0;
        let mut p_csr: *mut u8 = unsafe { p.offset(1 as isize) };
        {
            i = 0;
            '__b18: loop {
                if !(i < unsafe { (*p_grp).n_col }) { break '__b18; }
                '__c18: loop {
                    if unsafe { *p_csr.offset(0 as isize) } != 0 &&
                            unsafe { *unsafe { (*p_grp).a_pk.offset(i as isize) } } as
                                    i32 == 0 {
                        { let __p = &mut n_def; let __t = *__p; *__p += 1; __t };
                    }
                    fuzz_change_size(p_csr, &mut sz);
                    {
                        let __n = sz;
                        let __p = &mut p_csr;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    break '__c18;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if n_def <= 1 { return -1; }
        n_def = fuzz_random_int(n_def as u32) as i32;
        p_csr = unsafe { p.offset(1 as isize) };
        {
            i = 0;
            '__b19: loop {
                if !(i < unsafe { (*p_grp).n_col }) { break '__b19; }
                '__c19: loop {
                    if unsafe { *p_csr.offset(0 as isize) } != 0 &&
                            unsafe { *unsafe { (*p_grp).a_pk.offset(i as isize) } } as
                                    i32 == 0 {
                        if n_def == 0 { i_undef = i; }
                        { let __p = &mut n_def; let __t = *__p; *__p -= 1; __t };
                    }
                    fuzz_change_size(p_csr, &mut sz);
                    {
                        let __n = sz;
                        let __p = &mut p_csr;
                        *__p = unsafe { (*__p).offset(__n as isize) };
                    };
                    break '__c19;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
    unsafe {
        *{
                    let __p = &mut p_out;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                } = e_new
    };
    if (*p_fuzz_1).e_type == 8 &&
            (*p_fuzz_1).i_current == (*p_fuzz_1).i_change {
        unsafe {
            *{
                        let __p = &mut p_out;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    } =
                (unsafe {
                                *{
                                        let __p = &mut p;
                                        let __t = *__p;
                                        *__p = unsafe { (*__p).offset(1) };
                                        __t
                                    }
                            } == 0) as i32 as u8
        };
    } else {
        unsafe {
            *{
                        let __p = &mut p_out;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    } =
                unsafe {
                    *{
                            let __p = &mut p;
                            let __t = *__p;
                            *__p = unsafe { (*__p).offset(1) };
                            __t
                        }
                }
        };
    }
    {
        i_rec = 0;
        '__b20: loop {
            if !(i_rec < n_rec) { break '__b20; }
            '__c20: loop {
                let mut i: i32 = 0;
                {
                    i = 0;
                    '__b21: loop {
                        if !(i < unsafe { (*p_grp).n_col }) { break '__b21; }
                        '__c21: loop {
                            let mut sz: i32 = 0;
                            let mut p_copy: *mut u8 = p;
                            if b_ps != 0 && e_type as i32 == 9 &&
                                    unsafe { *unsafe { (*p_grp).a_pk.offset(i as isize) } } as
                                            i32 == 0 {
                                if e_type as i32 != e_new as i32 {
                                    if !(e_new as i32 == 23) as i32 as i64 != 0 {
                                        unsafe {
                                            __assert_rtn(c"fuzzCopyChange".as_ptr() as *const i8,
                                                c"changesetfuzz.c".as_ptr() as *mut i8 as *const i8, 990,
                                                c"eNew==SQLITE_UPDATE".as_ptr() as *mut i8 as *const i8)
                                        }
                                    } else { { let _ = 0; } };
                                    '__b22: loop {
                                        '__c22: loop {
                                            p_copy =
                                                unsafe {
                                                    *(*p_parse_1).ap_val.add(fuzz_random_int((*p_parse_1).n_val
                                                                        as u32) as usize)
                                                };
                                            break '__c22;
                                        }
                                        if !(unsafe { *p_copy.offset(0 as isize) } as i32 == 0) {
                                            break '__b22;
                                        }
                                    }
                                    fuzz_change_size(p_copy, &mut sz);
                                    unsafe {
                                        memcpy(p_out as *mut (), p_copy as *const (), sz as u64)
                                    };
                                    {
                                        let __n = sz;
                                        let __p = &mut p_out;
                                        *__p = unsafe { (*__p).offset(__n as isize) };
                                    };
                                }
                                break '__c21;
                            }
                            if p == (*p_fuzz_1).p_sub1 {
                                p_copy = (*p_fuzz_1).p_sub2;
                            } else if p == (*p_fuzz_1).p_sub2 {
                                p_copy = (*p_fuzz_1).p_sub1;
                            } else if i == i_undef {
                                p_copy = b"\x00\x00".as_ptr() as *mut u8;
                            }
                            if unsafe { *p_copy.offset(0 as isize) } as i32 == 0 &&
                                            e_new as i32 != e_type as i32 && e_type as i32 == 23 &&
                                    i_rec == 0 {
                                while unsafe { *p_copy.offset(0 as isize) } as i32 == 0 {
                                    p_copy =
                                        unsafe {
                                            *(*p_parse_1).ap_val.add(fuzz_random_int((*p_parse_1).n_val
                                                                as u32) as usize)
                                        };
                                }
                            } else if unsafe { *p.offset(0 as isize) } as i32 == 0 &&
                                    unsafe { *p_copy.offset(0 as isize) } as i32 != 0 {
                                return -1;
                            } else {
                                if unsafe { *unsafe { (*p_grp).a_pk.offset(i as isize) } }
                                                as i32 > 0 &&
                                        unsafe { *p_copy.offset(0 as isize) } as i32 == 5 {
                                    return -1;
                                }
                            }
                            if ((*p_fuzz_1).i_group != i_grp_1 ||
                                            i != (*p_fuzz_1).i_delete) &&
                                        (e_new as i32 == e_type as i32 || e_type as i32 != 23 ||
                                            i_rec == 0) &&
                                    (e_new as i32 == e_type as i32 || e_new as i32 != 9 ||
                                            (b_ps == 0) as i32 != 0 ||
                                        unsafe { *unsafe { (*p_grp).a_pk.offset(i as isize) } } !=
                                            0) {
                                fuzz_change_size(p_copy, &mut sz);
                                unsafe {
                                    memcpy(p_out as *mut (), p_copy as *const (), sz as u64)
                                };
                                {
                                    let __n = sz;
                                    let __p = &mut p_out;
                                    *__p = unsafe { (*__p).offset(__n as isize) };
                                };
                                n_update +=
                                    (unsafe { *unsafe { (*p_grp).a_pk.offset(i as isize) } } as
                                                    i32 == 0 &&
                                            unsafe { *p_copy.offset(0 as isize) } as i32 != 0) as i32;
                            }
                            fuzz_change_size(p, &mut sz);
                            {
                                let __n = sz;
                                let __p = &mut p;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            break '__c21;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                if i_grp_1 == (*p_fuzz_1).i_group {
                    if (*p_fuzz_1).e_type == 12 {
                        if (b_ps == 0) as i32 != 0 || e_type as i32 != 9 {
                            unsafe {
                                *{
                                            let __p = &mut p_out;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        } = 5 as u8
                            };
                        }
                    } else if (*p_fuzz_1).e_type == 13 {
                        if i_rec == 1 {
                            unsafe {
                                *{
                                            let __p = &mut p_out;
                                            let __t = *__p;
                                            *__p = unsafe { (*__p).offset(1) };
                                            __t
                                        } = 0 as u8
                            };
                        } else {
                            let mut p_new: *mut u8 = core::ptr::null_mut();
                            let mut sz_new: i32 = 0;
                            '__b24: loop {
                                '__c24: loop {
                                    p_new =
                                        unsafe {
                                            *(*p_parse_1).ap_val.add(fuzz_random_int((*p_parse_1).n_val
                                                                as u32) as usize)
                                        };
                                    break '__c24;
                                }
                                if !(unsafe { *p_new.offset(0 as isize) } as i32 == 0 ||
                                                unsafe { *p_new.offset(0 as isize) } as i32 == 5) {
                                    break '__b24;
                                }
                            }
                            fuzz_change_size(p_new, &mut sz_new);
                            unsafe {
                                memcpy(p_out as *mut (), p_new as *const (), sz_new as u64)
                            };
                            {
                                let __n = sz_new;
                                let __p = &mut p_out;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                        }
                    }
                }
                break '__c20;
            }
            { let __p = &mut i_rec; let __t = *__p; *__p += 1; __t };
        }
    }
    if (*p_fuzz_1).i_current == (*p_fuzz_1).i_change {
        if (*p_fuzz_1).e_type == 4 {
            let n_byte: i32 =
                unsafe { p_out.offset_from(*pp_out_1) } as i64 as i32;
            unsafe {
                memcpy(p_out as *mut (), *pp_out_1 as *const (),
                    n_byte as u64)
            };
            {
                let __n = n_byte;
                let __p = &mut p_out;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
        }
        if (*p_fuzz_1).e_type == 5 { p_out = *pp_out_1; }
        if e_new as i32 != e_type as i32 && e_new as i32 == 23 &&
                (b_ps == 0) as i32 != 0 {
            let mut i: i32 = 0;
            let mut p_csr_1: *mut u8 =
                unsafe { (*pp_out_1).offset(2 as isize) };
            {
                i = 0;
                '__b25: loop {
                    if !(i < unsafe { (*p_grp).n_col }) { break '__b25; }
                    '__c25: loop {
                        let mut sz: i32 = 0;
                        let mut p_copy_1: *mut u8 = p_csr_1;
                        if unsafe { *unsafe { (*p_grp).a_pk.offset(i as isize) } }
                                != 0 {
                            p_copy_1 = b"\x00\x00".as_ptr() as *mut u8;
                        }
                        fuzz_change_size(p_copy_1, &mut sz);
                        unsafe {
                            memcpy(p_out as *mut (), p_copy_1 as *const (), sz as u64)
                        };
                        {
                            let __n = sz;
                            let __p = &mut p_out;
                            *__p = unsafe { (*__p).offset(__n as isize) };
                        };
                        fuzz_change_size(p_csr_1, &mut sz);
                        {
                            let __n = sz;
                            let __p = &mut p_csr_1;
                            *__p = unsafe { (*__p).offset(__n as isize) };
                        };
                        break '__c25;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
    }
    if (*p_fuzz_1).e_type == 14 && (*p_fuzz_1).i_group == i_grp_1 &&
                e_type as i32 == 23 && n_update == 0 {
        p_out = *pp_out_1;
    }
    *pp = p;
    *pp_out_1 = p_out;
    (*p_fuzz_1).i_current +=
        (e_type as i32 == 23 || (*p_fuzz_1).e_type != 7) as i32;
    return 0;
}

extern "C" fn fuzz_do_one_fuzz(z_out_1: *const i8, p_buf_1: *mut u8,
    p_parse_1: *mut FuzzChangeset) -> i32 {
    let mut change: FuzzChange = unsafe { core::mem::zeroed() };
    let mut i_grp: i32 = 0;
    let mut rc: i32 = -1;
    while rc < 0 {
        let mut p_out: *mut u8 = p_buf_1;
        rc = fuzz_select_change(unsafe { &*p_parse_1 }, &mut change);
        {
            i_grp = 0;
            '__b27: loop {
                if !(rc == 0 && i_grp < unsafe { (*p_parse_1).n_group }) {
                    break '__b27;
                }
                '__c27: loop {
                    let p_grp: *const FuzzChangesetGroup =
                        unsafe {
                                *unsafe { (*p_parse_1).ap_group.offset(i_grp as isize) }
                            } as *const FuzzChangesetGroup;
                    let n_tab: i32 =
                        (unsafe { strlen(unsafe { (*p_grp).z_tab }) } + 1 as u64) as
                            i32;
                    let mut j: i32 = 0;
                    let mut n_rep: i32 = 1;
                    if change.i_group == i_grp {
                        if change.e_type == 10 {
                            if unsafe { (*p_parse_1).n_group } == 1 { rc = -1; }
                            break '__c27;
                        } else if change.e_type == 9 { n_rep = 2; }
                    }
                    {
                        j = 0;
                        '__b28: loop {
                            if !(j < n_rep) { break '__b28; }
                            '__c28: loop {
                                let mut i: i32 = 0;
                                let mut p_saved: *mut u8 = core::ptr::null_mut();
                                let mut p: *mut u8 = unsafe { (*p_grp).a_change };
                                let mut n_col: i32 = unsafe { (*p_grp).n_col };
                                let mut i_pk_del: i32 = 0;
                                if i_grp == change.i_group {
                                    if change.e_type == 12 || change.e_type == 13 {
                                        { let __p = &mut n_col; let __t = *__p; *__p += 1; __t };
                                    } else if change.e_type == 14 {
                                        { let __p = &mut n_col; let __t = *__p; *__p -= 1; __t };
                                        i_pk_del =
                                            unsafe {
                                                    *unsafe { (*p_grp).a_pk.offset(change.i_delete as isize) }
                                                } as i32;
                                    }
                                }
                                unsafe {
                                    *{
                                                    let __p = &mut p_out;
                                                    let __t = *__p;
                                                    *__p = unsafe { (*__p).offset(1) };
                                                    __t
                                                }.offset(0 as isize) =
                                        if unsafe { (*p_parse_1).b_patchset } != 0 {
                                                'P' as i32
                                            } else { 'T' as i32 } as u8
                                };
                                {
                                    let __n = fuzz_put_varint(p_out, n_col);
                                    let __p = &mut p_out;
                                    *__p = unsafe { (*__p).offset(__n as isize) };
                                };
                                {
                                    i = 0;
                                    '__b29: loop {
                                        if !(i < unsafe { (*p_grp).n_col }) { break '__b29; }
                                        '__c29: loop {
                                            if i_grp != change.i_group || i != change.i_delete {
                                                let mut v: u8 =
                                                    unsafe { *unsafe { (*p_grp).a_pk.offset(i as isize) } };
                                                if i_pk_del != 0 && v as i32 > i_pk_del {
                                                    { let __p = &mut v; let __t = *__p; *__p -= 1; __t };
                                                }
                                                unsafe {
                                                    *{
                                                                let __p = &mut p_out;
                                                                let __t = *__p;
                                                                *__p = unsafe { (*__p).offset(1) };
                                                                __t
                                                            } = v
                                                };
                                            }
                                            break '__c29;
                                        }
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                                if n_col > unsafe { (*p_grp).n_col } {
                                    if change.e_type == 12 {
                                        unsafe {
                                            *{
                                                        let __p = &mut p_out;
                                                        let __t = *__p;
                                                        *__p = unsafe { (*__p).offset(1) };
                                                        __t
                                                    } = 0 as u8
                                        };
                                    } else {
                                        let mut max: u8 = 0 as u8;
                                        {
                                            i = 0;
                                            '__b30: loop {
                                                if !(i < unsafe { (*p_grp).n_col }) { break '__b30; }
                                                '__c30: loop {
                                                    if unsafe { *unsafe { (*p_grp).a_pk.offset(i as isize) } }
                                                                as i32 > max as i32 {
                                                        max =
                                                            unsafe { *unsafe { (*p_grp).a_pk.offset(i as isize) } };
                                                    }
                                                    break '__c30;
                                                }
                                                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                            }
                                        }
                                        unsafe {
                                            *{
                                                        let __p = &mut p_out;
                                                        let __t = *__p;
                                                        *__p = unsafe { (*__p).offset(1) };
                                                        __t
                                                    } = (max as i32 + 1) as u8
                                        };
                                    }
                                }
                                unsafe {
                                    memcpy(p_out as *mut (),
                                        unsafe { (*p_grp).z_tab } as *const (), n_tab as u64)
                                };
                                {
                                    let __n = n_tab;
                                    let __p = &mut p_out;
                                    *__p = unsafe { (*__p).offset(__n as isize) };
                                };
                                p_saved = p_out;
                                {
                                    i = 0;
                                    '__b31: loop {
                                        if !(rc == 0 && i < unsafe { (*p_grp).n_change }) {
                                            break '__b31;
                                        }
                                        '__c31: loop {
                                            rc =
                                                fuzz_copy_change(unsafe { &*p_parse_1 }, i_grp, &mut change,
                                                    &mut p, &mut p_out);
                                            break '__c31;
                                        }
                                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                                if p_out == p_saved { rc = -1; }
                                break '__c28;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    break '__c27;
                }
                { let __p = &mut i_grp; let __t = *__p; *__p += 1; __t };
            }
        }
        if rc == 0 {
            fuzz_write_file(z_out_1,
                unsafe {
                    let __p = p_buf_1 as *mut u8 as *mut u8;
                    if __p.is_null() {
                        &mut []
                    } else {
                        core::slice::from_raw_parts_mut(__p,
                            unsafe { p_out.offset_from(p_buf_1) } as i64 as usize)
                    }
                });
        }
    }
    return rc;
}

extern "C" fn __main_inner(argc: i32, argv: *const *mut i8)
    -> Result<(), i32> {
    let mut n_repeat: i32 = 0;
    let mut i_seed: i32 = 0;
    let mut z_input: *const i8 = core::ptr::null();
    let mut p_changeset: *mut () = core::ptr::null_mut();
    let mut n_changeset: i32 = 0;
    let mut i: i32 = 0;
    let mut changeset: FuzzChangeset = unsafe { core::mem::zeroed() };
    let mut rc: i32 = 0;
    let mut p_buf: *mut u8 = core::ptr::null_mut();
    if argc != 4 && argc != 2 {
        usage(unsafe { *argv.offset(0 as isize) } as *const i8);
    }
    z_input = unsafe { *argv.offset(1 as isize) } as *const i8;
    fuzz_read_file(z_input, &mut n_changeset, &mut p_changeset);
    rc =
        fuzz_parse_changeset(p_changeset as *mut u8, n_changeset,
            &mut changeset);
    if rc == 0 {
        if argc == 2 {
            {
                i = 0;
                '__b32: loop {
                    if !(i < changeset.n_group) { break '__b32; }
                    '__c32: loop {
                        fuzz_print_group(&raw mut changeset as *const FuzzChangeset,
                            unsafe { *changeset.ap_group.offset(i as isize) });
                        break '__c32;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        } else {
            p_buf =
                fuzz_malloc(n_changeset as Sqlite3Int64 * 2 as Sqlite3Int64 +
                            1024 as Sqlite3Int64) as *mut u8;
            if p_buf == core::ptr::null_mut() {
                rc = 7;
            } else {
                i_seed =
                    unsafe {
                        atoi(unsafe { *argv.offset(2 as isize) } as *const i8)
                    };
                n_repeat =
                    unsafe {
                        atoi(unsafe { *argv.offset(3 as isize) } as *const i8)
                    };
                fuzz_random_seed(i_seed as u32);
                {
                    i = 0;
                    '__b33: loop {
                        if !(rc == 0 && i < n_repeat) { break '__b33; }
                        '__c33: loop {
                            let z_out: *mut i8 =
                                unsafe {
                                    sqlite3_mprintf(c"%s-%d".as_ptr() as *mut i8 as *const i8,
                                        z_input, i)
                                };
                            rc =
                                fuzz_do_one_fuzz(z_out as *const i8, p_buf, &mut changeset);
                            unsafe { sqlite3_free(z_out as *mut ()) };
                            break '__c33;
                        }
                        { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    }
                }
                fuzz_free(p_buf as *mut ());
            }
        }
    }
    if rc != 0 {
        eprintln!("error while processing changeset: {}", rc as i32);
    }
    return Err(rc);
}

#[unsafe(no_mangle)]
pub extern "C" fn main(argc: i32, argv: *const *mut i8) -> i32 {
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
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn exit(_: i32)
    -> ();
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    fn fseek(_: *mut FILE, _: i64, _: i32)
    -> i32;
    fn ftell(_: *mut FILE)
    -> i64;
    fn rewind(_: *mut FILE)
    -> ();
    fn fread(__ptr: *mut (), __size: u64, __nitems: u64, __stream: *mut FILE)
    -> u64;
    fn fclose(_: *mut FILE)
    -> i32;
    fn fwrite(__ptr: *const (), __size: u64, __nitems: u64,
    __stream: *mut FILE)
    -> u64;
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn memset(__b: *mut (), __c: i32, __len: u64)
    -> *mut ();
    fn strlen(__s: *const i8)
    -> u64;
    fn printf(_: *const i8, ...)
    -> i32;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn atoi(_: *const i8)
    -> i32;
    static mut __stderrp: *mut FILE;
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}

type FILE = SFILE;
