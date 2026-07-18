type DarwinSizeT = u64;

static z_hdr: [i8; 565] =
    [47 as i8, 42 as i8, 42 as i8, 42 as i8, 42 as i8, 42 as i8, 32 as i8,
            84 as i8, 104 as i8, 105 as i8, 115 as i8, 32 as i8, 102 as i8,
            105 as i8, 108 as i8, 101 as i8, 32 as i8, 99 as i8, 111 as i8,
            110 as i8, 116 as i8, 97 as i8, 105 as i8, 110 as i8, 115 as i8,
            32 as i8, 97 as i8, 117 as i8, 116 as i8, 111 as i8, 109 as i8,
            97 as i8, 116 as i8, 105 as i8, 99 as i8, 97 as i8, 108 as i8,
            108 as i8, 121 as i8, 32 as i8, 103 as i8, 101 as i8, 110 as i8,
            101 as i8, 114 as i8, 97 as i8, 116 as i8, 101 as i8, 100 as i8,
            32 as i8, 99 as i8, 111 as i8, 100 as i8, 101 as i8, 32 as i8,
            42 as i8, 42 as i8, 42 as i8, 42 as i8, 42 as i8, 42 as i8,
            10 as i8, 42 as i8, 42 as i8, 10 as i8, 42 as i8, 42 as i8,
            32 as i8, 84 as i8, 104 as i8, 101 as i8, 32 as i8, 99 as i8,
            111 as i8, 100 as i8, 101 as i8, 32 as i8, 105 as i8, 110 as i8,
            32 as i8, 116 as i8, 104 as i8, 105 as i8, 115 as i8, 32 as i8,
            102 as i8, 105 as i8, 108 as i8, 101 as i8, 32 as i8, 104 as i8,
            97 as i8, 115 as i8, 32 as i8, 98 as i8, 101 as i8, 101 as i8,
            110 as i8, 32 as i8, 97 as i8, 117 as i8, 116 as i8, 111 as i8,
            109 as i8, 97 as i8, 116 as i8, 105 as i8, 99 as i8, 97 as i8,
            108 as i8, 108 as i8, 121 as i8, 32 as i8, 103 as i8, 101 as i8,
            110 as i8, 101 as i8, 114 as i8, 97 as i8, 116 as i8, 101 as i8,
            100 as i8, 32 as i8, 98 as i8, 121 as i8, 10 as i8, 42 as i8,
            42 as i8, 10 as i8, 42 as i8, 42 as i8, 32 as i8, 32 as i8,
            32 as i8, 115 as i8, 113 as i8, 108 as i8, 105 as i8, 116 as i8,
            101 as i8, 47 as i8, 116 as i8, 111 as i8, 111 as i8, 108 as i8,
            47 as i8, 109 as i8, 107 as i8, 107 as i8, 101 as i8, 121 as i8,
            119 as i8, 111 as i8, 114 as i8, 100 as i8, 104 as i8, 97 as i8,
            115 as i8, 104 as i8, 46 as i8, 99 as i8, 10 as i8, 42 as i8,
            42 as i8, 10 as i8, 42 as i8, 42 as i8, 32 as i8, 84 as i8,
            104 as i8, 101 as i8, 32 as i8, 99 as i8, 111 as i8, 100 as i8,
            101 as i8, 32 as i8, 105 as i8, 110 as i8, 32 as i8, 116 as i8,
            104 as i8, 105 as i8, 115 as i8, 32 as i8, 102 as i8, 105 as i8,
            108 as i8, 101 as i8, 32 as i8, 105 as i8, 109 as i8, 112 as i8,
            108 as i8, 101 as i8, 109 as i8, 101 as i8, 110 as i8, 116 as i8,
            115 as i8, 32 as i8, 97 as i8, 32 as i8, 102 as i8, 117 as i8,
            110 as i8, 99 as i8, 116 as i8, 105 as i8, 111 as i8, 110 as i8,
            32 as i8, 116 as i8, 104 as i8, 97 as i8, 116 as i8, 32 as i8,
            100 as i8, 101 as i8, 116 as i8, 101 as i8, 114 as i8, 109 as i8,
            105 as i8, 110 as i8, 101 as i8, 115 as i8, 32 as i8, 119 as i8,
            104 as i8, 101 as i8, 116 as i8, 104 as i8, 101 as i8, 114 as i8,
            10 as i8, 42 as i8, 42 as i8, 32 as i8, 111 as i8, 114 as i8,
            32 as i8, 110 as i8, 111 as i8, 116 as i8, 32 as i8, 97 as i8,
            32 as i8, 103 as i8, 105 as i8, 118 as i8, 101 as i8, 110 as i8,
            32 as i8, 105 as i8, 100 as i8, 101 as i8, 110 as i8, 116 as i8,
            105 as i8, 102 as i8, 105 as i8, 101 as i8, 114 as i8, 32 as i8,
            105 as i8, 115 as i8, 32 as i8, 114 as i8, 101 as i8, 97 as i8,
            108 as i8, 108 as i8, 121 as i8, 32 as i8, 97 as i8, 110 as i8,
            32 as i8, 83 as i8, 81 as i8, 76 as i8, 32 as i8, 107 as i8,
            101 as i8, 121 as i8, 119 as i8, 111 as i8, 114 as i8, 100 as i8,
            46 as i8, 32 as i8, 32 as i8, 84 as i8, 104 as i8, 101 as i8,
            32 as i8, 115 as i8, 97 as i8, 109 as i8, 101 as i8, 32 as i8,
            116 as i8, 104 as i8, 105 as i8, 110 as i8, 103 as i8, 10 as i8,
            42 as i8, 42 as i8, 32 as i8, 109 as i8, 105 as i8, 103 as i8,
            104 as i8, 116 as i8, 32 as i8, 98 as i8, 101 as i8, 32 as i8,
            105 as i8, 109 as i8, 112 as i8, 108 as i8, 101 as i8, 109 as i8,
            101 as i8, 110 as i8, 116 as i8, 101 as i8, 100 as i8, 32 as i8,
            109 as i8, 111 as i8, 114 as i8, 101 as i8, 32 as i8, 100 as i8,
            105 as i8, 114 as i8, 101 as i8, 99 as i8, 116 as i8, 108 as i8,
            121 as i8, 32 as i8, 117 as i8, 115 as i8, 105 as i8, 110 as i8,
            103 as i8, 32 as i8, 97 as i8, 32 as i8, 104 as i8, 97 as i8,
            110 as i8, 100 as i8, 45 as i8, 119 as i8, 114 as i8, 105 as i8,
            116 as i8, 116 as i8, 101 as i8, 110 as i8, 32 as i8, 104 as i8,
            97 as i8, 115 as i8, 104 as i8, 32 as i8, 116 as i8, 97 as i8,
            98 as i8, 108 as i8, 101 as i8, 46 as i8, 10 as i8, 42 as i8,
            42 as i8, 32 as i8, 66 as i8, 117 as i8, 116 as i8, 32 as i8,
            98 as i8, 121 as i8, 32 as i8, 117 as i8, 115 as i8, 105 as i8,
            110 as i8, 103 as i8, 32 as i8, 116 as i8, 104 as i8, 105 as i8,
            115 as i8, 32 as i8, 97 as i8, 117 as i8, 116 as i8, 111 as i8,
            109 as i8, 97 as i8, 116 as i8, 105 as i8, 99 as i8, 97 as i8,
            108 as i8, 108 as i8, 121 as i8, 32 as i8, 103 as i8, 101 as i8,
            110 as i8, 101 as i8, 114 as i8, 97 as i8, 116 as i8, 101 as i8,
            100 as i8, 32 as i8, 99 as i8, 111 as i8, 100 as i8, 101 as i8,
            44 as i8, 32 as i8, 116 as i8, 104 as i8, 101 as i8, 32 as i8,
            115 as i8, 105 as i8, 122 as i8, 101 as i8, 32 as i8, 111 as i8,
            102 as i8, 32 as i8, 116 as i8, 104 as i8, 101 as i8, 32 as i8,
            99 as i8, 111 as i8, 100 as i8, 101 as i8, 10 as i8, 42 as i8,
            42 as i8, 32 as i8, 105 as i8, 115 as i8, 32 as i8, 115 as i8,
            117 as i8, 98 as i8, 115 as i8, 116 as i8, 97 as i8, 110 as i8,
            116 as i8, 105 as i8, 97 as i8, 108 as i8, 108 as i8, 121 as i8,
            32 as i8, 114 as i8, 101 as i8, 100 as i8, 117 as i8, 99 as i8,
            101 as i8, 100 as i8, 46 as i8, 32 as i8, 32 as i8, 84 as i8,
            104 as i8, 105 as i8, 115 as i8, 32 as i8, 105 as i8, 115 as i8,
            32 as i8, 105 as i8, 109 as i8, 112 as i8, 111 as i8, 114 as i8,
            116 as i8, 97 as i8, 110 as i8, 116 as i8, 32 as i8, 102 as i8,
            111 as i8, 114 as i8, 32 as i8, 101 as i8, 109 as i8, 98 as i8,
            101 as i8, 100 as i8, 100 as i8, 101 as i8, 100 as i8, 32 as i8,
            97 as i8, 112 as i8, 112 as i8, 108 as i8, 105 as i8, 99 as i8,
            97 as i8, 116 as i8, 105 as i8, 111 as i8, 110 as i8, 115 as i8,
            10 as i8, 42 as i8, 42 as i8, 32 as i8, 111 as i8, 110 as i8,
            32 as i8, 112 as i8, 108 as i8, 97 as i8, 116 as i8, 102 as i8,
            111 as i8, 114 as i8, 109 as i8, 115 as i8, 32 as i8, 119 as i8,
            105 as i8, 116 as i8, 104 as i8, 32 as i8, 108 as i8, 105 as i8,
            109 as i8, 105 as i8, 116 as i8, 101 as i8, 100 as i8, 32 as i8,
            109 as i8, 101 as i8, 109 as i8, 111 as i8, 114 as i8, 121 as i8,
            46 as i8, 10 as i8, 42 as i8, 47 as i8, 10 as i8, 0 as i8];

#[repr(C)]
#[derive(Copy, Clone)]
struct Keyword {
    z_name: *mut i8,
    z_token_type: *mut i8,
    mask: i32,
    priority: i32,
    id: i32,
    hash: i32,
    offset: i32,
    len: i32,
    prefix: i32,
    longest_suffix: i32,
    i_next: i32,
    substr_id: i32,
    substr_offset: i32,
    z_orig_name: [i8; 20],
}

static mut a_keyword_table: [Keyword; 148] =
    [Keyword {
                z_name: c"ABORT".as_ptr() as *mut i8,
                z_token_type: c"TK_ABORT".as_ptr() as *mut i8,
                mask: 128 | 8192,
                priority: 0,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"ACTION".as_ptr() as *mut i8,
                z_token_type: c"TK_ACTION".as_ptr() as *mut i8,
                mask: 512,
                priority: 0,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"ADD".as_ptr() as *mut i8,
                z_token_type: c"TK_ADD".as_ptr() as *mut i8,
                mask: 1,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"AFTER".as_ptr() as *mut i8,
                z_token_type: c"TK_AFTER".as_ptr() as *mut i8,
                mask: 8192,
                priority: 0,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"ALL".as_ptr() as *mut i8,
                z_token_type: c"TK_ALL".as_ptr() as *mut i8,
                mask: 2,
                priority: 0,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"ALTER".as_ptr() as *mut i8,
                z_token_type: c"TK_ALTER".as_ptr() as *mut i8,
                mask: 1,
                priority: 0,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"ALWAYS".as_ptr() as *mut i8,
                z_token_type: c"TK_ALWAYS".as_ptr() as *mut i8,
                mask: 2097152,
                priority: 0,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"ANALYZE".as_ptr() as *mut i8,
                z_token_type: c"TK_ANALYZE".as_ptr() as *mut i8,
                mask: 4,
                priority: 0,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"AND".as_ptr() as *mut i8,
                z_token_type: c"TK_AND".as_ptr() as *mut i8,
                mask: 2,
                priority: 10,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"AS".as_ptr() as *mut i8,
                z_token_type: c"TK_AS".as_ptr() as *mut i8,
                mask: 2,
                priority: 10,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"ASC".as_ptr() as *mut i8,
                z_token_type: c"TK_ASC".as_ptr() as *mut i8,
                mask: 2,
                priority: 0,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"ATTACH".as_ptr() as *mut i8,
                z_token_type: c"TK_ATTACH".as_ptr() as *mut i8,
                mask: 8,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"AUTOINCREMENT".as_ptr() as *mut i8,
                z_token_type: c"TK_AUTOINCR".as_ptr() as *mut i8,
                mask: 16,
                priority: 0,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"BEFORE".as_ptr() as *mut i8,
                z_token_type: c"TK_BEFORE".as_ptr() as *mut i8,
                mask: 8192,
                priority: 0,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"BEGIN".as_ptr() as *mut i8,
                z_token_type: c"TK_BEGIN".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"BETWEEN".as_ptr() as *mut i8,
                z_token_type: c"TK_BETWEEN".as_ptr() as *mut i8,
                mask: 2,
                priority: 5,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"BY".as_ptr() as *mut i8,
                z_token_type: c"TK_BY".as_ptr() as *mut i8,
                mask: 2,
                priority: 10,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"CASCADE".as_ptr() as *mut i8,
                z_token_type: c"TK_CASCADE".as_ptr() as *mut i8,
                mask: 512,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"CASE".as_ptr() as *mut i8,
                z_token_type: c"TK_CASE".as_ptr() as *mut i8,
                mask: 2,
                priority: 5,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"CAST".as_ptr() as *mut i8,
                z_token_type: c"TK_CAST".as_ptr() as *mut i8,
                mask: 32,
                priority: 5,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"CHECK".as_ptr() as *mut i8,
                z_token_type: c"TK_CHECK".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"COLLATE".as_ptr() as *mut i8,
                z_token_type: c"TK_COLLATE".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"COLUMN".as_ptr() as *mut i8,
                z_token_type: c"TK_COLUMNKW".as_ptr() as *mut i8,
                mask: 1,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"COMMIT".as_ptr() as *mut i8,
                z_token_type: c"TK_COMMIT".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"CONFLICT".as_ptr() as *mut i8,
                z_token_type: c"TK_CONFLICT".as_ptr() as *mut i8,
                mask: 128,
                priority: 0,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"CONSTRAINT".as_ptr() as *mut i8,
                z_token_type: c"TK_CONSTRAINT".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"CREATE".as_ptr() as *mut i8,
                z_token_type: c"TK_CREATE".as_ptr() as *mut i8,
                mask: 2,
                priority: 2,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"CROSS".as_ptr() as *mut i8,
                z_token_type: c"TK_JOIN_KW".as_ptr() as *mut i8,
                mask: 2,
                priority: 3,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"CURRENT".as_ptr() as *mut i8,
                z_token_type: c"TK_CURRENT".as_ptr() as *mut i8,
                mask: 1048576,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"CURRENT_DATE".as_ptr() as *mut i8,
                z_token_type: c"TK_CTIME_KW".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"CURRENT_TIME".as_ptr() as *mut i8,
                z_token_type: c"TK_CTIME_KW".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"CURRENT_TIMESTAMP".as_ptr() as *mut i8,
                z_token_type: c"TK_CTIME_KW".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"DATABASE".as_ptr() as *mut i8,
                z_token_type: c"TK_DATABASE".as_ptr() as *mut i8,
                mask: 8,
                priority: 0,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"DEFAULT".as_ptr() as *mut i8,
                z_token_type: c"TK_DEFAULT".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"DEFERRED".as_ptr() as *mut i8,
                z_token_type: c"TK_DEFERRED".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"DEFERRABLE".as_ptr() as *mut i8,
                z_token_type: c"TK_DEFERRABLE".as_ptr() as *mut i8,
                mask: 512,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"DELETE".as_ptr() as *mut i8,
                z_token_type: c"TK_DELETE".as_ptr() as *mut i8,
                mask: 2,
                priority: 10,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"DESC".as_ptr() as *mut i8,
                z_token_type: c"TK_DESC".as_ptr() as *mut i8,
                mask: 2,
                priority: 3,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"DETACH".as_ptr() as *mut i8,
                z_token_type: c"TK_DETACH".as_ptr() as *mut i8,
                mask: 8,
                priority: 0,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"DISTINCT".as_ptr() as *mut i8,
                z_token_type: c"TK_DISTINCT".as_ptr() as *mut i8,
                mask: 2,
                priority: 5,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"DO".as_ptr() as *mut i8,
                z_token_type: c"TK_DO".as_ptr() as *mut i8,
                mask: 524288,
                priority: 2,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"DROP".as_ptr() as *mut i8,
                z_token_type: c"TK_DROP".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"END".as_ptr() as *mut i8,
                z_token_type: c"TK_END".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"EACH".as_ptr() as *mut i8,
                z_token_type: c"TK_EACH".as_ptr() as *mut i8,
                mask: 8192,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"ELSE".as_ptr() as *mut i8,
                z_token_type: c"TK_ELSE".as_ptr() as *mut i8,
                mask: 2,
                priority: 2,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"ESCAPE".as_ptr() as *mut i8,
                z_token_type: c"TK_ESCAPE".as_ptr() as *mut i8,
                mask: 2,
                priority: 4,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"EXCEPT".as_ptr() as *mut i8,
                z_token_type: c"TK_EXCEPT".as_ptr() as *mut i8,
                mask: 64,
                priority: 4,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"EXCLUSIVE".as_ptr() as *mut i8,
                z_token_type: c"TK_EXCLUSIVE".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"EXCLUDE".as_ptr() as *mut i8,
                z_token_type: c"TK_EXCLUDE".as_ptr() as *mut i8,
                mask: 1048576,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"EXISTS".as_ptr() as *mut i8,
                z_token_type: c"TK_EXISTS".as_ptr() as *mut i8,
                mask: 2,
                priority: 4,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"EXPLAIN".as_ptr() as *mut i8,
                z_token_type: c"TK_EXPLAIN".as_ptr() as *mut i8,
                mask: 256,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"FAIL".as_ptr() as *mut i8,
                z_token_type: c"TK_FAIL".as_ptr() as *mut i8,
                mask: 128 | 8192,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"FILTER".as_ptr() as *mut i8,
                z_token_type: c"TK_FILTER".as_ptr() as *mut i8,
                mask: 1048576,
                priority: 4,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"FIRST".as_ptr() as *mut i8,
                z_token_type: c"TK_FIRST".as_ptr() as *mut i8,
                mask: 2,
                priority: 4,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"FOLLOWING".as_ptr() as *mut i8,
                z_token_type: c"TK_FOLLOWING".as_ptr() as *mut i8,
                mask: 1048576,
                priority: 4,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"FOR".as_ptr() as *mut i8,
                z_token_type: c"TK_FOR".as_ptr() as *mut i8,
                mask: 8192,
                priority: 2,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"FOREIGN".as_ptr() as *mut i8,
                z_token_type: c"TK_FOREIGN".as_ptr() as *mut i8,
                mask: 512,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"FROM".as_ptr() as *mut i8,
                z_token_type: c"TK_FROM".as_ptr() as *mut i8,
                mask: 2,
                priority: 10,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"FULL".as_ptr() as *mut i8,
                z_token_type: c"TK_JOIN_KW".as_ptr() as *mut i8,
                mask: 2,
                priority: 3,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"GENERATED".as_ptr() as *mut i8,
                z_token_type: c"TK_GENERATED".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"GLOB".as_ptr() as *mut i8,
                z_token_type: c"TK_LIKE_KW".as_ptr() as *mut i8,
                mask: 2,
                priority: 3,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"GROUP".as_ptr() as *mut i8,
                z_token_type: c"TK_GROUP".as_ptr() as *mut i8,
                mask: 2,
                priority: 5,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"GROUPS".as_ptr() as *mut i8,
                z_token_type: c"TK_GROUPS".as_ptr() as *mut i8,
                mask: 1048576,
                priority: 2,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"HAVING".as_ptr() as *mut i8,
                z_token_type: c"TK_HAVING".as_ptr() as *mut i8,
                mask: 2,
                priority: 5,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"IF".as_ptr() as *mut i8,
                z_token_type: c"TK_IF".as_ptr() as *mut i8,
                mask: 2,
                priority: 2,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"IGNORE".as_ptr() as *mut i8,
                z_token_type: c"TK_IGNORE".as_ptr() as *mut i8,
                mask: 128 | 8192,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"IMMEDIATE".as_ptr() as *mut i8,
                z_token_type: c"TK_IMMEDIATE".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"IN".as_ptr() as *mut i8,
                z_token_type: c"TK_IN".as_ptr() as *mut i8,
                mask: 2,
                priority: 10,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"INDEX".as_ptr() as *mut i8,
                z_token_type: c"TK_INDEX".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"INDEXED".as_ptr() as *mut i8,
                z_token_type: c"TK_INDEXED".as_ptr() as *mut i8,
                mask: 2,
                priority: 0,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"INITIALLY".as_ptr() as *mut i8,
                z_token_type: c"TK_INITIALLY".as_ptr() as *mut i8,
                mask: 512,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"INNER".as_ptr() as *mut i8,
                z_token_type: c"TK_JOIN_KW".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"INSERT".as_ptr() as *mut i8,
                z_token_type: c"TK_INSERT".as_ptr() as *mut i8,
                mask: 2,
                priority: 10,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"INSTEAD".as_ptr() as *mut i8,
                z_token_type: c"TK_INSTEAD".as_ptr() as *mut i8,
                mask: 8192,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"INTERSECT".as_ptr() as *mut i8,
                z_token_type: c"TK_INTERSECT".as_ptr() as *mut i8,
                mask: 64,
                priority: 5,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"INTO".as_ptr() as *mut i8,
                z_token_type: c"TK_INTO".as_ptr() as *mut i8,
                mask: 2,
                priority: 10,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"IS".as_ptr() as *mut i8,
                z_token_type: c"TK_IS".as_ptr() as *mut i8,
                mask: 2,
                priority: 5,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"ISNULL".as_ptr() as *mut i8,
                z_token_type: c"TK_ISNULL".as_ptr() as *mut i8,
                mask: 2,
                priority: 5,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"JOIN".as_ptr() as *mut i8,
                z_token_type: c"TK_JOIN".as_ptr() as *mut i8,
                mask: 2,
                priority: 5,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"KEY".as_ptr() as *mut i8,
                z_token_type: c"TK_KEY".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"LAST".as_ptr() as *mut i8,
                z_token_type: c"TK_LAST".as_ptr() as *mut i8,
                mask: 2,
                priority: 4,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"LEFT".as_ptr() as *mut i8,
                z_token_type: c"TK_JOIN_KW".as_ptr() as *mut i8,
                mask: 2,
                priority: 5,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"LIKE".as_ptr() as *mut i8,
                z_token_type: c"TK_LIKE_KW".as_ptr() as *mut i8,
                mask: 2,
                priority: 5,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"LIMIT".as_ptr() as *mut i8,
                z_token_type: c"TK_LIMIT".as_ptr() as *mut i8,
                mask: 2,
                priority: 3,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"MATCH".as_ptr() as *mut i8,
                z_token_type: c"TK_MATCH".as_ptr() as *mut i8,
                mask: 2,
                priority: 2,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"MATERIALIZED".as_ptr() as *mut i8,
                z_token_type: c"TK_MATERIALIZED".as_ptr() as *mut i8,
                mask: 262144,
                priority: 12,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"NATURAL".as_ptr() as *mut i8,
                z_token_type: c"TK_JOIN_KW".as_ptr() as *mut i8,
                mask: 2,
                priority: 3,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"NO".as_ptr() as *mut i8,
                z_token_type: c"TK_NO".as_ptr() as *mut i8,
                mask: 512 | 1048576,
                priority: 2,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"NOT".as_ptr() as *mut i8,
                z_token_type: c"TK_NOT".as_ptr() as *mut i8,
                mask: 2,
                priority: 10,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"NOTHING".as_ptr() as *mut i8,
                z_token_type: c"TK_NOTHING".as_ptr() as *mut i8,
                mask: 524288,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"NOTNULL".as_ptr() as *mut i8,
                z_token_type: c"TK_NOTNULL".as_ptr() as *mut i8,
                mask: 2,
                priority: 3,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"NULL".as_ptr() as *mut i8,
                z_token_type: c"TK_NULL".as_ptr() as *mut i8,
                mask: 2,
                priority: 10,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"NULLS".as_ptr() as *mut i8,
                z_token_type: c"TK_NULLS".as_ptr() as *mut i8,
                mask: 2,
                priority: 3,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"OF".as_ptr() as *mut i8,
                z_token_type: c"TK_OF".as_ptr() as *mut i8,
                mask: 2,
                priority: 3,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"OFFSET".as_ptr() as *mut i8,
                z_token_type: c"TK_OFFSET".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"ON".as_ptr() as *mut i8,
                z_token_type: c"TK_ON".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"OR".as_ptr() as *mut i8,
                z_token_type: c"TK_OR".as_ptr() as *mut i8,
                mask: 2,
                priority: 9,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"ORDER".as_ptr() as *mut i8,
                z_token_type: c"TK_ORDER".as_ptr() as *mut i8,
                mask: 2,
                priority: 10,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"OTHERS".as_ptr() as *mut i8,
                z_token_type: c"TK_OTHERS".as_ptr() as *mut i8,
                mask: 1048576,
                priority: 3,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"OUTER".as_ptr() as *mut i8,
                z_token_type: c"TK_JOIN_KW".as_ptr() as *mut i8,
                mask: 2,
                priority: 5,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"OVER".as_ptr() as *mut i8,
                z_token_type: c"TK_OVER".as_ptr() as *mut i8,
                mask: 1048576,
                priority: 3,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"PARTITION".as_ptr() as *mut i8,
                z_token_type: c"TK_PARTITION".as_ptr() as *mut i8,
                mask: 1048576,
                priority: 3,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"PLAN".as_ptr() as *mut i8,
                z_token_type: c"TK_PLAN".as_ptr() as *mut i8,
                mask: 256,
                priority: 0,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"PRAGMA".as_ptr() as *mut i8,
                z_token_type: c"TK_PRAGMA".as_ptr() as *mut i8,
                mask: 1024,
                priority: 0,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"PRECEDING".as_ptr() as *mut i8,
                z_token_type: c"TK_PRECEDING".as_ptr() as *mut i8,
                mask: 1048576,
                priority: 3,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"PRIMARY".as_ptr() as *mut i8,
                z_token_type: c"TK_PRIMARY".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"QUERY".as_ptr() as *mut i8,
                z_token_type: c"TK_QUERY".as_ptr() as *mut i8,
                mask: 256,
                priority: 0,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"RAISE".as_ptr() as *mut i8,
                z_token_type: c"TK_RAISE".as_ptr() as *mut i8,
                mask: 8192,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"RANGE".as_ptr() as *mut i8,
                z_token_type: c"TK_RANGE".as_ptr() as *mut i8,
                mask: 1048576,
                priority: 3,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"RECURSIVE".as_ptr() as *mut i8,
                z_token_type: c"TK_RECURSIVE".as_ptr() as *mut i8,
                mask: 262144,
                priority: 3,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"REFERENCES".as_ptr() as *mut i8,
                z_token_type: c"TK_REFERENCES".as_ptr() as *mut i8,
                mask: 512,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"REGEXP".as_ptr() as *mut i8,
                z_token_type: c"TK_LIKE_KW".as_ptr() as *mut i8,
                mask: 2,
                priority: 3,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"REINDEX".as_ptr() as *mut i8,
                z_token_type: c"TK_REINDEX".as_ptr() as *mut i8,
                mask: 2048,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"RELEASE".as_ptr() as *mut i8,
                z_token_type: c"TK_RELEASE".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"RENAME".as_ptr() as *mut i8,
                z_token_type: c"TK_RENAME".as_ptr() as *mut i8,
                mask: 1,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"REPLACE".as_ptr() as *mut i8,
                z_token_type: c"TK_REPLACE".as_ptr() as *mut i8,
                mask: 128,
                priority: 10,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"RESTRICT".as_ptr() as *mut i8,
                z_token_type: c"TK_RESTRICT".as_ptr() as *mut i8,
                mask: 512,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"RETURNING".as_ptr() as *mut i8,
                z_token_type: c"TK_RETURNING".as_ptr() as *mut i8,
                mask: 4194304,
                priority: 10,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"RIGHT".as_ptr() as *mut i8,
                z_token_type: c"TK_JOIN_KW".as_ptr() as *mut i8,
                mask: 2,
                priority: 0,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"ROLLBACK".as_ptr() as *mut i8,
                z_token_type: c"TK_ROLLBACK".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"ROW".as_ptr() as *mut i8,
                z_token_type: c"TK_ROW".as_ptr() as *mut i8,
                mask: 8192,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"ROWS".as_ptr() as *mut i8,
                z_token_type: c"TK_ROWS".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"SAVEPOINT".as_ptr() as *mut i8,
                z_token_type: c"TK_SAVEPOINT".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"SELECT".as_ptr() as *mut i8,
                z_token_type: c"TK_SELECT".as_ptr() as *mut i8,
                mask: 2,
                priority: 10,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"SET".as_ptr() as *mut i8,
                z_token_type: c"TK_SET".as_ptr() as *mut i8,
                mask: 2,
                priority: 10,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"TABLE".as_ptr() as *mut i8,
                z_token_type: c"TK_TABLE".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"TEMP".as_ptr() as *mut i8,
                z_token_type: c"TK_TEMP".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"TEMPORARY".as_ptr() as *mut i8,
                z_token_type: c"TK_TEMP".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"THEN".as_ptr() as *mut i8,
                z_token_type: c"TK_THEN".as_ptr() as *mut i8,
                mask: 2,
                priority: 3,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"TIES".as_ptr() as *mut i8,
                z_token_type: c"TK_TIES".as_ptr() as *mut i8,
                mask: 1048576,
                priority: 3,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"TO".as_ptr() as *mut i8,
                z_token_type: c"TK_TO".as_ptr() as *mut i8,
                mask: 2,
                priority: 3,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"TRANSACTION".as_ptr() as *mut i8,
                z_token_type: c"TK_TRANSACTION".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"TRIGGER".as_ptr() as *mut i8,
                z_token_type: c"TK_TRIGGER".as_ptr() as *mut i8,
                mask: 8192,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"UNBOUNDED".as_ptr() as *mut i8,
                z_token_type: c"TK_UNBOUNDED".as_ptr() as *mut i8,
                mask: 1048576,
                priority: 3,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"UNION".as_ptr() as *mut i8,
                z_token_type: c"TK_UNION".as_ptr() as *mut i8,
                mask: 64,
                priority: 3,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"UNIQUE".as_ptr() as *mut i8,
                z_token_type: c"TK_UNIQUE".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"UPDATE".as_ptr() as *mut i8,
                z_token_type: c"TK_UPDATE".as_ptr() as *mut i8,
                mask: 2,
                priority: 10,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"USING".as_ptr() as *mut i8,
                z_token_type: c"TK_USING".as_ptr() as *mut i8,
                mask: 2,
                priority: 8,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"VACUUM".as_ptr() as *mut i8,
                z_token_type: c"TK_VACUUM".as_ptr() as *mut i8,
                mask: 16384,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"VALUES".as_ptr() as *mut i8,
                z_token_type: c"TK_VALUES".as_ptr() as *mut i8,
                mask: 2,
                priority: 10,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"VIEW".as_ptr() as *mut i8,
                z_token_type: c"TK_VIEW".as_ptr() as *mut i8,
                mask: 32768,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"VIRTUAL".as_ptr() as *mut i8,
                z_token_type: c"TK_VIRTUAL".as_ptr() as *mut i8,
                mask: 65536,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"WHEN".as_ptr() as *mut i8,
                z_token_type: c"TK_WHEN".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"WHERE".as_ptr() as *mut i8,
                z_token_type: c"TK_WHERE".as_ptr() as *mut i8,
                mask: 2,
                priority: 10,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"WINDOW".as_ptr() as *mut i8,
                z_token_type: c"TK_WINDOW".as_ptr() as *mut i8,
                mask: 1048576,
                priority: 3,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"WITH".as_ptr() as *mut i8,
                z_token_type: c"TK_WITH".as_ptr() as *mut i8,
                mask: 262144,
                priority: 4,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"WITHIN".as_ptr() as *mut i8,
                z_token_type: c"TK_WITHIN".as_ptr() as *mut i8,
                mask: 0,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            },
            Keyword {
                z_name: c"WITHOUT".as_ptr() as *mut i8,
                z_token_type: c"TK_WITHOUT".as_ptr() as *mut i8,
                mask: 2,
                priority: 1,
                id: 0,
                hash: 0,
                offset: 0,
                len: 0,
                prefix: 0,
                longest_suffix: 0,
                i_next: 0,
                substr_id: 0,
                substr_offset: 0,
                z_orig_name: unsafe { core::mem::zeroed() },
            }];

static mut n_keyword: i32 =
    (core::mem::size_of::<[Keyword; 148]>() as u64 /
            core::mem::size_of::<Keyword>() as u64) as i32;

extern "C" fn keyword_compare1(a: *const (), b: *const ()) -> i32 {
    let p_a: *const Keyword = a as *mut Keyword as *const Keyword;
    let p_b: *const Keyword = b as *mut Keyword as *const Keyword;
    let mut n: i32 = unsafe { (*p_a).len } - unsafe { (*p_b).len } as i32;
    if n == 0 {
        n =
            unsafe {
                strcmp(unsafe { (*p_a).z_name } as *const i8,
                    unsafe { (*p_b).z_name } as *const i8)
            };
    }
    if !(n != 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"keywordCompare1".as_ptr() as *const i8,
                c"mkkeywordhash.c".as_ptr() as *mut i8 as *const i8, 347,
                c"n!=0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    return n;
}

extern "C" fn keyword_compare2(a: *const (), b: *const ()) -> i32 {
    let p_a: *const Keyword = a as *mut Keyword as *const Keyword;
    let p_b: *const Keyword = b as *mut Keyword as *const Keyword;
    let mut n: i32 =
        unsafe { (*p_b).longest_suffix } -
            unsafe { (*p_a).longest_suffix } as i32;
    if n == 0 {
        n =
            unsafe {
                strcmp(unsafe { (*p_a).z_name } as *const i8,
                    unsafe { (*p_b).z_name } as *const i8)
            };
    }
    if !(n != 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"keywordCompare2".as_ptr() as *const i8,
                c"mkkeywordhash.c".as_ptr() as *mut i8 as *const i8, 357,
                c"n!=0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    return n;
}

extern "C" fn keyword_compare3(a: *const (), b: *const ()) -> i32 {
    let p_a: *const Keyword = a as *mut Keyword as *const Keyword;
    let p_b: *const Keyword = b as *mut Keyword as *const Keyword;
    let mut n: i32 =
        unsafe { (*p_a).offset } - unsafe { (*p_b).offset } as i32;
    if n == 0 { n = unsafe { (*p_b).id } - unsafe { (*p_a).id } as i32; }
    if !(n != 0) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"keywordCompare3".as_ptr() as *const i8,
                c"mkkeywordhash.c".as_ptr() as *mut i8 as *const i8, 365,
                c"n!=0".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    return n;
}

extern "C" fn find_by_id(id: i32) -> *mut Keyword {
    unsafe {
        let mut i: i32 = 0;
        {
            i = 0;
            '__b0: loop {
                if !(i < n_keyword) { break '__b0; }
                '__c0: loop {
                    if a_keyword_table[i as usize].id == id { break '__b0; }
                    break '__c0;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        return &mut a_keyword_table[i as usize];
    }
}

extern "C" fn reorder(p_from_1: &mut i32) -> () {
    unsafe {
        let i: i32 = *p_from_1 - 1;
        let mut j: i32 = 0;
        if i < 0 { return; }
        j = a_keyword_table[i as usize].i_next;
        if j == 0 { return; }
        { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
        if a_keyword_table[i as usize].priority >=
                a_keyword_table[j as usize].priority {
            return;
        }
        a_keyword_table[i as usize].i_next =
            a_keyword_table[j as usize].i_next;
        a_keyword_table[j as usize].i_next = i + 1;
        *p_from_1 = j + 1;
        reorder(&mut a_keyword_table[i as usize].i_next);
    }
}

extern "C" fn __main_inner(argc: i32, argv: *const *mut i8)
    -> Result<(), i32> {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        let mut h: i32 = 0;
        let mut best_size: i32 = 0;
        let mut best_count: i32 = 0;
        let mut count: i32 = 0;
        let mut n_char: i32 = 0;
        let mut total_len: i32 = 0;
        let mut a_kw_hash: [i32; 1000] = [0; 1000];
        let mut z_kw_text: [i8; 2000] = [0; 2000];
        {
            i = { j = 0; j };
            '__b1: loop {
                if !(i < n_keyword) { break '__b1; }
                '__c1: loop {
                    if a_keyword_table[i as usize].mask == 0 { break '__c1; }
                    if j < i {
                        a_keyword_table[j as usize] = a_keyword_table[i as usize];
                    }
                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                    break '__c1;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        n_keyword = j;
        {
            i = 0;
            '__b2: loop {
                if !(i < n_keyword) { break '__b2; }
                '__c2: loop {
                    let mut p: *mut Keyword = &mut a_keyword_table[i as usize];
                    unsafe {
                        (*p).len =
                            unsafe { strlen(unsafe { (*p).z_name } as *const i8) } as
                                i32
                    };
                    if !((unsafe { (*p).len } as u64) <
                                            core::mem::size_of::<[i8; 20]>() as u64) as i32 as i64 != 0
                        {
                        unsafe {
                            __assert_rtn(c"main".as_ptr() as *const i8,
                                c"mkkeywordhash.c".as_ptr() as *mut i8 as *const i8, 433,
                                c"p->len<sizeof(p->zOrigName)".as_ptr() as *mut i8 as
                                    *const i8)
                        }
                    } else { { let _ = 0; } };
                    unsafe {
                        memcpy(unsafe { &raw mut (*p).z_orig_name[0 as usize] } as
                                    *mut i8 as *mut (), unsafe { (*p).z_name } as *const (),
                            (unsafe { (*p).len } + 1) as u64)
                    };
                    total_len += unsafe { (*p).len };
                    unsafe {
                        (*p).hash =
                            (32 |
                                            unsafe { *unsafe { (*p).z_name.offset(0 as isize) } } as
                                                i32) * 4 ^
                                    (32 |
                                            unsafe {
                                                    *unsafe {
                                                            (*p).z_name.offset((unsafe { (*p).len } - 1) as isize)
                                                        }
                                                } as i32) * 3 ^ unsafe { (*p).len } * 1
                    };
                    unsafe { (*p).id = i + 1 };
                    break '__c2;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            qsort(&raw mut a_keyword_table[0 as usize] as *mut Keyword as
                    *mut (), n_keyword as u64,
                core::mem::size_of::<Keyword>() as u64, keyword_compare1)
        };
        {
            i = n_keyword - 2;
            '__b3: loop {
                if !(i >= 0) { break '__b3; }
                '__c3: loop {
                    let mut p: *mut Keyword = &mut a_keyword_table[i as usize];
                    {
                        j = n_keyword - 1;
                        '__b4: loop {
                            if !(j > i && unsafe { (*p).substr_id } == 0) {
                                break '__b4;
                            }
                            '__c4: loop {
                                let p_other: *const Keyword =
                                    &raw mut a_keyword_table[j as usize] as *const Keyword;
                                if unsafe { (*p_other).substr_id } != 0 { break '__c4; }
                                if unsafe { (*p_other).len } <= unsafe { (*p).len } {
                                    break '__c4;
                                }
                                {
                                    k = 0;
                                    '__b5: loop {
                                        if !(k <= unsafe { (*p_other).len } - unsafe { (*p).len }) {
                                            break '__b5;
                                        }
                                        '__c5: loop {
                                            if unsafe {
                                                        memcmp(unsafe { (*p).z_name } as *const (),
                                                            unsafe {
                                                                    &raw mut *unsafe { (*p_other).z_name.offset(k as isize) }
                                                                } as *const (), unsafe { (*p).len } as u64)
                                                    } == 0 {
                                                unsafe { (*p).substr_id = unsafe { (*p_other).id } };
                                                unsafe { (*p).substr_offset = k };
                                                break '__b5;
                                            }
                                            break '__c5;
                                        }
                                        { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                                break '__c4;
                            }
                            { let __p = &mut j; let __t = *__p; *__p -= 1; __t };
                        }
                    }
                    break '__c3;
                }
                { let __p = &mut i; let __t = *__p; *__p -= 1; __t };
            }
        }
        {
            i = 0;
            '__b6: loop {
                if !(i < n_keyword) { break '__b6; }
                '__c6: loop {
                    let mut p: *mut Keyword = &mut a_keyword_table[i as usize];
                    if unsafe { (*p).substr_id } != 0 { break '__c6; }
                    {
                        j = 0;
                        '__b7: loop {
                            if !(j < n_keyword) { break '__b7; }
                            '__c7: loop {
                                let mut p_other_1: *const Keyword = core::ptr::null();
                                if j == i { break '__c7; }
                                p_other_1 = &mut a_keyword_table[j as usize];
                                if unsafe { (*p_other_1).substr_id } != 0 { break '__c7; }
                                {
                                    k = unsafe { (*p).longest_suffix } + 1;
                                    '__b8: loop {
                                        if !(k < unsafe { (*p).len } &&
                                                        k < unsafe { (*p_other_1).len }) {
                                            break '__b8;
                                        }
                                        '__c8: loop {
                                            if unsafe {
                                                        memcmp(unsafe {
                                                                    &raw mut *unsafe {
                                                                                (*p).z_name.offset((unsafe { (*p).len } - k) as isize)
                                                                            }
                                                                } as *const (), unsafe { (*p_other_1).z_name } as *const (),
                                                            k as u64)
                                                    } == 0 {
                                                unsafe { (*p).longest_suffix = k };
                                            }
                                            break '__c8;
                                        }
                                        { let __p = &mut k; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                                break '__c7;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    break '__c6;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            qsort(&raw mut a_keyword_table[0 as usize] as *mut Keyword as
                    *mut (), n_keyword as u64,
                core::mem::size_of::<Keyword>() as u64, keyword_compare2)
        };
        n_char = 0;
        {
            i = 0;
            '__b9: loop {
                if !(i < n_keyword) { break '__b9; }
                '__c9: loop {
                    let mut p: *mut Keyword = &mut a_keyword_table[i as usize];
                    if unsafe { (*p).offset } > 0 ||
                            unsafe { (*p).substr_id } != 0 {
                        break '__c9;
                    }
                    unsafe { (*p).offset = n_char };
                    n_char += unsafe { (*p).len };
                    {
                        k = unsafe { (*p).len } - 1;
                        '__b10: loop {
                            if !(k >= 1) { break '__b10; }
                            '__c10: loop {
                                {
                                    j = i + 1;
                                    '__b11: loop {
                                        if !(j < n_keyword) { break '__b11; }
                                        '__c11: loop {
                                            let p_other_2: *mut Keyword =
                                                &mut a_keyword_table[j as usize];
                                            if unsafe { (*p_other_2).offset } > 0 ||
                                                    unsafe { (*p_other_2).substr_id } != 0 {
                                                break '__c11;
                                            }
                                            if unsafe { (*p_other_2).len } <= k { break '__c11; }
                                            if unsafe {
                                                        memcmp(unsafe {
                                                                    &raw mut *unsafe {
                                                                                (*p).z_name.offset((unsafe { (*p).len } - k) as isize)
                                                                            }
                                                                } as *const (), unsafe { (*p_other_2).z_name } as *const (),
                                                            k as u64)
                                                    } == 0 {
                                                p = p_other_2;
                                                unsafe { (*p).offset = n_char - k };
                                                n_char = unsafe { (*p).offset } + unsafe { (*p).len };
                                                unsafe {
                                                    {
                                                        let __n = k;
                                                        let __p = &mut (*p).z_name;
                                                        *__p = unsafe { (*__p).offset(__n as isize) };
                                                    }
                                                };
                                                unsafe { (*p).len -= k };
                                                unsafe { (*p).prefix = k };
                                                j = i;
                                                k = unsafe { (*p).len };
                                            }
                                            break '__c11;
                                        }
                                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                                break '__c10;
                            }
                            { let __p = &mut k; let __t = *__p; *__p -= 1; __t };
                        }
                    }
                    break '__c9;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        {
            i = 0;
            '__b12: loop {
                if !(i < n_keyword) { break '__b12; }
                '__c12: loop {
                    let mut p: *mut Keyword = &mut a_keyword_table[i as usize];
                    if unsafe { (*p).substr_id } != 0 {
                        unsafe {
                            (*p).offset =
                                unsafe { (*find_by_id(unsafe { (*p).substr_id })).offset } +
                                    unsafe { (*p).substr_offset }
                        };
                    }
                    break '__c12;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            qsort(&raw mut a_keyword_table[0 as usize] as *mut Keyword as
                    *mut (), n_keyword as u64,
                core::mem::size_of::<Keyword>() as u64, keyword_compare3)
        };
        best_size = n_keyword;
        best_count = n_keyword * n_keyword;
        {
            i = n_keyword / 2;
            '__b13: loop {
                if !(i <= 2 * n_keyword) { break '__b13; }
                '__c13: loop {
                    if i <= 0 { break '__c13; }
                    {
                        j = 0;
                        '__b14: loop {
                            if !(j < i) { break '__b14; }
                            '__c14: loop { a_kw_hash[j as usize] = 0; break '__c14; }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    {
                        j = 0;
                        '__b15: loop {
                            if !(j < n_keyword) { break '__b15; }
                            '__c15: loop {
                                h = a_keyword_table[j as usize].hash % i;
                                a_kw_hash[h as usize] *= 2;
                                {
                                    let __p = &mut a_kw_hash[h as usize];
                                    let __t = *__p;
                                    *__p += 1;
                                    __t
                                };
                                break '__c15;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    {
                        j = { count = 0; count };
                        '__b16: loop {
                            if !(j < i) { break '__b16; }
                            '__c16: loop {
                                count += a_kw_hash[j as usize];
                                break '__c16;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if count < best_count { best_count = count; best_size = i; }
                    break '__c13;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        {
            i = 0;
            '__b17: loop {
                if !(i < best_size) { break '__b17; }
                '__c17: loop { a_kw_hash[i as usize] = 0; break '__c17; }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        {
            i = 0;
            '__b18: loop {
                if !(i < n_keyword) { break '__b18; }
                '__c18: loop {
                    h = a_keyword_table[i as usize].hash % best_size;
                    a_keyword_table[i as usize].i_next = a_kw_hash[h as usize];
                    a_kw_hash[h as usize] = i + 1;
                    reorder(&mut a_kw_hash[h as usize]);
                    break '__c18;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            printf(c"%s".as_ptr() as *mut i8 as *const i8,
                &raw const z_hdr[0 as usize] as *const i8)
        };
        unsafe {
            printf(c"/* Hash score: %d */\n".as_ptr() as *mut i8 as *const i8,
                best_count)
        };
        unsafe {
            printf(c"/* zKWText[] encodes %d bytes of keyword text in %d bytes */\n".as_ptr()
                        as *mut i8 as *const i8, total_len + n_keyword, n_char + 1)
        };
        {
            i = { j = { k = 0; k }; j };
            '__b19: loop {
                if !(i < n_keyword) { break '__b19; }
                '__c19: loop {
                    let mut p: *const Keyword =
                        &raw mut a_keyword_table[i as usize] as *const Keyword;
                    if unsafe { (*p).substr_id } != 0 { break '__c19; }
                    unsafe {
                        memcpy(&raw mut z_kw_text[k as usize] as *mut (),
                            unsafe { (*p).z_name } as *const (),
                            unsafe { (*p).len } as u64)
                    };
                    k += unsafe { (*p).len };
                    if j + unsafe { (*p).len } > 70 {
                        unsafe {
                            printf(c"%*s */\n".as_ptr() as *mut i8 as *const i8, 74 - j,
                                c"".as_ptr() as *mut i8)
                        };
                        j = 0;
                    }
                    if j == 0 {
                        unsafe {
                            printf(c"/*   ".as_ptr() as *mut i8 as *const i8)
                        };
                        j = 8;
                    }
                    unsafe {
                        printf(c"%s".as_ptr() as *mut i8 as *const i8,
                            unsafe { (*p).z_name })
                    };
                    j += unsafe { (*p).len };
                    break '__c19;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if j > 0 {
            unsafe {
                printf(c"%*s */\n".as_ptr() as *mut i8 as *const i8, 74 - j,
                    c"".as_ptr() as *mut i8)
            };
        }
        unsafe {
            printf(c"static const char zKWText[%d] = {\n".as_ptr() as *mut i8
                    as *const i8, n_char)
        };
        z_kw_text[n_char as usize] = 0 as i8;
        {
            i = { j = 0; j };
            '__b20: loop {
                if !(i < k) { break '__b20; }
                '__c20: loop {
                    if j == 0 {
                        unsafe { printf(c"  ".as_ptr() as *mut i8 as *const i8) };
                    }
                    if z_kw_text[i as usize] as i32 == 0 {
                        unsafe { printf(c"0".as_ptr() as *mut i8 as *const i8) };
                    } else {
                        unsafe {
                            printf(c"\'%c\',".as_ptr() as *mut i8 as *const i8,
                                z_kw_text[i as usize] as i32)
                        };
                    }
                    j += 4;
                    if j > 68 {
                        unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
                        j = 0;
                    }
                    break '__c20;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if j > 0 {
            unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
        }
        unsafe { printf(c"};\n".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            printf(c"/* aKWHash[i] is the hash value for the i-th keyword */\n".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            printf(c"static const unsigned char aKWHash[%d] = {\n".as_ptr() as
                        *mut i8 as *const i8, best_size)
        };
        {
            i = { j = 0; j };
            '__b21: loop {
                if !(i < best_size) { break '__b21; }
                '__c21: loop {
                    if j == 0 {
                        unsafe { printf(c"  ".as_ptr() as *mut i8 as *const i8) };
                    }
                    unsafe {
                        printf(c" %3d,".as_ptr() as *mut i8 as *const i8,
                            a_kw_hash[i as usize])
                    };
                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                    if j > 12 {
                        unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
                        j = 0;
                    }
                    break '__c21;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            printf(c"%s};\n".as_ptr() as *mut i8 as *const i8,
                if j == 0 {
                    c"".as_ptr() as *mut i8
                } else { c"\n".as_ptr() as *mut i8 })
        };
        unsafe {
            printf(c"/* aKWNext[] forms the hash collision chain.  If aKWHash[i]==0\n".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            printf(c"** then the i-th keyword has no more hash collisions.  Otherwise,\n".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            printf(c"** the next keyword with the same hash is aKWHash[i]-1. */\n".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            printf(c"static const unsigned char aKWNext[%d] = {0,\n".as_ptr()
                        as *mut i8 as *const i8, n_keyword + 1)
        };
        {
            i = { j = 0; j };
            '__b22: loop {
                if !(i < n_keyword) { break '__b22; }
                '__c22: loop {
                    if j == 0 {
                        unsafe { printf(c"  ".as_ptr() as *mut i8 as *const i8) };
                    }
                    unsafe {
                        printf(c" %3d,".as_ptr() as *mut i8 as *const i8,
                            a_keyword_table[i as usize].i_next)
                    };
                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                    if j > 12 {
                        unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
                        j = 0;
                    }
                    break '__c22;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            printf(c"%s};\n".as_ptr() as *mut i8 as *const i8,
                if j == 0 {
                    c"".as_ptr() as *mut i8
                } else { c"\n".as_ptr() as *mut i8 })
        };
        unsafe {
            printf(c"/* aKWLen[i] is the length (in bytes) of the i-th keyword */\n".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            printf(c"static const unsigned char aKWLen[%d] = {0,\n".as_ptr()
                        as *mut i8 as *const i8, n_keyword + 1)
        };
        {
            i = { j = 0; j };
            '__b23: loop {
                if !(i < n_keyword) { break '__b23; }
                '__c23: loop {
                    if j == 0 {
                        unsafe { printf(c"  ".as_ptr() as *mut i8 as *const i8) };
                    }
                    unsafe {
                        printf(c" %3d,".as_ptr() as *mut i8 as *const i8,
                            a_keyword_table[i as usize].len +
                                a_keyword_table[i as usize].prefix)
                    };
                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                    if j > 12 {
                        unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
                        j = 0;
                    }
                    break '__c23;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            printf(c"%s};\n".as_ptr() as *mut i8 as *const i8,
                if j == 0 {
                    c"".as_ptr() as *mut i8
                } else { c"\n".as_ptr() as *mut i8 })
        };
        unsafe {
            printf(c"/* aKWOffset[i] is the index into zKWText[] of the start of\n".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            printf(c"** the text for the i-th keyword. */\n".as_ptr() as
                        *mut i8 as *const i8)
        };
        unsafe {
            printf(c"static const unsigned short int aKWOffset[%d] = {0,\n".as_ptr()
                        as *mut i8 as *const i8, n_keyword + 1)
        };
        {
            i = { j = 0; j };
            '__b24: loop {
                if !(i < n_keyword) { break '__b24; }
                '__c24: loop {
                    if j == 0 {
                        unsafe { printf(c"  ".as_ptr() as *mut i8 as *const i8) };
                    }
                    unsafe {
                        printf(c" %3d,".as_ptr() as *mut i8 as *const i8,
                            a_keyword_table[i as usize].offset)
                    };
                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                    if j > 12 {
                        unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
                        j = 0;
                    }
                    break '__c24;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            printf(c"%s};\n".as_ptr() as *mut i8 as *const i8,
                if j == 0 {
                    c"".as_ptr() as *mut i8
                } else { c"\n".as_ptr() as *mut i8 })
        };
        unsafe {
            printf(c"/* aKWCode[i] is the parser symbol code for the i-th keyword */\n".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            printf(c"static const unsigned char aKWCode[%d] = {0,\n".as_ptr()
                        as *mut i8 as *const i8, n_keyword + 1)
        };
        {
            i = { j = 0; j };
            '__b25: loop {
                if !(i < n_keyword) { break '__b25; }
                '__c25: loop {
                    let z_token: *mut i8 =
                        a_keyword_table[i as usize].z_token_type;
                    if j == 0 {
                        unsafe { printf(c"  ".as_ptr() as *mut i8 as *const i8) };
                    }
                    unsafe {
                        printf(c"%s,%*s".as_ptr() as *mut i8 as *const i8, z_token,
                            (14 as u64 - unsafe { strlen(z_token as *const i8) }) as
                                i32, c"".as_ptr() as *mut i8)
                    };
                    { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                    if j >= 5 {
                        unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
                        j = 0;
                    }
                    break '__c25;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            printf(c"%s};\n".as_ptr() as *mut i8 as *const i8,
                if j == 0 {
                    c"".as_ptr() as *mut i8
                } else { c"\n".as_ptr() as *mut i8 })
        };
        unsafe {
            printf(c"/* Hash table decoded:\n".as_ptr() as *mut i8 as
                    *const i8)
        };
        {
            i = 0;
            '__b26: loop {
                if !(i < best_size) { break '__b26; }
                '__c26: loop {
                    j = a_kw_hash[i as usize];
                    unsafe {
                        printf(c"** %3d:".as_ptr() as *mut i8 as *const i8, i)
                    };
                    while j != 0 {
                        unsafe {
                            printf(c" %s".as_ptr() as *mut i8 as *const i8,
                                &raw mut a_keyword_table[(j - 1) as
                                                        usize].z_orig_name[0 as usize] as *mut i8)
                        };
                        j = a_keyword_table[(j - 1) as usize].i_next;
                    }
                    unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
                    break '__c26;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe { printf(c"*/\n".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            printf(c"/* Check to see if z[0..n-1] is a keyword. If it is, write the\n".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            printf(c"** parser symbol code for that keyword into *pType.  Always\n".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            printf(c"** return the integer n (the length of the token). */\n".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            printf(c"static i64 keywordCode(const char *z, i64 n, int *pType){\n".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe { printf(c"  i64 i, j;\n".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            printf(c"  const char *zKW;\n".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            printf(c"  assert( n>=2 );\n".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            printf(c"  i = ((charMap(z[0])*%d) %c".as_ptr() as *mut i8 as
                    *const i8, 4, '^' as i32)
        };
        unsafe {
            printf(c" (charMap(z[n-1])*%d) %c".as_ptr() as *mut i8 as
                    *const i8, 3, '^' as i32)
        };
        unsafe {
            printf(c" n*%d) %% %d;\n".as_ptr() as *mut i8 as *const i8, 1,
                best_size)
        };
        unsafe {
            printf(c"  for(i=(int)aKWHash[i]; i>0; i=aKWNext[i]){\n".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            printf(c"    if( aKWLen[i]!=n ) continue;\n".as_ptr() as *mut i8
                    as *const i8)
        };
        unsafe {
            printf(c"    zKW = &zKWText[aKWOffset[i]];\n".as_ptr() as *mut i8
                    as *const i8)
        };
        unsafe {
            printf(c"#ifdef SQLITE_ASCII\n".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            printf(c"    if( (z[0]&~0x20)!=zKW[0] ) continue;\n".as_ptr() as
                        *mut i8 as *const i8)
        };
        unsafe {
            printf(c"    if( (z[1]&~0x20)!=zKW[1] ) continue;\n".as_ptr() as
                        *mut i8 as *const i8)
        };
        unsafe { printf(c"    j = 2;\n".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            printf(c"    while( j<n && (z[j]&~0x20)==zKW[j] ){ j++; }\n".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe { printf(c"#endif\n".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            printf(c"#ifdef SQLITE_EBCDIC\n".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            printf(c"    if( toupper(z[0])!=zKW[0] ) continue;\n".as_ptr() as
                        *mut i8 as *const i8)
        };
        unsafe {
            printf(c"    if( toupper(z[1])!=zKW[1] ) continue;\n".as_ptr() as
                        *mut i8 as *const i8)
        };
        unsafe { printf(c"    j = 2;\n".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            printf(c"    while( j<n && toupper(z[j])==zKW[j] ){ j++; }\n".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe { printf(c"#endif\n".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            printf(c"    if( j<n ) continue;\n".as_ptr() as *mut i8 as
                    *const i8)
        };
        {
            i = 0;
            '__b28: loop {
                if !(i < n_keyword) { break '__b28; }
                '__c28: loop {
                    unsafe {
                        printf(c"    testcase( i==%d ); /* %s */\n".as_ptr() as
                                    *mut i8 as *const i8, i + 1,
                            &raw mut a_keyword_table[i as usize].z_orig_name[0 as usize]
                                as *mut i8)
                    };
                    break '__c28;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            printf(c"    *pType = aKWCode[i];\n".as_ptr() as *mut i8 as
                    *const i8)
        };
        unsafe { printf(c"    break;\n".as_ptr() as *mut i8 as *const i8) };
        unsafe { printf(c"  }\n".as_ptr() as *mut i8 as *const i8) };
        unsafe { printf(c"  return n;\n".as_ptr() as *mut i8 as *const i8) };
        unsafe { printf(c"}\n".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            printf(c"int sqlite3KeywordCode(const unsigned char *z, int n){\n".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            printf(c"  int id = TK_ID;\n".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            printf(c"  if( n>=2 ) keywordCode((char*)z, n, &id);\n".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe { printf(c"  return id;\n".as_ptr() as *mut i8 as *const i8) };
        unsafe { printf(c"}\n".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            printf(c"#define SQLITE_N_KEYWORD %d\n".as_ptr() as *mut i8 as
                    *const i8, n_keyword)
        };
        unsafe {
            printf(c"int sqlite3_keyword_name(int i,const char **pzName,int *pnName){\n".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            printf(c"  if( i<0 || i>=SQLITE_N_KEYWORD ) return SQLITE_ERROR;\n".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe { printf(c"  i++;\n".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            printf(c"  *pzName = zKWText + aKWOffset[i];\n".as_ptr() as
                        *mut i8 as *const i8)
        };
        unsafe {
            printf(c"  *pnName = aKWLen[i];\n".as_ptr() as *mut i8 as
                    *const i8)
        };
        unsafe {
            printf(c"  return SQLITE_OK;\n".as_ptr() as *mut i8 as *const i8)
        };
        unsafe { printf(c"}\n".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            printf(c"int sqlite3_keyword_count(void){ return SQLITE_N_KEYWORD; }\n".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            printf(c"int sqlite3_keyword_check(const char *zName, int nName){\n".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            printf(c"  return TK_ID!=sqlite3KeywordCode((const u8*)zName, nName);\n".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe { printf(c"}\n".as_ptr() as *mut i8 as *const i8) };
        return Ok(());
    }
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
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn strlen(__s: *const i8)
    -> u64;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn qsort(__base: *mut (), __nel: u64, __width: u64,
    __compar: unsafe extern "C" fn(*const (), *const ()) -> i32)
    -> ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64)
    -> i32;
    fn printf(_: *const i8, ...)
    -> i32;
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}
