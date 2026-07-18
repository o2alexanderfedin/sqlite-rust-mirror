#![allow(unused_imports, dead_code)]

mod sqlite3_h;
pub(crate) use crate::sqlite3_h::*;

type DarwinSizeT = u64;

static z_help: [i8; 340] =
    [85 as i8, 115 as i8, 97 as i8, 103 as i8, 101 as i8, 58 as i8, 32 as i8,
            37 as i8, 115 as i8, 32 as i8, 67 as i8, 79 as i8, 77 as i8,
            77 as i8, 65 as i8, 78 as i8, 68 as i8, 10 as i8, 67 as i8,
            111 as i8, 109 as i8, 109 as i8, 97 as i8, 110 as i8, 100 as i8,
            115 as i8, 58 as i8, 10 as i8, 32 as i8, 32 as i8, 105 as i8,
            110 as i8, 105 as i8, 116 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 73 as i8, 110 as i8, 105 as i8, 116 as i8, 105 as i8,
            97 as i8, 108 as i8, 105 as i8, 122 as i8, 101 as i8, 100 as i8,
            32 as i8, 116 as i8, 104 as i8, 101 as i8, 32 as i8, 115 as i8,
            116 as i8, 97 as i8, 114 as i8, 116 as i8, 117 as i8, 112 as i8,
            46 as i8, 100 as i8, 98 as i8, 32 as i8, 100 as i8, 97 as i8,
            116 as i8, 97 as i8, 98 as i8, 97 as i8, 115 as i8, 101 as i8,
            32 as i8, 102 as i8, 105 as i8, 108 as i8, 101 as i8, 10 as i8,
            32 as i8, 32 as i8, 114 as i8, 117 as i8, 110 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 82 as i8, 117 as i8,
            110 as i8, 32 as i8, 116 as i8, 104 as i8, 101 as i8, 32 as i8,
            115 as i8, 116 as i8, 97 as i8, 114 as i8, 116 as i8, 117 as i8,
            112 as i8, 32 as i8, 112 as i8, 101 as i8, 114 as i8, 102 as i8,
            111 as i8, 114 as i8, 109 as i8, 97 as i8, 110 as i8, 99 as i8,
            101 as i8, 32 as i8, 116 as i8, 101 as i8, 115 as i8, 116 as i8,
            10 as i8, 79 as i8, 112 as i8, 116 as i8, 105 as i8, 111 as i8,
            110 as i8, 115 as i8, 58 as i8, 10 as i8, 32 as i8, 32 as i8,
            45 as i8, 45 as i8, 100 as i8, 98 as i8, 110 as i8, 97 as i8,
            109 as i8, 101 as i8, 32 as i8, 78 as i8, 65 as i8, 77 as i8,
            69 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 83 as i8, 101 as i8, 116 as i8, 32 as i8,
            116 as i8, 104 as i8, 101 as i8, 32 as i8, 110 as i8, 97 as i8,
            109 as i8, 101 as i8, 32 as i8, 111 as i8, 102 as i8, 32 as i8,
            116 as i8, 104 as i8, 101 as i8, 32 as i8, 116 as i8, 101 as i8,
            115 as i8, 116 as i8, 32 as i8, 100 as i8, 97 as i8, 116 as i8,
            97 as i8, 98 as i8, 97 as i8, 115 as i8, 101 as i8, 32 as i8,
            102 as i8, 105 as i8, 108 as i8, 101 as i8, 10 as i8, 32 as i8,
            32 as i8, 45 as i8, 45 as i8, 104 as i8, 101 as i8, 97 as i8,
            112 as i8, 32 as i8, 83 as i8, 90 as i8, 32 as i8, 77 as i8,
            73 as i8, 78 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 77 as i8, 101 as i8, 109 as i8,
            111 as i8, 114 as i8, 121 as i8, 32 as i8, 97 as i8, 108 as i8,
            108 as i8, 111 as i8, 99 as i8, 97 as i8, 116 as i8, 111 as i8,
            114 as i8, 32 as i8, 117 as i8, 115 as i8, 101 as i8, 115 as i8,
            32 as i8, 83 as i8, 90 as i8, 32 as i8, 98 as i8, 121 as i8,
            116 as i8, 101 as i8, 115 as i8, 32 as i8, 38 as i8, 32 as i8,
            109 as i8, 105 as i8, 110 as i8, 32 as i8, 97 as i8, 108 as i8,
            108 as i8, 111 as i8, 99 as i8, 97 as i8, 116 as i8, 105 as i8,
            111 as i8, 110 as i8, 32 as i8, 77 as i8, 73 as i8, 78 as i8,
            10 as i8, 32 as i8, 32 as i8, 45 as i8, 45 as i8, 115 as i8,
            116 as i8, 97 as i8, 116 as i8, 115 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 83 as i8,
            104 as i8, 111 as i8, 119 as i8, 32 as i8, 115 as i8, 116 as i8,
            97 as i8, 116 as i8, 105 as i8, 115 as i8, 116 as i8, 105 as i8,
            99 as i8, 115 as i8, 32 as i8, 97 as i8, 116 as i8, 32 as i8,
            116 as i8, 104 as i8, 101 as i8, 32 as i8, 101 as i8, 110 as i8,
            100 as i8, 10 as i8, 0 as i8];

extern "C" fn usage(argv0: *const i8) -> () {
    unsafe { printf(&raw const z_help[0 as usize] as *const i8, argv0) };
    unsafe { exit(1) };
}

static z_test_schema: [i8; 7703] =
    [67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8,
            84 as i8, 65 as i8, 66 as i8, 76 as i8, 69 as i8, 32 as i8,
            114 as i8, 101 as i8, 112 as i8, 111 as i8, 95 as i8, 98 as i8,
            108 as i8, 111 as i8, 98 as i8, 40 as i8, 10 as i8, 32 as i8,
            32 as i8, 114 as i8, 105 as i8, 100 as i8, 32 as i8, 73 as i8,
            78 as i8, 84 as i8, 69 as i8, 71 as i8, 69 as i8, 82 as i8,
            32 as i8, 80 as i8, 82 as i8, 73 as i8, 77 as i8, 65 as i8,
            82 as i8, 89 as i8, 32 as i8, 75 as i8, 69 as i8, 89 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 114 as i8, 99 as i8,
            118 as i8, 105 as i8, 100 as i8, 32 as i8, 73 as i8, 78 as i8,
            84 as i8, 69 as i8, 71 as i8, 69 as i8, 82 as i8, 44 as i8,
            10 as i8, 32 as i8, 32 as i8, 115 as i8, 105 as i8, 122 as i8,
            101 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8, 69 as i8,
            71 as i8, 69 as i8, 82 as i8, 44 as i8, 10 as i8, 32 as i8,
            32 as i8, 117 as i8, 117 as i8, 105 as i8, 100 as i8, 32 as i8,
            84 as i8, 69 as i8, 88 as i8, 84 as i8, 32 as i8, 85 as i8,
            78 as i8, 73 as i8, 81 as i8, 85 as i8, 69 as i8, 32 as i8,
            78 as i8, 79 as i8, 84 as i8, 32 as i8, 78 as i8, 85 as i8,
            76 as i8, 76 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8,
            99 as i8, 111 as i8, 110 as i8, 116 as i8, 101 as i8, 110 as i8,
            116 as i8, 32 as i8, 66 as i8, 76 as i8, 79 as i8, 66 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 67 as i8, 72 as i8,
            69 as i8, 67 as i8, 75 as i8, 40 as i8, 32 as i8, 108 as i8,
            101 as i8, 110 as i8, 103 as i8, 116 as i8, 104 as i8, 40 as i8,
            117 as i8, 117 as i8, 105 as i8, 100 as i8, 41 as i8, 62 as i8,
            61 as i8, 52 as i8, 48 as i8, 32 as i8, 65 as i8, 78 as i8,
            68 as i8, 32 as i8, 114 as i8, 105 as i8, 100 as i8, 62 as i8,
            48 as i8, 32 as i8, 41 as i8, 10 as i8, 41 as i8, 59 as i8,
            10 as i8, 67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8,
            69 as i8, 32 as i8, 84 as i8, 65 as i8, 66 as i8, 76 as i8,
            69 as i8, 32 as i8, 114 as i8, 101 as i8, 112 as i8, 111 as i8,
            95 as i8, 100 as i8, 101 as i8, 108 as i8, 116 as i8, 97 as i8,
            40 as i8, 10 as i8, 32 as i8, 32 as i8, 114 as i8, 105 as i8,
            100 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8, 69 as i8,
            71 as i8, 69 as i8, 82 as i8, 32 as i8, 80 as i8, 82 as i8,
            73 as i8, 77 as i8, 65 as i8, 82 as i8, 89 as i8, 32 as i8,
            75 as i8, 69 as i8, 89 as i8, 44 as i8, 10 as i8, 32 as i8,
            32 as i8, 115 as i8, 114 as i8, 99 as i8, 105 as i8, 100 as i8,
            32 as i8, 73 as i8, 78 as i8, 84 as i8, 69 as i8, 71 as i8,
            69 as i8, 82 as i8, 32 as i8, 78 as i8, 79 as i8, 84 as i8,
            32 as i8, 78 as i8, 85 as i8, 76 as i8, 76 as i8, 32 as i8,
            82 as i8, 69 as i8, 70 as i8, 69 as i8, 82 as i8, 69 as i8,
            78 as i8, 67 as i8, 69 as i8, 83 as i8, 32 as i8, 98 as i8,
            108 as i8, 111 as i8, 98 as i8, 10 as i8, 41 as i8, 59 as i8,
            10 as i8, 67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8,
            69 as i8, 32 as i8, 84 as i8, 65 as i8, 66 as i8, 76 as i8,
            69 as i8, 32 as i8, 114 as i8, 101 as i8, 112 as i8, 111 as i8,
            95 as i8, 114 as i8, 99 as i8, 118 as i8, 102 as i8, 114 as i8,
            111 as i8, 109 as i8, 40 as i8, 10 as i8, 32 as i8, 32 as i8,
            114 as i8, 99 as i8, 118 as i8, 105 as i8, 100 as i8, 32 as i8,
            73 as i8, 78 as i8, 84 as i8, 69 as i8, 71 as i8, 69 as i8,
            82 as i8, 32 as i8, 80 as i8, 82 as i8, 73 as i8, 77 as i8,
            65 as i8, 82 as i8, 89 as i8, 32 as i8, 75 as i8, 69 as i8,
            89 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8, 117 as i8,
            105 as i8, 100 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8,
            69 as i8, 71 as i8, 69 as i8, 82 as i8, 32 as i8, 82 as i8,
            69 as i8, 70 as i8, 69 as i8, 82 as i8, 69 as i8, 78 as i8,
            67 as i8, 69 as i8, 83 as i8, 32 as i8, 117 as i8, 115 as i8,
            101 as i8, 114 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8,
            109 as i8, 116 as i8, 105 as i8, 109 as i8, 101 as i8, 32 as i8,
            68 as i8, 65 as i8, 84 as i8, 69 as i8, 84 as i8, 73 as i8,
            77 as i8, 69 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8,
            110 as i8, 111 as i8, 110 as i8, 99 as i8, 101 as i8, 32 as i8,
            84 as i8, 69 as i8, 88 as i8, 84 as i8, 32 as i8, 85 as i8,
            78 as i8, 73 as i8, 81 as i8, 85 as i8, 69 as i8, 44 as i8,
            10 as i8, 32 as i8, 32 as i8, 105 as i8, 112 as i8, 97 as i8,
            100 as i8, 100 as i8, 114 as i8, 32 as i8, 84 as i8, 69 as i8,
            88 as i8, 84 as i8, 10 as i8, 41 as i8, 59 as i8, 10 as i8,
            67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8, 69 as i8,
            32 as i8, 84 as i8, 65 as i8, 66 as i8, 76 as i8, 69 as i8,
            32 as i8, 114 as i8, 101 as i8, 112 as i8, 111 as i8, 95 as i8,
            112 as i8, 114 as i8, 105 as i8, 118 as i8, 97 as i8, 116 as i8,
            101 as i8, 40 as i8, 114 as i8, 105 as i8, 100 as i8, 32 as i8,
            73 as i8, 78 as i8, 84 as i8, 69 as i8, 71 as i8, 69 as i8,
            82 as i8, 32 as i8, 80 as i8, 82 as i8, 73 as i8, 77 as i8,
            65 as i8, 82 as i8, 89 as i8, 32 as i8, 75 as i8, 69 as i8,
            89 as i8, 41 as i8, 59 as i8, 10 as i8, 67 as i8, 82 as i8,
            69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8, 84 as i8,
            65 as i8, 66 as i8, 76 as i8, 69 as i8, 32 as i8, 114 as i8,
            101 as i8, 112 as i8, 111 as i8, 95 as i8, 97 as i8, 99 as i8,
            99 as i8, 101 as i8, 115 as i8, 115 as i8, 108 as i8, 111 as i8,
            103 as i8, 40 as i8, 10 as i8, 32 as i8, 32 as i8, 117 as i8,
            110 as i8, 97 as i8, 109 as i8, 101 as i8, 32 as i8, 84 as i8,
            69 as i8, 88 as i8, 84 as i8, 44 as i8, 10 as i8, 32 as i8,
            32 as i8, 105 as i8, 112 as i8, 97 as i8, 100 as i8, 100 as i8,
            114 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8, 84 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 115 as i8, 117 as i8,
            99 as i8, 99 as i8, 101 as i8, 115 as i8, 115 as i8, 32 as i8,
            66 as i8, 79 as i8, 79 as i8, 76 as i8, 69 as i8, 65 as i8,
            78 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8, 109 as i8,
            116 as i8, 105 as i8, 109 as i8, 101 as i8, 32 as i8, 84 as i8,
            73 as i8, 77 as i8, 69 as i8, 83 as i8, 84 as i8, 65 as i8,
            77 as i8, 80 as i8, 41 as i8, 59 as i8, 10 as i8, 67 as i8,
            82 as i8, 69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8,
            84 as i8, 65 as i8, 66 as i8, 76 as i8, 69 as i8, 32 as i8,
            114 as i8, 101 as i8, 112 as i8, 111 as i8, 95 as i8, 117 as i8,
            115 as i8, 101 as i8, 114 as i8, 40 as i8, 10 as i8, 32 as i8,
            32 as i8, 117 as i8, 105 as i8, 100 as i8, 32 as i8, 73 as i8,
            78 as i8, 84 as i8, 69 as i8, 71 as i8, 69 as i8, 82 as i8,
            32 as i8, 80 as i8, 82 as i8, 73 as i8, 77 as i8, 65 as i8,
            82 as i8, 89 as i8, 32 as i8, 75 as i8, 69 as i8, 89 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 108 as i8, 111 as i8,
            103 as i8, 105 as i8, 110 as i8, 32 as i8, 84 as i8, 69 as i8,
            88 as i8, 84 as i8, 32 as i8, 85 as i8, 78 as i8, 73 as i8,
            81 as i8, 85 as i8, 69 as i8, 44 as i8, 10 as i8, 32 as i8,
            32 as i8, 112 as i8, 119 as i8, 32 as i8, 84 as i8, 69 as i8,
            88 as i8, 84 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8,
            99 as i8, 97 as i8, 112 as i8, 32 as i8, 84 as i8, 69 as i8,
            88 as i8, 84 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8,
            99 as i8, 111 as i8, 111 as i8, 107 as i8, 105 as i8, 101 as i8,
            32 as i8, 84 as i8, 69 as i8, 88 as i8, 84 as i8, 44 as i8,
            10 as i8, 32 as i8, 32 as i8, 105 as i8, 112 as i8, 97 as i8,
            100 as i8, 100 as i8, 114 as i8, 32 as i8, 84 as i8, 69 as i8,
            88 as i8, 84 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8,
            99 as i8, 101 as i8, 120 as i8, 112 as i8, 105 as i8, 114 as i8,
            101 as i8, 32 as i8, 68 as i8, 65 as i8, 84 as i8, 69 as i8,
            84 as i8, 73 as i8, 77 as i8, 69 as i8, 44 as i8, 10 as i8,
            32 as i8, 32 as i8, 105 as i8, 110 as i8, 102 as i8, 111 as i8,
            32 as i8, 84 as i8, 69 as i8, 88 as i8, 84 as i8, 44 as i8,
            10 as i8, 32 as i8, 32 as i8, 109 as i8, 116 as i8, 105 as i8,
            109 as i8, 101 as i8, 32 as i8, 68 as i8, 65 as i8, 84 as i8,
            69 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8, 112 as i8,
            104 as i8, 111 as i8, 116 as i8, 111 as i8, 32 as i8, 66 as i8,
            76 as i8, 79 as i8, 66 as i8, 10 as i8, 41 as i8, 59 as i8,
            10 as i8, 67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8,
            69 as i8, 32 as i8, 84 as i8, 65 as i8, 66 as i8, 76 as i8,
            69 as i8, 32 as i8, 114 as i8, 101 as i8, 112 as i8, 111 as i8,
            95 as i8, 114 as i8, 101 as i8, 112 as i8, 111 as i8, 114 as i8,
            116 as i8, 102 as i8, 109 as i8, 116 as i8, 40 as i8, 10 as i8,
            32 as i8, 32 as i8, 32 as i8, 114 as i8, 110 as i8, 32 as i8,
            73 as i8, 78 as i8, 84 as i8, 69 as i8, 71 as i8, 69 as i8,
            82 as i8, 32 as i8, 80 as i8, 82 as i8, 73 as i8, 77 as i8,
            65 as i8, 82 as i8, 89 as i8, 32 as i8, 75 as i8, 69 as i8,
            89 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8, 32 as i8,
            111 as i8, 119 as i8, 110 as i8, 101 as i8, 114 as i8, 32 as i8,
            84 as i8, 69 as i8, 88 as i8, 84 as i8, 44 as i8, 10 as i8,
            32 as i8, 32 as i8, 32 as i8, 116 as i8, 105 as i8, 116 as i8,
            108 as i8, 101 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8,
            84 as i8, 32 as i8, 85 as i8, 78 as i8, 73 as i8, 81 as i8,
            85 as i8, 69 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8,
            32 as i8, 109 as i8, 116 as i8, 105 as i8, 109 as i8, 101 as i8,
            32 as i8, 73 as i8, 78 as i8, 84 as i8, 69 as i8, 71 as i8,
            69 as i8, 82 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8,
            32 as i8, 99 as i8, 111 as i8, 108 as i8, 115 as i8, 32 as i8,
            84 as i8, 69 as i8, 88 as i8, 84 as i8, 44 as i8, 10 as i8,
            32 as i8, 32 as i8, 32 as i8, 115 as i8, 113 as i8, 108 as i8,
            99 as i8, 111 as i8, 100 as i8, 101 as i8, 32 as i8, 84 as i8,
            69 as i8, 88 as i8, 84 as i8, 10 as i8, 41 as i8, 59 as i8,
            10 as i8, 67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8,
            69 as i8, 32 as i8, 84 as i8, 65 as i8, 66 as i8, 76 as i8,
            69 as i8, 32 as i8, 114 as i8, 101 as i8, 112 as i8, 111 as i8,
            95 as i8, 115 as i8, 113 as i8, 108 as i8, 105 as i8, 116 as i8,
            101 as i8, 95 as i8, 115 as i8, 116 as i8, 97 as i8, 116 as i8,
            50 as i8, 40 as i8, 116 as i8, 98 as i8, 108 as i8, 44 as i8,
            105 as i8, 100 as i8, 120 as i8, 44 as i8, 115 as i8, 97 as i8,
            109 as i8, 112 as i8, 108 as i8, 101 as i8, 110 as i8, 111 as i8,
            44 as i8, 115 as i8, 97 as i8, 109 as i8, 112 as i8, 108 as i8,
            101 as i8, 41 as i8, 59 as i8, 10 as i8, 67 as i8, 82 as i8,
            69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8, 84 as i8,
            65 as i8, 66 as i8, 76 as i8, 69 as i8, 32 as i8, 114 as i8,
            101 as i8, 112 as i8, 111 as i8, 95 as i8, 115 as i8, 113 as i8,
            108 as i8, 105 as i8, 116 as i8, 101 as i8, 95 as i8, 115 as i8,
            116 as i8, 97 as i8, 116 as i8, 49 as i8, 40 as i8, 116 as i8,
            98 as i8, 108 as i8, 44 as i8, 105 as i8, 100 as i8, 120 as i8,
            44 as i8, 115 as i8, 116 as i8, 97 as i8, 116 as i8, 41 as i8,
            59 as i8, 10 as i8, 67 as i8, 82 as i8, 69 as i8, 65 as i8,
            84 as i8, 69 as i8, 32 as i8, 84 as i8, 65 as i8, 66 as i8,
            76 as i8, 69 as i8, 32 as i8, 114 as i8, 101 as i8, 112 as i8,
            111 as i8, 95 as i8, 115 as i8, 113 as i8, 108 as i8, 105 as i8,
            116 as i8, 101 as i8, 95 as i8, 115 as i8, 116 as i8, 97 as i8,
            116 as i8, 51 as i8, 40 as i8, 116 as i8, 98 as i8, 108 as i8,
            44 as i8, 105 as i8, 100 as i8, 120 as i8, 44 as i8, 110 as i8,
            101 as i8, 113 as i8, 44 as i8, 110 as i8, 108 as i8, 116 as i8,
            44 as i8, 110 as i8, 100 as i8, 108 as i8, 116 as i8, 44 as i8,
            115 as i8, 97 as i8, 109 as i8, 112 as i8, 108 as i8, 101 as i8,
            41 as i8, 59 as i8, 10 as i8, 67 as i8, 82 as i8, 69 as i8,
            65 as i8, 84 as i8, 69 as i8, 32 as i8, 84 as i8, 65 as i8,
            66 as i8, 76 as i8, 69 as i8, 32 as i8, 114 as i8, 101 as i8,
            112 as i8, 111 as i8, 95 as i8, 99 as i8, 111 as i8, 110 as i8,
            102 as i8, 105 as i8, 103 as i8, 40 as i8, 10 as i8, 32 as i8,
            32 as i8, 110 as i8, 97 as i8, 109 as i8, 101 as i8, 32 as i8,
            84 as i8, 69 as i8, 88 as i8, 84 as i8, 32 as i8, 80 as i8,
            82 as i8, 73 as i8, 77 as i8, 65 as i8, 82 as i8, 89 as i8,
            32 as i8, 75 as i8, 69 as i8, 89 as i8, 32 as i8, 78 as i8,
            79 as i8, 84 as i8, 32 as i8, 78 as i8, 85 as i8, 76 as i8,
            76 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8, 118 as i8,
            97 as i8, 108 as i8, 117 as i8, 101 as i8, 32 as i8, 67 as i8,
            76 as i8, 79 as i8, 66 as i8, 44 as i8, 32 as i8, 109 as i8,
            116 as i8, 105 as i8, 109 as i8, 101 as i8, 32 as i8, 73 as i8,
            78 as i8, 84 as i8, 69 as i8, 71 as i8, 69 as i8, 82 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 67 as i8, 72 as i8,
            69 as i8, 67 as i8, 75 as i8, 40 as i8, 32 as i8, 116 as i8,
            121 as i8, 112 as i8, 101 as i8, 111 as i8, 102 as i8, 40 as i8,
            110 as i8, 97 as i8, 109 as i8, 101 as i8, 41 as i8, 61 as i8,
            39 as i8, 116 as i8, 101 as i8, 120 as i8, 116 as i8, 39 as i8,
            32 as i8, 65 as i8, 78 as i8, 68 as i8, 32 as i8, 108 as i8,
            101 as i8, 110 as i8, 103 as i8, 116 as i8, 104 as i8, 40 as i8,
            110 as i8, 97 as i8, 109 as i8, 101 as i8, 41 as i8, 62 as i8,
            61 as i8, 49 as i8, 32 as i8, 41 as i8, 10 as i8, 41 as i8,
            32 as i8, 87 as i8, 73 as i8, 84 as i8, 72 as i8, 79 as i8,
            85 as i8, 84 as i8, 32 as i8, 82 as i8, 79 as i8, 87 as i8,
            73 as i8, 68 as i8, 59 as i8, 10 as i8, 67 as i8, 82 as i8,
            69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8, 84 as i8,
            65 as i8, 66 as i8, 76 as i8, 69 as i8, 32 as i8, 114 as i8,
            101 as i8, 112 as i8, 111 as i8, 95 as i8, 115 as i8, 104 as i8,
            117 as i8, 110 as i8, 40 as i8, 117 as i8, 117 as i8, 105 as i8,
            100 as i8, 32 as i8, 80 as i8, 82 as i8, 73 as i8, 77 as i8,
            65 as i8, 82 as i8, 89 as i8, 32 as i8, 75 as i8, 69 as i8,
            89 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8, 109 as i8,
            116 as i8, 105 as i8, 109 as i8, 101 as i8, 32 as i8, 73 as i8,
            78 as i8, 84 as i8, 69 as i8, 71 as i8, 69 as i8, 82 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 115 as i8, 99 as i8,
            111 as i8, 109 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8,
            84 as i8, 41 as i8, 32 as i8, 87 as i8, 73 as i8, 84 as i8,
            72 as i8, 79 as i8, 85 as i8, 84 as i8, 32 as i8, 82 as i8,
            79 as i8, 87 as i8, 73 as i8, 68 as i8, 59 as i8, 10 as i8,
            67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8, 69 as i8,
            32 as i8, 84 as i8, 65 as i8, 66 as i8, 76 as i8, 69 as i8,
            32 as i8, 114 as i8, 101 as i8, 112 as i8, 111 as i8, 95 as i8,
            99 as i8, 111 as i8, 110 as i8, 99 as i8, 101 as i8, 97 as i8,
            108 as i8, 101 as i8, 100 as i8, 40 as i8, 10 as i8, 32 as i8,
            32 as i8, 104 as i8, 97 as i8, 115 as i8, 104 as i8, 32 as i8,
            84 as i8, 69 as i8, 88 as i8, 84 as i8, 32 as i8, 80 as i8,
            82 as i8, 73 as i8, 77 as i8, 65 as i8, 82 as i8, 89 as i8,
            32 as i8, 75 as i8, 69 as i8, 89 as i8, 44 as i8, 10 as i8,
            32 as i8, 32 as i8, 99 as i8, 111 as i8, 110 as i8, 116 as i8,
            101 as i8, 110 as i8, 116 as i8, 32 as i8, 84 as i8, 69 as i8,
            88 as i8, 84 as i8, 10 as i8, 44 as i8, 32 as i8, 109 as i8,
            116 as i8, 105 as i8, 109 as i8, 101 as i8, 32 as i8, 73 as i8,
            78 as i8, 84 as i8, 69 as i8, 71 as i8, 69 as i8, 82 as i8,
            41 as i8, 32 as i8, 87 as i8, 73 as i8, 84 as i8, 72 as i8,
            79 as i8, 85 as i8, 84 as i8, 32 as i8, 82 as i8, 79 as i8,
            87 as i8, 73 as i8, 68 as i8, 59 as i8, 10 as i8, 67 as i8,
            82 as i8, 69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8,
            84 as i8, 65 as i8, 66 as i8, 76 as i8, 69 as i8, 32 as i8,
            114 as i8, 101 as i8, 112 as i8, 111 as i8, 95 as i8, 97 as i8,
            100 as i8, 109 as i8, 105 as i8, 110 as i8, 95 as i8, 108 as i8,
            111 as i8, 103 as i8, 40 as i8, 10 as i8, 32 as i8, 105 as i8,
            100 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8, 69 as i8,
            71 as i8, 69 as i8, 82 as i8, 32 as i8, 80 as i8, 82 as i8,
            73 as i8, 77 as i8, 65 as i8, 82 as i8, 89 as i8, 32 as i8,
            75 as i8, 69 as i8, 89 as i8, 44 as i8, 10 as i8, 32 as i8,
            116 as i8, 105 as i8, 109 as i8, 101 as i8, 32 as i8, 73 as i8,
            78 as i8, 84 as i8, 69 as i8, 71 as i8, 69 as i8, 82 as i8,
            44 as i8, 32 as i8, 45 as i8, 45 as i8, 32 as i8, 83 as i8,
            101 as i8, 99 as i8, 111 as i8, 110 as i8, 100 as i8, 115 as i8,
            32 as i8, 115 as i8, 105 as i8, 110 as i8, 99 as i8, 101 as i8,
            32 as i8, 49 as i8, 57 as i8, 55 as i8, 48 as i8, 10 as i8,
            32 as i8, 112 as i8, 97 as i8, 103 as i8, 101 as i8, 32 as i8,
            84 as i8, 69 as i8, 88 as i8, 84 as i8, 44 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 45 as i8, 45 as i8, 32 as i8,
            112 as i8, 97 as i8, 116 as i8, 104 as i8, 32 as i8, 111 as i8,
            102 as i8, 32 as i8, 112 as i8, 97 as i8, 103 as i8, 101 as i8,
            10 as i8, 32 as i8, 119 as i8, 104 as i8, 111 as i8, 32 as i8,
            84 as i8, 69 as i8, 88 as i8, 84 as i8, 44 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 45 as i8, 45 as i8,
            32 as i8, 85 as i8, 115 as i8, 101 as i8, 114 as i8, 32 as i8,
            119 as i8, 104 as i8, 111 as i8, 32 as i8, 109 as i8, 97 as i8,
            100 as i8, 101 as i8, 32 as i8, 116 as i8, 104 as i8, 101 as i8,
            32 as i8, 99 as i8, 104 as i8, 97 as i8, 110 as i8, 103 as i8,
            101 as i8, 10 as i8, 32 as i8, 32 as i8, 119 as i8, 104 as i8,
            97 as i8, 116 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8,
            84 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            45 as i8, 45 as i8, 32 as i8, 87 as i8, 104 as i8, 97 as i8,
            116 as i8, 32 as i8, 99 as i8, 104 as i8, 97 as i8, 110 as i8,
            103 as i8, 101 as i8, 100 as i8, 10 as i8, 41 as i8, 59 as i8,
            10 as i8, 67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8,
            69 as i8, 32 as i8, 84 as i8, 65 as i8, 66 as i8, 76 as i8,
            69 as i8, 32 as i8, 114 as i8, 101 as i8, 112 as i8, 111 as i8,
            95 as i8, 117 as i8, 110 as i8, 118 as i8, 101 as i8, 114 as i8,
            115 as i8, 105 as i8, 111 as i8, 110 as i8, 101 as i8, 100 as i8,
            40 as i8, 10 as i8, 32 as i8, 32 as i8, 110 as i8, 97 as i8,
            109 as i8, 101 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8,
            84 as i8, 32 as i8, 80 as i8, 82 as i8, 73 as i8, 77 as i8,
            65 as i8, 82 as i8, 89 as i8, 32 as i8, 75 as i8, 69 as i8,
            89 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8, 114 as i8,
            99 as i8, 118 as i8, 105 as i8, 100 as i8, 32 as i8, 73 as i8,
            78 as i8, 84 as i8, 69 as i8, 71 as i8, 69 as i8, 82 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 109 as i8, 116 as i8,
            105 as i8, 109 as i8, 101 as i8, 32 as i8, 68 as i8, 65 as i8,
            84 as i8, 69 as i8, 84 as i8, 73 as i8, 77 as i8, 69 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 104 as i8, 97 as i8,
            115 as i8, 104 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8,
            84 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8, 115 as i8,
            122 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8, 69 as i8,
            71 as i8, 69 as i8, 82 as i8, 44 as i8, 10 as i8, 32 as i8,
            32 as i8, 101 as i8, 110 as i8, 99 as i8, 111 as i8, 100 as i8,
            105 as i8, 110 as i8, 103 as i8, 32 as i8, 73 as i8, 78 as i8,
            84 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8, 99 as i8,
            111 as i8, 110 as i8, 116 as i8, 101 as i8, 110 as i8, 116 as i8,
            32 as i8, 66 as i8, 76 as i8, 79 as i8, 66 as i8, 10 as i8,
            41 as i8, 32 as i8, 87 as i8, 73 as i8, 84 as i8, 72 as i8,
            79 as i8, 85 as i8, 84 as i8, 32 as i8, 82 as i8, 79 as i8,
            87 as i8, 73 as i8, 68 as i8, 59 as i8, 10 as i8, 67 as i8,
            82 as i8, 69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8,
            84 as i8, 65 as i8, 66 as i8, 76 as i8, 69 as i8, 32 as i8,
            114 as i8, 101 as i8, 112 as i8, 111 as i8, 95 as i8, 115 as i8,
            117 as i8, 98 as i8, 115 as i8, 99 as i8, 114 as i8, 105 as i8,
            98 as i8, 101 as i8, 114 as i8, 40 as i8, 10 as i8, 32 as i8,
            32 as i8, 115 as i8, 117 as i8, 98 as i8, 115 as i8, 99 as i8,
            114 as i8, 105 as i8, 98 as i8, 101 as i8, 114 as i8, 73 as i8,
            100 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8, 69 as i8,
            71 as i8, 69 as i8, 82 as i8, 32 as i8, 80 as i8, 82 as i8,
            73 as i8, 77 as i8, 65 as i8, 82 as i8, 89 as i8, 32 as i8,
            75 as i8, 69 as i8, 89 as i8, 44 as i8, 10 as i8, 32 as i8,
            32 as i8, 115 as i8, 117 as i8, 98 as i8, 115 as i8, 99 as i8,
            114 as i8, 105 as i8, 98 as i8, 101 as i8, 114 as i8, 67 as i8,
            111 as i8, 100 as i8, 101 as i8, 32 as i8, 66 as i8, 76 as i8,
            79 as i8, 66 as i8, 32 as i8, 68 as i8, 69 as i8, 70 as i8,
            65 as i8, 85 as i8, 76 as i8, 84 as i8, 32 as i8, 40 as i8,
            114 as i8, 97 as i8, 110 as i8, 100 as i8, 111 as i8, 109 as i8,
            98 as i8, 108 as i8, 111 as i8, 98 as i8, 40 as i8, 51 as i8,
            50 as i8, 41 as i8, 41 as i8, 32 as i8, 85 as i8, 78 as i8,
            73 as i8, 81 as i8, 85 as i8, 69 as i8, 44 as i8, 10 as i8,
            32 as i8, 32 as i8, 115 as i8, 101 as i8, 109 as i8, 97 as i8,
            105 as i8, 108 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8,
            84 as i8, 32 as i8, 85 as i8, 78 as i8, 73 as i8, 81 as i8,
            85 as i8, 69 as i8, 32 as i8, 67 as i8, 79 as i8, 76 as i8,
            76 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8, 110 as i8,
            111 as i8, 99 as i8, 97 as i8, 115 as i8, 101 as i8, 44 as i8,
            10 as i8, 32 as i8, 32 as i8, 115 as i8, 117 as i8, 110 as i8,
            97 as i8, 109 as i8, 101 as i8, 32 as i8, 84 as i8, 69 as i8,
            88 as i8, 84 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8,
            115 as i8, 118 as i8, 101 as i8, 114 as i8, 105 as i8, 102 as i8,
            105 as i8, 101 as i8, 100 as i8, 32 as i8, 66 as i8, 79 as i8,
            79 as i8, 76 as i8, 69 as i8, 65 as i8, 78 as i8, 32 as i8,
            68 as i8, 69 as i8, 70 as i8, 65 as i8, 85 as i8, 76 as i8,
            84 as i8, 32 as i8, 116 as i8, 114 as i8, 117 as i8, 101 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 115 as i8, 100 as i8,
            111 as i8, 110 as i8, 111 as i8, 116 as i8, 99 as i8, 97 as i8,
            108 as i8, 108 as i8, 32 as i8, 66 as i8, 79 as i8, 79 as i8,
            76 as i8, 69 as i8, 65 as i8, 78 as i8, 44 as i8, 10 as i8,
            32 as i8, 32 as i8, 115 as i8, 100 as i8, 105 as i8, 103 as i8,
            101 as i8, 115 as i8, 116 as i8, 32 as i8, 66 as i8, 79 as i8,
            79 as i8, 76 as i8, 69 as i8, 65 as i8, 78 as i8, 44 as i8,
            10 as i8, 32 as i8, 32 as i8, 115 as i8, 115 as i8, 117 as i8,
            98 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8, 84 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 115 as i8, 99 as i8,
            116 as i8, 105 as i8, 109 as i8, 101 as i8, 32 as i8, 73 as i8,
            78 as i8, 84 as i8, 68 as i8, 65 as i8, 84 as i8, 69 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 109 as i8, 116 as i8,
            105 as i8, 109 as i8, 101 as i8, 32 as i8, 73 as i8, 78 as i8,
            84 as i8, 68 as i8, 65 as i8, 84 as i8, 69 as i8, 44 as i8,
            10 as i8, 32 as i8, 32 as i8, 115 as i8, 109 as i8, 105 as i8,
            112 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8, 84 as i8,
            10 as i8, 41 as i8, 59 as i8, 10 as i8, 67 as i8, 82 as i8,
            69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8, 84 as i8,
            65 as i8, 66 as i8, 76 as i8, 69 as i8, 32 as i8, 114 as i8,
            101 as i8, 112 as i8, 111 as i8, 95 as i8, 112 as i8, 101 as i8,
            110 as i8, 100 as i8, 105 as i8, 110 as i8, 103 as i8, 95 as i8,
            97 as i8, 108 as i8, 101 as i8, 114 as i8, 116 as i8, 40 as i8,
            10 as i8, 32 as i8, 32 as i8, 101 as i8, 118 as i8, 101 as i8,
            110 as i8, 116 as i8, 105 as i8, 100 as i8, 32 as i8, 84 as i8,
            69 as i8, 88 as i8, 84 as i8, 32 as i8, 80 as i8, 82 as i8,
            73 as i8, 77 as i8, 65 as i8, 82 as i8, 89 as i8, 32 as i8,
            75 as i8, 69 as i8, 89 as i8, 44 as i8, 10 as i8, 32 as i8,
            32 as i8, 115 as i8, 101 as i8, 110 as i8, 116 as i8, 83 as i8,
            101 as i8, 112 as i8, 32 as i8, 66 as i8, 79 as i8, 79 as i8,
            76 as i8, 69 as i8, 65 as i8, 78 as i8, 32 as i8, 68 as i8,
            69 as i8, 70 as i8, 65 as i8, 85 as i8, 76 as i8, 84 as i8,
            32 as i8, 102 as i8, 97 as i8, 108 as i8, 115 as i8, 101 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 115 as i8, 101 as i8,
            110 as i8, 116 as i8, 68 as i8, 105 as i8, 103 as i8, 101 as i8,
            115 as i8, 116 as i8, 32 as i8, 66 as i8, 79 as i8, 79 as i8,
            76 as i8, 69 as i8, 65 as i8, 78 as i8, 32 as i8, 68 as i8,
            69 as i8, 70 as i8, 65 as i8, 85 as i8, 76 as i8, 84 as i8,
            32 as i8, 102 as i8, 97 as i8, 108 as i8, 115 as i8, 101 as i8,
            10 as i8, 44 as i8, 32 as i8, 115 as i8, 101 as i8, 110 as i8,
            116 as i8, 77 as i8, 111 as i8, 100 as i8, 32 as i8, 66 as i8,
            79 as i8, 79 as i8, 76 as i8, 69 as i8, 65 as i8, 78 as i8,
            32 as i8, 68 as i8, 69 as i8, 70 as i8, 65 as i8, 85 as i8,
            76 as i8, 84 as i8, 32 as i8, 102 as i8, 97 as i8, 108 as i8,
            115 as i8, 101 as i8, 41 as i8, 32 as i8, 87 as i8, 73 as i8,
            84 as i8, 72 as i8, 79 as i8, 85 as i8, 84 as i8, 32 as i8,
            82 as i8, 79 as i8, 87 as i8, 73 as i8, 68 as i8, 59 as i8,
            10 as i8, 67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8,
            69 as i8, 32 as i8, 73 as i8, 78 as i8, 68 as i8, 69 as i8,
            88 as i8, 32 as i8, 114 as i8, 101 as i8, 112 as i8, 111 as i8,
            95 as i8, 100 as i8, 101 as i8, 108 as i8, 116 as i8, 97 as i8,
            95 as i8, 105 as i8, 49 as i8, 32 as i8, 79 as i8, 78 as i8,
            32 as i8, 114 as i8, 101 as i8, 112 as i8, 111 as i8, 95 as i8,
            100 as i8, 101 as i8, 108 as i8, 116 as i8, 97 as i8, 40 as i8,
            115 as i8, 114 as i8, 99 as i8, 105 as i8, 100 as i8, 41 as i8,
            59 as i8, 10 as i8, 67 as i8, 82 as i8, 69 as i8, 65 as i8,
            84 as i8, 69 as i8, 32 as i8, 73 as i8, 78 as i8, 68 as i8,
            69 as i8, 88 as i8, 32 as i8, 114 as i8, 101 as i8, 112 as i8,
            111 as i8, 95 as i8, 98 as i8, 108 as i8, 111 as i8, 98 as i8,
            95 as i8, 114 as i8, 99 as i8, 118 as i8, 105 as i8, 100 as i8,
            32 as i8, 79 as i8, 78 as i8, 32 as i8, 114 as i8, 101 as i8,
            112 as i8, 111 as i8, 95 as i8, 98 as i8, 108 as i8, 111 as i8,
            98 as i8, 40 as i8, 114 as i8, 99 as i8, 118 as i8, 105 as i8,
            100 as i8, 41 as i8, 59 as i8, 10 as i8, 67 as i8, 82 as i8,
            69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8, 73 as i8,
            78 as i8, 68 as i8, 69 as i8, 88 as i8, 32 as i8, 114 as i8,
            101 as i8, 112 as i8, 111 as i8, 95 as i8, 115 as i8, 117 as i8,
            98 as i8, 115 as i8, 99 as i8, 114 as i8, 105 as i8, 98 as i8,
            101 as i8, 114 as i8, 85 as i8, 110 as i8, 97 as i8, 109 as i8,
            101 as i8, 10 as i8, 32 as i8, 32 as i8, 79 as i8, 78 as i8,
            32 as i8, 114 as i8, 101 as i8, 112 as i8, 111 as i8, 95 as i8,
            115 as i8, 117 as i8, 98 as i8, 115 as i8, 99 as i8, 114 as i8,
            105 as i8, 98 as i8, 101 as i8, 114 as i8, 40 as i8, 115 as i8,
            117 as i8, 110 as i8, 97 as i8, 109 as i8, 101 as i8, 41 as i8,
            32 as i8, 87 as i8, 72 as i8, 69 as i8, 82 as i8, 69 as i8,
            32 as i8, 115 as i8, 117 as i8, 110 as i8, 97 as i8, 109 as i8,
            101 as i8, 32 as i8, 73 as i8, 83 as i8, 32 as i8, 78 as i8,
            79 as i8, 84 as i8, 32 as i8, 78 as i8, 85 as i8, 76 as i8,
            76 as i8, 59 as i8, 10 as i8, 67 as i8, 82 as i8, 69 as i8,
            65 as i8, 84 as i8, 69 as i8, 32 as i8, 86 as i8, 73 as i8,
            69 as i8, 87 as i8, 32 as i8, 114 as i8, 101 as i8, 112 as i8,
            111 as i8, 95 as i8, 97 as i8, 114 as i8, 116 as i8, 105 as i8,
            102 as i8, 97 as i8, 99 as i8, 116 as i8, 40 as i8, 114 as i8,
            105 as i8, 100 as i8, 44 as i8, 114 as i8, 99 as i8, 118 as i8,
            105 as i8, 100 as i8, 44 as i8, 115 as i8, 105 as i8, 122 as i8,
            101 as i8, 44 as i8, 97 as i8, 116 as i8, 121 as i8, 112 as i8,
            101 as i8, 44 as i8, 115 as i8, 114 as i8, 99 as i8, 105 as i8,
            100 as i8, 44 as i8, 104 as i8, 97 as i8, 115 as i8, 104 as i8,
            44 as i8, 99 as i8, 111 as i8, 110 as i8, 116 as i8, 101 as i8,
            110 as i8, 116 as i8, 41 as i8, 32 as i8, 65 as i8, 83 as i8,
            10 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            83 as i8, 69 as i8, 76 as i8, 69 as i8, 67 as i8, 84 as i8,
            32 as i8, 98 as i8, 108 as i8, 111 as i8, 98 as i8, 46 as i8,
            114 as i8, 105 as i8, 100 as i8, 44 as i8, 114 as i8, 99 as i8,
            118 as i8, 105 as i8, 100 as i8, 44 as i8, 115 as i8, 105 as i8,
            122 as i8, 101 as i8, 44 as i8, 49 as i8, 44 as i8, 115 as i8,
            114 as i8, 99 as i8, 105 as i8, 100 as i8, 44 as i8, 117 as i8,
            117 as i8, 105 as i8, 100 as i8, 44 as i8, 99 as i8, 111 as i8,
            110 as i8, 116 as i8, 101 as i8, 110 as i8, 116 as i8, 10 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 70 as i8, 82 as i8, 79 as i8, 77 as i8, 32 as i8,
            114 as i8, 101 as i8, 112 as i8, 111 as i8, 95 as i8, 98 as i8,
            108 as i8, 111 as i8, 98 as i8, 32 as i8, 76 as i8, 69 as i8,
            70 as i8, 84 as i8, 32 as i8, 74 as i8, 79 as i8, 73 as i8,
            78 as i8, 32 as i8, 114 as i8, 101 as i8, 112 as i8, 111 as i8,
            95 as i8, 100 as i8, 101 as i8, 108 as i8, 116 as i8, 97 as i8,
            32 as i8, 79 as i8, 78 as i8, 32 as i8, 40 as i8, 98 as i8,
            108 as i8, 111 as i8, 98 as i8, 46 as i8, 114 as i8, 105 as i8,
            100 as i8, 61 as i8, 100 as i8, 101 as i8, 108 as i8, 116 as i8,
            97 as i8, 46 as i8, 114 as i8, 105 as i8, 100 as i8, 41 as i8,
            59 as i8, 10 as i8, 67 as i8, 82 as i8, 69 as i8, 65 as i8,
            84 as i8, 69 as i8, 32 as i8, 84 as i8, 65 as i8, 66 as i8,
            76 as i8, 69 as i8, 32 as i8, 114 as i8, 101 as i8, 112 as i8,
            111 as i8, 95 as i8, 102 as i8, 105 as i8, 108 as i8, 101 as i8,
            110 as i8, 97 as i8, 109 as i8, 101 as i8, 40 as i8, 10 as i8,
            32 as i8, 32 as i8, 102 as i8, 110 as i8, 105 as i8, 100 as i8,
            32 as i8, 73 as i8, 78 as i8, 84 as i8, 69 as i8, 71 as i8,
            69 as i8, 82 as i8, 32 as i8, 80 as i8, 82 as i8, 73 as i8,
            77 as i8, 65 as i8, 82 as i8, 89 as i8, 32 as i8, 75 as i8,
            69 as i8, 89 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8,
            110 as i8, 97 as i8, 109 as i8, 101 as i8, 32 as i8, 84 as i8,
            69 as i8, 88 as i8, 84 as i8, 32 as i8, 85 as i8, 78 as i8,
            73 as i8, 81 as i8, 85 as i8, 69 as i8, 10 as i8, 41 as i8,
            59 as i8, 10 as i8, 67 as i8, 82 as i8, 69 as i8, 65 as i8,
            84 as i8, 69 as i8, 32 as i8, 84 as i8, 65 as i8, 66 as i8,
            76 as i8, 69 as i8, 32 as i8, 114 as i8, 101 as i8, 112 as i8,
            111 as i8, 95 as i8, 109 as i8, 108 as i8, 105 as i8, 110 as i8,
            107 as i8, 40 as i8, 10 as i8, 32 as i8, 32 as i8, 109 as i8,
            105 as i8, 100 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8,
            69 as i8, 71 as i8, 69 as i8, 82 as i8, 44 as i8, 10 as i8,
            32 as i8, 32 as i8, 102 as i8, 105 as i8, 100 as i8, 32 as i8,
            73 as i8, 78 as i8, 84 as i8, 69 as i8, 71 as i8, 69 as i8,
            82 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8, 112 as i8,
            109 as i8, 105 as i8, 100 as i8, 32 as i8, 73 as i8, 78 as i8,
            84 as i8, 69 as i8, 71 as i8, 69 as i8, 82 as i8, 44 as i8,
            10 as i8, 32 as i8, 32 as i8, 112 as i8, 105 as i8, 100 as i8,
            32 as i8, 73 as i8, 78 as i8, 84 as i8, 69 as i8, 71 as i8,
            69 as i8, 82 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8,
            102 as i8, 110 as i8, 105 as i8, 100 as i8, 32 as i8, 73 as i8,
            78 as i8, 84 as i8, 69 as i8, 71 as i8, 69 as i8, 82 as i8,
            32 as i8, 82 as i8, 69 as i8, 70 as i8, 69 as i8, 82 as i8,
            69 as i8, 78 as i8, 67 as i8, 69 as i8, 83 as i8, 32 as i8,
            102 as i8, 105 as i8, 108 as i8, 101 as i8, 110 as i8, 97 as i8,
            109 as i8, 101 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8,
            112 as i8, 102 as i8, 110 as i8, 105 as i8, 100 as i8, 32 as i8,
            73 as i8, 78 as i8, 84 as i8, 69 as i8, 71 as i8, 69 as i8,
            82 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8, 109 as i8,
            112 as i8, 101 as i8, 114 as i8, 109 as i8, 32 as i8, 73 as i8,
            78 as i8, 84 as i8, 69 as i8, 71 as i8, 69 as i8, 82 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 105 as i8, 115 as i8,
            97 as i8, 117 as i8, 120 as i8, 32 as i8, 66 as i8, 79 as i8,
            79 as i8, 76 as i8, 69 as i8, 65 as i8, 78 as i8, 32 as i8,
            68 as i8, 69 as i8, 70 as i8, 65 as i8, 85 as i8, 76 as i8,
            84 as i8, 32 as i8, 48 as i8, 10 as i8, 41 as i8, 59 as i8,
            10 as i8, 67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8,
            69 as i8, 32 as i8, 73 as i8, 78 as i8, 68 as i8, 69 as i8,
            88 as i8, 32 as i8, 114 as i8, 101 as i8, 112 as i8, 111 as i8,
            95 as i8, 109 as i8, 108 as i8, 105 as i8, 110 as i8, 107 as i8,
            95 as i8, 105 as i8, 49 as i8, 32 as i8, 79 as i8, 78 as i8,
            32 as i8, 114 as i8, 101 as i8, 112 as i8, 111 as i8, 95 as i8,
            109 as i8, 108 as i8, 105 as i8, 110 as i8, 107 as i8, 40 as i8,
            109 as i8, 105 as i8, 100 as i8, 41 as i8, 59 as i8, 10 as i8,
            67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8, 69 as i8,
            32 as i8, 73 as i8, 78 as i8, 68 as i8, 69 as i8, 88 as i8,
            32 as i8, 114 as i8, 101 as i8, 112 as i8, 111 as i8, 95 as i8,
            109 as i8, 108 as i8, 105 as i8, 110 as i8, 107 as i8, 95 as i8,
            105 as i8, 50 as i8, 32 as i8, 79 as i8, 78 as i8, 32 as i8,
            114 as i8, 101 as i8, 112 as i8, 111 as i8, 95 as i8, 109 as i8,
            108 as i8, 105 as i8, 110 as i8, 107 as i8, 40 as i8, 102 as i8,
            110 as i8, 105 as i8, 100 as i8, 41 as i8, 59 as i8, 10 as i8,
            67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8, 69 as i8,
            32 as i8, 73 as i8, 78 as i8, 68 as i8, 69 as i8, 88 as i8,
            32 as i8, 114 as i8, 101 as i8, 112 as i8, 111 as i8, 95 as i8,
            109 as i8, 108 as i8, 105 as i8, 110 as i8, 107 as i8, 95 as i8,
            105 as i8, 51 as i8, 32 as i8, 79 as i8, 78 as i8, 32 as i8,
            114 as i8, 101 as i8, 112 as i8, 111 as i8, 95 as i8, 109 as i8,
            108 as i8, 105 as i8, 110 as i8, 107 as i8, 40 as i8, 102 as i8,
            105 as i8, 100 as i8, 41 as i8, 59 as i8, 10 as i8, 67 as i8,
            82 as i8, 69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8,
            73 as i8, 78 as i8, 68 as i8, 69 as i8, 88 as i8, 32 as i8,
            114 as i8, 101 as i8, 112 as i8, 111 as i8, 95 as i8, 109 as i8,
            108 as i8, 105 as i8, 110 as i8, 107 as i8, 95 as i8, 105 as i8,
            52 as i8, 32 as i8, 79 as i8, 78 as i8, 32 as i8, 114 as i8,
            101 as i8, 112 as i8, 111 as i8, 95 as i8, 109 as i8, 108 as i8,
            105 as i8, 110 as i8, 107 as i8, 40 as i8, 112 as i8, 105 as i8,
            100 as i8, 41 as i8, 59 as i8, 10 as i8, 67 as i8, 82 as i8,
            69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8, 84 as i8,
            65 as i8, 66 as i8, 76 as i8, 69 as i8, 32 as i8, 114 as i8,
            101 as i8, 112 as i8, 111 as i8, 95 as i8, 112 as i8, 108 as i8,
            105 as i8, 110 as i8, 107 as i8, 40 as i8, 10 as i8, 32 as i8,
            32 as i8, 112 as i8, 105 as i8, 100 as i8, 32 as i8, 73 as i8,
            78 as i8, 84 as i8, 69 as i8, 71 as i8, 69 as i8, 82 as i8,
            32 as i8, 82 as i8, 69 as i8, 70 as i8, 69 as i8, 82 as i8,
            69 as i8, 78 as i8, 67 as i8, 69 as i8, 83 as i8, 32 as i8,
            98 as i8, 108 as i8, 111 as i8, 98 as i8, 44 as i8, 10 as i8,
            32 as i8, 32 as i8, 99 as i8, 105 as i8, 100 as i8, 32 as i8,
            73 as i8, 78 as i8, 84 as i8, 69 as i8, 71 as i8, 69 as i8,
            82 as i8, 32 as i8, 82 as i8, 69 as i8, 70 as i8, 69 as i8,
            82 as i8, 69 as i8, 78 as i8, 67 as i8, 69 as i8, 83 as i8,
            32 as i8, 98 as i8, 108 as i8, 111 as i8, 98 as i8, 44 as i8,
            10 as i8, 32 as i8, 32 as i8, 105 as i8, 115 as i8, 112 as i8,
            114 as i8, 105 as i8, 109 as i8, 32 as i8, 66 as i8, 79 as i8,
            79 as i8, 76 as i8, 69 as i8, 65 as i8, 78 as i8, 44 as i8,
            10 as i8, 32 as i8, 32 as i8, 109 as i8, 116 as i8, 105 as i8,
            109 as i8, 101 as i8, 32 as i8, 68 as i8, 65 as i8, 84 as i8,
            69 as i8, 84 as i8, 73 as i8, 77 as i8, 69 as i8, 44 as i8,
            10 as i8, 32 as i8, 32 as i8, 98 as i8, 97 as i8, 115 as i8,
            101 as i8, 105 as i8, 100 as i8, 32 as i8, 73 as i8, 78 as i8,
            84 as i8, 69 as i8, 71 as i8, 69 as i8, 82 as i8, 32 as i8,
            82 as i8, 69 as i8, 70 as i8, 69 as i8, 82 as i8, 69 as i8,
            78 as i8, 67 as i8, 69 as i8, 83 as i8, 32 as i8, 98 as i8,
            108 as i8, 111 as i8, 98 as i8, 44 as i8, 10 as i8, 32 as i8,
            32 as i8, 85 as i8, 78 as i8, 73 as i8, 81 as i8, 85 as i8,
            69 as i8, 40 as i8, 112 as i8, 105 as i8, 100 as i8, 44 as i8,
            32 as i8, 99 as i8, 105 as i8, 100 as i8, 41 as i8, 10 as i8,
            41 as i8, 59 as i8, 10 as i8, 67 as i8, 82 as i8, 69 as i8,
            65 as i8, 84 as i8, 69 as i8, 32 as i8, 73 as i8, 78 as i8,
            68 as i8, 69 as i8, 88 as i8, 32 as i8, 114 as i8, 101 as i8,
            112 as i8, 111 as i8, 95 as i8, 112 as i8, 108 as i8, 105 as i8,
            110 as i8, 107 as i8, 95 as i8, 105 as i8, 50 as i8, 32 as i8,
            79 as i8, 78 as i8, 32 as i8, 114 as i8, 101 as i8, 112 as i8,
            111 as i8, 95 as i8, 112 as i8, 108 as i8, 105 as i8, 110 as i8,
            107 as i8, 40 as i8, 99 as i8, 105 as i8, 100 as i8, 44 as i8,
            112 as i8, 105 as i8, 100 as i8, 41 as i8, 59 as i8, 10 as i8,
            67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8, 69 as i8,
            32 as i8, 84 as i8, 65 as i8, 66 as i8, 76 as i8, 69 as i8,
            32 as i8, 114 as i8, 101 as i8, 112 as i8, 111 as i8, 95 as i8,
            108 as i8, 101 as i8, 97 as i8, 102 as i8, 40 as i8, 114 as i8,
            105 as i8, 100 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8,
            69 as i8, 71 as i8, 69 as i8, 82 as i8, 32 as i8, 80 as i8,
            82 as i8, 73 as i8, 77 as i8, 65 as i8, 82 as i8, 89 as i8,
            32 as i8, 75 as i8, 69 as i8, 89 as i8, 41 as i8, 59 as i8,
            10 as i8, 67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8,
            69 as i8, 32 as i8, 84 as i8, 65 as i8, 66 as i8, 76 as i8,
            69 as i8, 32 as i8, 114 as i8, 101 as i8, 112 as i8, 111 as i8,
            95 as i8, 101 as i8, 118 as i8, 101 as i8, 110 as i8, 116 as i8,
            40 as i8, 10 as i8, 32 as i8, 32 as i8, 116 as i8, 121 as i8,
            112 as i8, 101 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8,
            84 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8, 109 as i8,
            116 as i8, 105 as i8, 109 as i8, 101 as i8, 32 as i8, 68 as i8,
            65 as i8, 84 as i8, 69 as i8, 84 as i8, 73 as i8, 77 as i8,
            69 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8, 111 as i8,
            98 as i8, 106 as i8, 105 as i8, 100 as i8, 32 as i8, 73 as i8,
            78 as i8, 84 as i8, 69 as i8, 71 as i8, 69 as i8, 82 as i8,
            32 as i8, 80 as i8, 82 as i8, 73 as i8, 77 as i8, 65 as i8,
            82 as i8, 89 as i8, 32 as i8, 75 as i8, 69 as i8, 89 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 116 as i8, 97 as i8,
            103 as i8, 105 as i8, 100 as i8, 32 as i8, 73 as i8, 78 as i8,
            84 as i8, 69 as i8, 71 as i8, 69 as i8, 82 as i8, 44 as i8,
            10 as i8, 32 as i8, 32 as i8, 117 as i8, 105 as i8, 100 as i8,
            32 as i8, 73 as i8, 78 as i8, 84 as i8, 69 as i8, 71 as i8,
            69 as i8, 82 as i8, 32 as i8, 82 as i8, 69 as i8, 70 as i8,
            69 as i8, 82 as i8, 69 as i8, 78 as i8, 67 as i8, 69 as i8,
            83 as i8, 32 as i8, 117 as i8, 115 as i8, 101 as i8, 114 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 98 as i8, 103 as i8,
            99 as i8, 111 as i8, 108 as i8, 111 as i8, 114 as i8, 32 as i8,
            84 as i8, 69 as i8, 88 as i8, 84 as i8, 44 as i8, 10 as i8,
            32 as i8, 32 as i8, 101 as i8, 117 as i8, 115 as i8, 101 as i8,
            114 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8, 84 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 117 as i8, 115 as i8,
            101 as i8, 114 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8,
            84 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8, 101 as i8,
            99 as i8, 111 as i8, 109 as i8, 109 as i8, 101 as i8, 110 as i8,
            116 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8, 84 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 99 as i8, 111 as i8,
            109 as i8, 109 as i8, 101 as i8, 110 as i8, 116 as i8, 32 as i8,
            84 as i8, 69 as i8, 88 as i8, 84 as i8, 44 as i8, 10 as i8,
            32 as i8, 32 as i8, 98 as i8, 114 as i8, 105 as i8, 101 as i8,
            102 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8, 84 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 111 as i8, 109 as i8,
            116 as i8, 105 as i8, 109 as i8, 101 as i8, 32 as i8, 68 as i8,
            65 as i8, 84 as i8, 69 as i8, 84 as i8, 73 as i8, 77 as i8,
            69 as i8, 10 as i8, 41 as i8, 59 as i8, 10 as i8, 67 as i8,
            82 as i8, 69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8,
            73 as i8, 78 as i8, 68 as i8, 69 as i8, 88 as i8, 32 as i8,
            114 as i8, 101 as i8, 112 as i8, 111 as i8, 95 as i8, 101 as i8,
            118 as i8, 101 as i8, 110 as i8, 116 as i8, 95 as i8, 105 as i8,
            49 as i8, 32 as i8, 79 as i8, 78 as i8, 32 as i8, 114 as i8,
            101 as i8, 112 as i8, 111 as i8, 95 as i8, 101 as i8, 118 as i8,
            101 as i8, 110 as i8, 116 as i8, 40 as i8, 109 as i8, 116 as i8,
            105 as i8, 109 as i8, 101 as i8, 41 as i8, 59 as i8, 10 as i8,
            67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8, 69 as i8,
            32 as i8, 84 as i8, 65 as i8, 66 as i8, 76 as i8, 69 as i8,
            32 as i8, 114 as i8, 101 as i8, 112 as i8, 111 as i8, 95 as i8,
            112 as i8, 104 as i8, 97 as i8, 110 as i8, 116 as i8, 111 as i8,
            109 as i8, 40 as i8, 10 as i8, 32 as i8, 32 as i8, 114 as i8,
            105 as i8, 100 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8,
            69 as i8, 71 as i8, 69 as i8, 82 as i8, 32 as i8, 80 as i8,
            82 as i8, 73 as i8, 77 as i8, 65 as i8, 82 as i8, 89 as i8,
            32 as i8, 75 as i8, 69 as i8, 89 as i8, 10 as i8, 41 as i8,
            59 as i8, 10 as i8, 67 as i8, 82 as i8, 69 as i8, 65 as i8,
            84 as i8, 69 as i8, 32 as i8, 84 as i8, 65 as i8, 66 as i8,
            76 as i8, 69 as i8, 32 as i8, 114 as i8, 101 as i8, 112 as i8,
            111 as i8, 95 as i8, 111 as i8, 114 as i8, 112 as i8, 104 as i8,
            97 as i8, 110 as i8, 40 as i8, 10 as i8, 32 as i8, 32 as i8,
            114 as i8, 105 as i8, 100 as i8, 32 as i8, 73 as i8, 78 as i8,
            84 as i8, 69 as i8, 71 as i8, 69 as i8, 82 as i8, 32 as i8,
            80 as i8, 82 as i8, 73 as i8, 77 as i8, 65 as i8, 82 as i8,
            89 as i8, 32 as i8, 75 as i8, 69 as i8, 89 as i8, 44 as i8,
            10 as i8, 32 as i8, 32 as i8, 98 as i8, 97 as i8, 115 as i8,
            101 as i8, 108 as i8, 105 as i8, 110 as i8, 101 as i8, 32 as i8,
            73 as i8, 78 as i8, 84 as i8, 69 as i8, 71 as i8, 69 as i8,
            82 as i8, 10 as i8, 41 as i8, 59 as i8, 10 as i8, 67 as i8,
            82 as i8, 69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8,
            73 as i8, 78 as i8, 68 as i8, 69 as i8, 88 as i8, 32 as i8,
            114 as i8, 101 as i8, 112 as i8, 111 as i8, 95 as i8, 111 as i8,
            114 as i8, 112 as i8, 104 as i8, 97 as i8, 110 as i8, 95 as i8,
            98 as i8, 97 as i8, 115 as i8, 101 as i8, 108 as i8, 105 as i8,
            110 as i8, 101 as i8, 32 as i8, 79 as i8, 78 as i8, 32 as i8,
            114 as i8, 101 as i8, 112 as i8, 111 as i8, 95 as i8, 111 as i8,
            114 as i8, 112 as i8, 104 as i8, 97 as i8, 110 as i8, 40 as i8,
            98 as i8, 97 as i8, 115 as i8, 101 as i8, 108 as i8, 105 as i8,
            110 as i8, 101 as i8, 41 as i8, 59 as i8, 10 as i8, 67 as i8,
            82 as i8, 69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8,
            84 as i8, 65 as i8, 66 as i8, 76 as i8, 69 as i8, 32 as i8,
            114 as i8, 101 as i8, 112 as i8, 111 as i8, 95 as i8, 117 as i8,
            110 as i8, 99 as i8, 108 as i8, 117 as i8, 115 as i8, 116 as i8,
            101 as i8, 114 as i8, 101 as i8, 100 as i8, 40 as i8, 10 as i8,
            32 as i8, 32 as i8, 114 as i8, 105 as i8, 100 as i8, 32 as i8,
            73 as i8, 78 as i8, 84 as i8, 69 as i8, 71 as i8, 69 as i8,
            82 as i8, 32 as i8, 80 as i8, 82 as i8, 73 as i8, 77 as i8,
            65 as i8, 82 as i8, 89 as i8, 32 as i8, 75 as i8, 69 as i8,
            89 as i8, 10 as i8, 41 as i8, 59 as i8, 10 as i8, 67 as i8,
            82 as i8, 69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8,
            84 as i8, 65 as i8, 66 as i8, 76 as i8, 69 as i8, 32 as i8,
            114 as i8, 101 as i8, 112 as i8, 111 as i8, 95 as i8, 117 as i8,
            110 as i8, 115 as i8, 101 as i8, 110 as i8, 116 as i8, 40 as i8,
            10 as i8, 32 as i8, 32 as i8, 114 as i8, 105 as i8, 100 as i8,
            32 as i8, 73 as i8, 78 as i8, 84 as i8, 69 as i8, 71 as i8,
            69 as i8, 82 as i8, 32 as i8, 80 as i8, 82 as i8, 73 as i8,
            77 as i8, 65 as i8, 82 as i8, 89 as i8, 32 as i8, 75 as i8,
            69 as i8, 89 as i8, 10 as i8, 41 as i8, 59 as i8, 10 as i8,
            67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8, 69 as i8,
            32 as i8, 84 as i8, 65 as i8, 66 as i8, 76 as i8, 69 as i8,
            32 as i8, 114 as i8, 101 as i8, 112 as i8, 111 as i8, 95 as i8,
            116 as i8, 97 as i8, 103 as i8, 40 as i8, 10 as i8, 32 as i8,
            32 as i8, 116 as i8, 97 as i8, 103 as i8, 105 as i8, 100 as i8,
            32 as i8, 73 as i8, 78 as i8, 84 as i8, 69 as i8, 71 as i8,
            69 as i8, 82 as i8, 32 as i8, 80 as i8, 82 as i8, 73 as i8,
            77 as i8, 65 as i8, 82 as i8, 89 as i8, 32 as i8, 75 as i8,
            69 as i8, 89 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8,
            116 as i8, 97 as i8, 103 as i8, 110 as i8, 97 as i8, 109 as i8,
            101 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8, 84 as i8,
            32 as i8, 85 as i8, 78 as i8, 73 as i8, 81 as i8, 85 as i8,
            69 as i8, 10 as i8, 41 as i8, 59 as i8, 10 as i8, 67 as i8,
            82 as i8, 69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8,
            84 as i8, 65 as i8, 66 as i8, 76 as i8, 69 as i8, 32 as i8,
            114 as i8, 101 as i8, 112 as i8, 111 as i8, 95 as i8, 116 as i8,
            97 as i8, 103 as i8, 120 as i8, 114 as i8, 101 as i8, 102 as i8,
            40 as i8, 10 as i8, 32 as i8, 32 as i8, 116 as i8, 97 as i8,
            103 as i8, 105 as i8, 100 as i8, 32 as i8, 73 as i8, 78 as i8,
            84 as i8, 69 as i8, 71 as i8, 69 as i8, 82 as i8, 32 as i8,
            82 as i8, 69 as i8, 70 as i8, 69 as i8, 82 as i8, 69 as i8,
            78 as i8, 67 as i8, 69 as i8, 83 as i8, 32 as i8, 116 as i8,
            97 as i8, 103 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8,
            116 as i8, 97 as i8, 103 as i8, 116 as i8, 121 as i8, 112 as i8,
            101 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8, 69 as i8,
            71 as i8, 69 as i8, 82 as i8, 44 as i8, 10 as i8, 32 as i8,
            32 as i8, 115 as i8, 114 as i8, 99 as i8, 105 as i8, 100 as i8,
            32 as i8, 73 as i8, 78 as i8, 84 as i8, 69 as i8, 71 as i8,
            69 as i8, 82 as i8, 32 as i8, 82 as i8, 69 as i8, 70 as i8,
            69 as i8, 82 as i8, 69 as i8, 78 as i8, 67 as i8, 69 as i8,
            83 as i8, 32 as i8, 98 as i8, 108 as i8, 111 as i8, 98 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 111 as i8, 114 as i8,
            105 as i8, 103 as i8, 105 as i8, 100 as i8, 32 as i8, 73 as i8,
            78 as i8, 84 as i8, 69 as i8, 71 as i8, 69 as i8, 82 as i8,
            32 as i8, 82 as i8, 69 as i8, 70 as i8, 69 as i8, 82 as i8,
            69 as i8, 78 as i8, 67 as i8, 69 as i8, 83 as i8, 32 as i8,
            98 as i8, 108 as i8, 111 as i8, 98 as i8, 44 as i8, 10 as i8,
            32 as i8, 32 as i8, 118 as i8, 97 as i8, 108 as i8, 117 as i8,
            101 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8, 84 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 109 as i8, 116 as i8,
            105 as i8, 109 as i8, 101 as i8, 32 as i8, 84 as i8, 73 as i8,
            77 as i8, 69 as i8, 83 as i8, 84 as i8, 65 as i8, 77 as i8,
            80 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8, 114 as i8,
            105 as i8, 100 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8,
            69 as i8, 71 as i8, 69 as i8, 82 as i8, 32 as i8, 82 as i8,
            69 as i8, 70 as i8, 69 as i8, 82 as i8, 69 as i8, 78 as i8,
            67 as i8, 69 as i8, 32 as i8, 98 as i8, 108 as i8, 111 as i8,
            98 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8, 85 as i8,
            78 as i8, 73 as i8, 81 as i8, 85 as i8, 69 as i8, 40 as i8,
            114 as i8, 105 as i8, 100 as i8, 44 as i8, 32 as i8, 116 as i8,
            97 as i8, 103 as i8, 105 as i8, 100 as i8, 41 as i8, 10 as i8,
            41 as i8, 59 as i8, 10 as i8, 67 as i8, 82 as i8, 69 as i8,
            65 as i8, 84 as i8, 69 as i8, 32 as i8, 73 as i8, 78 as i8,
            68 as i8, 69 as i8, 88 as i8, 32 as i8, 114 as i8, 101 as i8,
            112 as i8, 111 as i8, 95 as i8, 116 as i8, 97 as i8, 103 as i8,
            120 as i8, 114 as i8, 101 as i8, 102 as i8, 95 as i8, 105 as i8,
            49 as i8, 32 as i8, 79 as i8, 78 as i8, 32 as i8, 114 as i8,
            101 as i8, 112 as i8, 111 as i8, 95 as i8, 116 as i8, 97 as i8,
            103 as i8, 120 as i8, 114 as i8, 101 as i8, 102 as i8, 40 as i8,
            116 as i8, 97 as i8, 103 as i8, 105 as i8, 100 as i8, 44 as i8,
            32 as i8, 109 as i8, 116 as i8, 105 as i8, 109 as i8, 101 as i8,
            41 as i8, 59 as i8, 10 as i8, 67 as i8, 82 as i8, 69 as i8,
            65 as i8, 84 as i8, 69 as i8, 32 as i8, 84 as i8, 65 as i8,
            66 as i8, 76 as i8, 69 as i8, 32 as i8, 114 as i8, 101 as i8,
            112 as i8, 111 as i8, 95 as i8, 98 as i8, 97 as i8, 99 as i8,
            107 as i8, 108 as i8, 105 as i8, 110 as i8, 107 as i8, 40 as i8,
            10 as i8, 32 as i8, 32 as i8, 116 as i8, 97 as i8, 114 as i8,
            103 as i8, 101 as i8, 116 as i8, 32 as i8, 84 as i8, 69 as i8,
            88 as i8, 84 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8,
            115 as i8, 114 as i8, 99 as i8, 116 as i8, 121 as i8, 112 as i8,
            101 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8, 44 as i8,
            10 as i8, 32 as i8, 32 as i8, 115 as i8, 114 as i8, 99 as i8,
            105 as i8, 100 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 109 as i8, 116 as i8,
            105 as i8, 109 as i8, 101 as i8, 32 as i8, 84 as i8, 73 as i8,
            77 as i8, 69 as i8, 83 as i8, 84 as i8, 65 as i8, 77 as i8,
            80 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8, 85 as i8,
            78 as i8, 73 as i8, 81 as i8, 85 as i8, 69 as i8, 40 as i8,
            116 as i8, 97 as i8, 114 as i8, 103 as i8, 101 as i8, 116 as i8,
            44 as i8, 32 as i8, 115 as i8, 114 as i8, 99 as i8, 116 as i8,
            121 as i8, 112 as i8, 101 as i8, 44 as i8, 32 as i8, 115 as i8,
            114 as i8, 99 as i8, 105 as i8, 100 as i8, 41 as i8, 10 as i8,
            41 as i8, 59 as i8, 10 as i8, 67 as i8, 82 as i8, 69 as i8,
            65 as i8, 84 as i8, 69 as i8, 32 as i8, 73 as i8, 78 as i8,
            68 as i8, 69 as i8, 88 as i8, 32 as i8, 114 as i8, 101 as i8,
            112 as i8, 111 as i8, 95 as i8, 98 as i8, 97 as i8, 99 as i8,
            107 as i8, 108 as i8, 105 as i8, 110 as i8, 107 as i8, 95 as i8,
            115 as i8, 114 as i8, 99 as i8, 32 as i8, 79 as i8, 78 as i8,
            32 as i8, 114 as i8, 101 as i8, 112 as i8, 111 as i8, 95 as i8,
            98 as i8, 97 as i8, 99 as i8, 107 as i8, 108 as i8, 105 as i8,
            110 as i8, 107 as i8, 40 as i8, 115 as i8, 114 as i8, 99 as i8,
            105 as i8, 100 as i8, 44 as i8, 32 as i8, 115 as i8, 114 as i8,
            99 as i8, 116 as i8, 121 as i8, 112 as i8, 101 as i8, 41 as i8,
            59 as i8, 10 as i8, 67 as i8, 82 as i8, 69 as i8, 65 as i8,
            84 as i8, 69 as i8, 32 as i8, 84 as i8, 65 as i8, 66 as i8,
            76 as i8, 69 as i8, 32 as i8, 114 as i8, 101 as i8, 112 as i8,
            111 as i8, 95 as i8, 97 as i8, 116 as i8, 116 as i8, 97 as i8,
            99 as i8, 104 as i8, 109 as i8, 101 as i8, 110 as i8, 116 as i8,
            40 as i8, 10 as i8, 32 as i8, 32 as i8, 97 as i8, 116 as i8,
            116 as i8, 97 as i8, 99 as i8, 104 as i8, 105 as i8, 100 as i8,
            32 as i8, 73 as i8, 78 as i8, 84 as i8, 69 as i8, 71 as i8,
            69 as i8, 82 as i8, 32 as i8, 80 as i8, 82 as i8, 73 as i8,
            77 as i8, 65 as i8, 82 as i8, 89 as i8, 32 as i8, 75 as i8,
            69 as i8, 89 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8,
            105 as i8, 115 as i8, 76 as i8, 97 as i8, 116 as i8, 101 as i8,
            115 as i8, 116 as i8, 32 as i8, 66 as i8, 79 as i8, 79 as i8,
            76 as i8, 69 as i8, 65 as i8, 78 as i8, 32 as i8, 68 as i8,
            69 as i8, 70 as i8, 65 as i8, 85 as i8, 76 as i8, 84 as i8,
            32 as i8, 48 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8,
            109 as i8, 116 as i8, 105 as i8, 109 as i8, 101 as i8, 32 as i8,
            84 as i8, 73 as i8, 77 as i8, 69 as i8, 83 as i8, 84 as i8,
            65 as i8, 77 as i8, 80 as i8, 44 as i8, 10 as i8, 32 as i8,
            32 as i8, 115 as i8, 114 as i8, 99 as i8, 32 as i8, 84 as i8,
            69 as i8, 88 as i8, 84 as i8, 44 as i8, 10 as i8, 32 as i8,
            32 as i8, 116 as i8, 97 as i8, 114 as i8, 103 as i8, 101 as i8,
            116 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8, 84 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 102 as i8, 105 as i8,
            108 as i8, 101 as i8, 110 as i8, 97 as i8, 109 as i8, 101 as i8,
            32 as i8, 84 as i8, 69 as i8, 88 as i8, 84 as i8, 44 as i8,
            10 as i8, 32 as i8, 32 as i8, 99 as i8, 111 as i8, 109 as i8,
            109 as i8, 101 as i8, 110 as i8, 116 as i8, 32 as i8, 84 as i8,
            69 as i8, 88 as i8, 84 as i8, 44 as i8, 10 as i8, 32 as i8,
            32 as i8, 117 as i8, 115 as i8, 101 as i8, 114 as i8, 32 as i8,
            84 as i8, 69 as i8, 88 as i8, 84 as i8, 10 as i8, 41 as i8,
            59 as i8, 10 as i8, 67 as i8, 82 as i8, 69 as i8, 65 as i8,
            84 as i8, 69 as i8, 32 as i8, 73 as i8, 78 as i8, 68 as i8,
            69 as i8, 88 as i8, 32 as i8, 114 as i8, 101 as i8, 112 as i8,
            111 as i8, 95 as i8, 97 as i8, 116 as i8, 116 as i8, 97 as i8,
            99 as i8, 104 as i8, 109 as i8, 101 as i8, 110 as i8, 116 as i8,
            95 as i8, 105 as i8, 100 as i8, 120 as i8, 49 as i8, 10 as i8,
            32 as i8, 79 as i8, 78 as i8, 32 as i8, 114 as i8, 101 as i8,
            112 as i8, 111 as i8, 95 as i8, 97 as i8, 116 as i8, 116 as i8,
            97 as i8, 99 as i8, 104 as i8, 109 as i8, 101 as i8, 110 as i8,
            116 as i8, 40 as i8, 116 as i8, 97 as i8, 114 as i8, 103 as i8,
            101 as i8, 116 as i8, 44 as i8, 32 as i8, 102 as i8, 105 as i8,
            108 as i8, 101 as i8, 110 as i8, 97 as i8, 109 as i8, 101 as i8,
            44 as i8, 32 as i8, 109 as i8, 116 as i8, 105 as i8, 109 as i8,
            101 as i8, 41 as i8, 59 as i8, 10 as i8, 67 as i8, 82 as i8,
            69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8, 73 as i8,
            78 as i8, 68 as i8, 69 as i8, 88 as i8, 32 as i8, 114 as i8,
            101 as i8, 112 as i8, 111 as i8, 95 as i8, 97 as i8, 116 as i8,
            116 as i8, 97 as i8, 99 as i8, 104 as i8, 109 as i8, 101 as i8,
            110 as i8, 116 as i8, 95 as i8, 105 as i8, 100 as i8, 120 as i8,
            50 as i8, 32 as i8, 79 as i8, 78 as i8, 32 as i8, 114 as i8,
            101 as i8, 112 as i8, 111 as i8, 95 as i8, 97 as i8, 116 as i8,
            116 as i8, 97 as i8, 99 as i8, 104 as i8, 109 as i8, 101 as i8,
            110 as i8, 116 as i8, 40 as i8, 115 as i8, 114 as i8, 99 as i8,
            41 as i8, 59 as i8, 10 as i8, 67 as i8, 82 as i8, 69 as i8,
            65 as i8, 84 as i8, 69 as i8, 32 as i8, 84 as i8, 65 as i8,
            66 as i8, 76 as i8, 69 as i8, 32 as i8, 114 as i8, 101 as i8,
            112 as i8, 111 as i8, 95 as i8, 99 as i8, 104 as i8, 101 as i8,
            114 as i8, 114 as i8, 121 as i8, 112 as i8, 105 as i8, 99 as i8,
            107 as i8, 40 as i8, 10 as i8, 32 as i8, 32 as i8, 112 as i8,
            97 as i8, 114 as i8, 101 as i8, 110 as i8, 116 as i8, 105 as i8,
            100 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8, 44 as i8,
            10 as i8, 32 as i8, 32 as i8, 99 as i8, 104 as i8, 105 as i8,
            108 as i8, 100 as i8, 105 as i8, 100 as i8, 32 as i8, 73 as i8,
            78 as i8, 84 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8,
            105 as i8, 115 as i8, 69 as i8, 120 as i8, 99 as i8, 108 as i8,
            117 as i8, 100 as i8, 101 as i8, 32 as i8, 66 as i8, 79 as i8,
            79 as i8, 76 as i8, 69 as i8, 65 as i8, 78 as i8, 32 as i8,
            68 as i8, 69 as i8, 70 as i8, 65 as i8, 85 as i8, 76 as i8,
            84 as i8, 32 as i8, 102 as i8, 97 as i8, 108 as i8, 115 as i8,
            101 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8, 80 as i8,
            82 as i8, 73 as i8, 77 as i8, 65 as i8, 82 as i8, 89 as i8,
            32 as i8, 75 as i8, 69 as i8, 89 as i8, 40 as i8, 112 as i8,
            97 as i8, 114 as i8, 101 as i8, 110 as i8, 116 as i8, 105 as i8,
            100 as i8, 44 as i8, 32 as i8, 99 as i8, 104 as i8, 105 as i8,
            108 as i8, 100 as i8, 105 as i8, 100 as i8, 41 as i8, 10 as i8,
            41 as i8, 32 as i8, 87 as i8, 73 as i8, 84 as i8, 72 as i8,
            79 as i8, 85 as i8, 84 as i8, 32 as i8, 82 as i8, 79 as i8,
            87 as i8, 73 as i8, 68 as i8, 59 as i8, 10 as i8, 67 as i8,
            82 as i8, 69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8,
            73 as i8, 78 as i8, 68 as i8, 69 as i8, 88 as i8, 32 as i8,
            114 as i8, 101 as i8, 112 as i8, 111 as i8, 95 as i8, 99 as i8,
            104 as i8, 101 as i8, 114 as i8, 114 as i8, 121 as i8, 112 as i8,
            105 as i8, 99 as i8, 107 as i8, 95 as i8, 99 as i8, 105 as i8,
            100 as i8, 32 as i8, 79 as i8, 78 as i8, 32 as i8, 114 as i8,
            101 as i8, 112 as i8, 111 as i8, 95 as i8, 99 as i8, 104 as i8,
            101 as i8, 114 as i8, 114 as i8, 121 as i8, 112 as i8, 105 as i8,
            99 as i8, 107 as i8, 40 as i8, 99 as i8, 104 as i8, 105 as i8,
            108 as i8, 100 as i8, 105 as i8, 100 as i8, 41 as i8, 59 as i8,
            10 as i8, 67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8,
            69 as i8, 32 as i8, 84 as i8, 65 as i8, 66 as i8, 76 as i8,
            69 as i8, 32 as i8, 114 as i8, 101 as i8, 112 as i8, 111 as i8,
            95 as i8, 116 as i8, 105 as i8, 99 as i8, 107 as i8, 101 as i8,
            116 as i8, 40 as i8, 10 as i8, 32 as i8, 32 as i8, 45 as i8,
            45 as i8, 32 as i8, 68 as i8, 111 as i8, 32 as i8, 110 as i8,
            111 as i8, 116 as i8, 32 as i8, 99 as i8, 104 as i8, 97 as i8,
            110 as i8, 103 as i8, 101 as i8, 32 as i8, 97 as i8, 110 as i8,
            121 as i8, 32 as i8, 99 as i8, 111 as i8, 108 as i8, 117 as i8,
            109 as i8, 110 as i8, 32 as i8, 116 as i8, 104 as i8, 97 as i8,
            116 as i8, 32 as i8, 98 as i8, 101 as i8, 103 as i8, 105 as i8,
            110 as i8, 115 as i8, 32 as i8, 119 as i8, 105 as i8, 116 as i8,
            104 as i8, 32 as i8, 116 as i8, 107 as i8, 116 as i8, 95 as i8,
            10 as i8, 32 as i8, 32 as i8, 116 as i8, 107 as i8, 116 as i8,
            95 as i8, 105 as i8, 100 as i8, 32 as i8, 73 as i8, 78 as i8,
            84 as i8, 69 as i8, 71 as i8, 69 as i8, 82 as i8, 32 as i8,
            80 as i8, 82 as i8, 73 as i8, 77 as i8, 65 as i8, 82 as i8,
            89 as i8, 32 as i8, 75 as i8, 69 as i8, 89 as i8, 44 as i8,
            10 as i8, 32 as i8, 32 as i8, 116 as i8, 107 as i8, 116 as i8,
            95 as i8, 117 as i8, 117 as i8, 105 as i8, 100 as i8, 32 as i8,
            84 as i8, 69 as i8, 88 as i8, 84 as i8, 32 as i8, 85 as i8,
            78 as i8, 73 as i8, 81 as i8, 85 as i8, 69 as i8, 44 as i8,
            10 as i8, 32 as i8, 32 as i8, 116 as i8, 107 as i8, 116 as i8,
            95 as i8, 109 as i8, 116 as i8, 105 as i8, 109 as i8, 101 as i8,
            32 as i8, 68 as i8, 65 as i8, 84 as i8, 69 as i8, 44 as i8,
            10 as i8, 32 as i8, 32 as i8, 116 as i8, 107 as i8, 116 as i8,
            95 as i8, 99 as i8, 116 as i8, 105 as i8, 109 as i8, 101 as i8,
            32 as i8, 68 as i8, 65 as i8, 84 as i8, 69 as i8, 44 as i8,
            10 as i8, 32 as i8, 32 as i8, 45 as i8, 45 as i8, 32 as i8,
            65 as i8, 100 as i8, 100 as i8, 32 as i8, 97 as i8, 115 as i8,
            32 as i8, 109 as i8, 97 as i8, 110 as i8, 121 as i8, 32 as i8,
            102 as i8, 105 as i8, 101 as i8, 108 as i8, 100 as i8, 115 as i8,
            32 as i8, 97 as i8, 115 as i8, 32 as i8, 114 as i8, 101 as i8,
            113 as i8, 117 as i8, 105 as i8, 114 as i8, 101 as i8, 100 as i8,
            32 as i8, 98 as i8, 101 as i8, 108 as i8, 111 as i8, 119 as i8,
            32 as i8, 116 as i8, 104 as i8, 105 as i8, 115 as i8, 32 as i8,
            108 as i8, 105 as i8, 110 as i8, 101 as i8, 10 as i8, 32 as i8,
            32 as i8, 116 as i8, 121 as i8, 112 as i8, 101 as i8, 32 as i8,
            84 as i8, 69 as i8, 88 as i8, 84 as i8, 44 as i8, 10 as i8,
            32 as i8, 32 as i8, 115 as i8, 116 as i8, 97 as i8, 116 as i8,
            117 as i8, 115 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8,
            84 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8, 115 as i8,
            117 as i8, 98 as i8, 115 as i8, 121 as i8, 115 as i8, 116 as i8,
            101 as i8, 109 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8,
            84 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8, 112 as i8,
            114 as i8, 105 as i8, 111 as i8, 114 as i8, 105 as i8, 116 as i8,
            121 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8, 84 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 115 as i8, 101 as i8,
            118 as i8, 101 as i8, 114 as i8, 105 as i8, 116 as i8, 121 as i8,
            32 as i8, 84 as i8, 69 as i8, 88 as i8, 84 as i8, 44 as i8,
            10 as i8, 32 as i8, 32 as i8, 102 as i8, 111 as i8, 117 as i8,
            110 as i8, 100 as i8, 105 as i8, 110 as i8, 32 as i8, 84 as i8,
            69 as i8, 88 as i8, 84 as i8, 44 as i8, 10 as i8, 32 as i8,
            32 as i8, 112 as i8, 114 as i8, 105 as i8, 118 as i8, 97 as i8,
            116 as i8, 101 as i8, 95 as i8, 99 as i8, 111 as i8, 110 as i8,
            116 as i8, 97 as i8, 99 as i8, 116 as i8, 32 as i8, 84 as i8,
            69 as i8, 88 as i8, 84 as i8, 44 as i8, 10 as i8, 32 as i8,
            32 as i8, 114 as i8, 101 as i8, 115 as i8, 111 as i8, 108 as i8,
            117 as i8, 116 as i8, 105 as i8, 111 as i8, 110 as i8, 32 as i8,
            84 as i8, 69 as i8, 88 as i8, 84 as i8, 44 as i8, 10 as i8,
            32 as i8, 32 as i8, 116 as i8, 105 as i8, 116 as i8, 108 as i8,
            101 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8, 84 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 99 as i8, 111 as i8,
            109 as i8, 109 as i8, 101 as i8, 110 as i8, 116 as i8, 32 as i8,
            84 as i8, 69 as i8, 88 as i8, 84 as i8, 10 as i8, 41 as i8,
            59 as i8, 10 as i8, 67 as i8, 82 as i8, 69 as i8, 65 as i8,
            84 as i8, 69 as i8, 32 as i8, 84 as i8, 65 as i8, 66 as i8,
            76 as i8, 69 as i8, 32 as i8, 114 as i8, 101 as i8, 112 as i8,
            111 as i8, 95 as i8, 116 as i8, 105 as i8, 99 as i8, 107 as i8,
            101 as i8, 116 as i8, 99 as i8, 104 as i8, 110 as i8, 103 as i8,
            40 as i8, 10 as i8, 32 as i8, 32 as i8, 45 as i8, 45 as i8,
            32 as i8, 68 as i8, 111 as i8, 32 as i8, 110 as i8, 111 as i8,
            116 as i8, 32 as i8, 99 as i8, 104 as i8, 97 as i8, 110 as i8,
            103 as i8, 101 as i8, 32 as i8, 97 as i8, 110 as i8, 121 as i8,
            32 as i8, 99 as i8, 111 as i8, 108 as i8, 117 as i8, 109 as i8,
            110 as i8, 32 as i8, 116 as i8, 104 as i8, 97 as i8, 116 as i8,
            32 as i8, 98 as i8, 101 as i8, 103 as i8, 105 as i8, 110 as i8,
            115 as i8, 32 as i8, 119 as i8, 105 as i8, 116 as i8, 104 as i8,
            32 as i8, 116 as i8, 107 as i8, 116 as i8, 95 as i8, 10 as i8,
            32 as i8, 32 as i8, 116 as i8, 107 as i8, 116 as i8, 95 as i8,
            105 as i8, 100 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8,
            69 as i8, 71 as i8, 69 as i8, 82 as i8, 32 as i8, 82 as i8,
            69 as i8, 70 as i8, 69 as i8, 82 as i8, 69 as i8, 78 as i8,
            67 as i8, 69 as i8, 83 as i8, 32 as i8, 116 as i8, 105 as i8,
            99 as i8, 107 as i8, 101 as i8, 116 as i8, 44 as i8, 10 as i8,
            32 as i8, 32 as i8, 116 as i8, 107 as i8, 116 as i8, 95 as i8,
            114 as i8, 105 as i8, 100 as i8, 32 as i8, 73 as i8, 78 as i8,
            84 as i8, 69 as i8, 71 as i8, 69 as i8, 82 as i8, 32 as i8,
            82 as i8, 69 as i8, 70 as i8, 69 as i8, 82 as i8, 69 as i8,
            78 as i8, 67 as i8, 69 as i8, 83 as i8, 32 as i8, 98 as i8,
            108 as i8, 111 as i8, 98 as i8, 44 as i8, 10 as i8, 32 as i8,
            32 as i8, 116 as i8, 107 as i8, 116 as i8, 95 as i8, 109 as i8,
            116 as i8, 105 as i8, 109 as i8, 101 as i8, 32 as i8, 68 as i8,
            65 as i8, 84 as i8, 69 as i8, 44 as i8, 10 as i8, 32 as i8,
            32 as i8, 45 as i8, 45 as i8, 32 as i8, 65 as i8, 100 as i8,
            100 as i8, 32 as i8, 97 as i8, 115 as i8, 32 as i8, 109 as i8,
            97 as i8, 110 as i8, 121 as i8, 32 as i8, 102 as i8, 105 as i8,
            101 as i8, 108 as i8, 100 as i8, 115 as i8, 32 as i8, 97 as i8,
            115 as i8, 32 as i8, 114 as i8, 101 as i8, 113 as i8, 117 as i8,
            105 as i8, 114 as i8, 101 as i8, 100 as i8, 32 as i8, 98 as i8,
            101 as i8, 108 as i8, 111 as i8, 119 as i8, 32 as i8, 116 as i8,
            104 as i8, 105 as i8, 115 as i8, 32 as i8, 108 as i8, 105 as i8,
            110 as i8, 101 as i8, 10 as i8, 32 as i8, 32 as i8, 108 as i8,
            111 as i8, 103 as i8, 105 as i8, 110 as i8, 32 as i8, 84 as i8,
            69 as i8, 88 as i8, 84 as i8, 44 as i8, 10 as i8, 32 as i8,
            32 as i8, 117 as i8, 115 as i8, 101 as i8, 114 as i8, 110 as i8,
            97 as i8, 109 as i8, 101 as i8, 32 as i8, 84 as i8, 69 as i8,
            88 as i8, 84 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8,
            109 as i8, 105 as i8, 109 as i8, 101 as i8, 116 as i8, 121 as i8,
            112 as i8, 101 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8,
            84 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8, 105 as i8,
            99 as i8, 111 as i8, 109 as i8, 109 as i8, 101 as i8, 110 as i8,
            116 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8, 84 as i8,
            10 as i8, 41 as i8, 59 as i8, 10 as i8, 67 as i8, 82 as i8,
            69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8, 73 as i8,
            78 as i8, 68 as i8, 69 as i8, 88 as i8, 32 as i8, 114 as i8,
            101 as i8, 112 as i8, 111 as i8, 95 as i8, 116 as i8, 105 as i8,
            99 as i8, 107 as i8, 101 as i8, 116 as i8, 99 as i8, 104 as i8,
            110 as i8, 103 as i8, 95 as i8, 105 as i8, 100 as i8, 120 as i8,
            49 as i8, 32 as i8, 79 as i8, 78 as i8, 32 as i8, 114 as i8,
            101 as i8, 112 as i8, 111 as i8, 95 as i8, 116 as i8, 105 as i8,
            99 as i8, 107 as i8, 101 as i8, 116 as i8, 99 as i8, 104 as i8,
            110 as i8, 103 as i8, 40 as i8, 116 as i8, 107 as i8, 116 as i8,
            95 as i8, 105 as i8, 100 as i8, 44 as i8, 32 as i8, 116 as i8,
            107 as i8, 116 as i8, 95 as i8, 109 as i8, 116 as i8, 105 as i8,
            109 as i8, 101 as i8, 41 as i8, 59 as i8, 10 as i8, 67 as i8,
            82 as i8, 69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8,
            84 as i8, 82 as i8, 73 as i8, 71 as i8, 71 as i8, 69 as i8,
            82 as i8, 32 as i8, 114 as i8, 101 as i8, 112 as i8, 111 as i8,
            95 as i8, 97 as i8, 108 as i8, 101 as i8, 114 as i8, 116 as i8,
            95 as i8, 116 as i8, 114 as i8, 105 as i8, 103 as i8, 103 as i8,
            101 as i8, 114 as i8, 49 as i8, 10 as i8, 65 as i8, 70 as i8,
            84 as i8, 69 as i8, 82 as i8, 32 as i8, 73 as i8, 78 as i8,
            83 as i8, 69 as i8, 82 as i8, 84 as i8, 32 as i8, 79 as i8,
            78 as i8, 32 as i8, 114 as i8, 101 as i8, 112 as i8, 111 as i8,
            95 as i8, 101 as i8, 118 as i8, 101 as i8, 110 as i8, 116 as i8,
            32 as i8, 66 as i8, 69 as i8, 71 as i8, 73 as i8, 78 as i8,
            10 as i8, 32 as i8, 32 as i8, 73 as i8, 78 as i8, 83 as i8,
            69 as i8, 82 as i8, 84 as i8, 32 as i8, 73 as i8, 78 as i8,
            84 as i8, 79 as i8, 32 as i8, 114 as i8, 101 as i8, 112 as i8,
            111 as i8, 95 as i8, 112 as i8, 101 as i8, 110 as i8, 100 as i8,
            105 as i8, 110 as i8, 103 as i8, 95 as i8, 97 as i8, 108 as i8,
            101 as i8, 114 as i8, 116 as i8, 40 as i8, 101 as i8, 118 as i8,
            101 as i8, 110 as i8, 116 as i8, 105 as i8, 100 as i8, 41 as i8,
            10 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 83 as i8,
            69 as i8, 76 as i8, 69 as i8, 67 as i8, 84 as i8, 32 as i8,
            112 as i8, 114 as i8, 105 as i8, 110 as i8, 116 as i8, 102 as i8,
            40 as i8, 39 as i8, 37 as i8, 46 as i8, 49 as i8, 99 as i8,
            37 as i8, 100 as i8, 39 as i8, 44 as i8, 110 as i8, 101 as i8,
            119 as i8, 46 as i8, 116 as i8, 121 as i8, 112 as i8, 101 as i8,
            44 as i8, 110 as i8, 101 as i8, 119 as i8, 46 as i8, 111 as i8,
            98 as i8, 106 as i8, 105 as i8, 100 as i8, 41 as i8, 32 as i8,
            87 as i8, 72 as i8, 69 as i8, 82 as i8, 69 as i8, 32 as i8,
            116 as i8, 114 as i8, 117 as i8, 101 as i8, 10 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 79 as i8, 78 as i8, 32 as i8,
            67 as i8, 79 as i8, 78 as i8, 70 as i8, 76 as i8, 73 as i8,
            67 as i8, 84 as i8, 40 as i8, 101 as i8, 118 as i8, 101 as i8,
            110 as i8, 116 as i8, 73 as i8, 100 as i8, 41 as i8, 32 as i8,
            68 as i8, 79 as i8, 32 as i8, 78 as i8, 79 as i8, 84 as i8,
            72 as i8, 73 as i8, 78 as i8, 71 as i8, 59 as i8, 10 as i8,
            69 as i8, 78 as i8, 68 as i8, 59 as i8, 10 as i8, 67 as i8,
            82 as i8, 69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8,
            84 as i8, 65 as i8, 66 as i8, 76 as i8, 69 as i8, 32 as i8,
            114 as i8, 101 as i8, 112 as i8, 111 as i8, 95 as i8, 118 as i8,
            99 as i8, 97 as i8, 99 as i8, 104 as i8, 101 as i8, 40 as i8,
            10 as i8, 32 as i8, 32 as i8, 118 as i8, 105 as i8, 100 as i8,
            32 as i8, 73 as i8, 78 as i8, 84 as i8, 69 as i8, 71 as i8,
            69 as i8, 82 as i8, 44 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            45 as i8, 45 as i8, 32 as i8, 99 as i8, 104 as i8, 101 as i8,
            99 as i8, 107 as i8, 45 as i8, 105 as i8, 110 as i8, 32 as i8,
            73 as i8, 68 as i8, 10 as i8, 32 as i8, 32 as i8, 102 as i8,
            110 as i8, 97 as i8, 109 as i8, 101 as i8, 32 as i8, 84 as i8,
            69 as i8, 88 as i8, 84 as i8, 44 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 45 as i8, 45 as i8, 32 as i8, 102 as i8,
            105 as i8, 108 as i8, 101 as i8, 110 as i8, 97 as i8, 109 as i8,
            101 as i8, 10 as i8, 32 as i8, 32 as i8, 114 as i8, 105 as i8,
            100 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8, 69 as i8,
            71 as i8, 69 as i8, 82 as i8, 44 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 45 as i8, 45 as i8, 32 as i8, 97 as i8, 114 as i8,
            116 as i8, 105 as i8, 102 as i8, 97 as i8, 99 as i8, 116 as i8,
            32 as i8, 73 as i8, 68 as i8, 10 as i8, 32 as i8, 32 as i8,
            80 as i8, 82 as i8, 73 as i8, 77 as i8, 65 as i8, 82 as i8,
            89 as i8, 32 as i8, 75 as i8, 69 as i8, 89 as i8, 40 as i8,
            118 as i8, 105 as i8, 100 as i8, 44 as i8, 102 as i8, 110 as i8,
            97 as i8, 109 as i8, 101 as i8, 41 as i8, 10 as i8, 41 as i8,
            32 as i8, 87 as i8, 73 as i8, 84 as i8, 72 as i8, 79 as i8,
            85 as i8, 84 as i8, 32 as i8, 82 as i8, 79 as i8, 87 as i8,
            73 as i8, 68 as i8, 59 as i8, 10 as i8, 67 as i8, 82 as i8,
            69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8, 84 as i8,
            65 as i8, 66 as i8, 76 as i8, 69 as i8, 32 as i8, 108 as i8,
            111 as i8, 99 as i8, 97 as i8, 108 as i8, 100 as i8, 98 as i8,
            95 as i8, 118 as i8, 118 as i8, 97 as i8, 114 as i8, 40 as i8,
            10 as i8, 32 as i8, 32 as i8, 110 as i8, 97 as i8, 109 as i8,
            101 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8, 84 as i8,
            32 as i8, 80 as i8, 82 as i8, 73 as i8, 77 as i8, 65 as i8,
            82 as i8, 89 as i8, 32 as i8, 75 as i8, 69 as i8, 89 as i8,
            32 as i8, 78 as i8, 79 as i8, 84 as i8, 32 as i8, 78 as i8,
            85 as i8, 76 as i8, 76 as i8, 44 as i8, 10 as i8, 32 as i8,
            32 as i8, 118 as i8, 97 as i8, 108 as i8, 117 as i8, 101 as i8,
            32 as i8, 67 as i8, 76 as i8, 79 as i8, 66 as i8, 44 as i8,
            10 as i8, 32 as i8, 32 as i8, 67 as i8, 72 as i8, 69 as i8,
            67 as i8, 75 as i8, 40 as i8, 32 as i8, 116 as i8, 121 as i8,
            112 as i8, 101 as i8, 111 as i8, 102 as i8, 40 as i8, 110 as i8,
            97 as i8, 109 as i8, 101 as i8, 41 as i8, 61 as i8, 39 as i8,
            116 as i8, 101 as i8, 120 as i8, 116 as i8, 39 as i8, 32 as i8,
            65 as i8, 78 as i8, 68 as i8, 32 as i8, 108 as i8, 101 as i8,
            110 as i8, 103 as i8, 116 as i8, 104 as i8, 40 as i8, 110 as i8,
            97 as i8, 109 as i8, 101 as i8, 41 as i8, 62 as i8, 61 as i8,
            49 as i8, 32 as i8, 41 as i8, 10 as i8, 41 as i8, 59 as i8,
            10 as i8, 67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8,
            69 as i8, 32 as i8, 84 as i8, 65 as i8, 66 as i8, 76 as i8,
            69 as i8, 32 as i8, 108 as i8, 111 as i8, 99 as i8, 97 as i8,
            108 as i8, 100 as i8, 98 as i8, 95 as i8, 118 as i8, 102 as i8,
            105 as i8, 108 as i8, 101 as i8, 40 as i8, 10 as i8, 32 as i8,
            32 as i8, 105 as i8, 100 as i8, 32 as i8, 73 as i8, 78 as i8,
            84 as i8, 69 as i8, 71 as i8, 69 as i8, 82 as i8, 32 as i8,
            80 as i8, 82 as i8, 73 as i8, 77 as i8, 65 as i8, 82 as i8,
            89 as i8, 32 as i8, 75 as i8, 69 as i8, 89 as i8, 44 as i8,
            10 as i8, 32 as i8, 32 as i8, 118 as i8, 105 as i8, 100 as i8,
            32 as i8, 73 as i8, 78 as i8, 84 as i8, 69 as i8, 71 as i8,
            69 as i8, 82 as i8, 32 as i8, 82 as i8, 69 as i8, 70 as i8,
            69 as i8, 82 as i8, 69 as i8, 78 as i8, 67 as i8, 69 as i8,
            83 as i8, 32 as i8, 98 as i8, 108 as i8, 111 as i8, 98 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 99 as i8, 104 as i8,
            110 as i8, 103 as i8, 101 as i8, 100 as i8, 32 as i8, 73 as i8,
            78 as i8, 84 as i8, 32 as i8, 68 as i8, 69 as i8, 70 as i8,
            65 as i8, 85 as i8, 76 as i8, 84 as i8, 32 as i8, 48 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 100 as i8, 101 as i8,
            108 as i8, 101 as i8, 116 as i8, 101 as i8, 100 as i8, 32 as i8,
            66 as i8, 79 as i8, 79 as i8, 76 as i8, 69 as i8, 65 as i8,
            78 as i8, 32 as i8, 68 as i8, 69 as i8, 70 as i8, 65 as i8,
            85 as i8, 76 as i8, 84 as i8, 32 as i8, 48 as i8, 44 as i8,
            10 as i8, 32 as i8, 32 as i8, 105 as i8, 115 as i8, 101 as i8,
            120 as i8, 101 as i8, 32 as i8, 66 as i8, 79 as i8, 79 as i8,
            76 as i8, 69 as i8, 65 as i8, 78 as i8, 44 as i8, 10 as i8,
            32 as i8, 32 as i8, 105 as i8, 115 as i8, 108 as i8, 105 as i8,
            110 as i8, 107 as i8, 32 as i8, 66 as i8, 79 as i8, 79 as i8,
            76 as i8, 69 as i8, 65 as i8, 78 as i8, 44 as i8, 10 as i8,
            32 as i8, 32 as i8, 114 as i8, 105 as i8, 100 as i8, 32 as i8,
            73 as i8, 78 as i8, 84 as i8, 69 as i8, 71 as i8, 69 as i8,
            82 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8, 109 as i8,
            114 as i8, 105 as i8, 100 as i8, 32 as i8, 73 as i8, 78 as i8,
            84 as i8, 69 as i8, 71 as i8, 69 as i8, 82 as i8, 44 as i8,
            10 as i8, 32 as i8, 32 as i8, 109 as i8, 116 as i8, 105 as i8,
            109 as i8, 101 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8,
            69 as i8, 71 as i8, 69 as i8, 82 as i8, 44 as i8, 10 as i8,
            32 as i8, 32 as i8, 112 as i8, 97 as i8, 116 as i8, 104 as i8,
            110 as i8, 97 as i8, 109 as i8, 101 as i8, 32 as i8, 84 as i8,
            69 as i8, 88 as i8, 84 as i8, 44 as i8, 10 as i8, 32 as i8,
            32 as i8, 111 as i8, 114 as i8, 105 as i8, 103 as i8, 110 as i8,
            97 as i8, 109 as i8, 101 as i8, 32 as i8, 84 as i8, 69 as i8,
            88 as i8, 84 as i8, 44 as i8, 32 as i8, 109 as i8, 104 as i8,
            97 as i8, 115 as i8, 104 as i8, 44 as i8, 10 as i8, 32 as i8,
            32 as i8, 85 as i8, 78 as i8, 73 as i8, 81 as i8, 85 as i8,
            69 as i8, 40 as i8, 112 as i8, 97 as i8, 116 as i8, 104 as i8,
            110 as i8, 97 as i8, 109 as i8, 101 as i8, 44 as i8, 118 as i8,
            105 as i8, 100 as i8, 41 as i8, 10 as i8, 41 as i8, 59 as i8,
            10 as i8, 67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8,
            69 as i8, 32 as i8, 84 as i8, 65 as i8, 66 as i8, 76 as i8,
            69 as i8, 32 as i8, 108 as i8, 111 as i8, 99 as i8, 97 as i8,
            108 as i8, 100 as i8, 98 as i8, 95 as i8, 115 as i8, 113 as i8,
            108 as i8, 105 as i8, 116 as i8, 101 as i8, 95 as i8, 115 as i8,
            116 as i8, 97 as i8, 116 as i8, 49 as i8, 40 as i8, 116 as i8,
            98 as i8, 108 as i8, 44 as i8, 105 as i8, 100 as i8, 120 as i8,
            44 as i8, 115 as i8, 116 as i8, 97 as i8, 116 as i8, 41 as i8,
            59 as i8, 10 as i8, 67 as i8, 82 as i8, 69 as i8, 65 as i8,
            84 as i8, 69 as i8, 32 as i8, 84 as i8, 65 as i8, 66 as i8,
            76 as i8, 69 as i8, 32 as i8, 108 as i8, 111 as i8, 99 as i8,
            97 as i8, 108 as i8, 100 as i8, 98 as i8, 95 as i8, 118 as i8,
            99 as i8, 97 as i8, 99 as i8, 104 as i8, 101 as i8, 40 as i8,
            10 as i8, 32 as i8, 32 as i8, 118 as i8, 105 as i8, 100 as i8,
            32 as i8, 73 as i8, 78 as i8, 84 as i8, 69 as i8, 71 as i8,
            69 as i8, 82 as i8, 44 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            45 as i8, 45 as i8, 32 as i8, 99 as i8, 104 as i8, 101 as i8,
            99 as i8, 107 as i8, 45 as i8, 105 as i8, 110 as i8, 32 as i8,
            73 as i8, 68 as i8, 10 as i8, 32 as i8, 32 as i8, 102 as i8,
            110 as i8, 97 as i8, 109 as i8, 101 as i8, 32 as i8, 84 as i8,
            69 as i8, 88 as i8, 84 as i8, 44 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 45 as i8, 45 as i8, 32 as i8, 102 as i8,
            105 as i8, 108 as i8, 101 as i8, 110 as i8, 97 as i8, 109 as i8,
            101 as i8, 10 as i8, 32 as i8, 32 as i8, 114 as i8, 105 as i8,
            100 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8, 69 as i8,
            71 as i8, 69 as i8, 82 as i8, 44 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 45 as i8, 45 as i8, 32 as i8, 97 as i8, 114 as i8,
            116 as i8, 105 as i8, 102 as i8, 97 as i8, 99 as i8, 116 as i8,
            32 as i8, 73 as i8, 68 as i8, 10 as i8, 32 as i8, 32 as i8,
            80 as i8, 82 as i8, 73 as i8, 77 as i8, 65 as i8, 82 as i8,
            89 as i8, 32 as i8, 75 as i8, 69 as i8, 89 as i8, 40 as i8,
            118 as i8, 105 as i8, 100 as i8, 44 as i8, 102 as i8, 110 as i8,
            97 as i8, 109 as i8, 101 as i8, 41 as i8, 10 as i8, 41 as i8,
            32 as i8, 87 as i8, 73 as i8, 84 as i8, 72 as i8, 79 as i8,
            85 as i8, 84 as i8, 32 as i8, 82 as i8, 79 as i8, 87 as i8,
            73 as i8, 68 as i8, 59 as i8, 10 as i8, 67 as i8, 82 as i8,
            69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8, 84 as i8,
            65 as i8, 66 as i8, 76 as i8, 69 as i8, 32 as i8, 108 as i8,
            111 as i8, 99 as i8, 97 as i8, 108 as i8, 100 as i8, 98 as i8,
            95 as i8, 115 as i8, 116 as i8, 97 as i8, 115 as i8, 104 as i8,
            40 as i8, 10 as i8, 32 as i8, 32 as i8, 115 as i8, 116 as i8,
            97 as i8, 115 as i8, 104 as i8, 105 as i8, 100 as i8, 32 as i8,
            73 as i8, 78 as i8, 84 as i8, 69 as i8, 71 as i8, 69 as i8,
            82 as i8, 32 as i8, 80 as i8, 82 as i8, 73 as i8, 77 as i8,
            65 as i8, 82 as i8, 89 as i8, 32 as i8, 75 as i8, 69 as i8,
            89 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8, 118 as i8,
            105 as i8, 100 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8,
            69 as i8, 71 as i8, 69 as i8, 82 as i8, 44 as i8, 10 as i8,
            32 as i8, 32 as i8, 104 as i8, 97 as i8, 115 as i8, 104 as i8,
            32 as i8, 84 as i8, 69 as i8, 88 as i8, 84 as i8, 44 as i8,
            10 as i8, 32 as i8, 32 as i8, 99 as i8, 111 as i8, 109 as i8,
            109 as i8, 101 as i8, 110 as i8, 116 as i8, 32 as i8, 84 as i8,
            69 as i8, 88 as i8, 84 as i8, 44 as i8, 10 as i8, 32 as i8,
            32 as i8, 99 as i8, 116 as i8, 105 as i8, 109 as i8, 101 as i8,
            32 as i8, 84 as i8, 73 as i8, 77 as i8, 69 as i8, 83 as i8,
            84 as i8, 65 as i8, 77 as i8, 80 as i8, 10 as i8, 41 as i8,
            59 as i8, 10 as i8, 67 as i8, 82 as i8, 69 as i8, 65 as i8,
            84 as i8, 69 as i8, 32 as i8, 84 as i8, 65 as i8, 66 as i8,
            76 as i8, 69 as i8, 32 as i8, 108 as i8, 111 as i8, 99 as i8,
            97 as i8, 108 as i8, 100 as i8, 98 as i8, 95 as i8, 115 as i8,
            116 as i8, 97 as i8, 115 as i8, 104 as i8, 102 as i8, 105 as i8,
            108 as i8, 101 as i8, 40 as i8, 10 as i8, 32 as i8, 32 as i8,
            115 as i8, 116 as i8, 97 as i8, 115 as i8, 104 as i8, 105 as i8,
            100 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8, 69 as i8,
            71 as i8, 69 as i8, 82 as i8, 32 as i8, 82 as i8, 69 as i8,
            70 as i8, 69 as i8, 82 as i8, 69 as i8, 78 as i8, 67 as i8,
            69 as i8, 83 as i8, 32 as i8, 115 as i8, 116 as i8, 97 as i8,
            115 as i8, 104 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8,
            105 as i8, 115 as i8, 65 as i8, 100 as i8, 100 as i8, 101 as i8,
            100 as i8, 32 as i8, 66 as i8, 79 as i8, 79 as i8, 76 as i8,
            69 as i8, 65 as i8, 78 as i8, 44 as i8, 10 as i8, 32 as i8,
            32 as i8, 105 as i8, 115 as i8, 82 as i8, 101 as i8, 109 as i8,
            111 as i8, 118 as i8, 101 as i8, 100 as i8, 32 as i8, 66 as i8,
            79 as i8, 79 as i8, 76 as i8, 69 as i8, 65 as i8, 78 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 105 as i8, 115 as i8,
            69 as i8, 120 as i8, 101 as i8, 99 as i8, 32 as i8, 66 as i8,
            79 as i8, 79 as i8, 76 as i8, 69 as i8, 65 as i8, 78 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 105 as i8, 115 as i8,
            76 as i8, 105 as i8, 110 as i8, 107 as i8, 32 as i8, 66 as i8,
            79 as i8, 79 as i8, 76 as i8, 69 as i8, 65 as i8, 78 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 114 as i8, 105 as i8,
            100 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8, 69 as i8,
            71 as i8, 69 as i8, 82 as i8, 44 as i8, 10 as i8, 32 as i8,
            32 as i8, 104 as i8, 97 as i8, 115 as i8, 104 as i8, 32 as i8,
            84 as i8, 69 as i8, 88 as i8, 84 as i8, 44 as i8, 10 as i8,
            32 as i8, 32 as i8, 111 as i8, 114 as i8, 105 as i8, 103 as i8,
            110 as i8, 97 as i8, 109 as i8, 101 as i8, 32 as i8, 84 as i8,
            69 as i8, 88 as i8, 84 as i8, 44 as i8, 10 as i8, 32 as i8,
            32 as i8, 110 as i8, 101 as i8, 119 as i8, 110 as i8, 97 as i8,
            109 as i8, 101 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8,
            84 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8, 100 as i8,
            101 as i8, 108 as i8, 116 as i8, 97 as i8, 32 as i8, 66 as i8,
            76 as i8, 79 as i8, 66 as i8, 44 as i8, 10 as i8, 32 as i8,
            32 as i8, 80 as i8, 82 as i8, 73 as i8, 77 as i8, 65 as i8,
            82 as i8, 89 as i8, 32 as i8, 75 as i8, 69 as i8, 89 as i8,
            40 as i8, 110 as i8, 101 as i8, 119 as i8, 110 as i8, 97 as i8,
            109 as i8, 101 as i8, 44 as i8, 32 as i8, 115 as i8, 116 as i8,
            97 as i8, 115 as i8, 104 as i8, 105 as i8, 100 as i8, 41 as i8,
            10 as i8, 41 as i8, 59 as i8, 10 as i8, 67 as i8, 82 as i8,
            69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8, 84 as i8,
            65 as i8, 66 as i8, 76 as i8, 69 as i8, 32 as i8, 108 as i8,
            111 as i8, 99 as i8, 97 as i8, 108 as i8, 100 as i8, 98 as i8,
            95 as i8, 118 as i8, 109 as i8, 101 as i8, 114 as i8, 103 as i8,
            101 as i8, 40 as i8, 10 as i8, 32 as i8, 32 as i8, 105 as i8,
            100 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8, 69 as i8,
            71 as i8, 69 as i8, 82 as i8, 32 as i8, 82 as i8, 69 as i8,
            70 as i8, 69 as i8, 82 as i8, 69 as i8, 78 as i8, 67 as i8,
            69 as i8, 83 as i8, 32 as i8, 118 as i8, 102 as i8, 105 as i8,
            108 as i8, 101 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8,
            109 as i8, 101 as i8, 114 as i8, 103 as i8, 101 as i8, 32 as i8,
            73 as i8, 78 as i8, 84 as i8, 69 as i8, 71 as i8, 69 as i8,
            82 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8, 109 as i8,
            104 as i8, 97 as i8, 115 as i8, 104 as i8, 32 as i8, 84 as i8,
            69 as i8, 88 as i8, 84 as i8, 10 as i8, 41 as i8, 59 as i8,
            10 as i8, 67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8,
            69 as i8, 32 as i8, 85 as i8, 78 as i8, 73 as i8, 81 as i8,
            85 as i8, 69 as i8, 32 as i8, 73 as i8, 78 as i8, 68 as i8,
            69 as i8, 88 as i8, 32 as i8, 108 as i8, 111 as i8, 99 as i8,
            97 as i8, 108 as i8, 100 as i8, 98 as i8, 95 as i8, 118 as i8,
            109 as i8, 101 as i8, 114 as i8, 103 as i8, 101 as i8, 120 as i8,
            49 as i8, 32 as i8, 79 as i8, 78 as i8, 32 as i8, 108 as i8,
            111 as i8, 99 as i8, 97 as i8, 108 as i8, 100 as i8, 98 as i8,
            95 as i8, 118 as i8, 109 as i8, 101 as i8, 114 as i8, 103 as i8,
            101 as i8, 40 as i8, 105 as i8, 100 as i8, 44 as i8, 109 as i8,
            104 as i8, 97 as i8, 115 as i8, 104 as i8, 41 as i8, 59 as i8,
            10 as i8, 67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8,
            69 as i8, 32 as i8, 84 as i8, 82 as i8, 73 as i8, 71 as i8,
            71 as i8, 69 as i8, 82 as i8, 32 as i8, 108 as i8, 111 as i8,
            99 as i8, 97 as i8, 108 as i8, 100 as i8, 98 as i8, 95 as i8,
            118 as i8, 109 as i8, 101 as i8, 114 as i8, 103 as i8, 101 as i8,
            95 as i8, 99 as i8, 107 as i8, 49 as i8, 32 as i8, 65 as i8,
            70 as i8, 84 as i8, 69 as i8, 82 as i8, 32 as i8, 73 as i8,
            78 as i8, 83 as i8, 69 as i8, 82 as i8, 84 as i8, 32 as i8,
            79 as i8, 78 as i8, 32 as i8, 108 as i8, 111 as i8, 99 as i8,
            97 as i8, 108 as i8, 100 as i8, 98 as i8, 95 as i8, 118 as i8,
            109 as i8, 101 as i8, 114 as i8, 103 as i8, 101 as i8, 10 as i8,
            87 as i8, 72 as i8, 69 as i8, 78 as i8, 32 as i8, 110 as i8,
            101 as i8, 119 as i8, 46 as i8, 109 as i8, 104 as i8, 97 as i8,
            115 as i8, 104 as i8, 32 as i8, 73 as i8, 83 as i8, 32 as i8,
            78 as i8, 85 as i8, 76 as i8, 76 as i8, 32 as i8, 66 as i8,
            69 as i8, 71 as i8, 73 as i8, 78 as i8, 10 as i8, 32 as i8,
            32 as i8, 83 as i8, 69 as i8, 76 as i8, 69 as i8, 67 as i8,
            84 as i8, 32 as i8, 114 as i8, 97 as i8, 105 as i8, 115 as i8,
            101 as i8, 40 as i8, 70 as i8, 65 as i8, 73 as i8, 76 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 39 as i8, 116 as i8,
            114 as i8, 121 as i8, 105 as i8, 110 as i8, 103 as i8, 32 as i8,
            116 as i8, 111 as i8, 32 as i8, 117 as i8, 112 as i8, 100 as i8,
            97 as i8, 116 as i8, 101 as i8, 32 as i8, 97 as i8, 32 as i8,
            110 as i8, 101 as i8, 119 as i8, 101 as i8, 114 as i8, 32 as i8,
            99 as i8, 104 as i8, 101 as i8, 99 as i8, 107 as i8, 111 as i8,
            117 as i8, 116 as i8, 32 as i8, 119 as i8, 105 as i8, 116 as i8,
            104 as i8, 32 as i8, 97 as i8, 110 as i8, 32 as i8, 111 as i8,
            108 as i8, 100 as i8, 101 as i8, 114 as i8, 32 as i8, 118 as i8,
            101 as i8, 114 as i8, 115 as i8, 105 as i8, 111 as i8, 110 as i8,
            32 as i8, 111 as i8, 102 as i8, 32 as i8, 70 as i8, 111 as i8,
            115 as i8, 115 as i8, 105 as i8, 108 as i8, 39 as i8, 41 as i8,
            59 as i8, 10 as i8, 69 as i8, 78 as i8, 68 as i8, 59 as i8,
            10 as i8, 67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8,
            69 as i8, 32 as i8, 84 as i8, 65 as i8, 66 as i8, 76 as i8,
            69 as i8, 32 as i8, 99 as i8, 111 as i8, 110 as i8, 102 as i8,
            105 as i8, 103 as i8, 100 as i8, 98 as i8, 95 as i8, 103 as i8,
            108 as i8, 111 as i8, 98 as i8, 97 as i8, 108 as i8, 95 as i8,
            99 as i8, 111 as i8, 110 as i8, 102 as i8, 105 as i8, 103 as i8,
            40 as i8, 10 as i8, 32 as i8, 32 as i8, 110 as i8, 97 as i8,
            109 as i8, 101 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8,
            84 as i8, 32 as i8, 80 as i8, 82 as i8, 73 as i8, 77 as i8,
            65 as i8, 82 as i8, 89 as i8, 32 as i8, 75 as i8, 69 as i8,
            89 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8, 118 as i8,
            97 as i8, 108 as i8, 117 as i8, 101 as i8, 32 as i8, 84 as i8,
            69 as i8, 88 as i8, 84 as i8, 10 as i8, 41 as i8, 59 as i8,
            10 as i8, 67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8,
            69 as i8, 32 as i8, 84 as i8, 65 as i8, 66 as i8, 76 as i8,
            69 as i8, 32 as i8, 99 as i8, 111 as i8, 110 as i8, 102 as i8,
            105 as i8, 103 as i8, 100 as i8, 98 as i8, 95 as i8, 115 as i8,
            113 as i8, 108 as i8, 105 as i8, 116 as i8, 101 as i8, 95 as i8,
            115 as i8, 116 as i8, 97 as i8, 116 as i8, 49 as i8, 40 as i8,
            116 as i8, 98 as i8, 108 as i8, 44 as i8, 105 as i8, 100 as i8,
            120 as i8, 44 as i8, 115 as i8, 116 as i8, 97 as i8, 116 as i8,
            41 as i8, 59 as i8, 10 as i8, 0 as i8];

extern "C" fn hex_digit_value(c: i8) -> i32 {
    if c as i32 >= '0' as i32 && c as i32 <= '9' as i32 {
        return c as i32 - '0' as i32;
    }
    if c as i32 >= 'a' as i32 && c as i32 <= 'f' as i32 {
        return c as i32 - 'a' as i32 + 10;
    }
    if c as i32 >= 'A' as i32 && c as i32 <= 'F' as i32 {
        return c as i32 - 'A' as i32 + 10;
    }
    return -1;
}

extern "C" fn integer_value(mut z_arg_1: *const i8) -> i32 {
    unsafe {
        let mut v: Sqlite3Int64 = 0 as Sqlite3Int64;
        let mut i: i32 = 0;
        let mut is_neg: i32 = 0;
        if unsafe { *z_arg_1.offset(0 as isize) } as i32 == '-' as i32 {
            is_neg = 1;
            {
                let __p = &mut z_arg_1;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        } else if unsafe { *z_arg_1.offset(0 as isize) } as i32 == '+' as i32
            {
            {
                let __p = &mut z_arg_1;
                let __t = *__p;
                *__p = unsafe { (*__p).offset(1) };
                __t
            };
        }
        if unsafe { *z_arg_1.offset(0 as isize) } as i32 == '0' as i32 &&
                unsafe { *z_arg_1.offset(1 as isize) } as i32 == 'x' as i32 {
            let mut x: i32 = 0;
            {
                let __n = 2;
                let __p = &mut z_arg_1;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
            while {
                        x = hex_digit_value(unsafe { *z_arg_1.offset(0 as isize) });
                        x
                    } >= 0 {
                v = (v << 4) + x as Sqlite3Int64;
                {
                    let __p = &mut z_arg_1;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
        } else {
            while unsafe {
                        isdigit(unsafe { *z_arg_1.offset(0 as isize) } as i32)
                    } != 0 {
                v =
                    v * 10 as Sqlite3Int64 +
                            unsafe { *z_arg_1.offset(0 as isize) } as Sqlite3Int64 -
                        '0' as i32 as Sqlite3Int64;
                {
                    let __p = &mut z_arg_1;
                    let __t = *__p;
                    *__p = unsafe { (*__p).offset(1) };
                    __t
                };
            }
        }
        {
            i = 0;
            '__b2: loop {
                if !((i as u64) <
                                core::mem::size_of::<[IntegerValueS0N16integerValueS0; 9]>()
                                        as u64 / 16) {
                    break '__b2;
                }
                '__c2: loop {
                    if unsafe {
                                sqlite3_stricmp(a_mult[i as usize].z_suffix as *const i8,
                                    z_arg_1)
                            } == 0 {
                        v *= a_mult[i as usize].i_mult as Sqlite3Int64;
                        break '__b2;
                    }
                    break '__c2;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if v > 2147483647 as i64 {
            unsafe {
                printf(c"ERROR: parameter too large - max 2147483648\n".as_ptr()
                            as *mut i8 as *const i8)
            };
            unsafe { exit(1) };
        }
        return if is_neg != 0 { -v } else { v } as i32;
    }
}

extern "C" fn __main_inner(argc: i32, argv: *const *mut i8)
    -> Result<(), i32> {
    let mut z_cmd: *const i8 = core::ptr::null();
    let mut i: i32 = 0;
    let mut b_autovac: i32 = 0;
    let mut show_stats: i32 = 0;
    let mut z_db_name: *const i8 =
        c"./startup.db".as_ptr() as *mut i8 as *const i8;
    let mut n_heap: i32 = 0;
    let mut mn_heap: i32 = 0;
    {
        i = 1;
        '__b3: loop {
            if !(i < argc) { break '__b3; }
            '__c3: loop {
                let mut z: *const i8 =
                    unsafe { *argv.offset(i as isize) } as *const i8;
                if unsafe { *z.offset(0 as isize) } as i32 != '-' as i32 {
                    if !(z_cmd).is_null() {
                        usage(unsafe { *argv.offset(0 as isize) } as *const i8);
                    }
                    z_cmd = z;
                    break '__c3;
                }
                if unsafe { *z.offset(1 as isize) } as i32 == '-' as i32 {
                    {
                        let __p = &mut z;
                        let __t = *__p;
                        *__p = unsafe { (*__p).offset(1) };
                        __t
                    };
                }
                if unsafe {
                            strcmp(z, c"-autovacuum".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    b_autovac = 1;
                } else if unsafe {
                            strcmp(z, c"-dbname".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    if i == argc - 1 {
                        unsafe {
                            printf(c"ERROR: missing argument on \"%s\"\n".as_ptr() as
                                        *mut i8 as *const i8, unsafe { *argv.offset(0 as isize) })
                        };
                        unsafe { exit(1) };
                    }
                    z_db_name =
                        unsafe {
                                *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                            } as *const i8;
                } else if unsafe {
                            strcmp(z, c"-heap".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    if i >= argc - 2 {
                        unsafe {
                            printf(c"ERROR: missing arguments on %s\n".as_ptr() as
                                        *mut i8 as *const i8, unsafe { *argv.offset(i as isize) })
                        };
                        unsafe { exit(1) };
                    }
                    n_heap =
                        integer_value(unsafe { *argv.offset((i + 1) as isize) } as
                                *const i8);
                    mn_heap =
                        integer_value(unsafe { *argv.offset((i + 2) as isize) } as
                                *const i8);
                    i += 2;
                } else if unsafe {
                            strcmp(z, c"-stats".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    show_stats = 1;
                } else {
                    unsafe {
                        printf(c"ERROR: unknown option \"%s\"\n".as_ptr() as *mut i8
                                as *const i8, unsafe { *argv.offset(i as isize) })
                    };
                    usage(unsafe { *argv.offset(0 as isize) } as *const i8);
                }
                break '__c3;
            }
            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
        }
    }
    if z_cmd == core::ptr::null() {
        unsafe {
            printf(c"ERROR: no COMMAND specified\n".as_ptr() as *mut i8 as
                    *const i8)
        };
        usage(unsafe { *argv.offset(0 as isize) } as *const i8);
    }
    if unsafe { strcmp(z_cmd, c"run".as_ptr() as *mut i8 as *const i8) } == 0
        {
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        let mut rc: i32 = 0;
        let mut z_err: *mut i8 = core::ptr::null_mut();
        let mut p_heap: *mut () = core::ptr::null_mut();
        if n_heap > 0 {
            p_heap = unsafe { malloc(n_heap as u64) };
            if p_heap == core::ptr::null_mut() {
                unsafe {
                    printf(c"ERROR: cannot allocate %d-byte heap\n".as_ptr() as
                                *mut i8 as *const i8, n_heap)
                };
                unsafe { exit(1) };
            }
            rc = unsafe { sqlite3_config(8, p_heap, n_heap, mn_heap) };
            if rc != 0 {
                unsafe {
                    printf(c"ERROR: heap configuration failed: %d\n".as_ptr() as
                                *mut i8 as *const i8, rc)
                };
                unsafe { exit(1) };
            }
        }
        rc = unsafe { sqlite3_open(z_db_name, &mut db) };
        if rc != 0 {
            unsafe {
                printf(c"SQLite error: %s\n".as_ptr() as *mut i8 as *const i8,
                    unsafe { sqlite3_errmsg(db) })
            };
        } else {
            unsafe {
                sqlite3_exec(db,
                    c"PRAGMA synchronous".as_ptr() as *mut i8 as *const i8,
                    None, core::ptr::null_mut(), &mut z_err)
            };
        }
        if !(z_err).is_null() {
            unsafe {
                printf(c"ERROR: %s\n".as_ptr() as *mut i8 as *const i8, z_err)
            };
            unsafe { sqlite3_free(z_err as *mut ()) };
        }
        if show_stats != 0 {
            let mut i_cur: i32 = 0;
            let mut i_hi: i32 = 0;
            unsafe { sqlite3_db_status(db, 0, &mut i_cur, &mut i_hi, 0) };
            unsafe {
                printf(c"-- Lookaside Slots Used:        %d (max %d)\n".as_ptr()
                            as *mut i8 as *const i8, i_cur, i_hi)
            };
            unsafe { sqlite3_db_status(db, 4, &mut i_cur, &mut i_hi, 0) };
            unsafe {
                printf(c"-- Successful lookasides:       %d\n".as_ptr() as
                            *mut i8 as *const i8, i_hi)
            };
            unsafe { sqlite3_db_status(db, 5, &mut i_cur, &mut i_hi, 0) };
            unsafe {
                printf(c"-- Lookaside size faults:       %d\n".as_ptr() as
                            *mut i8 as *const i8, i_hi)
            };
            unsafe { sqlite3_db_status(db, 6, &mut i_cur, &mut i_hi, 0) };
            unsafe {
                printf(c"-- Lookaside OOM faults:        %d\n".as_ptr() as
                            *mut i8 as *const i8, i_hi)
            };
            unsafe { sqlite3_db_status(db, 1, &mut i_cur, &mut i_hi, 0) };
            unsafe {
                printf(c"-- Pager Heap Usage:            %d bytes\n".as_ptr()
                            as *mut i8 as *const i8, i_cur)
            };
            unsafe { sqlite3_db_status(db, 7, &mut i_cur, &mut i_hi, 1) };
            unsafe {
                printf(c"-- Page cache hits:             %d\n".as_ptr() as
                            *mut i8 as *const i8, i_cur)
            };
            unsafe { sqlite3_db_status(db, 8, &mut i_cur, &mut i_hi, 1) };
            unsafe {
                printf(c"-- Page cache misses:           %d\n".as_ptr() as
                            *mut i8 as *const i8, i_cur)
            };
            unsafe { sqlite3_db_status(db, 9, &mut i_cur, &mut i_hi, 1) };
            unsafe {
                printf(c"-- Page cache writes:           %d\n".as_ptr() as
                            *mut i8 as *const i8, i_cur)
            };
            unsafe { sqlite3_db_status(db, 2, &mut i_cur, &mut i_hi, 0) };
            unsafe {
                printf(c"-- Schema Heap Usage:           %d bytes\n".as_ptr()
                            as *mut i8 as *const i8, i_cur)
            };
            unsafe { sqlite3_db_status(db, 3, &mut i_cur, &mut i_hi, 0) };
            unsafe {
                printf(c"-- Statement Heap Usage:        %d bytes\n".as_ptr()
                            as *mut i8 as *const i8, i_cur)
            };
        }
        unsafe { sqlite3_close(db) };
        unsafe { free(p_heap) };
        if show_stats != 0 {
            let mut i_cur_1: i32 = 0;
            let mut i_hi_1: i32 = 0;
            unsafe { sqlite3_status(0, &mut i_cur_1, &mut i_hi_1, 0) };
            unsafe {
                printf(c"-- Memory Used (bytes):         %d (max %d)\n".as_ptr()
                            as *mut i8 as *const i8, i_cur_1, i_hi_1)
            };
            unsafe { sqlite3_status(9, &mut i_cur_1, &mut i_hi_1, 0) };
            unsafe {
                printf(c"-- Outstanding Allocations:     %d (max %d)\n".as_ptr()
                            as *mut i8 as *const i8, i_cur_1, i_hi_1)
            };
            unsafe { sqlite3_status(2, &mut i_cur_1, &mut i_hi_1, 0) };
            unsafe {
                printf(c"-- Pcache Overflow Bytes:       %d (max %d)\n".as_ptr()
                            as *mut i8 as *const i8, i_cur_1, i_hi_1)
            };
            unsafe { sqlite3_status(5, &mut i_cur_1, &mut i_hi_1, 0) };
            unsafe {
                printf(c"-- Largest Allocation:          %d bytes\n".as_ptr()
                            as *mut i8 as *const i8, i_hi_1)
            };
            unsafe { sqlite3_status(7, &mut i_cur_1, &mut i_hi_1, 0) };
            unsafe {
                printf(c"-- Largest Pcache Allocation:   %d bytes\n".as_ptr()
                            as *mut i8 as *const i8, i_hi_1)
            };
        }
        return Ok(());
    }
    if unsafe { strcmp(z_cmd, c"init".as_ptr() as *mut i8 as *const i8) } == 0
        {
        let mut db: *mut Sqlite3 = core::ptr::null_mut();
        let mut z_aux: *mut i8 = core::ptr::null_mut();
        let mut z_err_1: *mut i8 = core::ptr::null_mut();
        let mut rc: i32 = 0;
        unsafe { unlink(z_db_name) };
        z_aux =
            unsafe {
                sqlite3_mprintf(c"%s-journal".as_ptr() as *mut i8 as
                        *const i8, z_db_name)
            };
        unsafe { unlink(z_aux as *const i8) };
        unsafe { sqlite3_free(z_aux as *mut ()) };
        z_aux =
            unsafe {
                sqlite3_mprintf(c"%s-wal".as_ptr() as *mut i8 as *const i8,
                    z_db_name)
            };
        unsafe { unlink(z_aux as *const i8) };
        unsafe { sqlite3_free(z_aux as *mut ()) };
        rc = unsafe { sqlite3_open(z_db_name, &mut db) };
        if rc != 0 {
            unsafe {
                printf(c"SQLite error: %s\n".as_ptr() as *mut i8 as *const i8,
                    unsafe { sqlite3_errmsg(db) })
            };
        } else {
            unsafe {
                sqlite3_exec(db, c"BEGIN".as_ptr() as *mut i8 as *const i8,
                    None, core::ptr::null_mut(), core::ptr::null_mut())
            };
            unsafe {
                sqlite3_exec(db,
                    &raw const z_test_schema[0 as usize] as *const i8, None,
                    core::ptr::null_mut(), &mut z_err_1)
            };
            unsafe {
                sqlite3_exec(db, c"COMMIT".as_ptr() as *mut i8 as *const i8,
                    None, core::ptr::null_mut(), core::ptr::null_mut())
            };
        }
        if !(z_err_1).is_null() {
            unsafe {
                printf(c"ERROR: %s\n".as_ptr() as *mut i8 as *const i8,
                    z_err_1)
            };
            unsafe { sqlite3_free(z_err_1 as *mut ()) };
        }
        unsafe { sqlite3_close(db) };
        return Ok(());
    }
    return Ok(());
}

#[repr(C)]
#[derive(Copy, Clone)]
struct IntegerValueS0N16integerValueS0 {
    z_suffix: *mut i8,
    i_mult: i32,
}

static mut a_mult: [IntegerValueS0N16integerValueS0; 9] =
    [IntegerValueS0N16integerValueS0 {
                z_suffix: c"KiB".as_ptr() as *mut i8,
                i_mult: 1024,
            },
            IntegerValueS0N16integerValueS0 {
                z_suffix: c"MiB".as_ptr() as *mut i8,
                i_mult: 1024 * 1024,
            },
            IntegerValueS0N16integerValueS0 {
                z_suffix: c"GiB".as_ptr() as *mut i8,
                i_mult: 1024 * 1024 * 1024,
            },
            IntegerValueS0N16integerValueS0 {
                z_suffix: c"KB".as_ptr() as *mut i8,
                i_mult: 1000,
            },
            IntegerValueS0N16integerValueS0 {
                z_suffix: c"MB".as_ptr() as *mut i8,
                i_mult: 1000000,
            },
            IntegerValueS0N16integerValueS0 {
                z_suffix: c"GB".as_ptr() as *mut i8,
                i_mult: 1000000000,
            },
            IntegerValueS0N16integerValueS0 {
                z_suffix: c"K".as_ptr() as *mut i8,
                i_mult: 1000,
            },
            IntegerValueS0N16integerValueS0 {
                z_suffix: c"M".as_ptr() as *mut i8,
                i_mult: 1000000,
            },
            IntegerValueS0N16integerValueS0 {
                z_suffix: c"G".as_ptr() as *mut i8,
                i_mult: 1000000000,
            }];

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
    fn printf(_: *const i8, ...)
    -> i32;
    fn exit(_: i32)
    -> ();
    fn isdigit(_c: i32)
    -> i32;
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn malloc(__size: u64)
    -> *mut ();
    fn free(_: *mut ())
    -> ();
    fn unlink(_: *const i8)
    -> i32;
}
