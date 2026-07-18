#![feature(c_variadic)]
#![allow(unused_imports, dead_code)]
mod sqlite3_h;
pub(crate) use crate::sqlite3_h::*;
type DarwinSizeT = u64;
static z_help: [i8; 2897] =
    [85 as i8, 115 as i8, 97 as i8, 103 as i8, 101 as i8, 58 as i8, 32 as i8,
            37 as i8, 115 as i8, 32 as i8, 91 as i8, 45 as i8, 45 as i8,
            111 as i8, 112 as i8, 116 as i8, 105 as i8, 111 as i8, 110 as i8,
            115 as i8, 93 as i8, 32 as i8, 68 as i8, 65 as i8, 84 as i8,
            65 as i8, 66 as i8, 65 as i8, 83 as i8, 69 as i8, 10 as i8,
            79 as i8, 112 as i8, 116 as i8, 105 as i8, 111 as i8, 110 as i8,
            115 as i8, 58 as i8, 10 as i8, 32 as i8, 32 as i8, 45 as i8,
            45 as i8, 97 as i8, 117 as i8, 116 as i8, 111 as i8, 118 as i8,
            97 as i8, 99 as i8, 117 as i8, 117 as i8, 109 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 69 as i8, 110 as i8, 97 as i8, 98 as i8, 108 as i8,
            101 as i8, 32 as i8, 65 as i8, 85 as i8, 84 as i8, 79 as i8,
            86 as i8, 65 as i8, 67 as i8, 85 as i8, 85 as i8, 77 as i8,
            32 as i8, 109 as i8, 111 as i8, 100 as i8, 101 as i8, 10 as i8,
            32 as i8, 32 as i8, 45 as i8, 45 as i8, 98 as i8, 105 as i8,
            103 as i8, 45 as i8, 116 as i8, 114 as i8, 97 as i8, 110 as i8,
            115 as i8, 97 as i8, 99 as i8, 116 as i8, 105 as i8, 111 as i8,
            110 as i8, 115 as i8, 32 as i8, 32 as i8, 65 as i8, 100 as i8,
            100 as i8, 32 as i8, 66 as i8, 69 as i8, 71 as i8, 73 as i8,
            78 as i8, 47 as i8, 69 as i8, 78 as i8, 68 as i8, 32 as i8,
            97 as i8, 114 as i8, 111 as i8, 117 as i8, 110 as i8, 100 as i8,
            32 as i8, 97 as i8, 108 as i8, 108 as i8, 32 as i8, 108 as i8,
            97 as i8, 114 as i8, 103 as i8, 101 as i8, 32 as i8, 116 as i8,
            101 as i8, 115 as i8, 116 as i8, 115 as i8, 10 as i8, 32 as i8,
            32 as i8, 45 as i8, 45 as i8, 99 as i8, 97 as i8, 99 as i8,
            104 as i8, 101 as i8, 115 as i8, 105 as i8, 122 as i8, 101 as i8,
            32 as i8, 78 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 83 as i8, 101 as i8, 116 as i8,
            32 as i8, 80 as i8, 82 as i8, 65 as i8, 71 as i8, 77 as i8,
            65 as i8, 32 as i8, 99 as i8, 97 as i8, 99 as i8, 104 as i8,
            101 as i8, 95 as i8, 115 as i8, 105 as i8, 122 as i8, 101 as i8,
            61 as i8, 78 as i8, 46 as i8, 32 as i8, 78 as i8, 111 as i8,
            116 as i8, 101 as i8, 58 as i8, 32 as i8, 78 as i8, 32 as i8,
            105 as i8, 115 as i8, 32 as i8, 112 as i8, 97 as i8, 103 as i8,
            101 as i8, 115 as i8, 44 as i8, 32 as i8, 110 as i8, 111 as i8,
            116 as i8, 32 as i8, 98 as i8, 121 as i8, 116 as i8, 101 as i8,
            115 as i8, 10 as i8, 32 as i8, 32 as i8, 45 as i8, 45 as i8,
            99 as i8, 104 as i8, 101 as i8, 99 as i8, 107 as i8, 112 as i8,
            111 as i8, 105 as i8, 110 as i8, 116 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            82 as i8, 117 as i8, 110 as i8, 32 as i8, 80 as i8, 82 as i8,
            65 as i8, 71 as i8, 77 as i8, 65 as i8, 32 as i8, 119 as i8,
            97 as i8, 108 as i8, 95 as i8, 99 as i8, 104 as i8, 101 as i8,
            99 as i8, 107 as i8, 112 as i8, 111 as i8, 105 as i8, 110 as i8,
            116 as i8, 32 as i8, 97 as i8, 102 as i8, 116 as i8, 101 as i8,
            114 as i8, 32 as i8, 101 as i8, 97 as i8, 99 as i8, 104 as i8,
            32 as i8, 116 as i8, 101 as i8, 115 as i8, 116 as i8, 32 as i8,
            99 as i8, 97 as i8, 115 as i8, 101 as i8, 10 as i8, 32 as i8,
            32 as i8, 45 as i8, 45 as i8, 101 as i8, 120 as i8, 99 as i8,
            108 as i8, 117 as i8, 115 as i8, 105 as i8, 118 as i8, 101 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 69 as i8, 110 as i8, 97 as i8,
            98 as i8, 108 as i8, 101 as i8, 32 as i8, 108 as i8, 111 as i8,
            99 as i8, 107 as i8, 105 as i8, 110 as i8, 103 as i8, 95 as i8,
            109 as i8, 111 as i8, 100 as i8, 101 as i8, 61 as i8, 69 as i8,
            88 as i8, 67 as i8, 76 as i8, 85 as i8, 83 as i8, 73 as i8,
            86 as i8, 69 as i8, 10 as i8, 32 as i8, 32 as i8, 45 as i8,
            45 as i8, 101 as i8, 120 as i8, 112 as i8, 108 as i8, 97 as i8,
            105 as i8, 110 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 76 as i8, 105 as i8, 107 as i8, 101 as i8, 32 as i8,
            45 as i8, 45 as i8, 115 as i8, 113 as i8, 108 as i8, 111 as i8,
            110 as i8, 108 as i8, 121 as i8, 32 as i8, 98 as i8, 117 as i8,
            116 as i8, 32 as i8, 119 as i8, 105 as i8, 116 as i8, 104 as i8,
            32 as i8, 97 as i8, 100 as i8, 100 as i8, 101 as i8, 100 as i8,
            32 as i8, 69 as i8, 88 as i8, 80 as i8, 76 as i8, 65 as i8,
            73 as i8, 78 as i8, 32 as i8, 107 as i8, 101 as i8, 121 as i8,
            119 as i8, 111 as i8, 114 as i8, 100 as i8, 115 as i8, 10 as i8,
            32 as i8, 32 as i8, 45 as i8, 45 as i8, 102 as i8, 117 as i8,
            108 as i8, 108 as i8, 102 as i8, 115 as i8, 121 as i8, 110 as i8,
            99 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 69 as i8, 110 as i8,
            97 as i8, 98 as i8, 108 as i8, 101 as i8, 32 as i8, 102 as i8,
            117 as i8, 108 as i8, 108 as i8, 102 as i8, 115 as i8, 121 as i8,
            110 as i8, 99 as i8, 61 as i8, 84 as i8, 82 as i8, 85 as i8,
            69 as i8, 10 as i8, 32 as i8, 32 as i8, 45 as i8, 45 as i8,
            104 as i8, 97 as i8, 114 as i8, 100 as i8, 45 as i8, 104 as i8,
            101 as i8, 97 as i8, 112 as i8, 45 as i8, 108 as i8, 105 as i8,
            109 as i8, 105 as i8, 116 as i8, 32 as i8, 78 as i8, 32 as i8,
            84 as i8, 104 as i8, 101 as i8, 32 as i8, 104 as i8, 97 as i8,
            114 as i8, 100 as i8, 32 as i8, 108 as i8, 105 as i8, 109 as i8,
            105 as i8, 116 as i8, 32 as i8, 111 as i8, 110 as i8, 32 as i8,
            116 as i8, 104 as i8, 101 as i8, 32 as i8, 109 as i8, 97 as i8,
            120 as i8, 105 as i8, 109 as i8, 117 as i8, 109 as i8, 32 as i8,
            104 as i8, 101 as i8, 97 as i8, 112 as i8, 32 as i8, 115 as i8,
            105 as i8, 122 as i8, 101 as i8, 10 as i8, 32 as i8, 32 as i8,
            45 as i8, 45 as i8, 104 as i8, 101 as i8, 97 as i8, 112 as i8,
            32 as i8, 83 as i8, 90 as i8, 32 as i8, 77 as i8, 73 as i8,
            78 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 77 as i8, 101 as i8, 109 as i8, 111 as i8,
            114 as i8, 121 as i8, 32 as i8, 97 as i8, 108 as i8, 108 as i8,
            111 as i8, 99 as i8, 97 as i8, 116 as i8, 111 as i8, 114 as i8,
            32 as i8, 117 as i8, 115 as i8, 101 as i8, 115 as i8, 32 as i8,
            83 as i8, 90 as i8, 32 as i8, 98 as i8, 121 as i8, 116 as i8,
            101 as i8, 115 as i8, 32 as i8, 38 as i8, 32 as i8, 109 as i8,
            105 as i8, 110 as i8, 32 as i8, 97 as i8, 108 as i8, 108 as i8,
            111 as i8, 99 as i8, 97 as i8, 116 as i8, 105 as i8, 111 as i8,
            110 as i8, 32 as i8, 77 as i8, 73 as i8, 78 as i8, 10 as i8,
            32 as i8, 32 as i8, 45 as i8, 45 as i8, 105 as i8, 110 as i8,
            99 as i8, 114 as i8, 118 as i8, 97 as i8, 99 as i8, 117 as i8,
            117 as i8, 109 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 69 as i8, 110 as i8,
            97 as i8, 98 as i8, 108 as i8, 101 as i8, 32 as i8, 105 as i8,
            110 as i8, 99 as i8, 114 as i8, 101 as i8, 109 as i8, 101 as i8,
            110 as i8, 97 as i8, 116 as i8, 97 as i8, 108 as i8, 32 as i8,
            118 as i8, 97 as i8, 99 as i8, 117 as i8, 117 as i8, 109 as i8,
            32 as i8, 109 as i8, 111 as i8, 100 as i8, 101 as i8, 10 as i8,
            32 as i8, 32 as i8, 45 as i8, 45 as i8, 106 as i8, 111 as i8,
            117 as i8, 114 as i8, 110 as i8, 97 as i8, 108 as i8, 32 as i8,
            77 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 83 as i8, 101 as i8,
            116 as i8, 32 as i8, 116 as i8, 104 as i8, 101 as i8, 32 as i8,
            106 as i8, 111 as i8, 117 as i8, 114 as i8, 110 as i8, 97 as i8,
            108 as i8, 95 as i8, 109 as i8, 111 as i8, 100 as i8, 101 as i8,
            32 as i8, 116 as i8, 111 as i8, 32 as i8, 77 as i8, 10 as i8,
            32 as i8, 32 as i8, 45 as i8, 45 as i8, 107 as i8, 101 as i8,
            121 as i8, 32 as i8, 75 as i8, 69 as i8, 89 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 83 as i8, 101 as i8,
            116 as i8, 32 as i8, 116 as i8, 104 as i8, 101 as i8, 32 as i8,
            101 as i8, 110 as i8, 99 as i8, 114 as i8, 121 as i8, 112 as i8,
            116 as i8, 105 as i8, 111 as i8, 110 as i8, 32 as i8, 107 as i8,
            101 as i8, 121 as i8, 32 as i8, 116 as i8, 111 as i8, 32 as i8,
            75 as i8, 69 as i8, 89 as i8, 10 as i8, 32 as i8, 32 as i8,
            45 as i8, 45 as i8, 108 as i8, 111 as i8, 111 as i8, 107 as i8,
            97 as i8, 115 as i8, 105 as i8, 100 as i8, 101 as i8, 32 as i8,
            78 as i8, 32 as i8, 83 as i8, 90 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 67 as i8, 111 as i8, 110 as i8, 102 as i8,
            105 as i8, 103 as i8, 117 as i8, 114 as i8, 101 as i8, 32 as i8,
            108 as i8, 111 as i8, 111 as i8, 107 as i8, 97 as i8, 115 as i8,
            105 as i8, 100 as i8, 101 as i8, 32 as i8, 102 as i8, 111 as i8,
            114 as i8, 32 as i8, 78 as i8, 32 as i8, 115 as i8, 108 as i8,
            111 as i8, 116 as i8, 115 as i8, 32 as i8, 111 as i8, 102 as i8,
            32 as i8, 83 as i8, 90 as i8, 32 as i8, 98 as i8, 121 as i8,
            116 as i8, 101 as i8, 115 as i8, 32 as i8, 101 as i8, 97 as i8,
            99 as i8, 104 as i8, 10 as i8, 32 as i8, 32 as i8, 45 as i8,
            45 as i8, 109 as i8, 101 as i8, 109 as i8, 100 as i8, 98 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 85 as i8, 115 as i8, 101 as i8, 32 as i8, 97 as i8,
            110 as i8, 32 as i8, 105 as i8, 110 as i8, 45 as i8, 109 as i8,
            101 as i8, 109 as i8, 111 as i8, 114 as i8, 121 as i8, 32 as i8,
            100 as i8, 97 as i8, 116 as i8, 97 as i8, 98 as i8, 97 as i8,
            115 as i8, 101 as i8, 10 as i8, 32 as i8, 32 as i8, 45 as i8,
            45 as i8, 109 as i8, 109 as i8, 97 as i8, 112 as i8, 32 as i8,
            83 as i8, 90 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 77 as i8, 77 as i8, 65 as i8, 80 as i8, 32 as i8,
            116 as i8, 104 as i8, 101 as i8, 32 as i8, 102 as i8, 105 as i8,
            114 as i8, 115 as i8, 116 as i8, 32 as i8, 83 as i8, 90 as i8,
            32 as i8, 98 as i8, 121 as i8, 116 as i8, 101 as i8, 115 as i8,
            32 as i8, 111 as i8, 102 as i8, 32 as i8, 116 as i8, 104 as i8,
            101 as i8, 32 as i8, 100 as i8, 97 as i8, 116 as i8, 97 as i8,
            98 as i8, 97 as i8, 115 as i8, 101 as i8, 32 as i8, 102 as i8,
            105 as i8, 108 as i8, 101 as i8, 10 as i8, 32 as i8, 32 as i8,
            45 as i8, 45 as i8, 109 as i8, 117 as i8, 108 as i8, 116 as i8,
            105 as i8, 116 as i8, 104 as i8, 114 as i8, 101 as i8, 97 as i8,
            100 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 83 as i8, 101 as i8, 116 as i8, 32 as i8,
            109 as i8, 117 as i8, 108 as i8, 116 as i8, 105 as i8, 116 as i8,
            104 as i8, 114 as i8, 101 as i8, 97 as i8, 100 as i8, 101 as i8,
            100 as i8, 32 as i8, 109 as i8, 111 as i8, 100 as i8, 101 as i8,
            10 as i8, 32 as i8, 32 as i8, 45 as i8, 45 as i8, 110 as i8,
            111 as i8, 109 as i8, 101 as i8, 109 as i8, 115 as i8, 116 as i8,
            97 as i8, 116 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 68 as i8,
            105 as i8, 115 as i8, 97 as i8, 98 as i8, 108 as i8, 101 as i8,
            32 as i8, 109 as i8, 101 as i8, 109 as i8, 111 as i8, 114 as i8,
            121 as i8, 32 as i8, 115 as i8, 116 as i8, 97 as i8, 116 as i8,
            105 as i8, 115 as i8, 116 as i8, 105 as i8, 99 as i8, 115 as i8,
            10 as i8, 32 as i8, 32 as i8, 45 as i8, 45 as i8, 110 as i8,
            111 as i8, 109 as i8, 117 as i8, 116 as i8, 101 as i8, 120 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 79 as i8,
            112 as i8, 101 as i8, 110 as i8, 32 as i8, 100 as i8, 98 as i8,
            32 as i8, 119 as i8, 105 as i8, 116 as i8, 104 as i8, 32 as i8,
            83 as i8, 81 as i8, 76 as i8, 73 as i8, 84 as i8, 69 as i8,
            95 as i8, 79 as i8, 80 as i8, 69 as i8, 78 as i8, 95 as i8,
            78 as i8, 79 as i8, 77 as i8, 85 as i8, 84 as i8, 69 as i8,
            88 as i8, 10 as i8, 32 as i8, 32 as i8, 45 as i8, 45 as i8,
            110 as i8, 111 as i8, 115 as i8, 121 as i8, 110 as i8, 99 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            83 as i8, 101 as i8, 116 as i8, 32 as i8, 80 as i8, 82 as i8,
            65 as i8, 71 as i8, 77 as i8, 65 as i8, 32 as i8, 115 as i8,
            121 as i8, 110 as i8, 99 as i8, 104 as i8, 114 as i8, 111 as i8,
            110 as i8, 111 as i8, 117 as i8, 115 as i8, 61 as i8, 79 as i8,
            70 as i8, 70 as i8, 10 as i8, 32 as i8, 32 as i8, 45 as i8,
            45 as i8, 110 as i8, 111 as i8, 116 as i8, 110 as i8, 117 as i8,
            108 as i8, 108 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 65 as i8, 100 as i8, 100 as i8, 32 as i8, 78 as i8,
            79 as i8, 84 as i8, 32 as i8, 78 as i8, 85 as i8, 76 as i8,
            76 as i8, 32 as i8, 99 as i8, 111 as i8, 110 as i8, 115 as i8,
            116 as i8, 114 as i8, 97 as i8, 105 as i8, 110 as i8, 116 as i8,
            115 as i8, 32 as i8, 116 as i8, 111 as i8, 32 as i8, 116 as i8,
            97 as i8, 98 as i8, 108 as i8, 101 as i8, 32 as i8, 99 as i8,
            111 as i8, 108 as i8, 117 as i8, 109 as i8, 110 as i8, 115 as i8,
            10 as i8, 32 as i8, 32 as i8, 45 as i8, 45 as i8, 111 as i8,
            117 as i8, 116 as i8, 112 as i8, 117 as i8, 116 as i8, 32 as i8,
            70 as i8, 73 as i8, 76 as i8, 69 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 83 as i8,
            116 as i8, 111 as i8, 114 as i8, 101 as i8, 32 as i8, 83 as i8,
            81 as i8, 76 as i8, 32 as i8, 111 as i8, 117 as i8, 116 as i8,
            112 as i8, 117 as i8, 116 as i8, 32 as i8, 105 as i8, 110 as i8,
            32 as i8, 70 as i8, 73 as i8, 76 as i8, 69 as i8, 10 as i8,
            32 as i8, 32 as i8, 45 as i8, 45 as i8, 112 as i8, 97 as i8,
            103 as i8, 101 as i8, 115 as i8, 105 as i8, 122 as i8, 101 as i8,
            32 as i8, 78 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 83 as i8, 101 as i8,
            116 as i8, 32 as i8, 116 as i8, 104 as i8, 101 as i8, 32 as i8,
            112 as i8, 97 as i8, 103 as i8, 101 as i8, 32 as i8, 115 as i8,
            105 as i8, 122 as i8, 101 as i8, 32 as i8, 116 as i8, 111 as i8,
            32 as i8, 78 as i8, 10 as i8, 32 as i8, 32 as i8, 45 as i8,
            45 as i8, 112 as i8, 99 as i8, 97 as i8, 99 as i8, 104 as i8,
            101 as i8, 32 as i8, 78 as i8, 32 as i8, 83 as i8, 90 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 67 as i8, 111 as i8, 110 as i8, 102 as i8, 105 as i8,
            103 as i8, 117 as i8, 114 as i8, 101 as i8, 32 as i8, 78 as i8,
            32 as i8, 112 as i8, 97 as i8, 103 as i8, 101 as i8, 115 as i8,
            32 as i8, 111 as i8, 102 as i8, 32 as i8, 112 as i8, 97 as i8,
            103 as i8, 101 as i8, 99 as i8, 97 as i8, 99 as i8, 104 as i8,
            101 as i8, 32 as i8, 101 as i8, 97 as i8, 99 as i8, 104 as i8,
            32 as i8, 111 as i8, 102 as i8, 32 as i8, 115 as i8, 105 as i8,
            122 as i8, 101 as i8, 32 as i8, 83 as i8, 90 as i8, 32 as i8,
            98 as i8, 121 as i8, 116 as i8, 101 as i8, 115 as i8, 10 as i8,
            32 as i8, 32 as i8, 45 as i8, 45 as i8, 112 as i8, 114 as i8,
            105 as i8, 109 as i8, 97 as i8, 114 as i8, 121 as i8, 107 as i8,
            101 as i8, 121 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 85 as i8, 115 as i8,
            101 as i8, 32 as i8, 80 as i8, 82 as i8, 73 as i8, 77 as i8,
            65 as i8, 82 as i8, 89 as i8, 32 as i8, 75 as i8, 69 as i8,
            89 as i8, 32 as i8, 105 as i8, 110 as i8, 115 as i8, 116 as i8,
            101 as i8, 97 as i8, 100 as i8, 32 as i8, 111 as i8, 102 as i8,
            32 as i8, 85 as i8, 78 as i8, 73 as i8, 81 as i8, 85 as i8,
            69 as i8, 32 as i8, 119 as i8, 104 as i8, 101 as i8, 114 as i8,
            101 as i8, 32 as i8, 97 as i8, 112 as i8, 112 as i8, 114 as i8,
            111 as i8, 112 as i8, 114 as i8, 105 as i8, 97 as i8, 116 as i8,
            101 as i8, 10 as i8, 32 as i8, 32 as i8, 45 as i8, 45 as i8,
            114 as i8, 101 as i8, 112 as i8, 101 as i8, 97 as i8, 116 as i8,
            32 as i8, 78 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            82 as i8, 101 as i8, 112 as i8, 101 as i8, 97 as i8, 116 as i8,
            32 as i8, 101 as i8, 97 as i8, 99 as i8, 104 as i8, 32 as i8,
            83 as i8, 69 as i8, 76 as i8, 69 as i8, 67 as i8, 84 as i8,
            32 as i8, 78 as i8, 32 as i8, 116 as i8, 105 as i8, 109 as i8,
            101 as i8, 115 as i8, 32 as i8, 40 as i8, 100 as i8, 101 as i8,
            102 as i8, 97 as i8, 117 as i8, 108 as i8, 116 as i8, 58 as i8,
            32 as i8, 49 as i8, 41 as i8, 10 as i8, 32 as i8, 32 as i8,
            45 as i8, 45 as i8, 114 as i8, 101 as i8, 112 as i8, 114 as i8,
            101 as i8, 112 as i8, 97 as i8, 114 as i8, 101 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 82 as i8, 101 as i8, 112 as i8, 114 as i8,
            101 as i8, 112 as i8, 97 as i8, 114 as i8, 101 as i8, 32 as i8,
            101 as i8, 97 as i8, 99 as i8, 104 as i8, 32 as i8, 115 as i8,
            116 as i8, 97 as i8, 116 as i8, 101 as i8, 109 as i8, 101 as i8,
            110 as i8, 116 as i8, 32 as i8, 117 as i8, 112 as i8, 111 as i8,
            110 as i8, 32 as i8, 101 as i8, 118 as i8, 101 as i8, 114 as i8,
            121 as i8, 32 as i8, 105 as i8, 110 as i8, 118 as i8, 111 as i8,
            99 as i8, 97 as i8, 116 as i8, 105 as i8, 111 as i8, 110 as i8,
            10 as i8, 32 as i8, 32 as i8, 45 as i8, 45 as i8, 114 as i8,
            101 as i8, 115 as i8, 101 as i8, 114 as i8, 118 as i8, 101 as i8,
            32 as i8, 78 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 82 as i8,
            101 as i8, 115 as i8, 101 as i8, 114 as i8, 118 as i8, 101 as i8,
            32 as i8, 78 as i8, 32 as i8, 98 as i8, 121 as i8, 116 as i8,
            101 as i8, 115 as i8, 32 as i8, 111 as i8, 110 as i8, 32 as i8,
            101 as i8, 97 as i8, 99 as i8, 104 as i8, 32 as i8, 100 as i8,
            97 as i8, 116 as i8, 97 as i8, 98 as i8, 97 as i8, 115 as i8,
            101 as i8, 32 as i8, 112 as i8, 97 as i8, 103 as i8, 101 as i8,
            10 as i8, 32 as i8, 32 as i8, 45 as i8, 45 as i8, 115 as i8,
            99 as i8, 114 as i8, 105 as i8, 112 as i8, 116 as i8, 32 as i8,
            70 as i8, 73 as i8, 76 as i8, 69 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 87 as i8,
            114 as i8, 105 as i8, 116 as i8, 101 as i8, 32 as i8, 97 as i8,
            110 as i8, 32 as i8, 83 as i8, 81 as i8, 76 as i8, 32 as i8,
            115 as i8, 99 as i8, 114 as i8, 105 as i8, 112 as i8, 116 as i8,
            32 as i8, 102 as i8, 111 as i8, 114 as i8, 32 as i8, 116 as i8,
            104 as i8, 101 as i8, 32 as i8, 116 as i8, 101 as i8, 115 as i8,
            116 as i8, 32 as i8, 105 as i8, 110 as i8, 116 as i8, 111 as i8,
            32 as i8, 70 as i8, 73 as i8, 76 as i8, 69 as i8, 10 as i8,
            32 as i8, 32 as i8, 45 as i8, 45 as i8, 115 as i8, 101 as i8,
            114 as i8, 105 as i8, 97 as i8, 108 as i8, 105 as i8, 122 as i8,
            101 as i8, 100 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 83 as i8, 101 as i8,
            116 as i8, 32 as i8, 115 as i8, 101 as i8, 114 as i8, 105 as i8,
            97 as i8, 108 as i8, 105 as i8, 122 as i8, 101 as i8, 100 as i8,
            32 as i8, 116 as i8, 104 as i8, 114 as i8, 101 as i8, 97 as i8,
            100 as i8, 105 as i8, 110 as i8, 103 as i8, 32 as i8, 109 as i8,
            111 as i8, 100 as i8, 101 as i8, 10 as i8, 32 as i8, 32 as i8,
            45 as i8, 45 as i8, 115 as i8, 105 as i8, 110 as i8, 103 as i8,
            108 as i8, 101 as i8, 116 as i8, 104 as i8, 114 as i8, 101 as i8,
            97 as i8, 100 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 83 as i8, 101 as i8, 116 as i8, 32 as i8,
            115 as i8, 105 as i8, 110 as i8, 103 as i8, 108 as i8, 101 as i8,
            45 as i8, 116 as i8, 104 as i8, 114 as i8, 101 as i8, 97 as i8,
            100 as i8, 101 as i8, 100 as i8, 32 as i8, 109 as i8, 111 as i8,
            100 as i8, 101 as i8, 32 as i8, 45 as i8, 32 as i8, 100 as i8,
            105 as i8, 115 as i8, 97 as i8, 98 as i8, 108 as i8, 101 as i8,
            115 as i8, 32 as i8, 97 as i8, 108 as i8, 108 as i8, 32 as i8,
            109 as i8, 117 as i8, 116 as i8, 101 as i8, 120 as i8, 105 as i8,
            110 as i8, 103 as i8, 10 as i8, 32 as i8, 32 as i8, 45 as i8,
            45 as i8, 115 as i8, 113 as i8, 108 as i8, 111 as i8, 110 as i8,
            108 as i8, 121 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 78 as i8, 111 as i8, 45 as i8, 111 as i8, 112 as i8,
            46 as i8, 32 as i8, 32 as i8, 79 as i8, 110 as i8, 108 as i8,
            121 as i8, 32 as i8, 115 as i8, 104 as i8, 111 as i8, 119 as i8,
            32 as i8, 116 as i8, 104 as i8, 101 as i8, 32 as i8, 83 as i8,
            81 as i8, 76 as i8, 32 as i8, 116 as i8, 104 as i8, 97 as i8,
            116 as i8, 32 as i8, 119 as i8, 111 as i8, 117 as i8, 108 as i8,
            100 as i8, 32 as i8, 104 as i8, 97 as i8, 118 as i8, 101 as i8,
            32 as i8, 98 as i8, 101 as i8, 101 as i8, 110 as i8, 32 as i8,
            114 as i8, 117 as i8, 110 as i8, 46 as i8, 10 as i8, 32 as i8,
            32 as i8, 45 as i8, 45 as i8, 115 as i8, 104 as i8, 114 as i8,
            105 as i8, 110 as i8, 107 as i8, 45 as i8, 109 as i8, 101 as i8,
            109 as i8, 111 as i8, 114 as i8, 121 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 73 as i8, 110 as i8, 118 as i8,
            111 as i8, 107 as i8, 101 as i8, 32 as i8, 115 as i8, 113 as i8,
            108 as i8, 105 as i8, 116 as i8, 101 as i8, 51 as i8, 95 as i8,
            100 as i8, 98 as i8, 95 as i8, 114 as i8, 101 as i8, 108 as i8,
            101 as i8, 97 as i8, 115 as i8, 101 as i8, 95 as i8, 109 as i8,
            101 as i8, 109 as i8, 111 as i8, 114 as i8, 121 as i8, 40 as i8,
            41 as i8, 32 as i8, 102 as i8, 114 as i8, 101 as i8, 113 as i8,
            117 as i8, 101 as i8, 110 as i8, 116 as i8, 108 as i8, 121 as i8,
            46 as i8, 10 as i8, 32 as i8, 32 as i8, 45 as i8, 45 as i8,
            115 as i8, 105 as i8, 122 as i8, 101 as i8, 32 as i8, 78 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            82 as i8, 101 as i8, 108 as i8, 97 as i8, 116 as i8, 105 as i8,
            118 as i8, 101 as i8, 32 as i8, 116 as i8, 101 as i8, 115 as i8,
            116 as i8, 32 as i8, 115 as i8, 105 as i8, 122 as i8, 101 as i8,
            46 as i8, 32 as i8, 32 as i8, 68 as i8, 101 as i8, 102 as i8,
            97 as i8, 117 as i8, 108 as i8, 116 as i8, 61 as i8, 49 as i8,
            48 as i8, 48 as i8, 10 as i8, 32 as i8, 32 as i8, 45 as i8,
            45 as i8, 115 as i8, 111 as i8, 102 as i8, 116 as i8, 45 as i8,
            104 as i8, 101 as i8, 97 as i8, 112 as i8, 45 as i8, 108 as i8,
            105 as i8, 109 as i8, 105 as i8, 116 as i8, 32 as i8, 78 as i8,
            32 as i8, 84 as i8, 104 as i8, 101 as i8, 32 as i8, 115 as i8,
            111 as i8, 102 as i8, 116 as i8, 32 as i8, 108 as i8, 105 as i8,
            109 as i8, 105 as i8, 116 as i8, 32 as i8, 111 as i8, 110 as i8,
            32 as i8, 116 as i8, 104 as i8, 101 as i8, 32 as i8, 109 as i8,
            97 as i8, 120 as i8, 105 as i8, 109 as i8, 117 as i8, 109 as i8,
            32 as i8, 104 as i8, 101 as i8, 97 as i8, 112 as i8, 32 as i8,
            115 as i8, 105 as i8, 122 as i8, 101 as i8, 10 as i8, 32 as i8,
            32 as i8, 45 as i8, 45 as i8, 115 as i8, 116 as i8, 114 as i8,
            105 as i8, 99 as i8, 116 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 85 as i8, 115 as i8, 101 as i8,
            32 as i8, 83 as i8, 84 as i8, 82 as i8, 73 as i8, 67 as i8,
            84 as i8, 32 as i8, 116 as i8, 97 as i8, 98 as i8, 108 as i8,
            101 as i8, 32 as i8, 119 as i8, 104 as i8, 101 as i8, 114 as i8,
            101 as i8, 32 as i8, 97 as i8, 112 as i8, 112 as i8, 114 as i8,
            111 as i8, 112 as i8, 114 as i8, 105 as i8, 97 as i8, 116 as i8,
            101 as i8, 10 as i8, 32 as i8, 32 as i8, 45 as i8, 45 as i8,
            115 as i8, 116 as i8, 97 as i8, 116 as i8, 115 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            83 as i8, 104 as i8, 111 as i8, 119 as i8, 32 as i8, 115 as i8,
            116 as i8, 97 as i8, 116 as i8, 105 as i8, 115 as i8, 116 as i8,
            105 as i8, 99 as i8, 115 as i8, 32 as i8, 97 as i8, 116 as i8,
            32 as i8, 116 as i8, 104 as i8, 101 as i8, 32 as i8, 101 as i8,
            110 as i8, 100 as i8, 10 as i8, 32 as i8, 32 as i8, 45 as i8,
            45 as i8, 115 as i8, 116 as i8, 109 as i8, 116 as i8, 115 as i8,
            99 as i8, 97 as i8, 110 as i8, 115 as i8, 116 as i8, 97 as i8,
            116 as i8, 117 as i8, 115 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 65 as i8, 99 as i8, 116 as i8, 105 as i8, 118 as i8,
            97 as i8, 116 as i8, 101 as i8, 32 as i8, 83 as i8, 81 as i8,
            76 as i8, 73 as i8, 84 as i8, 69 as i8, 95 as i8, 68 as i8,
            66 as i8, 67 as i8, 79 as i8, 78 as i8, 70 as i8, 73 as i8,
            71 as i8, 95 as i8, 83 as i8, 84 as i8, 77 as i8, 84 as i8,
            95 as i8, 83 as i8, 67 as i8, 65 as i8, 78 as i8, 83 as i8,
            84 as i8, 65 as i8, 84 as i8, 85 as i8, 83 as i8, 10 as i8,
            32 as i8, 32 as i8, 45 as i8, 45 as i8, 116 as i8, 101 as i8,
            109 as i8, 112 as i8, 32 as i8, 78 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 78 as i8, 32 as i8,
            102 as i8, 114 as i8, 111 as i8, 109 as i8, 32 as i8, 48 as i8,
            32 as i8, 116 as i8, 111 as i8, 32 as i8, 57 as i8, 46 as i8,
            32 as i8, 32 as i8, 48 as i8, 58 as i8, 32 as i8, 110 as i8,
            111 as i8, 32 as i8, 116 as i8, 101 as i8, 109 as i8, 112 as i8,
            32 as i8, 116 as i8, 97 as i8, 98 as i8, 108 as i8, 101 as i8,
            46 as i8, 32 as i8, 57 as i8, 58 as i8, 32 as i8, 97 as i8,
            108 as i8, 108 as i8, 32 as i8, 116 as i8, 101 as i8, 109 as i8,
            112 as i8, 32 as i8, 116 as i8, 97 as i8, 98 as i8, 108 as i8,
            101 as i8, 115 as i8, 10 as i8, 32 as i8, 32 as i8, 45 as i8,
            45 as i8, 116 as i8, 101 as i8, 115 as i8, 116 as i8, 115 as i8,
            101 as i8, 116 as i8, 32 as i8, 84 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 82 as i8, 117 as i8, 110 as i8, 32 as i8, 116 as i8,
            101 as i8, 115 as i8, 116 as i8, 45 as i8, 115 as i8, 101 as i8,
            116 as i8, 32 as i8, 84 as i8, 32 as i8, 40 as i8, 109 as i8,
            97 as i8, 105 as i8, 110 as i8, 44 as i8, 32 as i8, 99 as i8,
            116 as i8, 101 as i8, 44 as i8, 32 as i8, 114 as i8, 116 as i8,
            114 as i8, 101 as i8, 101 as i8, 44 as i8, 32 as i8, 111 as i8,
            114 as i8, 109 as i8, 44 as i8, 32 as i8, 102 as i8, 112 as i8,
            44 as i8, 32 as i8, 106 as i8, 115 as i8, 111 as i8, 110 as i8,
            44 as i8, 10 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            115 as i8, 116 as i8, 97 as i8, 114 as i8, 44 as i8, 32 as i8,
            97 as i8, 112 as i8, 112 as i8, 44 as i8, 32 as i8, 100 as i8,
            101 as i8, 98 as i8, 117 as i8, 103 as i8, 41 as i8, 46 as i8,
            32 as i8, 32 as i8, 67 as i8, 97 as i8, 110 as i8, 32 as i8,
            98 as i8, 101 as i8, 32 as i8, 97 as i8, 32 as i8, 99 as i8,
            111 as i8, 109 as i8, 109 as i8, 97 as i8, 45 as i8, 115 as i8,
            101 as i8, 112 as i8, 97 as i8, 114 as i8, 97 as i8, 116 as i8,
            101 as i8, 100 as i8, 32 as i8, 108 as i8, 105 as i8, 115 as i8,
            116 as i8, 10 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            111 as i8, 102 as i8, 32 as i8, 118 as i8, 97 as i8, 108 as i8,
            117 as i8, 101 as i8, 115 as i8, 44 as i8, 32 as i8, 119 as i8,
            105 as i8, 116 as i8, 104 as i8, 32 as i8, 47 as i8, 83 as i8,
            67 as i8, 65 as i8, 76 as i8, 69 as i8, 32 as i8, 115 as i8,
            117 as i8, 102 as i8, 102 as i8, 105 as i8, 120 as i8, 101 as i8,
            115 as i8, 32 as i8, 111 as i8, 114 as i8, 32 as i8, 109 as i8,
            97 as i8, 99 as i8, 114 as i8, 111 as i8, 32 as i8, 34 as i8,
            109 as i8, 105 as i8, 120 as i8, 49 as i8, 34 as i8, 10 as i8,
            32 as i8, 32 as i8, 45 as i8, 45 as i8, 116 as i8, 114 as i8,
            97 as i8, 99 as i8, 101 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 84 as i8, 117 as i8,
            114 as i8, 110 as i8, 32 as i8, 111 as i8, 110 as i8, 32 as i8,
            83 as i8, 81 as i8, 76 as i8, 32 as i8, 116 as i8, 114 as i8,
            97 as i8, 99 as i8, 105 as i8, 110 as i8, 103 as i8, 10 as i8,
            32 as i8, 32 as i8, 45 as i8, 45 as i8, 116 as i8, 104 as i8,
            114 as i8, 101 as i8, 97 as i8, 100 as i8, 115 as i8, 32 as i8,
            78 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 85 as i8, 115 as i8,
            101 as i8, 32 as i8, 117 as i8, 112 as i8, 32 as i8, 116 as i8,
            111 as i8, 32 as i8, 78 as i8, 32 as i8, 116 as i8, 104 as i8,
            114 as i8, 101 as i8, 97 as i8, 100 as i8, 115 as i8, 32 as i8,
            102 as i8, 111 as i8, 114 as i8, 32 as i8, 115 as i8, 111 as i8,
            114 as i8, 116 as i8, 105 as i8, 110 as i8, 103 as i8, 10 as i8,
            32 as i8, 32 as i8, 45 as i8, 45 as i8, 117 as i8, 116 as i8,
            102 as i8, 49 as i8, 54 as i8, 98 as i8, 101 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 83 as i8, 101 as i8,
            116 as i8, 32 as i8, 116 as i8, 101 as i8, 120 as i8, 116 as i8,
            32 as i8, 101 as i8, 110 as i8, 99 as i8, 111 as i8, 100 as i8,
            105 as i8, 110 as i8, 103 as i8, 32 as i8, 116 as i8, 111 as i8,
            32 as i8, 85 as i8, 84 as i8, 70 as i8, 45 as i8, 49 as i8,
            54 as i8, 66 as i8, 69 as i8, 10 as i8, 32 as i8, 32 as i8,
            45 as i8, 45 as i8, 117 as i8, 116 as i8, 102 as i8, 49 as i8,
            54 as i8, 108 as i8, 101 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 83 as i8, 101 as i8, 116 as i8, 32 as i8,
            116 as i8, 101 as i8, 120 as i8, 116 as i8, 32 as i8, 101 as i8,
            110 as i8, 99 as i8, 111 as i8, 100 as i8, 105 as i8, 110 as i8,
            103 as i8, 32 as i8, 116 as i8, 111 as i8, 32 as i8, 85 as i8,
            84 as i8, 70 as i8, 45 as i8, 49 as i8, 54 as i8, 76 as i8,
            69 as i8, 10 as i8, 32 as i8, 32 as i8, 45 as i8, 45 as i8,
            118 as i8, 101 as i8, 114 as i8, 105 as i8, 102 as i8, 121 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            82 as i8, 117 as i8, 110 as i8, 32 as i8, 97 as i8, 100 as i8,
            100 as i8, 105 as i8, 116 as i8, 105 as i8, 111 as i8, 110 as i8,
            97 as i8, 108 as i8, 32 as i8, 118 as i8, 101 as i8, 114 as i8,
            105 as i8, 102 as i8, 105 as i8, 99 as i8, 97 as i8, 116 as i8,
            105 as i8, 111 as i8, 110 as i8, 32 as i8, 115 as i8, 116 as i8,
            101 as i8, 112 as i8, 115 as i8, 10 as i8, 32 as i8, 32 as i8,
            45 as i8, 45 as i8, 118 as i8, 102 as i8, 115 as i8, 32 as i8,
            78 as i8, 65 as i8, 77 as i8, 69 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 85 as i8, 115 as i8, 101 as i8, 32 as i8,
            116 as i8, 104 as i8, 101 as i8, 32 as i8, 103 as i8, 105 as i8,
            118 as i8, 101 as i8, 110 as i8, 32 as i8, 40 as i8, 112 as i8,
            114 as i8, 101 as i8, 105 as i8, 110 as i8, 115 as i8, 116 as i8,
            97 as i8, 108 as i8, 108 as i8, 101 as i8, 100 as i8, 41 as i8,
            32 as i8, 86 as i8, 70 as i8, 83 as i8, 10 as i8, 32 as i8,
            32 as i8, 45 as i8, 45 as i8, 119 as i8, 105 as i8, 116 as i8,
            104 as i8, 111 as i8, 117 as i8, 116 as i8, 45 as i8, 114 as i8,
            111 as i8, 119 as i8, 105 as i8, 100 as i8, 32 as i8, 32 as i8,
            32 as i8, 32 as i8, 32 as i8, 85 as i8, 115 as i8, 101 as i8,
            32 as i8, 87 as i8, 73 as i8, 84 as i8, 72 as i8, 79 as i8,
            85 as i8, 84 as i8, 32 as i8, 82 as i8, 79 as i8, 87 as i8,
            73 as i8, 68 as i8, 32 as i8, 119 as i8, 104 as i8, 101 as i8,
            114 as i8, 101 as i8, 32 as i8, 97 as i8, 112 as i8, 112 as i8,
            114 as i8, 111 as i8, 112 as i8, 114 as i8, 105 as i8, 97 as i8,
            116 as i8, 101 as i8, 10 as i8, 0 as i8];
#[repr(C)]
#[derive(Copy, Clone)]
struct HashContext {
    is_init: u8,
    i: u8,
    j: u8,
    s: [u8; 256],
    r: [u8; 32],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Global {
    db: *mut Sqlite3,
    z_db_name: *const i8,
    z_vfs: *const i8,
    p_stmt: *mut Sqlite3Stmt,
    i_start: Sqlite3Int64,
    i_total: Sqlite3Int64,
    b_without_rowid: i32,
    b_reprepare: i32,
    b_sql_only: i32,
    b_explain: i32,
    b_verify: i32,
    b_mem_shrink: i32,
    e_temp: i32,
    sz_test: i32,
    sz_base: i32,
    n_repeat: i32,
    do_checkpoint: i32,
    n_reserve: i32,
    stmt_scan_status: i32,
    do_big_transactions: i32,
    z_wr: *const i8,
    z_nn: *const i8,
    z_pk: *const i8,
    x: u32,
    y: u32,
    n_res_byte: u64,
    n_result: i32,
    z_result: [i8; 3000],
    p_script: *mut FILE,
    hash_file: *mut FILE,
    hash: HashContext,
}
static mut g: Global = unsafe { core::mem::zeroed() };
extern "C" fn is_temp(n_1: i32) -> *const i8 {
    unsafe {
        return if g.e_temp >= n_1 {
                    c" TEMP".as_ptr() as *mut i8
                } else { c"".as_ptr() as *mut i8 } as *const i8;
    }
}
unsafe extern "C" fn fatal_error(z_msg_1: *const i8, mut __va0: ...) -> () {
    unsafe {
        let mut ap: *mut i8 = core::ptr::null_mut();
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        unsafe { vfprintf(__stderrp, z_msg_1, ap) };
        ();
        unsafe { exit(1) };
    }
}
extern "C" fn hash_init() -> () {
    unsafe {
        let mut k: u32 = 0 as u32;
        g.hash.i = 0 as u8;
        g.hash.j = 0 as u8;
        {
            k = 0 as u32;
            '__b0: loop {
                if !(k < 256 as u32) { break '__b0; }
                '__c0: loop { g.hash.s[k as usize] = k as u8; break '__c0; }
                { let __p = &mut k; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}
extern "C" fn hash_update(a_data_1: *const u8, n_data_1: u32) -> () {
    unsafe {
        let mut t: u8 = 0 as u8;
        let mut i: u8 = g.hash.i;
        let mut j: u8 = g.hash.j;
        let mut k: u32 = 0 as u32;
        if !(g.hash_file).is_null() {
            unsafe {
                fwrite(a_data_1 as *const (), 1 as u64, n_data_1 as u64,
                    g.hash_file)
            };
        }
        {
            k = 0 as u32;
            '__b1: loop {
                if !(k < n_data_1) { break '__b1; }
                '__c1: loop {
                    j +=
                        (g.hash.s[i as usize] as i32 +
                                unsafe { *a_data_1.add(k as usize) } as i32) as u8;
                    t = g.hash.s[j as usize];
                    g.hash.s[j as usize] = g.hash.s[i as usize];
                    g.hash.s[i as usize] = t;
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    break '__c1;
                }
                { let __p = &mut k; let __t = *__p; *__p += 1; __t };
            }
        }
        g.hash.i = i;
        g.hash.j = j;
    }
}
extern "C" fn hash_final() -> () {
    unsafe {
        let mut k: u32 = 0 as u32;
        let mut t: u8 = 0 as u8;
        let mut i: u8 = 0 as u8;
        let mut j: u8 = 0 as u8;
        i = g.hash.i;
        j = g.hash.j;
        {
            k = 0 as u32;
            '__b2: loop {
                if !(k < 32 as u32) { break '__b2; }
                '__c2: loop {
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                    t = g.hash.s[i as usize];
                    j += t as i32 as u8;
                    g.hash.s[i as usize] = g.hash.s[j as usize];
                    g.hash.s[j as usize] = t;
                    t += g.hash.s[i as usize] as i32 as u8;
                    g.hash.r[k as usize] = g.hash.s[t as usize];
                    break '__c2;
                }
                { let __p = &mut k; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}
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
            '__b5: loop {
                if !((i as u64) <
                                core::mem::size_of::<[IntegerValueS0N16integerValueS0; 9]>()
                                        as u64 / 16) {
                    break '__b5;
                }
                '__c5: loop {
                    if unsafe {
                                sqlite3_stricmp(a_mult[i as usize].z_suffix as *const i8,
                                    z_arg_1)
                            } == 0 {
                        v *= a_mult[i as usize].i_mult as Sqlite3Int64;
                        break '__b5;
                    }
                    break '__c5;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if v > 2147483647 as i64 {
            unsafe {
                fatal_error(c"parameter too large - max 2147483648".as_ptr()
                            as *mut i8 as *const i8)
            };
        }
        return if is_neg != 0 { -v } else { v } as i32;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn speedtest1_timestamp() -> Sqlite3Int64 {
    unsafe {
        let mut t: Sqlite3Int64 = 0 as Sqlite3Int64;
        if clock_vfs == core::ptr::null_mut() {
            clock_vfs = unsafe { sqlite3_vfs_find(core::ptr::null()) };
        }
        if unsafe { (*clock_vfs).i_version } >= 2 &&
                unsafe { (*clock_vfs).x_current_time_int64.is_some() } {
            unsafe {
                (unsafe {
                        (*clock_vfs).x_current_time_int64.unwrap()
                    })(clock_vfs, &mut t)
            };
        } else {
            let mut r: f64 = 0.0;
            unsafe {
                (unsafe {
                        (*clock_vfs).x_current_time.unwrap()
                    })(clock_vfs, &mut r)
            };
            t = (r * 86400000.0) as Sqlite3Int64;
        }
        return t;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn speedtest1_random() -> u32 {
    unsafe {
        g.x = g.x >> 1 ^ 1 as u32 + !(g.x & 1 as u32) & 3489660929u32;
        g.y = g.y * 1103515245 as u32 + 12345 as u32;
        return g.x ^ g.y;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn swizzle(mut in__1: u32, mut limit: u32) -> u32 {
    let mut out: u32 = 0 as u32;
    while limit != 0 {
        out = out << 1 | in__1 & 1 as u32;
        in__1 >>= 1 as u32;
        limit >>= 1 as u32;
    }
    return out;
}
#[unsafe(no_mangle)]
pub extern "C" fn roundup_allones(limit: u32) -> u32 {
    let mut m: u32 = 1 as u32;
    while m < limit { m = (m << 1) + 1 as u32; }
    return m;
}
#[unsafe(no_mangle)]
pub extern "C" fn speedtest1_numbername(mut n: u32, z_out_1: *mut i8,
    n_out_1: i32) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        if n >= 1000000000 as u32 {
            i +=
                speedtest1_numbername(n / 1000000000 as u32,
                    unsafe { z_out_1.offset(i as isize) }, n_out_1 - i);
            unsafe {
                sqlite3_snprintf(n_out_1 - i,
                    unsafe { z_out_1.offset(i as isize) },
                    c" billion".as_ptr() as *mut i8 as *const i8)
            };
            i +=
                unsafe {
                        strlen(unsafe { z_out_1.offset(i as isize) } as *const i8)
                    } as i32;
            n = n % 1000000000 as u32;
        }
        if n >= 1000000 as u32 {
            if i != 0 && i < n_out_1 - 1 {
                unsafe {
                    *z_out_1.offset({
                                        let __p = &mut i;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as isize) = ' ' as i32 as i8
                };
            }
            i +=
                speedtest1_numbername(n / 1000000 as u32,
                    unsafe { z_out_1.offset(i as isize) }, n_out_1 - i);
            unsafe {
                sqlite3_snprintf(n_out_1 - i,
                    unsafe { z_out_1.offset(i as isize) },
                    c" million".as_ptr() as *mut i8 as *const i8)
            };
            i +=
                unsafe {
                        strlen(unsafe { z_out_1.offset(i as isize) } as *const i8)
                    } as i32;
            n = n % 1000000 as u32;
        }
        if n >= 1000 as u32 {
            if i != 0 && i < n_out_1 - 1 {
                unsafe {
                    *z_out_1.offset({
                                        let __p = &mut i;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as isize) = ' ' as i32 as i8
                };
            }
            i +=
                speedtest1_numbername(n / 1000 as u32,
                    unsafe { z_out_1.offset(i as isize) }, n_out_1 - i);
            unsafe {
                sqlite3_snprintf(n_out_1 - i,
                    unsafe { z_out_1.offset(i as isize) },
                    c" thousand".as_ptr() as *mut i8 as *const i8)
            };
            i +=
                unsafe {
                        strlen(unsafe { z_out_1.offset(i as isize) } as *const i8)
                    } as i32;
            n = n % 1000 as u32;
        }
        if n >= 100 as u32 {
            if i != 0 && i < n_out_1 - 1 {
                unsafe {
                    *z_out_1.offset({
                                        let __p = &mut i;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as isize) = ' ' as i32 as i8
                };
            }
            unsafe {
                sqlite3_snprintf(n_out_1 - i,
                    unsafe { z_out_1.offset(i as isize) },
                    c"%s hundred".as_ptr() as *mut i8 as *const i8,
                    ones[(n / 100 as u32) as usize])
            };
            i +=
                unsafe {
                        strlen(unsafe { z_out_1.offset(i as isize) } as *const i8)
                    } as i32;
            n = n % 100 as u32;
        }
        if n >= 20 as u32 {
            if i != 0 && i < n_out_1 - 1 {
                unsafe {
                    *z_out_1.offset({
                                        let __p = &mut i;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as isize) = ' ' as i32 as i8
                };
            }
            unsafe {
                sqlite3_snprintf(n_out_1 - i,
                    unsafe { z_out_1.offset(i as isize) },
                    c"%s".as_ptr() as *mut i8 as *const i8,
                    tens[(n / 10 as u32) as usize])
            };
            i +=
                unsafe {
                        strlen(unsafe { z_out_1.offset(i as isize) } as *const i8)
                    } as i32;
            n = n % 10 as u32;
        }
        if n > 0 as u32 {
            if i != 0 && i < n_out_1 - 1 {
                unsafe {
                    *z_out_1.offset({
                                        let __p = &mut i;
                                        let __t = *__p;
                                        *__p += 1;
                                        __t
                                    } as isize) = ' ' as i32 as i8
                };
            }
            unsafe {
                sqlite3_snprintf(n_out_1 - i,
                    unsafe { z_out_1.offset(i as isize) },
                    c"%s".as_ptr() as *mut i8 as *const i8, ones[n as usize])
            };
            i +=
                unsafe {
                        strlen(unsafe { z_out_1.offset(i as isize) } as *const i8)
                    } as i32;
        }
        if i == 0 {
            unsafe {
                sqlite3_snprintf(n_out_1 - i,
                    unsafe { z_out_1.offset(i as isize) },
                    c"zero".as_ptr() as *mut i8 as *const i8)
            };
            i +=
                unsafe {
                        strlen(unsafe { z_out_1.offset(i as isize) } as *const i8)
                    } as i32;
        }
        return i;
    }
}
static z_dots: [i8; 72] =
    [46 as i8, 46 as i8, 46 as i8, 46 as i8, 46 as i8, 46 as i8, 46 as i8,
            46 as i8, 46 as i8, 46 as i8, 46 as i8, 46 as i8, 46 as i8,
            46 as i8, 46 as i8, 46 as i8, 46 as i8, 46 as i8, 46 as i8,
            46 as i8, 46 as i8, 46 as i8, 46 as i8, 46 as i8, 46 as i8,
            46 as i8, 46 as i8, 46 as i8, 46 as i8, 46 as i8, 46 as i8,
            46 as i8, 46 as i8, 46 as i8, 46 as i8, 46 as i8, 46 as i8,
            46 as i8, 46 as i8, 46 as i8, 46 as i8, 46 as i8, 46 as i8,
            46 as i8, 46 as i8, 46 as i8, 46 as i8, 46 as i8, 46 as i8,
            46 as i8, 46 as i8, 46 as i8, 46 as i8, 46 as i8, 46 as i8,
            46 as i8, 46 as i8, 46 as i8, 46 as i8, 46 as i8, 46 as i8,
            46 as i8, 46 as i8, 46 as i8, 46 as i8, 46 as i8, 46 as i8,
            46 as i8, 46 as i8, 46 as i8, 46 as i8, 0 as i8];
static mut i_test_number: i32 = 0;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn speedtest1_begin_test(i_test_num_1: i32,
    z_test_name_1: *const i8, mut __va0: ...) -> () {
    unsafe {
        let mut n: i32 = unsafe { strlen(z_test_name_1) } as i32;
        let mut z_name: *mut i8 = core::ptr::null_mut();
        let mut ap: *mut i8 = core::ptr::null_mut();
        i_test_number = i_test_num_1;
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        z_name = unsafe { sqlite3_vmprintf(z_test_name_1, ap) };
        ();
        n = unsafe { strlen(z_name as *const i8) } as i32;
        if n > 60 {
            unsafe { *z_name.offset(60 as isize) = 0 as i8 };
            n = 60;
        }
        if !(g.p_script).is_null() {
            unsafe {
                fprintf(g.p_script,
                    c"-- begin test %d %.*s\n".as_ptr() as *mut i8 as *const i8,
                    i_test_number, n, z_name)
            };
        }
        if g.b_sql_only != 0 {
            unsafe {
                printf(c"/* %4d - %s%.*s */\n".as_ptr() as *mut i8 as
                        *const i8, i_test_num_1, z_name, 60 - n,
                    &raw const z_dots[0 as usize] as *const i8)
            };
        } else {
            unsafe {
                printf(c"%4d - %s%.*s ".as_ptr() as *mut i8 as *const i8,
                    i_test_num_1, z_name, 60 - n,
                    &raw const z_dots[0 as usize] as *const i8)
            };
            unsafe { fflush(__stdoutp) };
        }
        unsafe { sqlite3_free(z_name as *mut ()) };
        g.n_result = 0;
        g.i_start = speedtest1_timestamp();
        g.x = 2903710987u32;
        g.y = 1157229256 as u32;
    }
}
extern "C" fn print_sql(z_sql_1: *const i8) -> () {
    unsafe {
        let mut n: i32 = unsafe { strlen(z_sql_1) } as i32;
        while n > 0 &&
                (unsafe { *z_sql_1.offset((n - 1) as isize) } as i32 ==
                        ';' as i32 ||
                    unsafe {
                            isspace(unsafe { *z_sql_1.offset((n - 1) as isize) } as u8
                                    as i32)
                        } != 0) {
            { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
        }
        if g.b_explain != 0 {
            unsafe { printf(c"EXPLAIN ".as_ptr() as *mut i8 as *const i8) };
        }
        unsafe {
            printf(c"%.*s;\n".as_ptr() as *mut i8 as *const i8, n, z_sql_1)
        };
        if g.b_explain != 0 &&
                (unsafe {
                                sqlite3_strglob(c"CREATE *".as_ptr() as *mut i8 as
                                        *const i8, z_sql_1)
                            } == 0 ||
                        unsafe {
                                sqlite3_strglob(c"DROP *".as_ptr() as *mut i8 as *const i8,
                                    z_sql_1)
                            } == 0 ||
                    unsafe {
                            sqlite3_strglob(c"ALTER *".as_ptr() as *mut i8 as *const i8,
                                z_sql_1)
                        } == 0) {
            unsafe {
                printf(c"%.*s;\n".as_ptr() as *mut i8 as *const i8, n,
                    z_sql_1)
            };
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn speedtest1_shrink_memory() -> () {
    unsafe {
        if g.b_mem_shrink != 0 { unsafe { sqlite3_db_release_memory(g.db) }; }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn speedtest1_exec(z_format: *const i8, mut __va0: ...)
    -> () {
    unsafe {
        let mut ap: *mut i8 = core::ptr::null_mut();
        let mut z_sql: *mut i8 = core::ptr::null_mut();
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        z_sql = unsafe { sqlite3_vmprintf(z_format, ap) };
        ();
        if g.b_sql_only != 0 {
            print_sql(z_sql as *const i8);
        } else {
            let mut z_err_msg: *mut i8 = core::ptr::null_mut();
            let mut rc: i32 = 0;
            if !(g.p_script).is_null() {
                unsafe {
                    fprintf(g.p_script,
                        c"%s;\n".as_ptr() as *mut i8 as *const i8, z_sql)
                };
            }
            rc =
                unsafe {
                    sqlite3_exec(g.db, z_sql as *const i8, None,
                        core::ptr::null_mut(), &mut z_err_msg)
                };
            if !(z_err_msg).is_null() {
                unsafe {
                    fatal_error(c"SQL error: %s\n%s\n".as_ptr() as *mut i8 as
                            *const i8, z_err_msg, z_sql)
                };
            }
            if rc != 0 {
                unsafe {
                    fatal_error(c"exec error: %s\n".as_ptr() as *mut i8 as
                            *const i8, unsafe { sqlite3_errmsg(g.db) })
                };
            }
        }
        unsafe { sqlite3_free(z_sql as *mut ()) };
        speedtest1_shrink_memory();
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn speedtest1_end_test() -> () {
    unsafe {
        let i_elapse_time: Sqlite3Int64 = speedtest1_timestamp() - g.i_start;
        if g.do_checkpoint != 0 {
            unsafe {
                speedtest1_exec(c"PRAGMA wal_checkpoint;".as_ptr() as *mut i8
                        as *const i8)
            };
        }
        if !(i_test_number > 0) as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"speedtest1_end_test".as_ptr() as *const i8,
                    c"speedtest1.c".as_ptr() as *mut i8 as *const i8, 456,
                    c"iTestNumber > 0".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if !(g.p_script).is_null() {
            unsafe {
                fprintf(g.p_script,
                    c"-- end test %d\n".as_ptr() as *mut i8 as *const i8,
                    i_test_number)
            };
        }
        if (g.b_sql_only == 0) as i32 != 0 {
            g.i_total += i_elapse_time;
            unsafe {
                printf(c"%4d.%03ds\n".as_ptr() as *mut i8 as *const i8,
                    (i_elapse_time / 1000 as Sqlite3Int64) as i32,
                    (i_elapse_time % 1000 as Sqlite3Int64) as i32)
            };
        }
        if !(g.p_stmt).is_null() {
            unsafe { sqlite3_finalize(g.p_stmt) };
            g.p_stmt = core::ptr::null_mut();
        }
        i_test_number = 0;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn speedtest1_final() -> () {
    unsafe {
        if (g.b_sql_only == 0) as i32 != 0 {
            unsafe {
                printf(c"       TOTAL%.*s %4d.%03ds\n".as_ptr() as *mut i8 as
                        *const i8, 60 - 5,
                    &raw const z_dots[0 as usize] as *const i8,
                    (g.i_total / 1000 as Sqlite3Int64) as i32,
                    (g.i_total % 1000 as Sqlite3Int64) as i32)
            };
        }
        if g.b_verify != 0 {
            let mut i: i32 = 0;
            unsafe {
                printf(c"Verification Hash: %llu ".as_ptr() as *mut i8 as
                        *const i8, g.n_res_byte)
            };
            hash_update(c"\n".as_ptr() as *const u8, 1 as u32);
            hash_final();
            {
                i = 0;
                '__b9: loop {
                    if !(i < 24) { break '__b9; }
                    '__c9: loop {
                        unsafe {
                            printf(c"%02x".as_ptr() as *mut i8 as *const i8,
                                g.hash.r[i as usize] as i32)
                        };
                        break '__c9;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
            if !(g.hash_file).is_null() && g.hash_file != __stdoutp {
                unsafe { fclose(g.hash_file) };
            }
            unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn speedtest1_once(z_format_1: *const i8,
    mut __va0: ...) -> *mut i8 {
    unsafe {
        let mut ap: *mut i8 = core::ptr::null_mut();
        let mut z_sql: *mut i8 = core::ptr::null_mut();
        let mut p_stmt: *mut Sqlite3Stmt = core::ptr::null_mut();
        let mut z_result: *mut i8 = core::ptr::null_mut();
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        z_sql = unsafe { sqlite3_vmprintf(z_format_1, ap) };
        ();
        if g.b_sql_only != 0 {
            print_sql(z_sql as *const i8);
        } else {
            let mut rc: i32 =
                unsafe {
                    sqlite3_prepare_v2(g.db, z_sql as *const i8, -1,
                        &mut p_stmt, core::ptr::null_mut())
                };
            if rc != 0 {
                unsafe {
                    fatal_error(c"SQL error: %s\n".as_ptr() as *mut i8 as
                            *const i8, unsafe { sqlite3_errmsg(g.db) })
                };
            }
            if !(g.p_script).is_null() {
                let z: *mut i8 = unsafe { sqlite3_expanded_sql(p_stmt) };
                unsafe {
                    fprintf(g.p_script,
                        c"%s\n".as_ptr() as *mut i8 as *const i8, z)
                };
                unsafe { sqlite3_free(z as *mut ()) };
            }
            if unsafe { sqlite3_step(p_stmt) } == 100 {
                let z: *const i8 =
                    unsafe { sqlite3_column_text(p_stmt, 0) } as *const i8;
                if !(z).is_null() {
                    z_result =
                        unsafe {
                            sqlite3_mprintf(c"%s".as_ptr() as *mut i8 as *const i8, z)
                        };
                }
            }
            rc = unsafe { sqlite3_reset(p_stmt) };
            if rc != 0 {
                unsafe {
                    fatal_error(c"%s\nError code %d: %s\n".as_ptr() as *mut i8
                            as *const i8, unsafe { sqlite3_sql(p_stmt) }, rc,
                        unsafe { sqlite3_errmsg(g.db) })
                };
            }
            unsafe { sqlite3_finalize(p_stmt) };
        }
        unsafe { sqlite3_free(z_sql as *mut ()) };
        speedtest1_shrink_memory();
        return z_result;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn speedtest1_prepare(z_format_1: *const i8,
    mut __va0: ...) -> () {
    unsafe {
        let mut ap: *mut i8 = core::ptr::null_mut();
        let mut z_sql: *mut i8 = core::ptr::null_mut();
        unsafe { ap = core::mem::transmute_copy(&__va0) };
        z_sql = unsafe { sqlite3_vmprintf(z_format_1, ap) };
        ();
        if g.b_sql_only != 0 {
            print_sql(z_sql as *const i8);
        } else {
            let mut rc: i32 = 0;
            if !(g.p_stmt).is_null() {
                unsafe { sqlite3_finalize(g.p_stmt) };
            }
            rc =
                unsafe {
                    sqlite3_prepare_v2(g.db, z_sql as *const i8, -1,
                        &mut g.p_stmt, core::ptr::null_mut())
                };
            if rc != 0 {
                unsafe {
                    fatal_error(c"SQL error: %s\n".as_ptr() as *mut i8 as
                            *const i8, unsafe { sqlite3_errmsg(g.db) })
                };
            }
        }
        unsafe { sqlite3_free(z_sql as *mut ()) };
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn speedtest1_run() -> () {
    unsafe {
        let mut i: i32 = 0;
        let mut n: i32 = 0;
        let mut len: i32 = 0;
        let mut rc: i32 = 0;
        if g.b_sql_only != 0 { return; }
        if (g.p_stmt).is_null() as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"speedtest1_run".as_ptr() as *const i8,
                    c"speedtest1.c".as_ptr() as *mut i8 as *const i8, 608,
                    c"g.pStmt".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        g.n_result = 0;
        if !(g.p_script).is_null() {
            let mut z: *mut i8 = unsafe { sqlite3_expanded_sql(g.p_stmt) };
            unsafe {
                fprintf(g.p_script, c"%s\n".as_ptr() as *mut i8 as *const i8,
                    z)
            };
            unsafe { sqlite3_free(z as *mut ()) };
        }
        while unsafe { sqlite3_step(g.p_stmt) } == 100 {
            n = unsafe { sqlite3_column_count(g.p_stmt) };
            {
                i = 0;
                '__b11: loop {
                    if !(i < n) { break '__b11; }
                    '__c11: loop {
                        let mut z: *const i8 =
                            unsafe { sqlite3_column_text(g.p_stmt, i) } as *const i8;
                        if z == core::ptr::null() {
                            z = c"nil".as_ptr() as *mut i8 as *const i8;
                        }
                        len = unsafe { strlen(z) } as i32;
                        if g.b_verify != 0 {
                            let e_type: i32 =
                                unsafe { sqlite3_column_type(g.p_stmt, i) };
                            let mut z_prefix: [u8; 2] = [0; 2];
                            z_prefix[0 as usize] = '\n' as i32 as u8;
                            z_prefix[1 as usize] =
                                unsafe {
                                        *(c"-IFTBN".as_ptr() as *mut i8).offset(e_type as isize)
                                    } as u8;
                            if g.n_res_byte != 0 {
                                hash_update(&raw mut z_prefix[0 as usize] as *mut u8 as
                                        *const u8, 2 as u32);
                            } else {
                                hash_update(unsafe {
                                            (&raw mut z_prefix[0 as usize] as
                                                    *mut u8).offset(1 as isize)
                                        } as *const u8, 1 as u32);
                            }
                            if e_type == 2 {
                                g.n_res_byte += 2 as u64;
                            } else if e_type == 4 {
                                let n_blob: i32 =
                                    unsafe { sqlite3_column_bytes(g.p_stmt, i) };
                                let mut i_blob: i32 = 0;
                                let mut z_char: [u8; 2] = [0; 2];
                                let a_blob: *const u8 =
                                    unsafe { sqlite3_column_blob(g.p_stmt, i) } as *const u8;
                                {
                                    i_blob = 0;
                                    '__b12: loop {
                                        if !(i_blob < n_blob) { break '__b12; }
                                        '__c12: loop {
                                            z_char[0 as usize] =
                                                unsafe {
                                                        *(c"0123456789abcdef".as_ptr() as
                                                                    *mut i8).offset((unsafe { *a_blob.offset(i_blob as isize) }
                                                                            as i32 >> 4) as isize)
                                                    } as u8;
                                            z_char[1 as usize] =
                                                unsafe {
                                                        *(c"0123456789abcdef".as_ptr() as
                                                                    *mut i8).offset((unsafe { *a_blob.offset(i_blob as isize) }
                                                                            as i32 & 15) as isize)
                                                    } as u8;
                                            hash_update(&raw mut z_char[0 as usize] as *mut u8 as
                                                    *const u8, 2 as u32);
                                            break '__c12;
                                        }
                                        { let __p = &mut i_blob; let __t = *__p; *__p += 1; __t };
                                    }
                                }
                                g.n_res_byte += (n_blob * 2 + 2) as u64;
                            } else {
                                hash_update(z as *mut u8 as *const u8, len as u32);
                                g.n_res_byte += (len + 2) as u64;
                            }
                        }
                        if ((g.n_result + len) as u64) <
                                core::mem::size_of::<[i8; 3000]>() as u64 - 2 as u64 {
                            if g.n_result > 0 {
                                g.z_result[{
                                                let __p = &mut g.n_result;
                                                let __t = *__p;
                                                *__p += 1;
                                                __t
                                            } as usize] = ' ' as i32 as i8;
                            }
                            unsafe {
                                memcpy(unsafe {
                                            (&raw mut g.z_result[0 as usize] as
                                                    *mut i8).offset(g.n_result as isize)
                                        } as *mut (), z as *const (), (len + 1) as u64)
                            };
                            g.n_result += len;
                        }
                        break '__c11;
                    }
                    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                }
            }
        }
        if g.b_reprepare != 0 {
            let mut p_new: *mut Sqlite3Stmt = core::ptr::null_mut();
            unsafe {
                sqlite3_prepare_v2(g.db, unsafe { sqlite3_sql(g.p_stmt) }, -1,
                    &mut p_new, core::ptr::null_mut())
            };
            rc = unsafe { sqlite3_finalize(g.p_stmt) };
            if rc != 0 {
                unsafe {
                    fatal_error(c"%s\nError code %d: %s\n".as_ptr() as *mut i8
                            as *const i8, unsafe { sqlite3_sql(p_new) }, rc,
                        unsafe { sqlite3_errmsg(g.db) })
                };
            }
            g.p_stmt = p_new;
        } else {
            rc = unsafe { sqlite3_reset(g.p_stmt) };
            if rc != 0 {
                unsafe {
                    fatal_error(c"%s\nError code %d: %s\n".as_ptr() as *mut i8
                            as *const i8, unsafe { sqlite3_sql(g.p_stmt) }, rc,
                        unsafe { sqlite3_errmsg(g.db) })
                };
            }
        }
        speedtest1_shrink_memory();
    }
}
extern "C" fn trace_callback(not_used_1: *mut (), z_sql_1: *const i8) -> () {
    unsafe {
        let mut n: i32 = unsafe { strlen(z_sql_1) } as i32;
        while n > 0 &&
                (unsafe { *z_sql_1.offset((n - 1) as isize) } as i32 ==
                        ';' as i32 ||
                    unsafe {
                            isspace(unsafe { *z_sql_1.offset((n - 1) as isize) } as u8
                                    as i32)
                        } != 0) {
            { let __p = &mut n; let __t = *__p; *__p -= 1; __t };
        }
        unsafe {
            fprintf(__stderrp, c"%.*s;\n".as_ptr() as *mut i8 as *const i8, n,
                z_sql_1)
        };
    }
}
extern "C" fn random_func(context: *mut Sqlite3Context, not_used_1: i32,
    not_used2_1: *mut *mut Sqlite3Value) -> () {
    unsafe {
        sqlite3_result_int64(context, speedtest1_random() as Sqlite3Int64)
    };
}
extern "C" fn est_square_root(x: i32) -> i32 {
    let mut y0: i32 = x / 2;
    let mut y1: i32 = 0;
    let mut n: i32 = 0;
    {
        n = 0;
        '__b14: loop {
            if !(y0 > 0 && n < 10) { break '__b14; }
            '__c14: loop {
                y1 = (y0 + x / y0) / 2;
                if y1 == y0 { break '__b14; }
                y0 = y1;
                break '__c14;
            }
            { let __p = &mut n; let __t = *__p; *__p += 1; __t };
        }
    }
    return y0;
}
#[unsafe(no_mangle)]
pub extern "C" fn testset_main() -> () {
    unsafe {
        let mut i: i32 = 0;
        let mut n: i32 = 0;
        let mut sz: i32 = 0;
        let mut maxb: i32 = 0;
        let mut x1: u32 = 0 as u32;
        let mut x2: u32 = 0 as u32;
        let mut len: i32 = 0;
        let mut z_num: [i8; 2000] = [0; 2000];
        sz = { n = g.sz_test * 500; n };
        z_num[0 as usize] = 0 as i8;
        maxb = roundup_allones(sz as u32) as i32;
        unsafe {
            speedtest1_begin_test(100,
                c"%d INSERTs into table with no index".as_ptr() as *mut i8 as
                    *const i8, n)
        };
        unsafe { speedtest1_exec(c"BEGIN".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            speedtest1_exec(c"CREATE%s TABLE z1(a INTEGER %s, b INTEGER %s, c TEXT %s);".as_ptr()
                        as *mut i8 as *const i8, is_temp(9), g.z_nn, g.z_nn, g.z_nn)
        };
        unsafe {
            speedtest1_prepare(c"INSERT INTO z1 VALUES(?1,?2,?3); --  %d times".as_ptr()
                        as *mut i8 as *const i8, n)
        };
        {
            i = 1;
            '__b15: loop {
                if !(i <= n) { break '__b15; }
                '__c15: loop {
                    x1 = swizzle(i as u32, maxb as u32);
                    speedtest1_numbername(x1,
                        &raw mut z_num[0 as usize] as *mut i8,
                        core::mem::size_of::<[i8; 2000]>() as i32);
                    unsafe {
                        sqlite3_bind_int64(g.p_stmt, 1, x1 as Sqlite3Int64)
                    };
                    unsafe { sqlite3_bind_int(g.p_stmt, 2, i) };
                    unsafe {
                        sqlite3_bind_text(g.p_stmt, 3,
                            &raw mut z_num[0 as usize] as *mut i8 as *const i8, -1,
                            None)
                    };
                    speedtest1_run();
                    break '__c15;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            speedtest1_exec(c"COMMIT".as_ptr() as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        n = sz;
        unsafe {
            speedtest1_begin_test(110,
                c"%d ordered INSERTS with one index/PK".as_ptr() as *mut i8 as
                    *const i8, n)
        };
        unsafe { speedtest1_exec(c"BEGIN".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            speedtest1_exec(c"CREATE%s TABLE z2(a INTEGER %s %s, b INTEGER %s, c TEXT %s) %s".as_ptr()
                        as *mut i8 as *const i8, is_temp(5), g.z_nn, g.z_pk, g.z_nn,
                g.z_nn, g.z_wr)
        };
        unsafe {
            speedtest1_prepare(c"INSERT INTO z2 VALUES(?1,?2,?3); -- %d times".as_ptr()
                        as *mut i8 as *const i8, n)
        };
        {
            i = 1;
            '__b16: loop {
                if !(i <= n) { break '__b16; }
                '__c16: loop {
                    x1 = swizzle(i as u32, maxb as u32);
                    speedtest1_numbername(x1,
                        &raw mut z_num[0 as usize] as *mut i8,
                        core::mem::size_of::<[i8; 2000]>() as i32);
                    unsafe { sqlite3_bind_int(g.p_stmt, 1, i) };
                    unsafe {
                        sqlite3_bind_int64(g.p_stmt, 2, x1 as Sqlite3Int64)
                    };
                    unsafe {
                        sqlite3_bind_text(g.p_stmt, 3,
                            &raw mut z_num[0 as usize] as *mut i8 as *const i8, -1,
                            None)
                    };
                    speedtest1_run();
                    break '__c16;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            speedtest1_exec(c"COMMIT".as_ptr() as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        n = sz;
        unsafe {
            speedtest1_begin_test(120,
                c"%d unordered INSERTS with one index/PK".as_ptr() as *mut i8
                    as *const i8, n)
        };
        unsafe { speedtest1_exec(c"BEGIN".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            speedtest1_exec(c"CREATE%s TABLE t3(a INTEGER %s %s, b INTEGER %s, c TEXT %s) %s".as_ptr()
                        as *mut i8 as *const i8, is_temp(3), g.z_nn, g.z_pk, g.z_nn,
                g.z_nn, g.z_wr)
        };
        unsafe {
            speedtest1_prepare(c"INSERT INTO t3 VALUES(?1,?2,?3); -- %d times".as_ptr()
                        as *mut i8 as *const i8, n)
        };
        {
            i = 1;
            '__b17: loop {
                if !(i <= n) { break '__b17; }
                '__c17: loop {
                    x1 = swizzle(i as u32, maxb as u32);
                    speedtest1_numbername(x1,
                        &raw mut z_num[0 as usize] as *mut i8,
                        core::mem::size_of::<[i8; 2000]>() as i32);
                    unsafe { sqlite3_bind_int(g.p_stmt, 2, i) };
                    unsafe {
                        sqlite3_bind_int64(g.p_stmt, 1, x1 as Sqlite3Int64)
                    };
                    unsafe {
                        sqlite3_bind_text(g.p_stmt, 3,
                            &raw mut z_num[0 as usize] as *mut i8 as *const i8, -1,
                            None)
                    };
                    speedtest1_run();
                    break '__c17;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            speedtest1_exec(c"COMMIT".as_ptr() as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        n = 25;
        unsafe {
            speedtest1_begin_test(130,
                c"%d SELECTS, numeric BETWEEN, unindexed".as_ptr() as *mut i8
                    as *const i8, n)
        };
        unsafe { speedtest1_exec(c"BEGIN".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            speedtest1_prepare(c"SELECT count(*), avg(b), sum(length(c)), group_concat(c) FROM z1\n WHERE b BETWEEN ?1 AND ?2; -- %d times".as_ptr()
                        as *mut i8 as *const i8, n)
        };
        {
            i = 1;
            '__b18: loop {
                if !(i <= n) { break '__b18; }
                '__c18: loop {
                    if (i - 1) % g.n_repeat == 0 {
                        x1 = speedtest1_random() % maxb as u32;
                        x2 =
                            speedtest1_random() % 10 as u32 + (sz / 5000) as u32 + x1;
                    }
                    unsafe { sqlite3_bind_int(g.p_stmt, 1, x1 as i32) };
                    unsafe { sqlite3_bind_int(g.p_stmt, 2, x2 as i32) };
                    speedtest1_run();
                    break '__c18;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            speedtest1_exec(c"COMMIT".as_ptr() as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        n = 10;
        unsafe {
            speedtest1_begin_test(140,
                c"%d SELECTS, LIKE, unindexed".as_ptr() as *mut i8 as
                    *const i8, n)
        };
        unsafe { speedtest1_exec(c"BEGIN".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            speedtest1_prepare(c"SELECT count(*), avg(b), sum(length(c)), group_concat(c) FROM z1\n WHERE c LIKE ?1; -- %d times".as_ptr()
                        as *mut i8 as *const i8, n)
        };
        {
            i = 1;
            '__b19: loop {
                if !(i <= n) { break '__b19; }
                '__c19: loop {
                    if (i - 1) % g.n_repeat == 0 {
                        x1 = speedtest1_random() % maxb as u32;
                        z_num[0 as usize] = '%' as i32 as i8;
                        len =
                            speedtest1_numbername(i as u32,
                                unsafe {
                                    (&raw mut z_num[0 as usize] as *mut i8).offset(1 as isize)
                                },
                                (core::mem::size_of::<[i8; 2000]>() as u64 - 2 as u64) as
                                    i32);
                        z_num[len as usize] = '%' as i32 as i8;
                        z_num[(len + 1) as usize] = 0 as i8;
                    }
                    unsafe {
                        sqlite3_bind_text(g.p_stmt, 1,
                            &raw mut z_num[0 as usize] as *mut i8 as *const i8, len + 1,
                            None)
                    };
                    speedtest1_run();
                    break '__c19;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            speedtest1_exec(c"COMMIT".as_ptr() as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        n = 10;
        unsafe {
            speedtest1_begin_test(142,
                c"%d SELECTS w/ORDER BY, unindexed".as_ptr() as *mut i8 as
                    *const i8, n)
        };
        unsafe { speedtest1_exec(c"BEGIN".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            speedtest1_prepare(c"SELECT a, b, c FROM z1 WHERE c LIKE ?1\n ORDER BY a; -- %d times".as_ptr()
                        as *mut i8 as *const i8, n)
        };
        {
            i = 1;
            '__b20: loop {
                if !(i <= n) { break '__b20; }
                '__c20: loop {
                    if (i - 1) % g.n_repeat == 0 {
                        x1 = speedtest1_random() % maxb as u32;
                        z_num[0 as usize] = '%' as i32 as i8;
                        len =
                            speedtest1_numbername(i as u32,
                                unsafe {
                                    (&raw mut z_num[0 as usize] as *mut i8).offset(1 as isize)
                                },
                                (core::mem::size_of::<[i8; 2000]>() as u64 - 2 as u64) as
                                    i32);
                        z_num[len as usize] = '%' as i32 as i8;
                        z_num[(len + 1) as usize] = 0 as i8;
                    }
                    unsafe {
                        sqlite3_bind_text(g.p_stmt, 1,
                            &raw mut z_num[0 as usize] as *mut i8 as *const i8, len + 1,
                            None)
                    };
                    speedtest1_run();
                    break '__c20;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            speedtest1_exec(c"COMMIT".as_ptr() as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        n = 10;
        unsafe {
            speedtest1_begin_test(145,
                c"%d SELECTS w/ORDER BY and LIMIT, unindexed".as_ptr() as
                        *mut i8 as *const i8, n)
        };
        unsafe { speedtest1_exec(c"BEGIN".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            speedtest1_prepare(c"SELECT a, b, c FROM z1 WHERE c LIKE ?1\n ORDER BY a LIMIT 10; -- %d times".as_ptr()
                        as *mut i8 as *const i8, n)
        };
        {
            i = 1;
            '__b21: loop {
                if !(i <= n) { break '__b21; }
                '__c21: loop {
                    if (i - 1) % g.n_repeat == 0 {
                        x1 = speedtest1_random() % maxb as u32;
                        z_num[0 as usize] = '%' as i32 as i8;
                        len =
                            speedtest1_numbername(i as u32,
                                unsafe {
                                    (&raw mut z_num[0 as usize] as *mut i8).offset(1 as isize)
                                },
                                (core::mem::size_of::<[i8; 2000]>() as u64 - 2 as u64) as
                                    i32);
                        z_num[len as usize] = '%' as i32 as i8;
                        z_num[(len + 1) as usize] = 0 as i8;
                    }
                    unsafe {
                        sqlite3_bind_text(g.p_stmt, 1,
                            &raw mut z_num[0 as usize] as *mut i8 as *const i8, len + 1,
                            None)
                    };
                    speedtest1_run();
                    break '__c21;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            speedtest1_exec(c"COMMIT".as_ptr() as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(150,
                c"CREATE INDEX five times".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_exec(c"BEGIN;".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_exec(c"CREATE UNIQUE INDEX t1b ON z1(b);".as_ptr() as
                        *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_exec(c"CREATE INDEX t1c ON z1(c);".as_ptr() as *mut i8
                    as *const i8)
        };
        unsafe {
            speedtest1_exec(c"CREATE UNIQUE INDEX t2b ON z2(b);".as_ptr() as
                        *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_exec(c"CREATE INDEX t2c ON z2(c DESC);".as_ptr() as
                        *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_exec(c"CREATE INDEX t3bc ON t3(b,c);".as_ptr() as
                        *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_exec(c"COMMIT;".as_ptr() as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        n = sz / 5;
        unsafe {
            speedtest1_begin_test(160,
                c"%d SELECTS, numeric BETWEEN, indexed".as_ptr() as *mut i8 as
                    *const i8, n)
        };
        unsafe { speedtest1_exec(c"BEGIN".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            speedtest1_prepare(c"SELECT count(*), avg(b), sum(length(c)), group_concat(a) FROM z1\n WHERE b BETWEEN ?1 AND ?2; -- %d times".as_ptr()
                        as *mut i8 as *const i8, n)
        };
        {
            i = 1;
            '__b22: loop {
                if !(i <= n) { break '__b22; }
                '__c22: loop {
                    if (i - 1) % g.n_repeat == 0 {
                        x1 = speedtest1_random() % maxb as u32;
                        x2 =
                            speedtest1_random() % 10 as u32 + (sz / 5000) as u32 + x1;
                    }
                    unsafe { sqlite3_bind_int(g.p_stmt, 1, x1 as i32) };
                    unsafe { sqlite3_bind_int(g.p_stmt, 2, x2 as i32) };
                    speedtest1_run();
                    break '__c22;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            speedtest1_exec(c"COMMIT".as_ptr() as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        n = sz / 5;
        unsafe {
            speedtest1_begin_test(161,
                c"%d SELECTS, numeric BETWEEN, PK".as_ptr() as *mut i8 as
                    *const i8, n)
        };
        unsafe { speedtest1_exec(c"BEGIN".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            speedtest1_prepare(c"SELECT count(*), avg(b), sum(length(c)), group_concat(a) FROM z2\n WHERE a BETWEEN ?1 AND ?2; -- %d times".as_ptr()
                        as *mut i8 as *const i8, n)
        };
        {
            i = 1;
            '__b23: loop {
                if !(i <= n) { break '__b23; }
                '__c23: loop {
                    if (i - 1) % g.n_repeat == 0 {
                        x1 = speedtest1_random() % maxb as u32;
                        x2 =
                            speedtest1_random() % 10 as u32 + (sz / 5000) as u32 + x1;
                    }
                    unsafe { sqlite3_bind_int(g.p_stmt, 1, x1 as i32) };
                    unsafe { sqlite3_bind_int(g.p_stmt, 2, x2 as i32) };
                    speedtest1_run();
                    break '__c23;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            speedtest1_exec(c"COMMIT".as_ptr() as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        n = sz / 5;
        unsafe {
            speedtest1_begin_test(170,
                c"%d SELECTS, text BETWEEN, indexed".as_ptr() as *mut i8 as
                    *const i8, n)
        };
        unsafe { speedtest1_exec(c"BEGIN".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            speedtest1_prepare(c"SELECT count(*), avg(b), sum(length(c)), group_concat(a) FROM z1\n WHERE c BETWEEN ?1 AND (?1||\'~\'); -- %d times".as_ptr()
                        as *mut i8 as *const i8, n)
        };
        {
            i = 1;
            '__b24: loop {
                if !(i <= n) { break '__b24; }
                '__c24: loop {
                    if (i - 1) % g.n_repeat == 0 {
                        x1 = swizzle(i as u32, maxb as u32);
                        len =
                            speedtest1_numbername(x1,
                                &raw mut z_num[0 as usize] as *mut i8,
                                (core::mem::size_of::<[i8; 2000]>() as u64 - 1 as u64) as
                                    i32);
                    }
                    unsafe {
                        sqlite3_bind_text(g.p_stmt, 1,
                            &raw mut z_num[0 as usize] as *mut i8 as *const i8, len,
                            None)
                    };
                    speedtest1_run();
                    break '__c24;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            speedtest1_exec(c"COMMIT".as_ptr() as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        n = sz;
        unsafe {
            speedtest1_begin_test(180,
                c"%d INSERTS with three indexes".as_ptr() as *mut i8 as
                    *const i8, n)
        };
        unsafe { speedtest1_exec(c"BEGIN".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            speedtest1_exec(c"CREATE%s TABLE t4(\n  a INTEGER %s %s,\n  b INTEGER %s,\n  c TEXT %s\n) %s".as_ptr()
                        as *mut i8 as *const i8, is_temp(1), g.z_nn, g.z_pk, g.z_nn,
                g.z_nn, g.z_wr)
        };
        unsafe {
            speedtest1_exec(c"CREATE INDEX t4b ON t4(b)".as_ptr() as *mut i8
                    as *const i8)
        };
        unsafe {
            speedtest1_exec(c"CREATE INDEX t4c ON t4(c)".as_ptr() as *mut i8
                    as *const i8)
        };
        unsafe {
            speedtest1_exec(c"INSERT INTO t4 SELECT * FROM z1".as_ptr() as
                        *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_exec(c"COMMIT".as_ptr() as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        n = sz;
        unsafe {
            speedtest1_begin_test(190,
                c"DELETE and REFILL one table".as_ptr() as *mut i8 as
                    *const i8, n)
        };
        unsafe {
            speedtest1_exec(c"DELETE FROM z2;".as_ptr() as *mut i8 as
                    *const i8)
        };
        unsafe {
            speedtest1_exec(c"INSERT INTO z2 SELECT * FROM z1;".as_ptr() as
                        *mut i8 as *const i8)
        };
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(200,
                c"VACUUM".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_exec(c"VACUUM".as_ptr() as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(210,
                c"ALTER TABLE ADD COLUMN, and query".as_ptr() as *mut i8 as
                    *const i8)
        };
        unsafe {
            speedtest1_exec(c"ALTER TABLE z2 ADD COLUMN d INT DEFAULT 123".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_exec(c"SELECT sum(d) FROM z2".as_ptr() as *mut i8 as
                    *const i8)
        };
        speedtest1_end_test();
        n = sz / 5;
        unsafe {
            speedtest1_begin_test(230,
                c"%d UPDATES, numeric BETWEEN, indexed".as_ptr() as *mut i8 as
                    *const i8, n)
        };
        unsafe { speedtest1_exec(c"BEGIN".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            speedtest1_prepare(c"UPDATE z2 SET d=b*2 WHERE b BETWEEN ?1 AND ?2; -- %d times".as_ptr()
                        as *mut i8 as *const i8, n)
        };
        {
            i = 1;
            '__b25: loop {
                if !(i <= n) { break '__b25; }
                '__c25: loop {
                    x1 = speedtest1_random() % maxb as u32;
                    x2 =
                        speedtest1_random() % 10 as u32 + (sz / 5000) as u32 + x1;
                    unsafe { sqlite3_bind_int(g.p_stmt, 1, x1 as i32) };
                    unsafe { sqlite3_bind_int(g.p_stmt, 2, x2 as i32) };
                    speedtest1_run();
                    break '__c25;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            speedtest1_exec(c"COMMIT".as_ptr() as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        n = sz;
        unsafe {
            speedtest1_begin_test(240,
                c"%d UPDATES of individual rows".as_ptr() as *mut i8 as
                    *const i8, n)
        };
        unsafe { speedtest1_exec(c"BEGIN".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            speedtest1_prepare(c"UPDATE z2 SET d=b*3 WHERE a=?1; -- %d times".as_ptr()
                        as *mut i8 as *const i8, n)
        };
        {
            i = 1;
            '__b26: loop {
                if !(i <= n) { break '__b26; }
                '__c26: loop {
                    x1 = speedtest1_random() % sz as u32 + 1 as u32;
                    unsafe { sqlite3_bind_int(g.p_stmt, 1, x1 as i32) };
                    speedtest1_run();
                    break '__c26;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            speedtest1_exec(c"COMMIT".as_ptr() as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(250,
                c"One big UPDATE of the whole %d-row table".as_ptr() as
                        *mut i8 as *const i8, sz)
        };
        unsafe {
            speedtest1_exec(c"UPDATE z2 SET d=b*4".as_ptr() as *mut i8 as
                    *const i8)
        };
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(260,
                c"Query added column after filling".as_ptr() as *mut i8 as
                    *const i8)
        };
        unsafe {
            speedtest1_exec(c"SELECT sum(d) FROM z2".as_ptr() as *mut i8 as
                    *const i8)
        };
        speedtest1_end_test();
        n = sz / 5;
        unsafe {
            speedtest1_begin_test(270,
                c"%d DELETEs, numeric BETWEEN, indexed".as_ptr() as *mut i8 as
                    *const i8, n)
        };
        unsafe { speedtest1_exec(c"BEGIN".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            speedtest1_prepare(c"DELETE FROM z2 WHERE b BETWEEN ?1 AND ?2; -- %d times".as_ptr()
                        as *mut i8 as *const i8, n)
        };
        {
            i = 1;
            '__b27: loop {
                if !(i <= n) { break '__b27; }
                '__c27: loop {
                    x1 = speedtest1_random() % maxb as u32 + 1 as u32;
                    x2 =
                        speedtest1_random() % 10 as u32 + (sz / 5000) as u32 + x1;
                    unsafe { sqlite3_bind_int(g.p_stmt, 1, x1 as i32) };
                    unsafe { sqlite3_bind_int(g.p_stmt, 2, x2 as i32) };
                    speedtest1_run();
                    break '__c27;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            speedtest1_exec(c"COMMIT".as_ptr() as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        n = sz;
        unsafe {
            speedtest1_begin_test(280,
                c"%d DELETEs of individual rows".as_ptr() as *mut i8 as
                    *const i8, n)
        };
        unsafe { speedtest1_exec(c"BEGIN".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            speedtest1_prepare(c"DELETE FROM t3 WHERE a=?1; -- %d times".as_ptr()
                        as *mut i8 as *const i8, n)
        };
        {
            i = 1;
            '__b28: loop {
                if !(i <= n) { break '__b28; }
                '__c28: loop {
                    x1 = speedtest1_random() % sz as u32 + 1 as u32;
                    unsafe { sqlite3_bind_int(g.p_stmt, 1, x1 as i32) };
                    speedtest1_run();
                    break '__c28;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            speedtest1_exec(c"COMMIT".as_ptr() as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(290,
                c"Refill two %d-row tables using REPLACE".as_ptr() as *mut i8
                    as *const i8, sz)
        };
        unsafe {
            speedtest1_exec(c"REPLACE INTO z2(a,b,c) SELECT a,b,c FROM z1".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_exec(c"REPLACE INTO t3(a,b,c) SELECT a,b,c FROM z1".as_ptr()
                        as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(300,
                c"Refill a %d-row table using (b&1)==(a&1)".as_ptr() as
                        *mut i8 as *const i8, sz)
        };
        unsafe {
            speedtest1_exec(c"DELETE FROM z2;".as_ptr() as *mut i8 as
                    *const i8)
        };
        unsafe {
            speedtest1_exec(c"INSERT INTO z2(a,b,c)\n SELECT a,b,c FROM z1  WHERE (b&1)==(a&1);".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_exec(c"INSERT INTO z2(a,b,c)\n SELECT a,b,c FROM z1  WHERE (b&1)<>(a&1);".as_ptr()
                        as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        n = sz / 5;
        unsafe {
            speedtest1_begin_test(310,
                c"%d four-ways joins".as_ptr() as *mut i8 as *const i8, n)
        };
        unsafe { speedtest1_exec(c"BEGIN".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            speedtest1_prepare(c"SELECT z1.c FROM z1, z2, t3, t4\n WHERE t4.a BETWEEN ?1 AND ?2\n   AND t3.a=t4.b\n   AND z2.a=t3.b\n   AND z1.c=z2.c;".as_ptr()
                        as *mut i8 as *const i8)
        };
        {
            i = 1;
            '__b29: loop {
                if !(i <= n) { break '__b29; }
                '__c29: loop {
                    x1 = speedtest1_random() % sz as u32 + 1 as u32;
                    x2 = speedtest1_random() % 10 as u32 + x1 + 4 as u32;
                    unsafe { sqlite3_bind_int(g.p_stmt, 1, x1 as i32) };
                    unsafe { sqlite3_bind_int(g.p_stmt, 2, x2 as i32) };
                    speedtest1_run();
                    break '__c29;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            speedtest1_exec(c"COMMIT".as_ptr() as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(320,
                c"subquery in result set".as_ptr() as *mut i8 as *const i8, n)
        };
        unsafe {
            speedtest1_prepare(c"SELECT sum(a), max(c),\n       avg((SELECT a FROM z2 WHERE 5+z2.b=z1.b) AND rowid<?1), max(c)\n FROM z1 WHERE rowid<?1;".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            sqlite3_bind_int(g.p_stmt, 1, est_square_root(g.sz_test) * 50)
        };
        speedtest1_run();
        speedtest1_end_test();
        sz = { n = g.sz_test * 700; n };
        z_num[0 as usize] = 0 as i8;
        maxb = roundup_allones((sz / 3) as u32) as i32;
        unsafe {
            speedtest1_begin_test(400,
                c"%d REPLACE ops on an IPK".as_ptr() as *mut i8 as *const i8,
                n)
        };
        unsafe { speedtest1_exec(c"BEGIN".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            speedtest1_exec(c"CREATE%s TABLE t5(a INTEGER PRIMARY KEY, b %s);".as_ptr()
                        as *mut i8 as *const i8, is_temp(9), g.z_nn)
        };
        unsafe {
            speedtest1_prepare(c"REPLACE INTO t5 VALUES(?1,?2); --  %d times".as_ptr()
                        as *mut i8 as *const i8, n)
        };
        {
            i = 1;
            '__b30: loop {
                if !(i <= n) { break '__b30; }
                '__c30: loop {
                    x1 = swizzle(i as u32, maxb as u32);
                    speedtest1_numbername(i as u32,
                        &raw mut z_num[0 as usize] as *mut i8,
                        core::mem::size_of::<[i8; 2000]>() as i32);
                    unsafe {
                        sqlite3_bind_int(g.p_stmt, 1, x1 as Sqlite3Int64 as i32)
                    };
                    unsafe {
                        sqlite3_bind_text(g.p_stmt, 2,
                            &raw mut z_num[0 as usize] as *mut i8 as *const i8, -1,
                            None)
                    };
                    speedtest1_run();
                    break '__c30;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            speedtest1_exec(c"COMMIT".as_ptr() as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(410,
                c"%d SELECTS on an IPK".as_ptr() as *mut i8 as *const i8, n)
        };
        if g.do_big_transactions != 0 {
            unsafe {
                speedtest1_exec(c"BEGIN".as_ptr() as *mut i8 as *const i8)
            };
        }
        unsafe {
            speedtest1_prepare(c"SELECT b FROM t5 WHERE a=?1; --  %d times".as_ptr()
                        as *mut i8 as *const i8, n)
        };
        {
            i = 1;
            '__b31: loop {
                if !(i <= n) { break '__b31; }
                '__c31: loop {
                    x1 = swizzle(i as u32, maxb as u32);
                    unsafe {
                        sqlite3_bind_int(g.p_stmt, 1, x1 as Sqlite3Int64 as i32)
                    };
                    speedtest1_run();
                    break '__c31;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if g.do_big_transactions != 0 {
            unsafe {
                speedtest1_exec(c"COMMIT".as_ptr() as *mut i8 as *const i8)
            };
        }
        speedtest1_end_test();
        sz = { n = g.sz_test * 700; n };
        z_num[0 as usize] = 0 as i8;
        maxb = roundup_allones((sz / 3) as u32) as i32;
        unsafe {
            speedtest1_begin_test(500,
                c"%d REPLACE on TEXT PK".as_ptr() as *mut i8 as *const i8, n)
        };
        unsafe { speedtest1_exec(c"BEGIN".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            speedtest1_exec(c"CREATE%s TABLE t6(a TEXT PRIMARY KEY, b %s)%s;".as_ptr()
                        as *mut i8 as *const i8, is_temp(9), g.z_nn,
                if unsafe { sqlite3_libversion_number() } >= 3008002 {
                    c"WITHOUT ROWID".as_ptr() as *mut i8
                } else { c"".as_ptr() as *mut i8 })
        };
        unsafe {
            speedtest1_prepare(c"REPLACE INTO t6 VALUES(?1,?2); --  %d times".as_ptr()
                        as *mut i8 as *const i8, n)
        };
        {
            i = 1;
            '__b32: loop {
                if !(i <= n) { break '__b32; }
                '__c32: loop {
                    x1 = swizzle(i as u32, maxb as u32);
                    speedtest1_numbername(x1,
                        &raw mut z_num[0 as usize] as *mut i8,
                        core::mem::size_of::<[i8; 2000]>() as i32);
                    unsafe { sqlite3_bind_int(g.p_stmt, 2, i) };
                    unsafe {
                        sqlite3_bind_text(g.p_stmt, 1,
                            &raw mut z_num[0 as usize] as *mut i8 as *const i8, -1,
                            None)
                    };
                    speedtest1_run();
                    break '__c32;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            speedtest1_exec(c"COMMIT".as_ptr() as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(510,
                c"%d SELECTS on a TEXT PK".as_ptr() as *mut i8 as *const i8,
                n)
        };
        if g.do_big_transactions != 0 {
            unsafe {
                speedtest1_exec(c"BEGIN".as_ptr() as *mut i8 as *const i8)
            };
        }
        unsafe {
            speedtest1_prepare(c"SELECT b FROM t6 WHERE a=?1; --  %d times".as_ptr()
                        as *mut i8 as *const i8, n)
        };
        {
            i = 1;
            '__b33: loop {
                if !(i <= n) { break '__b33; }
                '__c33: loop {
                    x1 = swizzle(i as u32, maxb as u32);
                    speedtest1_numbername(x1,
                        &raw mut z_num[0 as usize] as *mut i8,
                        core::mem::size_of::<[i8; 2000]>() as i32);
                    unsafe {
                        sqlite3_bind_text(g.p_stmt, 1,
                            &raw mut z_num[0 as usize] as *mut i8 as *const i8, -1,
                            None)
                    };
                    speedtest1_run();
                    break '__c33;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if g.do_big_transactions != 0 {
            unsafe {
                speedtest1_exec(c"COMMIT".as_ptr() as *mut i8 as *const i8)
            };
        }
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(520,
                c"%d SELECT DISTINCT".as_ptr() as *mut i8 as *const i8, n)
        };
        unsafe {
            speedtest1_exec(c"SELECT DISTINCT b FROM t5;".as_ptr() as *mut i8
                    as *const i8)
        };
        unsafe {
            speedtest1_exec(c"SELECT DISTINCT b FROM t6;".as_ptr() as *mut i8
                    as *const i8)
        };
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(980,
                c"PRAGMA integrity_check".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_exec(c"PRAGMA integrity_check".as_ptr() as *mut i8 as
                    *const i8)
        };
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(990,
                c"ANALYZE".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_exec(c"ANALYZE".as_ptr() as *mut i8 as *const i8)
        };
        speedtest1_end_test();
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn testset_cte() -> () {
    unsafe {
        let mut z_puz: *const i8 = core::ptr::null();
        let mut r_spacing: f64 = 0.0;
        let mut n_elem: i32 = 0;
        if g.sz_test < 25 {
            z_puz = az_puzzle[0 as usize];
        } else if g.sz_test < 70 {
            z_puz = az_puzzle[1 as usize];
        } else { z_puz = az_puzzle[2 as usize]; }
        unsafe {
            speedtest1_begin_test(100,
                c"Sudoku with recursive \'digits\'".as_ptr() as *mut i8 as
                    *const i8)
        };
        unsafe {
            speedtest1_prepare(c"WITH RECURSIVE\n  input(sud) AS (VALUES(?1)),\n  digits(z,lp) AS (\n    VALUES(\'1\', 1)\n    UNION ALL\n    SELECT CAST(lp+1 AS TEXT), lp+1 FROM digits WHERE lp<9\n  ),\n  x(s, ind) AS (\n    SELECT sud, instr(sud, \'.\') FROM input\n    UNION ALL\n    SELECT\n      substr(s, 1, ind-1) || z || substr(s, ind+1),\n      instr( substr(s, 1, ind-1) || z || substr(s, ind+1), \'.\' )\n     FROM x, digits AS z\n    WHERE ind>0\n      AND NOT EXISTS (\n            SELECT 1\n              FROM digits AS lp\n             WHERE z.z = substr(s, ((ind-1)/9)*9 + lp, 1)\n                OR z.z = substr(s, ((ind-1)%%9) + (lp-1)*9 + 1, 1)\n                OR z.z = substr(s, (((ind-1)/3) %% 3) * 3\n                        + ((ind-1)/27) * 27 + lp\n                        + ((lp-1) / 3) * 6, 1)\n         )\n  )\nSELECT s FROM x WHERE ind=0;".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe { sqlite3_bind_text(g.p_stmt, 1, z_puz, -1, None) };
        speedtest1_run();
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(200,
                c"Sudoku with VALUES \'digits\'".as_ptr() as *mut i8 as
                    *const i8)
        };
        unsafe {
            speedtest1_prepare(c"WITH RECURSIVE\n  input(sud) AS (VALUES(?1)),\n  digits(z,lp) AS (VALUES(\'1\',1),(\'2\',2),(\'3\',3),(\'4\',4),(\'5\',5),\n                         (\'6\',6),(\'7\',7),(\'8\',8),(\'9\',9)),\n  x(s, ind) AS (\n    SELECT sud, instr(sud, \'.\') FROM input\n    UNION ALL\n    SELECT\n      substr(s, 1, ind-1) || z || substr(s, ind+1),\n      instr( substr(s, 1, ind-1) || z || substr(s, ind+1), \'.\' )\n     FROM x, digits AS z\n    WHERE ind>0\n      AND NOT EXISTS (\n            SELECT 1\n              FROM digits AS lp\n             WHERE z.z = substr(s, ((ind-1)/9)*9 + lp, 1)\n                OR z.z = substr(s, ((ind-1)%%9) + (lp-1)*9 + 1, 1)\n                OR z.z = substr(s, (((ind-1)/3) %% 3) * 3\n                        + ((ind-1)/27) * 27 + lp\n                        + ((lp-1) / 3) * 6, 1)\n         )\n  )\nSELECT s FROM x WHERE ind=0;".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe { sqlite3_bind_text(g.p_stmt, 1, z_puz, -1, None) };
        speedtest1_run();
        speedtest1_end_test();
        r_spacing = 5.0 / g.sz_test as f64;
        unsafe {
            speedtest1_begin_test(300,
                c"Mandelbrot Set with spacing=%f".as_ptr() as *mut i8 as
                    *const i8, r_spacing)
        };
        unsafe {
            speedtest1_prepare(c"WITH RECURSIVE \n  xaxis(x) AS (VALUES(-2.0) UNION ALL SELECT x+?1 FROM xaxis WHERE x<1.2),\n  yaxis(y) AS (VALUES(-1.0) UNION ALL SELECT y+?2 FROM yaxis WHERE y<1.0),\n  m(iter, cx, cy, x, y) AS (\n    SELECT 0, x, y, 0.0, 0.0 FROM xaxis, yaxis\n    UNION ALL\n    SELECT iter+1, cx, cy, x*x-y*y + cx, 2.0*x*y + cy FROM m \n     WHERE (x*x + y*y) < 4.0 AND iter<28\n  ),\n  m2(iter, cx, cy) AS (\n    SELECT max(iter), cx, cy FROM m GROUP BY cx, cy\n  ),\n  a(t) AS (\n    SELECT group_concat( substr(\' .+*#\', 1+min(iter/7,4), 1), \'\') \n    FROM m2 GROUP BY cy\n  )\nSELECT group_concat(rtrim(t),x\'0a\') FROM a;".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe { sqlite3_bind_double(g.p_stmt, 1, r_spacing * 0.05) };
        unsafe { sqlite3_bind_double(g.p_stmt, 2, r_spacing) };
        speedtest1_run();
        speedtest1_end_test();
        n_elem = 10000 * g.sz_test;
        unsafe {
            speedtest1_begin_test(400,
                c"EXCEPT operator on %d-element tables".as_ptr() as *mut i8 as
                    *const i8, n_elem)
        };
        unsafe {
            speedtest1_prepare(c"WITH RECURSIVE \n  z1(x) AS (VALUES(2) UNION ALL SELECT x+2 FROM z1 WHERE x<%d),\n  z2(y) AS (VALUES(3) UNION ALL SELECT y+3 FROM z2 WHERE y<%d)\nSELECT count(x), avg(x) FROM (\n  SELECT x FROM z1 EXCEPT SELECT y FROM z2 ORDER BY 1\n);".as_ptr()
                        as *mut i8 as *const i8, n_elem, n_elem)
        };
        speedtest1_run();
        speedtest1_end_test();
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn speedtest1_random_ascii_fp(z_fp_1: *mut i8) -> () {
    let x: i32 = speedtest1_random() as i32;
    let mut y: i32 = speedtest1_random() as i32;
    let mut z: i32 = 0;
    z = y % 10;
    if z < 0 { z = -z; }
    y /= 10;
    unsafe {
        sqlite3_snprintf(100, z_fp_1,
            c"%d.%de%d".as_ptr() as *mut i8 as *const i8, y, z, x % 200)
    };
}
#[unsafe(no_mangle)]
pub extern "C" fn testset_fp() -> () {
    unsafe {
        let mut n: i32 = 0;
        let mut i: i32 = 0;
        let mut z_fp1: [i8; 100] = [0; 100];
        let mut z_fp2: [i8; 100] = [0; 100];
        n = g.sz_test * 5000;
        unsafe {
            speedtest1_begin_test(100,
                c"Fill a table with %d FP values".as_ptr() as *mut i8 as
                    *const i8, n * 2)
        };
        unsafe { speedtest1_exec(c"BEGIN".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            speedtest1_exec(c"CREATE%s TABLE z1(a REAL %s, b REAL %s);".as_ptr()
                        as *mut i8 as *const i8, is_temp(1), g.z_nn, g.z_nn)
        };
        unsafe {
            speedtest1_prepare(c"INSERT INTO z1 VALUES(?1,?2); -- %d times".as_ptr()
                        as *mut i8 as *const i8, n)
        };
        {
            i = 1;
            '__b34: loop {
                if !(i <= n) { break '__b34; }
                '__c34: loop {
                    speedtest1_random_ascii_fp(&raw mut z_fp1[0 as usize] as
                            *mut i8);
                    speedtest1_random_ascii_fp(&raw mut z_fp2[0 as usize] as
                            *mut i8);
                    unsafe {
                        sqlite3_bind_text(g.p_stmt, 1,
                            &raw mut z_fp1[0 as usize] as *mut i8 as *const i8, -1,
                            None)
                    };
                    unsafe {
                        sqlite3_bind_text(g.p_stmt, 2,
                            &raw mut z_fp2[0 as usize] as *mut i8 as *const i8, -1,
                            None)
                    };
                    speedtest1_run();
                    break '__c34;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            speedtest1_exec(c"COMMIT".as_ptr() as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        n = g.sz_test / 25 + 2;
        unsafe {
            speedtest1_begin_test(110,
                c"%d range queries".as_ptr() as *mut i8 as *const i8, n)
        };
        unsafe {
            speedtest1_prepare(c"SELECT sum(b) FROM z1 WHERE a BETWEEN ?1 AND ?2".as_ptr()
                        as *mut i8 as *const i8)
        };
        {
            i = 1;
            '__b35: loop {
                if !(i <= n) { break '__b35; }
                '__c35: loop {
                    speedtest1_random_ascii_fp(&raw mut z_fp1[0 as usize] as
                            *mut i8);
                    speedtest1_random_ascii_fp(&raw mut z_fp2[0 as usize] as
                            *mut i8);
                    unsafe {
                        sqlite3_bind_text(g.p_stmt, 1,
                            &raw mut z_fp1[0 as usize] as *mut i8 as *const i8, -1,
                            None)
                    };
                    unsafe {
                        sqlite3_bind_text(g.p_stmt, 2,
                            &raw mut z_fp2[0 as usize] as *mut i8 as *const i8, -1,
                            None)
                    };
                    speedtest1_run();
                    break '__c35;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(120,
                c"CREATE INDEX three times".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_exec(c"BEGIN;".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_exec(c"CREATE INDEX t1a ON z1(a);".as_ptr() as *mut i8
                    as *const i8)
        };
        unsafe {
            speedtest1_exec(c"CREATE INDEX t1b ON z1(b);".as_ptr() as *mut i8
                    as *const i8)
        };
        unsafe {
            speedtest1_exec(c"CREATE INDEX t1ab ON z1(a,b);".as_ptr() as
                        *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_exec(c"COMMIT;".as_ptr() as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        n = g.sz_test / 3 + 2;
        unsafe {
            speedtest1_begin_test(130,
                c"%d indexed range queries".as_ptr() as *mut i8 as *const i8,
                n)
        };
        unsafe {
            speedtest1_prepare(c"SELECT sum(b) FROM z1 WHERE a BETWEEN ?1 AND ?2".as_ptr()
                        as *mut i8 as *const i8)
        };
        {
            i = 1;
            '__b36: loop {
                if !(i <= n) { break '__b36; }
                '__c36: loop {
                    speedtest1_random_ascii_fp(&raw mut z_fp1[0 as usize] as
                            *mut i8);
                    speedtest1_random_ascii_fp(&raw mut z_fp2[0 as usize] as
                            *mut i8);
                    unsafe {
                        sqlite3_bind_text(g.p_stmt, 1,
                            &raw mut z_fp1[0 as usize] as *mut i8 as *const i8, -1,
                            None)
                    };
                    unsafe {
                        sqlite3_bind_text(g.p_stmt, 2,
                            &raw mut z_fp2[0 as usize] as *mut i8 as *const i8, -1,
                            None)
                    };
                    speedtest1_run();
                    break '__c36;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        speedtest1_end_test();
        n = g.sz_test * 5000;
        unsafe {
            speedtest1_begin_test(140,
                c"%d calls to round()".as_ptr() as *mut i8 as *const i8, n)
        };
        unsafe {
            speedtest1_exec(c"SELECT sum(round(a,2)+round(b,4)) FROM z1;".as_ptr()
                        as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(150,
                c"%d printf() calls".as_ptr() as *mut i8 as *const i8, n * 4)
        };
        unsafe {
            speedtest1_exec(c"WITH c(fmt) AS (VALUES(\'%%g\'),(\'%%e\'),(\'%%!g\'),(\'%%.20f\'))SELECT sum(printf(fmt,a)) FROM z1, c".as_ptr()
                        as *mut i8 as *const i8)
        };
        speedtest1_end_test();
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn testset_star() -> () {
    unsafe {
        let mut n: i32 = 0;
        let mut i: i32 = 0;
        n = g.sz_test * 50;
        unsafe {
            speedtest1_begin_test(100,
                c"Create a fact table with %d entries".as_ptr() as *mut i8 as
                    *const i8, n)
        };
        unsafe {
            speedtest1_exec(c"CREATE TABLE facttab( attr01 INT, attr02 INT, attr03 INT, data01 TEXT, attr04 INT, attr05 INT, attr06 INT, attr07 INT, attr08 INT, factid INTEGER PRIMARY KEY, data02 TEXT);".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_exec(c"WITH RECURSIVE counter(nnn) AS(VALUES(1) UNION ALL SELECT nnn+1 FROM counter WHERE nnn<%d)INSERT INTO facttab(attr01,attr02,attr03,attr04,attr05,attr06,attr07,attr08,data01,data02)SELECT random()%%12, random()%%13, random()%%14, random()%%15,random()%%16, random()%%17, random()%%18, random()%%19,concat(\'data-\',nnn), format(\'%%x\',random()) FROM counter;".as_ptr()
                        as *mut i8 as *const i8, n)
        };
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(110,
                c"Create indexes on all attributes columns".as_ptr() as
                        *mut i8 as *const i8)
        };
        {
            i = 1;
            '__b37: loop {
                if !(i <= 8) { break '__b37; }
                '__c37: loop {
                    unsafe {
                        speedtest1_exec(c"CREATE INDEX fact_attr%02d ON facttab(attr%02d)".as_ptr()
                                    as *mut i8 as *const i8, i, i)
                    };
                    break '__c37;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(120,
                c"Create dimension tables".as_ptr() as *mut i8 as *const i8)
        };
        {
            i = 1;
            '__b38: loop {
                if !(i <= 8) { break '__b38; }
                '__c38: loop {
                    unsafe {
                        speedtest1_exec(c"CREATE TABLE dimension%02d(beta%02d INT, content%02d TEXT, rate%02d REAL)".as_ptr()
                                    as *mut i8 as *const i8, i, i, i, i)
                    };
                    unsafe {
                        speedtest1_exec(c"WITH RECURSIVE ctr(nn) AS (VALUES(1) UNION ALL SELECT nn+1 FROM ctr WHERE nn<%d) INSERT INTO dimension%02d   SELECT nn%%(%d), concat(\'content-%02d-\',nn), (random()%%10000)*0.125 FROM ctr;".as_ptr()
                                    as *mut i8 as *const i8, 4 * (i + 1), i, 2 * (i + 1), i)
                    };
                    if i & 2 != 0 {
                        unsafe {
                            speedtest1_exec(c"CREATE INDEX dim%02d ON dimension%02d(beta%02d);".as_ptr()
                                        as *mut i8 as *const i8, i, i, i)
                        };
                    } else {
                        unsafe {
                            speedtest1_exec(c"CREATE INDEX dim%02d ON dimension%02d(beta%02d,content%02d);".as_ptr()
                                        as *mut i8 as *const i8, i, i, i, i)
                        };
                    }
                    break '__c38;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(130,
                c"Star query over the entire fact table".as_ptr() as *mut i8
                    as *const i8)
        };
        unsafe {
            speedtest1_exec(c"SELECT count(*), max(content04), min(content03), sum(rate04), avg(rate05) FROM facttab, dimension01, dimension02, dimension03, dimension04, dimension05, dimension06, dimension07, dimension08 WHERE attr01=beta01 AND attr02=beta02 AND attr03=beta03 AND attr04=beta04 AND attr05=beta05 AND attr06=beta06 AND attr07=beta07 AND attr08=beta08;".as_ptr()
                        as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(130,
                c"Star query with LEFT JOINs".as_ptr() as *mut i8 as
                    *const i8)
        };
        unsafe {
            speedtest1_exec(c"SELECT count(*), max(content04), min(content03), sum(rate04), avg(rate05) FROM facttab LEFT JOIN dimension01 ON attr01=beta01 LEFT JOIN dimension02 ON attr02=beta02 JOIN dimension03 ON attr03=beta03 JOIN dimension04 ON attr04=beta04 JOIN dimension05 ON attr05=beta05 LEFT JOIN dimension06 ON attr06=beta06 JOIN dimension07 ON attr07=beta07 JOIN dimension08 ON attr08=beta08 WHERE facttab.data01 LIKE \'data-9%%\';".as_ptr()
                        as *mut i8 as *const i8)
        };
        speedtest1_end_test();
    }
}
extern "C" fn testset_app() -> () {
    unsafe {
        let mut i: i32 = 0;
        let mut n: i32 = 0;
        unsafe {
            speedtest1_begin_test(100,
                c"Generate a Fossil-like database schema".as_ptr() as *mut i8
                    as *const i8)
        };
        unsafe {
            speedtest1_exec(c"BEGIN;CREATE TABLE blob(\n  rid INTEGER PRIMARY KEY,\n  rcvid INTEGER,\n  size INTEGER,\n  uuid TEXT UNIQUE NOT NULL,\n  content BLOB,\n  CHECK( length(uuid)>=40 AND rid>0 )\n);\nCREATE TABLE delta(\n  rid INTEGER PRIMARY KEY,\n  srcid INTEGER NOT NULL REFERENCES blob\n);\nCREATE TABLE rcvfrom(\n  rcvid INTEGER PRIMARY KEY,\n  uid INTEGER REFERENCES user,\n  mtime DATETIME,\n  nonce TEXT UNIQUE,\n  ipaddr TEXT\n);\nCREATE TABLE private(rid INTEGER PRIMARY KEY);\nCREATE TABLE accesslog(\n  uname TEXT,\n  ipaddr TEXT,\n  success BOOLEAN,\n  mtime TIMESTAMP\n);\nCREATE TABLE user(\n  uid INTEGER PRIMARY KEY,\n  login TEXT UNIQUE,\n  pw TEXT,\n  cap TEXT,\n  cookie TEXT,\n  ipaddr TEXT,\n  cexpire DATETIME,\n  info TEXT,\n  mtime DATE,\n  photo BLOB\n, jx TEXT DEFAULT \'{}\');\nCREATE TABLE reportfmt(\n   rn INTEGER PRIMARY KEY,\n   owner TEXT,\n   title TEXT UNIQUE,\n   mtime INTEGER,\n   cols TEXT,\n   sqlcode TEXT\n, jx TEXT DEFAULT \'{}\');\nCREATE TABLE config(\n  name TEXT PRIMARY KEY NOT NULL,\n  value CLOB, mtime INTEGER,\n  CHECK( typeof(name)=\'text\' AND length(name)>=1 )\n) WITHOUT ROWID;\nCREATE TABLE shun(uuid PRIMARY KEY, mtime INTEGER, scom TEXT)\n  WITHOUT ROWID;\nCREATE TABLE concealed(\n  hash TEXT PRIMARY KEY,\n  content TEXT\n, mtime INTEGER) WITHOUT ROWID;\nCREATE TABLE admin_log(\n id INTEGER PRIMARY KEY,\n time INTEGER, -- Seconds since 1970\n page TEXT,    -- path of page\n who TEXT,     -- User who made the change\n  what TEXT     -- What changed\n);\nCREATE TABLE unversioned(\n  name TEXT PRIMARY KEY,\n  rcvid INTEGER,\n  mtime DATETIME,\n  hash TEXT,\n  sz INTEGER,\n  encoding INT,\n  content BLOB\n) WITHOUT ROWID;\nCREATE TABLE subscriber(\n  subscriberId INTEGER PRIMARY KEY,\n  subscriberCode BLOB DEFAULT (randomblob(32)) UNIQUE,\n  semail TEXT UNIQUE COLLATE nocase,\n  suname TEXT,\n  sverified BOOLEAN DEFAULT true,\n  sdonotcall BOOLEAN,\n  sdigest BOOLEAN,\n  ssub TEXT,\n  sctime INTDATE,\n  mtime INTDATE,\n  smip TEXT\n, lastContact INT);\nCREATE TABLE pending_alert(\n  eventid TEXT PRIMARY KEY,\n  sentSep BOOLEAN DEFAULT false,\n  sentDigest BOOLEAN DEFAULT false\n, sentMod BOOLEAN DEFAULT false) WITHOUT ROWID;\nCREATE TABLE filename(\n  fnid INTEGER PRIMARY KEY,\n  name TEXT UNIQUE\n) STRICT;\nCREATE TABLE mlink(\n  mid INTEGER,\n  fid INTEGER,\n  pmid INTEGER,\n  pid INTEGER,\n  fnid INTEGER REFERENCES filename,\n  pfnid INTEGER,\n  mperm INTEGER,\n  isaux INT DEFAULT 0\n) STRICT;\nCREATE TABLE plink(\n  pid INTEGER REFERENCES blob,\n  cid INTEGER REFERENCES blob,\n  isprim INT,\n  mtime REAL,\n  baseid INTEGER REFERENCES blob,\n  UNIQUE(pid, cid)\n) STRICT;\nCREATE TABLE leaf(rid INTEGER PRIMARY KEY);\nCREATE TABLE event(\n  type TEXT,\n  mtime REAL,\n  objid INTEGER PRIMARY KEY,\n  tagid INTEGER,\n  uid INTEGER REFERENCES user,\n  bgcolor TEXT,\n  euser TEXT,\n  user TEXT,\n  ecomment TEXT,\n  comment TEXT,\n  brief TEXT,\n  omtime REAL\n) STRICT;\nCREATE TABLE phantom(\n  rid INTEGER PRIMARY KEY\n);\nCREATE TABLE orphan(\n  rid INTEGER PRIMARY KEY,\n  baseline INTEGER\n) STRICT;\nCREATE TABLE unclustered(\n  rid INTEGER PRIMARY KEY\n);\nCREATE TABLE unsent(\n  rid INTEGER PRIMARY KEY\n);\nCREATE TABLE tag(\n  tagid INTEGER PRIMARY KEY,\n  tagname TEXT UNIQUE\n) STRICT;\nCREATE TABLE tagxref(\n  tagid INTEGER REFERENCES tag,\n  tagtype INTEGER,\n  srcid INTEGER REFERENCES blob,\n  origid INTEGER REFERENCES blob,\n  value TEXT,\n  mtime REAL,\n  rid INTEGER REFERENCES blob,\n  UNIQUE(rid, tagid)\n) STRICT;\nCREATE TABLE backlink(\n  target TEXT,\n  srctype INT,\n  srcid INT,\n  mtime REAL,\n  UNIQUE(target, srctype, srcid)\n) STRICT;\nCREATE TABLE attachment(\n  attachid INTEGER PRIMARY KEY,\n  isLatest INT DEFAULT 0,\n  mtime REAL,\n  src TEXT,\n  target TEXT,\n  filename TEXT,\n  comment TEXT,\n  user TEXT\n) STRICT;\nCREATE TABLE cherrypick(\n  parentid INT,\n  childid INT,\n  isExclude INT DEFAULT false,\n  PRIMARY KEY(parentid, childid)\n) WITHOUT ROWID, STRICT;\nCREATE TABLE vcache(\n  vid INTEGER,         -- check-in ID\n  fname TEXT,          -- filename\n  rid INTEGER,         -- artifact ID\n  PRIMARY KEY(vid,fname)\n) WITHOUT ROWID;\nCREATE TABLE synclog(\n  sfrom TEXT,\n  sto TEXT,\n  stime INT NOT NULL,\n  stype TEXT,\n  PRIMARY KEY(sfrom,sto)\n) WITHOUT ROWID;\nCREATE TABLE chat(\n  msgid INTEGER PRIMARY KEY AUTOINCREMENT,\n  mtime JULIANDAY,\n  lmtime TEXT,\n  xfrom TEXT,\n  xmsg  TEXT,\n  fname TEXT,\n  fmime TEXT,\n  mdel INT,\n  file  BLOB\n);\nCREATE TABLE ftsdocs(\n  rowid INTEGER PRIMARY KEY,\n  type CHAR(1),\n  rid INTEGER,\n  name TEXT,\n  idxed BOOLEAN,\n  label TEXT,\n  url TEXT,\n  mtime DATE,\n  bx TEXT,\n  UNIQUE(type,rid)\n);\nCREATE TABLE ticket(\n  -- Do not change any column that begins with tkt_\n  tkt_id INTEGER PRIMARY KEY,\n  tkt_uuid TEXT UNIQUE,\n  tkt_mtime DATE,\n  tkt_ctime DATE,\n  -- Add as many fields as required below this line\n  type TEXT,\n  status TEXT,\n  subsystem TEXT,\n  priority TEXT,\n  severity TEXT,\n  foundin TEXT,\n  private_contact TEXT,\n  resolution TEXT,\n  title TEXT,\n  comment TEXT\n);\nCREATE TABLE ticketchng(\n  -- Do not change any column that begins with tkt_\n  tkt_id INTEGER REFERENCES ticket,\n  tkt_rid INTEGER REFERENCES blob,\n  tkt_mtime DATE,\n  tkt_user TEXT,\n  -- Add as many fields as required below this line\n  login TEXT,\n  username TEXT,\n  mimetype TEXT,\n  icomment TEXT\n);\nCREATE TABLE forumpost(\n  fpid INTEGER PRIMARY KEY,\n  froot INT,\n  fprev INT,\n  firt INT,\n  fmtime REAL\n);\nCREATE INDEX delta_i1 ON delta(srcid);\nCREATE INDEX blob_rcvid ON blob(rcvid);\nCREATE INDEX subscriberUname\n  ON subscriber(suname) WHERE suname IS NOT NULL;\nCREATE INDEX mlink_i1 ON mlink(mid);\nCREATE INDEX mlink_i2 ON mlink(fnid);\nCREATE INDEX mlink_i3 ON mlink(fid);\nCREATE INDEX mlink_i4 ON mlink(pid);\nCREATE INDEX plink_i2 ON plink(cid,pid);\nCREATE INDEX event_i1 ON event(mtime);\nCREATE INDEX orphan_baseline ON orphan(baseline);\nCREATE INDEX tagxref_i1 ON tagxref(tagid, mtime);\nCREATE INDEX backlink_src ON backlink(srcid, srctype);\nCREATE INDEX attachment_idx1 ON attachment(target, filename, mtime);\nCREATE INDEX attachment_idx2 ON attachment(src);\nCREATE INDEX cherrypick_cid ON cherrypick(childid);\nCREATE INDEX ftsdocIdxed ON ftsdocs(type,rid,name) WHERE idxed==0;\nCREATE INDEX ftsdocName ON ftsdocs(name) WHERE type=\'w\';\nCREATE INDEX ticketchng_idx1 ON ticketchng(tkt_id, tkt_mtime);\nCREATE INDEX forumthread ON forumpost(froot,fmtime);\nCREATE VIEW artifact(rid,rcvid,size,atype,srcid,hash,content) AS\n  SELECT blob.rid,rcvid,size,1,srcid,uuid,content\n    FROM blob LEFT JOIN delta ON (blob.rid=delta.rid);\nCREATE VIEW ftscontent AS\n  SELECT rowid, type, rid, name, idxed, label, url, mtime,\n         title(type,rid,name) AS \'title\', body(type,rid,name) AS \'body\'\n    FROM ftsdocs;\n".as_ptr()
                        as *mut i8 as *const i8)
        };
        if unsafe {
                    sqlite3_compileoption_used(c"ENABLE_FTS5".as_ptr() as
                                *mut i8 as *const i8)
                } != 0 {
            unsafe {
                speedtest1_exec(c"CREATE VIRTUAL TABLE ftsidx\n  USING fts5(content=\"ftscontent\", title, body);\nCREATE VIRTUAL TABLE chatfts1 USING fts5(\n  xmsg, content=chat, content_rowid=msgid,tokenize=porter);\n".as_ptr()
                            as *mut i8 as *const i8)
            };
        } else {
            unsafe {
                speedtest1_exec(c"CREATE TABLE ftsidx_data(id INTEGER PRIMARY KEY, block BLOB);\nCREATE TABLE ftsidx_idx(segid, term, pgno, PRIMARY KEY(segid, term))\n  WITHOUT ROWID;\nCREATE TABLE ftsidx_docsize(id INTEGER PRIMARY KEY, sz BLOB);\nCREATE TABLE ftsidx_config(k PRIMARY KEY, v) WITHOUT ROWID;\nCREATE TABLE chatfts1_data(id INTEGER PRIMARY KEY, block BLOB);\nCREATE TABLE chatfts1_idx(segid, term, pgno, PRIMARY KEY(segid, term))\n  WITHOUT ROWID;\nCREATE TABLE chatfts1_docsize(id INTEGER PRIMARY KEY, sz BLOB);\nCREATE TABLE chatfts1_config(k PRIMARY KEY, v) WITHOUT ROWID;\n".as_ptr()
                            as *mut i8 as *const i8)
            };
        }
        unsafe {
            speedtest1_exec(c"ANALYZE sqlite_schema;\nINSERT INTO sqlite_stat1(tbl,idx,stat) VALUES\n  (\'ftsidx_config\',\'ftsidx_config\',\'1 1\'),\n  (\'ftsidx_idx\',\'ftsidx_idx\',\'4215 401 1\'),\n  (\'user\',\'sqlite_autoindex_user_1\',\'25 1\'),\n  (\'phantom\',NULL,\'26\'),\n  (\'reportfmt\',\'sqlite_autoindex_reportfmt_1\',\'9 1\'),\n  (\'rcvfrom\',\'sqlite_autoindex_rcvfrom_1\',\'18445 401\'),\n  (\'private\',NULL,\'99\'),\n  (\'mlink\',\'mlink_i4\',\'116678 401\'),\n  (\'mlink\',\'mlink_i3\',\'121212 2\'),\n  (\'mlink\',\'mlink_i2\',\'106372 401\'),\n  (\'mlink\',\'mlink_i1\',\'99298 5\'),\n  (\'ftsidx_data\',NULL,\'3795\'),\n  (\'leaf\',NULL,\'1559\'),\n  (\'delta\',\'delta_i1\',\'66340 1\'),\n  (\'unversioned\',\'unversioned\',\'3 1\'),\n  (\'pending_alert\',\'pending_alert\',\'3 1\'),\n  (\'cherrypick\',\'cherrypick_cid\',\'680 2\'),\n  (\'cherrypick\',\'cherrypick\',\'628 1 1\'),\n  (\'config\',\'config\',\'128 1\'),\n  (\'ftsidx_docsize\',NULL,\'33848\'),\n  (\'event\',\'event_i1\',\'36096 1\'),\n  (\'plink\',\'plink_i2\',\'38236 1 1\'),\n  (\'plink\',\'sqlite_autoindex_plink_1\',\'38357 1 1\'),\n  (\'shun\',\'shun\',\'10 1\'),\n  (\'concealed\',\'concealed\',\'110 1\'),\n  (\'vcache\',\'vcache\',\'1888 401 1\'),\n  (\'ftsdocs\',\'ftsdocName\',\'19 1\'),\n  (\'ftsdocs\',\'ftsdocIdxed\',\'168 84 1 1\'),\n  (\'ftsdocs\',\'sqlite_autoindex_ftsdocs_1\',\'37312 401 1\'),\n  (\'subscriber\',\'subscriberUname\',\'5 1\'),\n  (\'subscriber\',\'sqlite_autoindex_subscriber_2\',\'37 1\'),\n  (\'subscriber\',\'sqlite_autoindex_subscriber_1\',\'37 1\'),\n  (\'tag\',\'sqlite_autoindex_tag_1\',\'2990 1\'),\n  (\'filename\',\'sqlite_autoindex_filename_1\',\'3168 1\'),\n  (\'chat\',NULL,\'56124\'),\n  (\'tagxref\',\'tagxref_i1\',\'40992 401 2\'),\n  (\'tagxref\',\'sqlite_autoindex_tagxref_1\',\'79233 3 1\'),\n  (\'attachment\',\'attachment_idx2\',\'11 1\'),\n  (\'attachment\',\'attachment_idx1\',\'11 2 2 1\'),\n  (\'blob\',\'blob_rcvid\',\'128240 201\'),\n  (\'blob\',\'sqlite_autoindex_blob_1\',\'126480 1\'),\n  (\'synclog\',\'synclog\',\'12 3 1\'),\n  (\'backlink\',\'backlink_src\',\'2160 2 2\'),\n  (\'backlink\',\'sqlite_autoindex_backlink_1\',\'2340 2 2 1\'),\n  (\'accesslog\',NULL,\'38\'),\n  (\'chatfts1_config\',\'chatfts1_config\',\'1 1\'),\n  (\'chatfts1_idx\',\'chatfts1_idx\',\'688 230 1\'),\n  (\'ticket\',\'sqlite_autoindex_ticket_1\',\'794 1\'),\n  (\'ticketchng\',\'ticketchng_idx1\',\'2089 3 1\'),\n  (\'forumpost\',\'forumthread\',\'4 4 1\'),\n  (\'unclustered\',NULL,\'12\');\nCOMMIT;".as_ptr()
                        as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        n = g.sz_test * 3;
        unsafe {
            speedtest1_begin_test(110,
                c"Open and use the database %d times".as_ptr() as *mut i8 as
                    *const i8, n)
        };
        {
            i = 0;
            '__b39: loop {
                if !(i < n) { break '__b39; }
                '__c39: loop {
                    let db_main: *mut Sqlite3 = g.db;
                    let mut db_aux: *mut Sqlite3 = core::ptr::null_mut();
                    if !(g.z_db_name).is_null() &&
                            unsafe { *g.z_db_name.offset(0 as isize) } != 0 {
                        if unsafe {
                                    sqlite3_open_v2(g.z_db_name, &mut db_aux, 2, g.z_vfs)
                                } != 0 {
                            unsafe {
                                fatal_error(c"Cannot open database file: %s\n".as_ptr() as
                                            *mut i8 as *const i8, g.z_db_name)
                            };
                        }
                        g.db = db_aux;
                    }
                    unsafe {
                        speedtest1_exec(c"SELECT name FROM pragma_table_list /*scan*/ WHERE schema=\'repository\' AND type IN (\'table\',\'virtual\') AND name NOT IN (\'admin_log\', \'blob\',\'delta\',\'rcvfrom\',\'user\',\'alias\',\'config\',\'shun\',\'private\',\'reportfmt\',\'concealed\',\'accesslog\',\'modreq\',\'purgeevent\',\'purgeitem\',\'unversioned\',\'subscriber\',\'pending_alert\',\'chat\') AND name NOT GLOB \'sqlite_*\' AND name NOT GLOB \'fx_*\';SELECT 1 FROM pragma_table_xinfo(\'ticket\') WHERE name = \'mimetype\';".as_ptr()
                                    as *mut i8 as *const i8)
                    };
                    unsafe {
                        speedtest1_exec(c"SELECT name, value, unixepoch()/86400-value, date(value*86400,\'unixepoch\') FROM config WHERE name in (\'email-renew-warning\',\'email-renew-cutoff\');SELECT count(*) FROM pending_alert WHERE NOT sentDigest;".as_ptr()
                                    as *mut i8 as *const i8)
                    };
                    unsafe {
                        speedtest1_exec(c"WITH priors(rid,who) AS (  SELECT firt, coalesce(euser,user)    FROM forumpost LEFT JOIN event ON fpid=objid   WHERE fpid=12345  UNION ALL  SELECT firt, coalesce(euser,user)    FROM priors, forumpost LEFT JOIN event ON fpid=objid   WHERE fpid=rid)SELECT \',\'||group_concat(DISTINCT \'u\'||who)||\',\'||group_concat(rid) FROM priors;".as_ptr()
                                    as *mut i8 as *const i8)
                    };
                    unsafe {
                        speedtest1_exec(c"CREATE TEMP TABLE IF NOT EXISTS ok(rid INTEGER PRIMARY KEY);\n".as_ptr()
                                    as *mut i8 as *const i8)
                    };
                    unsafe {
                        speedtest1_exec(c"WITH RECURSIVE\n  parent(pid,cid,isCP) AS (\n    SELECT plink.pid, plink.cid, 0 AS xisCP FROM plink\n    UNION ALL\n    SELECT parentid, childid, 1 FROM cherrypick WHERE NOT isExclude\n  ),\n  ancestor(rid, mtime, isCP) AS (\n    SELECT 123, mtime, 0 FROM event WHERE objid=$object\n    UNION\n    SELECT parent.pid, event.mtime, parent.isCP\n      FROM ancestor, parent, event\n     WHERE parent.cid=ancestor.rid\n       AND event.objid=parent.pid\n       AND NOT ancestor.isCP\n       AND (event.mtime>=$date OR parent.pid=$pid)\n     ORDER BY mtime DESC LIMIT 10\n  )\n  INSERT OR IGNORE INTO ok SELECT rid FROM ancestor;".as_ptr()
                                    as *mut i8 as *const i8)
                    };
                    unsafe { sqlite3_close(db_aux) };
                    g.db = db_main;
                    break '__c39;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        speedtest1_end_test();
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn testset_orm() -> () {
    unsafe {
        let mut i: u32 = 0 as u32;
        let mut j: u32 = 0 as u32;
        let mut n: u32 = 0 as u32;
        let mut n_row: u32 = 0 as u32;
        let mut x1: u32 = 0 as u32;
        let mut len: u32 = 0 as u32;
        let mut z_num: [i8; 2000] = [0; 2000];
        n_row = { n = (g.sz_test * 250) as u32; n };
        unsafe {
            speedtest1_begin_test(100,
                c"Fill %d rows".as_ptr() as *mut i8 as *const i8, n)
        };
        unsafe {
            speedtest1_exec(c"BEGIN;CREATE TABLE ZLOOKSLIKECOREDATA (  ZPK INTEGER PRIMARY KEY,  ZTERMFITTINGHOUSINGCOMMAND INTEGER,  ZBRIEFGOBYDODGERHEIGHT BLOB,  ZCAPABLETRIPDOORALMOND BLOB,  ZDEPOSITPAIRCOLLEGECOMET INTEGER,  ZFRAMEENTERSIMPLEMOUTH INTEGER,  ZHOPEFULGATEHOLECHALK INTEGER,  ZSLEEPYUSERGRANDBOWL TIMESTAMP,  ZDEWPEACHCAREERCELERY INTEGER,  ZHANGERLITHIUMDINNERMEET VARCHAR,  ZCLUBRELEASELIZARDADVICE VARCHAR,  ZCHARGECLICKHUMANEHIRE INTEGER,  ZFINGERDUEPIZZAOPTION TIMESTAMP,  ZFLYINGDOCTORTABLEMELODY BLOB,  ZLONGFINLEAVEIMAGEOIL TIMESTAMP,  ZFAMILYVISUALOWNERMATTER BLOB,  ZGOLDYOUNGINITIALNOSE FLOAT,  ZCAUSESALAMITERMCYAN BLOB,  ZSPREADMOTORBISCUITBACON FLOAT,  ZGIFTICEFISHGLUEHAIR INTEGER,  ZNOTICEPEARPOLICYJUICE TIMESTAMP,  ZBANKBUFFALORECOVERORBIT TIMESTAMP,  ZLONGDIETESSAYNATURE FLOAT,  ZACTIONRANGEELEGANTNEUTRON BLOB,  ZCADETBRIGHTPLANETBANK TIMESTAMP,  ZAIRFORGIVEHEADFROG BLOB,  ZSHARKJUSTFRUITMOVIE VARCHAR,  ZFARMERMORNINGMIRRORCONCERN BLOB,  ZWOODPOETRYCOBBLERBENCH VARCHAR,  ZHAFNIUMSCRIPTSALADMOTOR INTEGER,  ZPROBLEMCLUBPOPOVERJELLY FLOAT,  ZEIGHTLEADERWORKERMOST TIMESTAMP,  ZGLASSRESERVEBARIUMMEAL BLOB,  ZCLAMBITARUGULAFAJITA BLOB,  ZDECADEJOYOUSWAVEHABIT FLOAT,  ZCOMPANYSUMMERFIBERELF INTEGER,  ZTREATTESTQUILLCHARGE TIMESTAMP,  ZBROWBALANCEKEYCHOWDER FLOAT,  ZPEACHCOPPERDINNERLAKE FLOAT,  ZDRYWALLBEYONDBROWNBOWL VARCHAR,  ZBELLYCRASHITEMLACK BLOB,  ZTENNISCYCLEBILLOFFICER INTEGER,  ZMALLEQUIPTHANKSGLUE FLOAT,  ZMISSREPLYHUMANLIVING INTEGER,  ZKIWIVISUALPRIDEAPPLE VARCHAR,  ZWISHHITSKINMOTOR BLOB,  ZCALMRACCOONPROGRAMDEBIT VARCHAR,  ZSHINYASSISTLIVINGCRAB VARCHAR,  ZRESOLVEWRISTWRAPAPPLE VARCHAR,  ZAPPEALSIMPLESECONDHOUSING BLOB,  ZCORNERANCHORTAPEDIVER TIMESTAMP,  ZMEMORYREQUESTSOURCEBIG VARCHAR,  ZTRYFACTKEEPMILK TIMESTAMP,  ZDIVERPAINTLEATHEREASY INTEGER,  ZSORTMISTYQUOTECABBAGE BLOB,  ZTUNEGASBUFFALOCAPITAL BLOB,  ZFILLSTOPLAWJOYFUL FLOAT,  ZSTEELCAREFULPLATENUMBER FLOAT,  ZGIVEVIVIDDIVINEMEANING INTEGER,  ZTREATPACKFUTURECONVERT VARCHAR,  ZCALMLYGEMFINISHEFFECT INTEGER,  ZCABBAGESOCKEASEMINUTE BLOB,  ZPLANETFAMILYPUREMEMORY TIMESTAMP,  ZMERRYCRACKTRAINLEADER BLOB,  ZMINORWAYPAPERCLASSY TIMESTAMP,  ZEAGLELINEMINEMAIL VARCHAR,  ZRESORTYARDGREENLET TIMESTAMP,  ZYARDOREGANOVIVIDJEWEL TIMESTAMP,  ZPURECAKEVIVIDNEATLY FLOAT,  ZASKCONTACTMONITORFUN TIMESTAMP,  ZMOVEWHOGAMMAINCH VARCHAR,  ZLETTUCEBIRDMEETDEBATE TIMESTAMP,  ZGENENATURALHEARINGKITE VARCHAR,  ZMUFFINDRYERDRAWFORTUNE FLOAT,  ZGRAYSURVEYWIRELOVE FLOAT,  ZPLIERSPRINTASKOREGANO INTEGER,  ZTRAVELDRIVERCONTESTLILY INTEGER,  ZHUMORSPICESANDKIDNEY TIMESTAMP,  ZARSENICSAMPLEWAITMUON INTEGER,  ZLACEADDRESSGROUNDCAREFUL FLOAT,  ZBAMBOOMESSWASABIEVENING BLOB,  ZONERELEASEAVERAGENURSE INTEGER,  ZRADIANTWHENTRYCARD TIMESTAMP,  ZREWARDINSIDEMANGOINTENSE FLOAT,  ZNEATSTEWPARTIRON TIMESTAMP,  ZOUTSIDEPEAHENCOUNTICE TIMESTAMP,  ZCREAMEVENINGLIPBRANCH FLOAT,  ZWHALEMATHAVOCADOCOPPER FLOAT,  ZLIFEUSELEAFYBELL FLOAT,  ZWEALTHLINENGLEEFULDAY VARCHAR,  ZFACEINVITETALKGOLD BLOB,  ZWESTAMOUNTAFFECTHEARING INTEGER,  ZDELAYOUTCOMEHORNAGENCY INTEGER,  ZBIGTHINKCONVERTECONOMY BLOB,  ZBASEGOUDAREGULARFORGIVE TIMESTAMP,  ZPATTERNCLORINEGRANDCOLBY TIMESTAMP,  ZCYANBASEFEEDADROIT INTEGER,  ZCARRYFLOORMINNOWDRAGON TIMESTAMP,  ZIMAGEPENCILOTHERBOTTOM FLOAT,  ZXENONFLIGHTPALEAPPLE TIMESTAMP,  ZHERRINGJOKEFEATUREHOPEFUL FLOAT,  ZCAPYEARLYRIVETBRUSH FLOAT,  ZAGEREEDFROGBASKET VARCHAR,  ZUSUALBODYHALIBUTDIAMOND VARCHAR,  ZFOOTTAPWORDENTRY VARCHAR,  ZDISHKEEPBLESTMONITOR FLOAT,  ZBROADABLESOLIDCASUAL INTEGER,  ZSQUAREGLEEFULCHILDLIGHT INTEGER,  ZHOLIDAYHEADPONYDETAIL INTEGER,  ZGENERALRESORTSKYOPEN TIMESTAMP,  ZGLADSPRAYKIDNEYGUPPY VARCHAR,  ZSWIMHEAVYMENTIONKIND BLOB,  ZMESSYSULFURDREAMFESTIVE BLOB,  ZSKYSKYCLASSICBRIEF VARCHAR,  ZDILLASKHOKILEMON FLOAT,  ZJUNIORSHOWPRESSNOVA FLOAT,  ZSIZETOEAWARDFRESH TIMESTAMP,  ZKEYFAILAPRICOTMETAL VARCHAR,  ZHANDYREPAIRPROTONAIRPORT VARCHAR,  ZPOSTPROTEINHANDLEACTOR BLOB);".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_prepare(c"INSERT INTO ZLOOKSLIKECOREDATA(ZPK,ZAIRFORGIVEHEADFROG,ZGIFTICEFISHGLUEHAIR,ZDELAYOUTCOMEHORNAGENCY,ZSLEEPYUSERGRANDBOWL,ZGLASSRESERVEBARIUMMEAL,ZBRIEFGOBYDODGERHEIGHT,ZBAMBOOMESSWASABIEVENING,ZFARMERMORNINGMIRRORCONCERN,ZTREATPACKFUTURECONVERT,ZCAUSESALAMITERMCYAN,ZCALMRACCOONPROGRAMDEBIT,ZHOLIDAYHEADPONYDETAIL,ZWOODPOETRYCOBBLERBENCH,ZHAFNIUMSCRIPTSALADMOTOR,ZUSUALBODYHALIBUTDIAMOND,ZOUTSIDEPEAHENCOUNTICE,ZDIVERPAINTLEATHEREASY,ZWESTAMOUNTAFFECTHEARING,ZSIZETOEAWARDFRESH,ZDEWPEACHCAREERCELERY,ZSTEELCAREFULPLATENUMBER,ZCYANBASEFEEDADROIT,ZCALMLYGEMFINISHEFFECT,ZHANDYREPAIRPROTONAIRPORT,ZGENENATURALHEARINGKITE,ZBROADABLESOLIDCASUAL,ZPOSTPROTEINHANDLEACTOR,ZLACEADDRESSGROUNDCAREFUL,ZIMAGEPENCILOTHERBOTTOM,ZPROBLEMCLUBPOPOVERJELLY,ZPATTERNCLORINEGRANDCOLBY,ZNEATSTEWPARTIRON,ZAPPEALSIMPLESECONDHOUSING,ZMOVEWHOGAMMAINCH,ZTENNISCYCLEBILLOFFICER,ZSHARKJUSTFRUITMOVIE,ZKEYFAILAPRICOTMETAL,ZCOMPANYSUMMERFIBERELF,ZTERMFITTINGHOUSINGCOMMAND,ZRESORTYARDGREENLET,ZCABBAGESOCKEASEMINUTE,ZSQUAREGLEEFULCHILDLIGHT,ZONERELEASEAVERAGENURSE,ZBIGTHINKCONVERTECONOMY,ZPLIERSPRINTASKOREGANO,ZDECADEJOYOUSWAVEHABIT,ZDRYWALLBEYONDBROWNBOWL,ZCLUBRELEASELIZARDADVICE,ZWHALEMATHAVOCADOCOPPER,ZBELLYCRASHITEMLACK,ZLETTUCEBIRDMEETDEBATE,ZCAPABLETRIPDOORALMOND,ZRADIANTWHENTRYCARD,ZCAPYEARLYRIVETBRUSH,ZAGEREEDFROGBASKET,ZSWIMHEAVYMENTIONKIND,ZTRAVELDRIVERCONTESTLILY,ZGLADSPRAYKIDNEYGUPPY,ZBANKBUFFALORECOVERORBIT,ZFINGERDUEPIZZAOPTION,ZCLAMBITARUGULAFAJITA,ZLONGFINLEAVEIMAGEOIL,ZLONGDIETESSAYNATURE,ZJUNIORSHOWPRESSNOVA,ZHOPEFULGATEHOLECHALK,ZDEPOSITPAIRCOLLEGECOMET,ZWEALTHLINENGLEEFULDAY,ZFILLSTOPLAWJOYFUL,ZTUNEGASBUFFALOCAPITAL,ZGRAYSURVEYWIRELOVE,ZCORNERANCHORTAPEDIVER,ZREWARDINSIDEMANGOINTENSE,ZCADETBRIGHTPLANETBANK,ZPLANETFAMILYPUREMEMORY,ZTREATTESTQUILLCHARGE,ZCREAMEVENINGLIPBRANCH,ZSKYSKYCLASSICBRIEF,ZARSENICSAMPLEWAITMUON,ZBROWBALANCEKEYCHOWDER,ZFLYINGDOCTORTABLEMELODY,ZHANGERLITHIUMDINNERMEET,ZNOTICEPEARPOLICYJUICE,ZSHINYASSISTLIVINGCRAB,ZLIFEUSELEAFYBELL,ZFACEINVITETALKGOLD,ZGENERALRESORTSKYOPEN,ZPURECAKEVIVIDNEATLY,ZKIWIVISUALPRIDEAPPLE,ZMESSYSULFURDREAMFESTIVE,ZCHARGECLICKHUMANEHIRE,ZHERRINGJOKEFEATUREHOPEFUL,ZYARDOREGANOVIVIDJEWEL,ZFOOTTAPWORDENTRY,ZWISHHITSKINMOTOR,ZBASEGOUDAREGULARFORGIVE,ZMUFFINDRYERDRAWFORTUNE,ZACTIONRANGEELEGANTNEUTRON,ZTRYFACTKEEPMILK,ZPEACHCOPPERDINNERLAKE,ZFRAMEENTERSIMPLEMOUTH,ZMERRYCRACKTRAINLEADER,ZMEMORYREQUESTSOURCEBIG,ZCARRYFLOORMINNOWDRAGON,ZMINORWAYPAPERCLASSY,ZDILLASKHOKILEMON,ZRESOLVEWRISTWRAPAPPLE,ZASKCONTACTMONITORFUN,ZGIVEVIVIDDIVINEMEANING,ZEIGHTLEADERWORKERMOST,ZMISSREPLYHUMANLIVING,ZXENONFLIGHTPALEAPPLE,ZSORTMISTYQUOTECABBAGE,ZEAGLELINEMINEMAIL,ZFAMILYVISUALOWNERMATTER,ZSPREADMOTORBISCUITBACON,ZDISHKEEPBLESTMONITOR,ZMALLEQUIPTHANKSGLUE,ZGOLDYOUNGINITIALNOSE,ZHUMORSPICESANDKIDNEY)VALUES(?1,?26,?20,?93,?8,?33,?3,?81,?28,?60,?18,?47,?109,?29,?30,?104,?86,?54,?92,?117,?9,?58,?97,?61,?119,?73,?107,?120,?80,?99,?31,?96,?85,?50,?71,?42,?27,?118,?36,?2,?67,?62,?108,?82,?94,?76,?35,?40,?11,?88,?41,?72,?4,?83,?102,?103,?112,?77,?111,?22,?13,?34,?15,?23,?116,?7,?5,?90,?57,?56,?75,?51,?84,?25,?63,?37,?87,?114,?79,?38,?14,?10,?21,?48,?89,?91,?110,?69,?45,?113,?12,?101,?68,?105,?46,?95,?74,?24,?53,?39,?6,?64,?52,?98,?65,?115,?49,?70,?59,?32,?44,?100,?55,?66,?16,?19,?106,?43,?17,?78);".as_ptr()
                        as *mut i8 as *const i8)
        };
        {
            i = 0 as u32;
            '__b40: loop {
                if !(i < n) { break '__b40; }
                '__c40: loop {
                    x1 = speedtest1_random();
                    speedtest1_numbername(x1 % 1000 as u32,
                        &raw mut z_num[0 as usize] as *mut i8,
                        core::mem::size_of::<[i8; 2000]>() as i32);
                    len =
                        unsafe {
                                    strlen(&raw mut z_num[0 as usize] as *mut i8 as *const i8)
                                } as i32 as u32;
                    unsafe {
                        sqlite3_bind_int(g.p_stmt, 1, (i ^ 15 as u32) as i32)
                    };
                    {
                        j = 0 as u32;
                        '__b41: loop {
                            if !(z_type[j as usize] != 0) { break '__b41; }
                            '__c41: loop {
                                '__s42:
                                    {
                                    match z_type[j as usize] {
                                        73 => {
                                            unsafe {
                                                sqlite3_bind_int64(g.p_stmt, (j + 2 as u32) as i32,
                                                    x1 as Sqlite3Int64)
                                            };
                                        }
                                        84 => {
                                            unsafe {
                                                sqlite3_bind_int64(g.p_stmt, (j + 2 as u32) as i32,
                                                    x1 as Sqlite3Int64)
                                            };
                                        }
                                        70 => {
                                            unsafe {
                                                sqlite3_bind_double(g.p_stmt, (j + 2 as u32) as i32,
                                                    x1 as f64)
                                            };
                                        }
                                        86 => {
                                            unsafe {
                                                sqlite3_bind_text64(g.p_stmt, (j + 2 as u32) as i32,
                                                    &raw mut z_num[0 as usize] as *mut i8 as *const i8,
                                                    len as Sqlite3Uint64, None, 1 as u8)
                                            };
                                        }
                                        66 => {
                                            unsafe {
                                                sqlite3_bind_text64(g.p_stmt, (j + 2 as u32) as i32,
                                                    &raw mut z_num[0 as usize] as *mut i8 as *const i8,
                                                    len as Sqlite3Uint64, None, 1 as u8)
                                            };
                                        }
                                        _ => {}
                                    }
                                }
                                break '__c41;
                            }
                            { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    speedtest1_run();
                    break '__c40;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            speedtest1_exec(c"COMMIT;".as_ptr() as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        n = (g.sz_test * 250) as u32;
        unsafe {
            speedtest1_begin_test(110,
                c"Query %d rows by rowid".as_ptr() as *mut i8 as *const i8, n)
        };
        unsafe {
            speedtest1_prepare(c"SELECT ZCYANBASEFEEDADROIT,ZJUNIORSHOWPRESSNOVA,ZCAUSESALAMITERMCYAN,ZHOPEFULGATEHOLECHALK,ZHUMORSPICESANDKIDNEY,ZSWIMHEAVYMENTIONKIND,ZMOVEWHOGAMMAINCH,ZAPPEALSIMPLESECONDHOUSING,ZHAFNIUMSCRIPTSALADMOTOR,ZNEATSTEWPARTIRON,ZLONGFINLEAVEIMAGEOIL,ZDEWPEACHCAREERCELERY,ZXENONFLIGHTPALEAPPLE,ZCALMRACCOONPROGRAMDEBIT,ZUSUALBODYHALIBUTDIAMOND,ZTRYFACTKEEPMILK,ZWEALTHLINENGLEEFULDAY,ZLONGDIETESSAYNATURE,ZLIFEUSELEAFYBELL,ZTREATPACKFUTURECONVERT,ZMEMORYREQUESTSOURCEBIG,ZYARDOREGANOVIVIDJEWEL,ZDEPOSITPAIRCOLLEGECOMET,ZSLEEPYUSERGRANDBOWL,ZBRIEFGOBYDODGERHEIGHT,ZCLUBRELEASELIZARDADVICE,ZCAPABLETRIPDOORALMOND,ZDRYWALLBEYONDBROWNBOWL,ZASKCONTACTMONITORFUN,ZKIWIVISUALPRIDEAPPLE,ZNOTICEPEARPOLICYJUICE,ZPEACHCOPPERDINNERLAKE,ZSTEELCAREFULPLATENUMBER,ZGLADSPRAYKIDNEYGUPPY,ZCOMPANYSUMMERFIBERELF,ZTENNISCYCLEBILLOFFICER,ZIMAGEPENCILOTHERBOTTOM,ZWESTAMOUNTAFFECTHEARING,ZDIVERPAINTLEATHEREASY,ZSKYSKYCLASSICBRIEF,ZMESSYSULFURDREAMFESTIVE,ZMERRYCRACKTRAINLEADER,ZBROADABLESOLIDCASUAL,ZGLASSRESERVEBARIUMMEAL,ZTUNEGASBUFFALOCAPITAL,ZBANKBUFFALORECOVERORBIT,ZTREATTESTQUILLCHARGE,ZBAMBOOMESSWASABIEVENING,ZREWARDINSIDEMANGOINTENSE,ZEAGLELINEMINEMAIL,ZCALMLYGEMFINISHEFFECT,ZKEYFAILAPRICOTMETAL,ZFINGERDUEPIZZAOPTION,ZCADETBRIGHTPLANETBANK,ZGOLDYOUNGINITIALNOSE,ZMISSREPLYHUMANLIVING,ZEIGHTLEADERWORKERMOST,ZFRAMEENTERSIMPLEMOUTH,ZBIGTHINKCONVERTECONOMY,ZFACEINVITETALKGOLD,ZPOSTPROTEINHANDLEACTOR,ZHERRINGJOKEFEATUREHOPEFUL,ZCABBAGESOCKEASEMINUTE,ZMUFFINDRYERDRAWFORTUNE,ZPROBLEMCLUBPOPOVERJELLY,ZGIVEVIVIDDIVINEMEANING,ZGENENATURALHEARINGKITE,ZGENERALRESORTSKYOPEN,ZLETTUCEBIRDMEETDEBATE,ZBASEGOUDAREGULARFORGIVE,ZCHARGECLICKHUMANEHIRE,ZPLANETFAMILYPUREMEMORY,ZMINORWAYPAPERCLASSY,ZCAPYEARLYRIVETBRUSH,ZSIZETOEAWARDFRESH,ZARSENICSAMPLEWAITMUON,ZSQUAREGLEEFULCHILDLIGHT,ZSHINYASSISTLIVINGCRAB,ZCORNERANCHORTAPEDIVER,ZDECADEJOYOUSWAVEHABIT,ZTRAVELDRIVERCONTESTLILY,ZFLYINGDOCTORTABLEMELODY,ZSHARKJUSTFRUITMOVIE,ZFAMILYVISUALOWNERMATTER,ZFARMERMORNINGMIRRORCONCERN,ZGIFTICEFISHGLUEHAIR,ZOUTSIDEPEAHENCOUNTICE,ZSPREADMOTORBISCUITBACON,ZWISHHITSKINMOTOR,ZHOLIDAYHEADPONYDETAIL,ZWOODPOETRYCOBBLERBENCH,ZAIRFORGIVEHEADFROG,ZBROWBALANCEKEYCHOWDER,ZDISHKEEPBLESTMONITOR,ZCLAMBITARUGULAFAJITA,ZPLIERSPRINTASKOREGANO,ZRADIANTWHENTRYCARD,ZDELAYOUTCOMEHORNAGENCY,ZPURECAKEVIVIDNEATLY,ZPATTERNCLORINEGRANDCOLBY,ZHANDYREPAIRPROTONAIRPORT,ZAGEREEDFROGBASKET,ZSORTMISTYQUOTECABBAGE,ZFOOTTAPWORDENTRY,ZRESOLVEWRISTWRAPAPPLE,ZDILLASKHOKILEMON,ZFILLSTOPLAWJOYFUL,ZACTIONRANGEELEGANTNEUTRON,ZRESORTYARDGREENLET,ZCREAMEVENINGLIPBRANCH,ZWHALEMATHAVOCADOCOPPER,ZGRAYSURVEYWIRELOVE,ZBELLYCRASHITEMLACK,ZHANGERLITHIUMDINNERMEET,ZCARRYFLOORMINNOWDRAGON,ZMALLEQUIPTHANKSGLUE,ZTERMFITTINGHOUSINGCOMMAND,ZONERELEASEAVERAGENURSE,ZLACEADDRESSGROUNDCAREFUL FROM ZLOOKSLIKECOREDATA WHERE ZPK=?1;".as_ptr()
                        as *mut i8 as *const i8)
        };
        {
            i = 0 as u32;
            '__b43: loop {
                if !(i < n) { break '__b43; }
                '__c43: loop {
                    x1 = speedtest1_random() % n_row;
                    unsafe { sqlite3_bind_int(g.p_stmt, 1, x1 as i32) };
                    speedtest1_run();
                    break '__c43;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        speedtest1_end_test();
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn testset_trigger() -> () {
    unsafe {
        let mut jj: i32 = 0;
        let mut ii: i32 = 0;
        let mut z_num: [i8; 2000] = [0; 2000];
        let nrow: i32 = (500 * g.sz_test) as i32;
        let nrow2: i32 = (100 * g.sz_test) as i32;
        unsafe {
            speedtest1_exec(c"BEGIN;CREATE TABLE z1(rowid INTEGER PRIMARY KEY, i INTEGER, t TEXT);CREATE TABLE z2(rowid INTEGER PRIMARY KEY, i INTEGER, t TEXT);CREATE TABLE t3(rowid INTEGER PRIMARY KEY, i INTEGER, t TEXT);CREATE VIEW v1 AS SELECT rowid, i, t FROM z1;CREATE VIEW v2 AS SELECT rowid, i, t FROM z2;CREATE VIEW v3 AS SELECT rowid, i, t FROM t3;".as_ptr()
                        as *mut i8 as *const i8)
        };
        {
            jj = 1;
            '__b44: loop {
                if !(jj <= 3) { break '__b44; }
                '__c44: loop {
                    unsafe {
                        speedtest1_prepare(c"INSERT INTO t%d VALUES(NULL,?1,?2)".as_ptr()
                                    as *mut i8 as *const i8, jj)
                    };
                    {
                        ii = 0;
                        '__b45: loop {
                            if !(ii < nrow) { break '__b45; }
                            '__c45: loop {
                                let x1: i32 = (speedtest1_random() % nrow as u32) as i32;
                                speedtest1_numbername(x1 as u32,
                                    &raw mut z_num[0 as usize] as *mut i8,
                                    core::mem::size_of::<[i8; 2000]>() as i32);
                                unsafe { sqlite3_bind_int(g.p_stmt, 1, x1) };
                                unsafe {
                                    sqlite3_bind_text(g.p_stmt, 2,
                                        &raw mut z_num[0 as usize] as *mut i8 as *const i8, -1,
                                        None)
                                };
                                speedtest1_run();
                                break '__c45;
                            }
                            { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    break '__c44;
                }
                { let __p = &mut jj; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            speedtest1_exec(c"CREATE INDEX i1 ON z1(t);CREATE INDEX i2 ON z2(t);CREATE INDEX i3 ON t3(t);COMMIT;".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_begin_test(100,
                c"speed4p-join1".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_prepare(c"SELECT * FROM z1, z2, t3 WHERE z1.oid = z2.oid AND z2.oid = t3.oid".as_ptr()
                        as *mut i8 as *const i8)
        };
        speedtest1_run();
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(110,
                c"speed4p-join2".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_prepare(c"SELECT * FROM z1, z2, t3 WHERE z1.t = z2.t AND z2.t = t3.t".as_ptr()
                        as *mut i8 as *const i8)
        };
        speedtest1_run();
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(120,
                c"speed4p-view1".as_ptr() as *mut i8 as *const i8)
        };
        {
            jj = 1;
            '__b46: loop {
                if !(jj <= 3) { break '__b46; }
                '__c46: loop {
                    unsafe {
                        speedtest1_prepare(c"SELECT * FROM v%d WHERE rowid = ?".as_ptr()
                                    as *mut i8 as *const i8, jj)
                    };
                    {
                        ii = 0;
                        '__b47: loop {
                            if !(ii < nrow2) { break '__b47; }
                            '__c47: loop {
                                unsafe { sqlite3_bind_int(g.p_stmt, 1, ii * 3) };
                                speedtest1_run();
                                break '__c47;
                            }
                            ii += 3;
                        }
                    }
                    break '__c46;
                }
                { let __p = &mut jj; let __t = *__p; *__p += 1; __t };
            }
        }
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(130,
                c"speed4p-table1".as_ptr() as *mut i8 as *const i8)
        };
        {
            jj = 1;
            '__b48: loop {
                if !(jj <= 3) { break '__b48; }
                '__c48: loop {
                    unsafe {
                        speedtest1_prepare(c"SELECT * FROM t%d WHERE rowid = ?".as_ptr()
                                    as *mut i8 as *const i8, jj)
                    };
                    {
                        ii = 0;
                        '__b49: loop {
                            if !(ii < nrow2) { break '__b49; }
                            '__c49: loop {
                                unsafe { sqlite3_bind_int(g.p_stmt, 1, ii * 3) };
                                speedtest1_run();
                                break '__c49;
                            }
                            ii += 3;
                        }
                    }
                    break '__c48;
                }
                { let __p = &mut jj; let __t = *__p; *__p += 1; __t };
            }
        }
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(140,
                c"speed4p-table1".as_ptr() as *mut i8 as *const i8)
        };
        {
            jj = 1;
            '__b50: loop {
                if !(jj <= 3) { break '__b50; }
                '__c50: loop {
                    unsafe {
                        speedtest1_prepare(c"SELECT * FROM t%d WHERE rowid = ?".as_ptr()
                                    as *mut i8 as *const i8, jj)
                    };
                    {
                        ii = 0;
                        '__b51: loop {
                            if !(ii < nrow2) { break '__b51; }
                            '__c51: loop {
                                unsafe { sqlite3_bind_int(g.p_stmt, 1, ii * 3) };
                                speedtest1_run();
                                break '__c51;
                            }
                            ii += 3;
                        }
                    }
                    break '__c50;
                }
                { let __p = &mut jj; let __t = *__p; *__p += 1; __t };
            }
        }
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(150,
                c"speed4p-subselect1".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_prepare(c"SELECT (SELECT t FROM z1 WHERE rowid = ?1),(SELECT t FROM z2 WHERE rowid = ?1),(SELECT t FROM t3 WHERE rowid = ?1)".as_ptr()
                        as *mut i8 as *const i8)
        };
        {
            jj = 0;
            '__b52: loop {
                if !(jj < nrow2) { break '__b52; }
                '__c52: loop {
                    unsafe { sqlite3_bind_int(g.p_stmt, 1, jj * 3) };
                    speedtest1_run();
                    break '__c52;
                }
                { let __p = &mut jj; let __t = *__p; *__p += 1; __t };
            }
        }
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(160,
                c"speed4p-rowid-update".as_ptr() as *mut i8 as *const i8)
        };
        unsafe { speedtest1_exec(c"BEGIN".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            speedtest1_prepare(c"UPDATE z1 SET i=i+1 WHERE rowid=?1".as_ptr()
                        as *mut i8 as *const i8)
        };
        {
            jj = 0;
            '__b53: loop {
                if !(jj < nrow2) { break '__b53; }
                '__c53: loop {
                    unsafe { sqlite3_bind_int(g.p_stmt, 1, jj) };
                    speedtest1_run();
                    break '__c53;
                }
                { let __p = &mut jj; let __t = *__p; *__p += 1; __t };
            }
        }
        unsafe {
            speedtest1_exec(c"COMMIT".as_ptr() as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        unsafe {
            speedtest1_exec(c"CREATE TABLE t5(t TEXT PRIMARY KEY, i INTEGER);".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_begin_test(170,
                c"speed4p-insert-ignore".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_exec(c"INSERT OR IGNORE INTO t5 SELECT t, i FROM z1".as_ptr()
                        as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        unsafe {
            speedtest1_exec(c"CREATE TABLE log(op TEXT, r INTEGER, i INTEGER, t TEXT);CREATE TABLE t4(rowid INTEGER PRIMARY KEY, i INTEGER, t TEXT);CREATE TRIGGER t4_trigger1 AFTER INSERT ON t4 BEGIN  INSERT INTO log VALUES(\'INSERT INTO t4\', new.rowid, new.i, new.t);END;CREATE TRIGGER t4_trigger2 AFTER UPDATE ON t4 BEGIN  INSERT INTO log VALUES(\'UPDATE OF t4\', new.rowid, new.i, new.t);END;CREATE TRIGGER t4_trigger3 AFTER DELETE ON t4 BEGIN  INSERT INTO log VALUES(\'DELETE OF t4\', old.rowid, old.i, old.t);END;BEGIN;".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_begin_test(180,
                c"speed4p-trigger1".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_prepare(c"INSERT INTO t4 VALUES(NULL, ?1, ?2)".as_ptr()
                        as *mut i8 as *const i8)
        };
        {
            jj = 0;
            '__b54: loop {
                if !(jj < nrow2) { break '__b54; }
                '__c54: loop {
                    speedtest1_numbername(jj as u32,
                        &raw mut z_num[0 as usize] as *mut i8,
                        core::mem::size_of::<[i8; 2000]>() as i32);
                    unsafe { sqlite3_bind_int(g.p_stmt, 1, jj) };
                    unsafe {
                        sqlite3_bind_text(g.p_stmt, 2,
                            &raw mut z_num[0 as usize] as *mut i8 as *const i8, -1,
                            None)
                    };
                    speedtest1_run();
                    break '__c54;
                }
                { let __p = &mut jj; let __t = *__p; *__p += 1; __t };
            }
        }
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(190,
                c"speed4p-trigger2".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_prepare(c"UPDATE t4 SET i = ?1, t = ?2 WHERE rowid = ?3".as_ptr()
                        as *mut i8 as *const i8)
        };
        {
            jj = 1;
            '__b55: loop {
                if !(jj <= nrow2 * 2) { break '__b55; }
                '__c55: loop {
                    speedtest1_numbername((jj * 2) as u32,
                        &raw mut z_num[0 as usize] as *mut i8,
                        core::mem::size_of::<[i8; 2000]>() as i32);
                    unsafe { sqlite3_bind_int(g.p_stmt, 1, jj * 2) };
                    unsafe {
                        sqlite3_bind_text(g.p_stmt, 2,
                            &raw mut z_num[0 as usize] as *mut i8 as *const i8, -1,
                            None)
                    };
                    unsafe { sqlite3_bind_int(g.p_stmt, 3, jj) };
                    speedtest1_run();
                    break '__c55;
                }
                jj += 2;
            }
        }
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(200,
                c"speed4p-trigger3".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_prepare(c"DELETE FROM t4 WHERE rowid = ?1".as_ptr() as
                        *mut i8 as *const i8)
        };
        {
            jj = 1;
            '__b56: loop {
                if !(jj <= nrow2 * 2) { break '__b56; }
                '__c56: loop {
                    unsafe { sqlite3_bind_int(g.p_stmt, 1, jj * 2) };
                    speedtest1_run();
                    break '__c56;
                }
                jj += 2;
            }
        }
        speedtest1_end_test();
        unsafe {
            speedtest1_exec(c"COMMIT".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_exec(c"DROP TABLE t4;DROP TABLE log;VACUUM;CREATE TABLE t4(rowid INTEGER PRIMARY KEY, i INTEGER, t TEXT);BEGIN;".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_begin_test(210,
                c"speed4p-notrigger1".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_prepare(c"INSERT INTO t4 VALUES(NULL, ?1, ?2)".as_ptr()
                        as *mut i8 as *const i8)
        };
        {
            jj = 0;
            '__b57: loop {
                if !(jj < nrow2) { break '__b57; }
                '__c57: loop {
                    speedtest1_numbername(jj as u32,
                        &raw mut z_num[0 as usize] as *mut i8,
                        core::mem::size_of::<[i8; 2000]>() as i32);
                    unsafe { sqlite3_bind_int(g.p_stmt, 1, jj) };
                    unsafe {
                        sqlite3_bind_text(g.p_stmt, 2,
                            &raw mut z_num[0 as usize] as *mut i8 as *const i8, -1,
                            None)
                    };
                    speedtest1_run();
                    break '__c57;
                }
                { let __p = &mut jj; let __t = *__p; *__p += 1; __t };
            }
        }
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(210,
                c"speed4p-notrigger2".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_prepare(c"UPDATE t4 SET i = ?1, t = ?2 WHERE rowid = ?3".as_ptr()
                        as *mut i8 as *const i8)
        };
        {
            jj = 1;
            '__b58: loop {
                if !(jj <= nrow2 * 2) { break '__b58; }
                '__c58: loop {
                    speedtest1_numbername((jj * 2) as u32,
                        &raw mut z_num[0 as usize] as *mut i8,
                        core::mem::size_of::<[i8; 2000]>() as i32);
                    unsafe { sqlite3_bind_int(g.p_stmt, 1, jj * 2) };
                    unsafe {
                        sqlite3_bind_text(g.p_stmt, 2,
                            &raw mut z_num[0 as usize] as *mut i8 as *const i8, -1,
                            None)
                    };
                    unsafe { sqlite3_bind_int(g.p_stmt, 3, jj) };
                    speedtest1_run();
                    break '__c58;
                }
                jj += 2;
            }
        }
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(220,
                c"speed4p-notrigger3".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_prepare(c"DELETE FROM t4 WHERE rowid = ?1".as_ptr() as
                        *mut i8 as *const i8)
        };
        {
            jj = 1;
            '__b59: loop {
                if !(jj <= nrow2 * 2) { break '__b59; }
                '__c59: loop {
                    unsafe { sqlite3_bind_int(g.p_stmt, 1, jj * 2) };
                    speedtest1_run();
                    break '__c59;
                }
                jj += 2;
            }
        }
        speedtest1_end_test();
        unsafe {
            speedtest1_exec(c"COMMIT".as_ptr() as *mut i8 as *const i8)
        };
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn testset_debug1() -> () {
    unsafe {
        let mut i: u32 = 0 as u32;
        let mut n: u32 = 0 as u32;
        let mut x1: u32 = 0 as u32;
        let mut x2: u32 = 0 as u32;
        let mut z_num: [i8; 2000] = [0; 2000];
        n = g.sz_test as u32;
        {
            i = 1 as u32;
            '__b60: loop {
                if !(i <= n) { break '__b60; }
                '__c60: loop {
                    x1 = swizzle(i, n);
                    x2 = swizzle(x1, n);
                    speedtest1_numbername(x1,
                        &raw mut z_num[0 as usize] as *mut i8,
                        core::mem::size_of::<[i8; 2000]>() as i32);
                    unsafe {
                        printf(c"%5d %5d %5d %s\n".as_ptr() as *mut i8 as *const i8,
                            i, x1, x2, &raw mut z_num[0 as usize] as *mut i8)
                    };
                    break '__c60;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn testset_json() -> () {
    unsafe {
        let r: u32 = 305419896 as u32;
        unsafe { sqlite3_test_control(28, r, g.db) };
        unsafe {
            speedtest1_begin_test(100,
                c"table J1 is %d rows of JSONB".as_ptr() as *mut i8 as
                    *const i8, g.sz_test * 5)
        };
        unsafe {
            speedtest1_exec(c"CREATE TABLE j1(x JSONB);\nWITH RECURSIVE\n  jval(n,j) AS (\n    VALUES(0,\'{}\'),(1,\'[]\'),(2,\'true\'),(3,\'false\'),(4,\'null\'),\n          (5,\'{x:1,y:2}\'),(6,\'0.0\'),(7,\'3.14159\'),(8,\'-99.9\'),\n          (9,\'[1,2,\"\\n\\u2192\\\"\\u2190\",4]\')\n  ),\n  c(x) AS (VALUES(1) UNION ALL SELECT x+1 FROM c WHERE x<26*26-1),\n  array1(y) AS MATERIALIZED (\n    SELECT jsonb_group_array(\n      jsonb_object(\'x\',x,\n                  \'y\',jsonb(coalesce(j,random()%%10000)),\n                  \'z\',hex(randomblob(50)))\n    )\n    FROM c LEFT JOIN jval ON (x%%20)=n\n  ),\n  object1(z) AS MATERIALIZED (\n    SELECT jsonb_group_object(char(0x61+x%%26,0x61+(x/26)%%26),\n                      jsonb( coalesce(j,random()%%10000)))\n      FROM c LEFT JOIN jval ON (x%%20)=n\n  ),\n  c2(n) AS (VALUES(1) UNION ALL SELECT n+1 FROM c2 WHERE n<%d)\nINSERT INTO j1(x)\n  SELECT jsonb_object(\'a\',n,\'b\',n+10000,\'c\',jsonb(y),\'d\',jsonb(z),\n                     \'e\',n+20000,\'f\',n+30000)\n    FROM array1, object1, c2;".as_ptr()
                        as *mut i8 as *const i8, g.sz_test * 5)
        };
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(110,
                c"table J2 is %d rows from J1 converted to text".as_ptr() as
                        *mut i8 as *const i8, g.sz_test)
        };
        unsafe {
            speedtest1_exec(c"CREATE TABLE j2(x JSON TEXT);\nINSERT INTO j2(x) SELECT json(x) FROM j1 LIMIT %d".as_ptr()
                        as *mut i8 as *const i8, g.sz_test)
        };
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(120,
                c"create indexes on JSON expressions on J1".as_ptr() as
                        *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_exec(c"BEGIN;\nCREATE INDEX j1x1 ON j1(x->>\'a\');\nCREATE INDEX j1x2 ON j1(x->>\'b\');\nCREATE INDEX j1x3 ON j1(x->>\'f\');\nCOMMIT;\n".as_ptr()
                        as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(130,
                c"create indexes on JSON expressions on J2".as_ptr() as
                        *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_exec(c"BEGIN;\nCREATE INDEX j2x1 ON j2(x->>\'a\');\nCREATE INDEX j2x2 ON j2(x->>\'b\');\nCREATE INDEX j2x3 ON j2(x->>\'f\');\nCOMMIT;\n".as_ptr()
                        as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(140,
                c"queries against J1".as_ptr() as *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_exec(c"WITH c(n) AS (VALUES(0) UNION ALL SELECT n+1 FROM c WHERE n<7)\n  SELECT sum(x->>format(\'$.c[%%d].x\',n)) FROM c, j1;\nWITH c(n) AS (VALUES(1) UNION ALL SELECT n+1 FROM c WHERE n<5)\n  SELECT sum(x->>format(\'$.\"c\"[#-%%d].y\',n)) FROM c, j1;\nSELECT sum(x->>\'$.d.ez\' + x->>\'$.d.\"xz\"\' + x->>\'a\' + x->>\'$.c[10].y\') FROM j1;\nSELECT x->>\'$.d.tz[2]\', x->\'$.d.tz\' FROM j1;\n".as_ptr()
                        as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(141,
                c"queries involving json_type()".as_ptr() as *mut i8 as
                    *const i8)
        };
        unsafe {
            speedtest1_exec(c"WITH c(n) AS (VALUES(1) UNION ALL SELECT n+1 FROM c WHERE n<20)\n  SELECT json_type(x,format(\'$.c[#-%%d].y\',n)), count(*)\n    FROM c, j1\n   WHERE j1.rowid=1\n   GROUP BY 1 ORDER BY 2;".as_ptr()
                        as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(150,
                c"json_insert()/set()/remove() on every row of J1".as_ptr() as
                        *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_exec(c"BEGIN;\nUPDATE j1 SET x=jsonb_insert(x,\'$.g\',(x->>\'f\')+1,\'$.h\',3.14159,\'$.i\',\'hello\',\n                               \'$.j\',json(\'{x:99}\'),\'$.k\',\'{y:98}\');\nUPDATE j1 SET x=jsonb_set(x,\'$.e\',(x->>\'f\')-1);\nUPDATE j1 SET x=jsonb_remove(x,\'$.d\');\nCOMMIT;\n".as_ptr()
                        as *mut i8 as *const i8)
        };
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(160,
                c"json_insert()/set()/remove() on every row of J2".as_ptr() as
                        *mut i8 as *const i8)
        };
        unsafe {
            speedtest1_exec(c"BEGIN;\nUPDATE j2 SET x=json_insert(x,\'$.g\',(x->>\'f\')+1);\nUPDATE j2 SET x=json_set(x,\'$.e\',(x->>\'f\')-1);\nUPDATE j2 SET x=json_remove(x,\'$.d\');\nCOMMIT;\n".as_ptr()
                        as *mut i8 as *const i8)
        };
        speedtest1_end_test();
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn testset_parsenumber() -> () {
    unsafe {
        let z_sql1: *const i8 =
            c"SELECT 1, 12, 123, 1234, 12345, 123456".as_ptr() as *mut i8 as
                *const i8;
        let z_sql2: *const i8 =
            c"SELECT 8227256643844975616, 7932208612563860480, 2010730661871032832, 9138463067404021760, 2557616153664746496, 2557616153664746496".as_ptr()
                    as *mut i8 as *const i8;
        let z_sql3: *const i8 =
            c"SELECT 1.0, 1.2, 1.23, 123.4, 1.2345, 1.23456".as_ptr() as
                    *mut i8 as *const i8;
        let z_sql4: *const i8 =
            c"SELECT 8.227256643844975616, 7.932208612563860480, 2.010730661871032832, 9.138463067404021760, 2.557616153664746496, 2.557616153664746496".as_ptr()
                    as *mut i8 as *const i8;
        let nrow: i32 = (100 * g.sz_test) as i32;
        let mut ii: i32 = 0;
        unsafe {
            speedtest1_begin_test(100,
                c"parsing %d small integers".as_ptr() as *mut i8 as *const i8,
                nrow)
        };
        {
            ii = 0;
            '__b61: loop {
                if !(ii < nrow) { break '__b61; }
                '__c61: loop {
                    unsafe {
                        sqlite3_exec(g.db, z_sql1, None, core::ptr::null_mut(),
                            core::ptr::null_mut())
                    };
                    break '__c61;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(110,
                c"parsing %d large integers".as_ptr() as *mut i8 as *const i8,
                nrow)
        };
        {
            ii = 0;
            '__b62: loop {
                if !(ii < nrow) { break '__b62; }
                '__c62: loop {
                    unsafe {
                        sqlite3_exec(g.db, z_sql2, None, core::ptr::null_mut(),
                            core::ptr::null_mut())
                    };
                    break '__c62;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(200,
                c"parsing %d small reals".as_ptr() as *mut i8 as *const i8,
                nrow)
        };
        {
            ii = 0;
            '__b63: loop {
                if !(ii < nrow) { break '__b63; }
                '__c63: loop {
                    unsafe {
                        sqlite3_exec(g.db, z_sql3, None, core::ptr::null_mut(),
                            core::ptr::null_mut())
                    };
                    break '__c63;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
        speedtest1_end_test();
        unsafe {
            speedtest1_begin_test(210,
                c"parsing %d large reals".as_ptr() as *mut i8 as *const i8,
                nrow)
        };
        {
            ii = 0;
            '__b64: loop {
                if !(ii < nrow) { break '__b64; }
                '__c64: loop {
                    unsafe {
                        sqlite3_exec(g.db, z_sql4, None, core::ptr::null_mut(),
                            core::ptr::null_mut())
                    };
                    break '__c64;
                }
                { let __p = &mut ii; let __t = *__p; *__p += 1; __t };
            }
        }
        speedtest1_end_test();
    }
}
extern "C" fn x_compile_options(p_ctx_1: *mut (), n_val_1: i32,
    az_val_1: *mut *mut i8, az_col_1: *mut *mut i8) -> i32 {
    unsafe {
        printf(c"-- Compile option: %s\n".as_ptr() as *mut i8 as *const i8,
            unsafe { *az_val_1.offset(0 as isize) })
    };
    return 0;
}
extern "C" fn __main_inner(argc: i32, argv: *const *mut i8)
    -> Result<(), i32> {
    unsafe {
        let mut do_autovac: i32 = 0;
        let mut cache_size: i32 = 0;
        let mut do_exclusive: i32 = 0;
        let mut do_full_f_sync: i32 = 0;
        let mut n_heap: i32 = 0;
        let mut mn_heap: i32 = 0;
        let mut do_incrvac: i32 = 0;
        let mut z_j_mode: *const i8 = core::ptr::null();
        let mut z_key: *const i8 = core::ptr::null();
        let mut n_hard_heap_lmt: i32 = 0;
        let mut n_soft_heap_lmt: i32 = 0;
        let mut n_look: i32 = -1;
        let mut sz_look: i32 = 0;
        let mut no_sync: i32 = 0;
        let mut page_size: i32 = 0;
        let mut n_p_cache: i32 = 0;
        let mut sz_p_cache: i32 = 0;
        let mut do_p_cache: i32 = 0;
        let mut show_stats: i32 = 0;
        let mut n_thread: i32 = 0;
        let mut mmap_size: i32 = 0;
        let mut mem_db: i32 = 0;
        let mut open_flags: i32 = 2 | 4;
        let mut z_t_set: *mut i8 = c"mix1".as_ptr() as *mut i8;
        let mut do_trace: i32 = 0;
        let mut z_encoding: *const i8 = core::ptr::null();
        let mut p_heap: *mut () = core::ptr::null_mut();
        let mut p_look: *mut () = core::ptr::null_mut();
        let mut p_p_cache: *mut () = core::ptr::null_mut();
        let mut i_cur: i32 = 0;
        let mut i_hi: i32 = 0;
        let mut i: i32 = 0;
        let mut rc: i32 = 0;
        unsafe {
            printf(c"-- Speedtest1 for SQLite %s %.48s\n".as_ptr() as *mut i8
                    as *const i8, unsafe { sqlite3_libversion() },
                unsafe { sqlite3_sourceid() })
        };
        g.z_db_name = core::ptr::null();
        g.z_vfs = core::ptr::null();
        g.z_wr = c"".as_ptr() as *mut i8 as *const i8;
        g.z_nn = c"".as_ptr() as *mut i8 as *const i8;
        g.z_pk = c"UNIQUE".as_ptr() as *mut i8 as *const i8;
        g.sz_test = 100;
        g.sz_base = 100;
        g.n_repeat = 1;
        {
            i = 1;
            '__b65: loop {
                if !(i < argc) { break '__b65; }
                '__c65: loop {
                    let mut z: *const i8 =
                        unsafe { *argv.offset(i as isize) } as *const i8;
                    if unsafe { *z.offset(0 as isize) } as i32 == '-' as i32 {
                        '__b66: loop {
                            '__c66: loop {
                                {
                                    let __p = &mut z;
                                    let __t = *__p;
                                    *__p = unsafe { (*__p).offset(1) };
                                    __t
                                };
                                break '__c66;
                            }
                            if !(unsafe { *z.offset(0 as isize) } as i32 == '-' as i32)
                                {
                                break '__b66;
                            }
                        }
                        if unsafe {
                                    strcmp(z, c"autovacuum".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            do_autovac = 1;
                        } else if unsafe {
                                    strcmp(z,
                                        c"big-transactions".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            g.do_big_transactions = 1;
                        } else if unsafe {
                                    strcmp(z, c"cachesize".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            if i >= argc - 1 {
                                unsafe {
                                    fatal_error(c"missing argument on %s\n".as_ptr() as *mut i8
                                            as *const i8, unsafe { *argv.offset(i as isize) })
                                };
                            }
                            cache_size =
                                integer_value(unsafe {
                                            *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                        } as *const i8);
                        } else if unsafe {
                                    strcmp(z, c"exclusive".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            do_exclusive = 1;
                        } else if unsafe {
                                    strcmp(z, c"fullfsync".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            do_full_f_sync = 1;
                        } else if unsafe {
                                    strcmp(z, c"checkpoint".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            g.do_checkpoint = 1;
                        } else if unsafe {
                                    strcmp(z, c"explain".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            g.b_sql_only = 1;
                            g.b_explain = 1;
                        } else if unsafe {
                                    strcmp(z,
                                        c"hard-heap-limit".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            if i >= argc - 1 {
                                unsafe {
                                    fatal_error(c"missing argument on %s\n".as_ptr() as *mut i8
                                            as *const i8, unsafe { *argv.offset(i as isize) })
                                };
                            }
                            n_hard_heap_lmt =
                                integer_value(unsafe { *argv.offset((i + 1) as isize) } as
                                        *const i8);
                            i += 1;
                        } else if unsafe {
                                    strcmp(z, c"heap".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            if i >= argc - 2 {
                                unsafe {
                                    fatal_error(c"missing argument on %s\n".as_ptr() as *mut i8
                                            as *const i8, unsafe { *argv.offset(i as isize) })
                                };
                            }
                            n_heap =
                                integer_value(unsafe { *argv.offset((i + 1) as isize) } as
                                        *const i8);
                            mn_heap =
                                integer_value(unsafe { *argv.offset((i + 2) as isize) } as
                                        *const i8);
                            i += 2;
                        } else if unsafe {
                                    strcmp(z, c"incrvacuum".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            do_incrvac = 1;
                        } else if unsafe {
                                    strcmp(z, c"journal".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            if i >= argc - 1 {
                                unsafe {
                                    fatal_error(c"missing argument on %s\n".as_ptr() as *mut i8
                                            as *const i8, unsafe { *argv.offset(i as isize) })
                                };
                            }
                            z_j_mode =
                                unsafe {
                                        *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                    } as *const i8;
                        } else if unsafe {
                                    strcmp(z, c"key".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            if i >= argc - 1 {
                                unsafe {
                                    fatal_error(c"missing argument on %s\n".as_ptr() as *mut i8
                                            as *const i8, unsafe { *argv.offset(i as isize) })
                                };
                            }
                            z_key =
                                unsafe {
                                        *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                    } as *const i8;
                        } else if unsafe {
                                    strcmp(z, c"lookaside".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            if i >= argc - 2 {
                                unsafe {
                                    fatal_error(c"missing argument on %s\n".as_ptr() as *mut i8
                                            as *const i8, unsafe { *argv.offset(i as isize) })
                                };
                            }
                            n_look =
                                integer_value(unsafe { *argv.offset((i + 1) as isize) } as
                                        *const i8);
                            sz_look =
                                integer_value(unsafe { *argv.offset((i + 2) as isize) } as
                                        *const i8);
                            i += 2;
                        } else if unsafe {
                                    strcmp(z, c"memdb".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            mem_db = 1;
                        } else if unsafe {
                                    strcmp(z, c"multithread".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            unsafe { sqlite3_config(2) };
                        } else if unsafe {
                                    strcmp(z, c"nomemstat".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            unsafe { sqlite3_config(9, 0) };
                        } else if unsafe {
                                    strcmp(z, c"mmap".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            if i >= argc - 1 {
                                unsafe {
                                    fatal_error(c"missing argument on %s\n".as_ptr() as *mut i8
                                            as *const i8, unsafe { *argv.offset(i as isize) })
                                };
                            }
                            mmap_size =
                                integer_value(unsafe {
                                            *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                        } as *const i8);
                        } else if unsafe {
                                    strcmp(z, c"nomutex".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            open_flags |= 32768;
                        } else if unsafe {
                                    strcmp(z, c"nosync".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            no_sync = 1;
                        } else if unsafe {
                                    strcmp(z, c"notnull".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            g.z_nn = c"NOT NULL".as_ptr() as *mut i8 as *const i8;
                        } else if unsafe {
                                    strcmp(z, c"output".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            if i >= argc - 1 {
                                unsafe {
                                    fatal_error(c"missing argument on %s\n".as_ptr() as *mut i8
                                            as *const i8, unsafe { *argv.offset(i as isize) })
                                };
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            if unsafe {
                                        strcmp(unsafe { *argv.offset(i as isize) } as *const i8,
                                            c"-".as_ptr() as *mut i8 as *const i8)
                                    } == 0 {
                                g.hash_file = __stdoutp;
                            } else {
                                g.hash_file =
                                    unsafe {
                                        fopen(unsafe { *argv.offset(i as isize) } as *const i8,
                                            c"wb".as_ptr() as *mut i8 as *const i8)
                                    };
                                if g.hash_file == core::ptr::null_mut() {
                                    unsafe {
                                        fatal_error(c"cannot open \"%s\" for writing\n".as_ptr() as
                                                    *mut i8 as *const i8, unsafe { *argv.offset(i as isize) })
                                    };
                                }
                            }
                        } else if unsafe {
                                    strcmp(z, c"pagesize".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            if i >= argc - 1 {
                                unsafe {
                                    fatal_error(c"missing argument on %s\n".as_ptr() as *mut i8
                                            as *const i8, unsafe { *argv.offset(i as isize) })
                                };
                            }
                            page_size =
                                integer_value(unsafe {
                                            *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                        } as *const i8);
                        } else if unsafe {
                                    strcmp(z, c"pcache".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            if i >= argc - 2 {
                                unsafe {
                                    fatal_error(c"missing argument on %s\n".as_ptr() as *mut i8
                                            as *const i8, unsafe { *argv.offset(i as isize) })
                                };
                            }
                            n_p_cache =
                                integer_value(unsafe { *argv.offset((i + 1) as isize) } as
                                        *const i8);
                            sz_p_cache =
                                integer_value(unsafe { *argv.offset((i + 2) as isize) } as
                                        *const i8);
                            do_p_cache = 1;
                            i += 2;
                        } else if unsafe {
                                    strcmp(z, c"primarykey".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            g.z_pk = c"PRIMARY KEY".as_ptr() as *mut i8 as *const i8;
                        } else if unsafe {
                                    strcmp(z, c"repeat".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            if i >= argc - 1 {
                                unsafe {
                                    fatal_error(c"missing argument on %s\n".as_ptr() as *mut i8
                                            as *const i8, unsafe { *argv.offset(i as isize) })
                                };
                            }
                            g.n_repeat =
                                integer_value(unsafe {
                                            *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                        } as *const i8);
                        } else if unsafe {
                                    strcmp(z, c"reprepare".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            g.b_reprepare = 1;
                        } else if unsafe {
                                    strcmp(z, c"serialized".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            unsafe { sqlite3_config(3) };
                        } else if unsafe {
                                    strcmp(z, c"singlethread".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            unsafe { sqlite3_config(1) };
                        } else if unsafe {
                                    strcmp(z, c"script".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            if i >= argc - 1 {
                                unsafe {
                                    fatal_error(c"missing argument on %s\n".as_ptr() as *mut i8
                                            as *const i8, unsafe { *argv.offset(i as isize) })
                                };
                            }
                            if !(g.p_script).is_null() {
                                unsafe { fclose(g.p_script) };
                            }
                            g.p_script =
                                unsafe {
                                    fopen(unsafe {
                                                *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                            } as *const i8, c"wb".as_ptr() as *mut i8 as *const i8)
                                };
                            if g.p_script == core::ptr::null_mut() {
                                unsafe {
                                    fatal_error(c"unable to open output file \"%s\"\n".as_ptr()
                                                as *mut i8 as *const i8,
                                        unsafe { *argv.offset(i as isize) })
                                };
                            }
                        } else if unsafe {
                                    strcmp(z, c"sqlonly".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            g.b_sql_only = 1;
                        } else if unsafe {
                                    strcmp(z, c"shrink-memory".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            g.b_mem_shrink = 1;
                        } else if unsafe {
                                    strcmp(z, c"size".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            if i >= argc - 1 {
                                unsafe {
                                    fatal_error(c"missing argument on %s\n".as_ptr() as *mut i8
                                            as *const i8, unsafe { *argv.offset(i as isize) })
                                };
                            }
                            g.sz_test =
                                {
                                    g.sz_base =
                                        integer_value(unsafe {
                                                    *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                                } as *const i8);
                                    g.sz_base
                                };
                        } else if unsafe {
                                    strcmp(z,
                                        c"soft-heap-limit".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            if i >= argc - 1 {
                                unsafe {
                                    fatal_error(c"missing argument on %s\n".as_ptr() as *mut i8
                                            as *const i8, unsafe { *argv.offset(i as isize) })
                                };
                            }
                            n_soft_heap_lmt =
                                integer_value(unsafe { *argv.offset((i + 1) as isize) } as
                                        *const i8);
                            i += 1;
                        } else if unsafe {
                                    strcmp(z, c"stats".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            show_stats = 1;
                        } else if unsafe {
                                    strcmp(z, c"temp".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            if i >= argc - 1 {
                                unsafe {
                                    fatal_error(c"missing argument on %s\n".as_ptr() as *mut i8
                                            as *const i8, unsafe { *argv.offset(i as isize) })
                                };
                            }
                            { let __p = &mut i; let __t = *__p; *__p += 1; __t };
                            if (unsafe {
                                                        *unsafe { (*argv.offset(i as isize)).offset(0 as isize) }
                                                    } as i32) < '0' as i32 ||
                                        unsafe {
                                                    *unsafe { (*argv.offset(i as isize)).offset(0 as isize) }
                                                } as i32 > '9' as i32 ||
                                    unsafe {
                                                *unsafe { (*argv.offset(i as isize)).offset(1 as isize) }
                                            } as i32 != 0 {
                                unsafe {
                                    fatal_error(c"argument to --temp should be integer between 0 and 9".as_ptr()
                                                as *mut i8 as *const i8)
                                };
                            }
                            g.e_temp =
                                unsafe {
                                            *unsafe { (*argv.offset(i as isize)).offset(0 as isize) }
                                        } as i32 - '0' as i32;
                        } else if unsafe {
                                    strcmp(z, c"testset".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            if i >= argc - 1 {
                                unsafe {
                                    fatal_error(c"missing argument on %s\n".as_ptr() as *mut i8
                                            as *const i8, unsafe { *argv.offset(i as isize) })
                                };
                            }
                            z_t_set =
                                unsafe {
                                    *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                };
                        } else if unsafe {
                                    strcmp(z, c"trace".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            do_trace = 1;
                        } else if unsafe {
                                    strcmp(z, c"threads".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            if i >= argc - 1 {
                                unsafe {
                                    fatal_error(c"missing argument on %s\n".as_ptr() as *mut i8
                                            as *const i8, unsafe { *argv.offset(i as isize) })
                                };
                            }
                            n_thread =
                                integer_value(unsafe {
                                            *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                        } as *const i8);
                        } else if unsafe {
                                    strcmp(z, c"utf16le".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            z_encoding = c"utf16le".as_ptr() as *mut i8 as *const i8;
                        } else if unsafe {
                                    strcmp(z, c"utf16be".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            z_encoding = c"utf16be".as_ptr() as *mut i8 as *const i8;
                        } else if unsafe {
                                    strcmp(z, c"verify".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            g.b_verify = 1;
                            hash_init();
                        } else if unsafe {
                                    strcmp(z, c"vfs".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            if i >= argc - 1 {
                                unsafe {
                                    fatal_error(c"missing argument on %s\n".as_ptr() as *mut i8
                                            as *const i8, unsafe { *argv.offset(i as isize) })
                                };
                            }
                            g.z_vfs =
                                unsafe {
                                        *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                    } as *const i8;
                        } else if unsafe {
                                    strcmp(z, c"reserve".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            if i >= argc - 1 {
                                unsafe {
                                    fatal_error(c"missing argument on %s\n".as_ptr() as *mut i8
                                            as *const i8, unsafe { *argv.offset(i as isize) })
                                };
                            }
                            g.n_reserve =
                                unsafe {
                                    atoi(unsafe {
                                                *argv.offset({ let __p = &mut i; *__p += 1; *__p } as isize)
                                            } as *const i8)
                                };
                        } else if unsafe {
                                    strcmp(z,
                                        c"stmtscanstatus".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            g.stmt_scan_status = 1;
                        } else if unsafe {
                                    strcmp(z, c"without-rowid".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            if unsafe {
                                        strstr(g.z_wr, c"WITHOUT".as_ptr() as *mut i8 as *const i8)
                                    } != core::ptr::null_mut()
                                {} else if unsafe {
                                        strstr(g.z_wr, c"STRICT".as_ptr() as *mut i8 as *const i8)
                                    } != core::ptr::null_mut() {
                                g.z_wr =
                                    c"WITHOUT ROWID,STRICT".as_ptr() as *mut i8 as *const i8;
                            } else {
                                g.z_wr = c"WITHOUT ROWID".as_ptr() as *mut i8 as *const i8;
                            }
                            g.z_pk = c"PRIMARY KEY".as_ptr() as *mut i8 as *const i8;
                        } else if unsafe {
                                    strcmp(z, c"strict".as_ptr() as *mut i8 as *const i8)
                                } == 0 {
                            if unsafe {
                                        strstr(g.z_wr, c"STRICT".as_ptr() as *mut i8 as *const i8)
                                    } != core::ptr::null_mut()
                                {} else if unsafe {
                                        strstr(g.z_wr, c"WITHOUT".as_ptr() as *mut i8 as *const i8)
                                    } != core::ptr::null_mut() {
                                g.z_wr =
                                    c"WITHOUT ROWID,STRICT".as_ptr() as *mut i8 as *const i8;
                            } else {
                                g.z_wr = c"STRICT".as_ptr() as *mut i8 as *const i8;
                            }
                        } else if unsafe {
                                        strcmp(z, c"help".as_ptr() as *mut i8 as *const i8)
                                    } == 0 ||
                                unsafe { strcmp(z, c"?".as_ptr() as *mut i8 as *const i8) }
                                    == 0 {
                            unsafe {
                                printf(&raw const z_help[0 as usize] as *const i8,
                                    unsafe { *argv.offset(0 as isize) })
                            };
                            unsafe { exit(0) };
                        } else {
                            unsafe {
                                fatal_error(c"unknown option: %s\nUse \"%s -?\" for help\n".as_ptr()
                                            as *mut i8 as *const i8,
                                    unsafe { *argv.offset(i as isize) },
                                    unsafe { *argv.offset(0 as isize) })
                            };
                        }
                    } else if g.z_db_name == core::ptr::null() {
                        g.z_db_name =
                            unsafe { *argv.offset(i as isize) } as *const i8;
                    } else {
                        unsafe {
                            fatal_error(c"surplus argument: %s\nUse \"%s -?\" for help\n".as_ptr()
                                        as *mut i8 as *const i8,
                                unsafe { *argv.offset(i as isize) },
                                unsafe { *argv.offset(0 as isize) })
                        };
                    }
                    break '__c65;
                }
                { let __p = &mut i; let __t = *__p; *__p += 1; __t };
            }
        }
        if n_heap > 0 {
            p_heap = unsafe { malloc(n_heap as u64) };
            if p_heap == core::ptr::null_mut() {
                unsafe {
                    fatal_error(c"cannot allocate %d-byte heap\n".as_ptr() as
                                *mut i8 as *const i8, n_heap)
                };
            }
            rc = unsafe { sqlite3_config(8, p_heap, n_heap, mn_heap) };
            if rc != 0 {
                unsafe {
                    fatal_error(c"heap configuration failed: %d\n".as_ptr() as
                                *mut i8 as *const i8, rc)
                };
            }
        }
        if do_p_cache != 0 {
            if n_p_cache > 0 && sz_p_cache > 0 {
                p_p_cache =
                    unsafe {
                        malloc((n_p_cache as Sqlite3Int64 *
                                    sz_p_cache as Sqlite3Int64) as u64)
                    };
                if p_p_cache == core::ptr::null_mut() {
                    unsafe {
                        fatal_error(c"cannot allocate %lld-byte pcache\n".as_ptr()
                                    as *mut i8 as *const i8,
                            n_p_cache as Sqlite3Int64 * sz_p_cache as Sqlite3Int64)
                    };
                }
            }
            rc =
                unsafe {
                    sqlite3_config(7, p_p_cache, sz_p_cache, n_p_cache)
                };
            if rc != 0 {
                unsafe {
                    fatal_error(c"pcache configuration failed: %d\n".as_ptr() as
                                *mut i8 as *const i8, rc)
                };
            }
        }
        if n_look >= 0 { unsafe { sqlite3_config(13, 0, 0) }; }
        unsafe { sqlite3_initialize() };
        if g.z_db_name != core::ptr::null() {
            let p_vfs: *mut Sqlite3Vfs = unsafe { sqlite3_vfs_find(g.z_vfs) };
            if p_vfs != core::ptr::null_mut() {
                unsafe {
                    (unsafe {
                            (*p_vfs).x_delete.unwrap()
                        })(p_vfs, g.z_db_name, 1)
                };
            }
            unsafe { unlink(g.z_db_name) };
        }
        if unsafe {
                    sqlite3_open_v2(if mem_db != 0 {
                            c":memory:".as_ptr() as *mut i8 as *const i8
                        } else { g.z_db_name }, &mut g.db, open_flags, g.z_vfs)
                } != 0 {
            unsafe {
                fatal_error(c"Cannot open database file: %s\n".as_ptr() as
                            *mut i8 as *const i8, g.z_db_name)
            };
        }
        if n_look > 0 && sz_look > 0 {
            p_look = unsafe { malloc((n_look * sz_look) as u64) };
            rc =
                unsafe {
                    sqlite3_db_config(g.db, 1001, p_look, sz_look, n_look)
                };
            if rc != 0 {
                unsafe {
                    fatal_error(c"lookaside configuration failed: %d\n".as_ptr()
                                as *mut i8 as *const i8, rc)
                };
            }
        }
        if g.n_reserve > 0 {
            unsafe {
                sqlite3_file_control(g.db, core::ptr::null(), 38,
                    &raw mut g.n_reserve as *mut ())
            };
        }
        if g.stmt_scan_status != 0 {
            unsafe { sqlite3_db_config(g.db, 1018, 1, 0) };
        }
        unsafe {
            sqlite3_create_function(g.db,
                c"random".as_ptr() as *mut i8 as *const i8, 0, 1,
                core::ptr::null_mut(), Some(random_func), None, None)
        };
        if do_trace != 0 {
            unsafe {
                sqlite3_trace(g.db, Some(trace_callback),
                    core::ptr::null_mut())
            };
        }
        if mem_db > 0 {
            unsafe {
                speedtest1_exec(c"PRAGMA temp_store=memory".as_ptr() as
                            *mut i8 as *const i8)
            };
        }
        if mmap_size > 0 {
            unsafe {
                speedtest1_exec(c"PRAGMA mmap_size=%d".as_ptr() as *mut i8 as
                        *const i8, mmap_size)
            };
        }
        unsafe {
            speedtest1_exec(c"PRAGMA threads=%d".as_ptr() as *mut i8 as
                    *const i8, n_thread)
        };
        if !(z_key).is_null() {
            unsafe {
                speedtest1_exec(c"PRAGMA key(\'%s\')".as_ptr() as *mut i8 as
                        *const i8, z_key)
            };
        }
        if !(z_encoding).is_null() {
            unsafe {
                speedtest1_exec(c"PRAGMA encoding=%s".as_ptr() as *mut i8 as
                        *const i8, z_encoding)
            };
        }
        if do_autovac != 0 {
            unsafe {
                speedtest1_exec(c"PRAGMA auto_vacuum=FULL".as_ptr() as *mut i8
                        as *const i8)
            };
        } else if do_incrvac != 0 {
            unsafe {
                speedtest1_exec(c"PRAGMA auto_vacuum=INCREMENTAL".as_ptr() as
                            *mut i8 as *const i8)
            };
        }
        if page_size != 0 {
            unsafe {
                speedtest1_exec(c"PRAGMA page_size=%d".as_ptr() as *mut i8 as
                        *const i8, page_size)
            };
        }
        if cache_size != 0 {
            unsafe {
                speedtest1_exec(c"PRAGMA cache_size=%d".as_ptr() as *mut i8 as
                        *const i8, cache_size)
            };
        }
        if no_sync != 0 {
            unsafe {
                speedtest1_exec(c"PRAGMA synchronous=OFF".as_ptr() as *mut i8
                        as *const i8)
            };
        } else if do_full_f_sync != 0 {
            unsafe {
                speedtest1_exec(c"PRAGMA fullfsync=ON".as_ptr() as *mut i8 as
                        *const i8)
            };
        }
        if do_exclusive != 0 {
            unsafe {
                speedtest1_exec(c"PRAGMA locking_mode=EXCLUSIVE".as_ptr() as
                            *mut i8 as *const i8)
            };
        }
        if !(z_j_mode).is_null() {
            unsafe {
                speedtest1_exec(c"PRAGMA journal_mode=%s".as_ptr() as *mut i8
                        as *const i8, z_j_mode)
            };
        }
        if n_hard_heap_lmt > 0 {
            unsafe {
                speedtest1_exec(c"PRAGMA hard_heap_limit=%d".as_ptr() as
                            *mut i8 as *const i8, n_hard_heap_lmt)
            };
        }
        if n_soft_heap_lmt > 0 {
            unsafe {
                speedtest1_exec(c"PRAGMA soft_heap_limit=%d".as_ptr() as
                            *mut i8 as *const i8, n_soft_heap_lmt)
            };
        }
        if !(z_j_mode).is_null() {
            unsafe {
                speedtest1_exec(c"PRAGMA journal_mode=%s".as_ptr() as *mut i8
                        as *const i8, z_j_mode)
            };
        }
        if g.b_explain != 0 {
            unsafe {
                printf(c".explain\n.echo on\n".as_ptr() as *mut i8 as
                        *const i8)
            };
        }
        if unsafe {
                    strcmp(z_t_set as *const i8,
                        c"mix1".as_ptr() as *mut i8 as *const i8)
                } == 0 {
            z_t_set = &raw mut z_mix1_tests[0 as usize] as *mut i8;
        }
        '__b67: loop {
            '__c67: loop {
                let z_this_test: *mut i8 = z_t_set;
                let mut z_sep: *mut i8 = core::ptr::null_mut();
                let z_comma: *mut i8 =
                    unsafe { strchr(z_this_test as *const i8, ',' as i32) };
                if !(z_comma).is_null() {
                    unsafe { *z_comma = 0 as i8 };
                    z_t_set = unsafe { z_comma.offset(1 as isize) };
                } else { z_t_set = c"".as_ptr() as *mut i8; }
                z_sep =
                    unsafe { strchr(z_this_test as *const i8, '/' as i32) };
                if !(z_sep).is_null() {
                    let mut kk: i32 = 0;
                    {
                        kk = 1;
                        '__b68: loop {
                            if !(unsafe { *z_sep.offset(kk as isize) } != 0 &&
                                            unsafe {
                                                    isdigit(unsafe { *z_sep.offset(kk as isize) } as u8 as i32)
                                                } != 0) {
                                break '__b68;
                            }
                            '__c68: loop { break '__c68; }
                            { let __p = &mut kk; let __t = *__p; *__p += 1; __t };
                        }
                    }
                    if kk == 1 ||
                            unsafe { *z_sep.offset(kk as isize) } as i32 != 0 {
                        unsafe {
                            fatal_error(c"bad modifier on testset name: \"%s\"".as_ptr()
                                        as *mut i8 as *const i8, z_this_test)
                        };
                    }
                    g.sz_test =
                        g.sz_base *
                                integer_value(unsafe { z_sep.offset(1 as isize) } as
                                        *const i8) / 100;
                    if g.sz_test <= 0 { g.sz_test = 1; }
                    unsafe { *z_sep.offset(0 as isize) = 0 as i8 };
                } else { g.sz_test = g.sz_base; }
                if g.i_total > 0 as i64 || z_comma == core::ptr::null_mut() {
                    unsafe {
                        printf(c"       Begin testset \"%s\"\n".as_ptr() as *mut i8
                                as *const i8, z_this_test)
                    };
                }
                if unsafe {
                            strcmp(z_this_test as *const i8,
                                c"main".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    testset_main();
                } else if unsafe {
                            strcmp(z_this_test as *const i8,
                                c"debug1".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    testset_debug1();
                } else if unsafe {
                            strcmp(z_this_test as *const i8,
                                c"orm".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    testset_orm();
                } else if unsafe {
                            strcmp(z_this_test as *const i8,
                                c"cte".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    testset_cte();
                } else if unsafe {
                            strcmp(z_this_test as *const i8,
                                c"star".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    testset_star();
                } else if unsafe {
                            strcmp(z_this_test as *const i8,
                                c"app".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    testset_app();
                } else if unsafe {
                            strcmp(z_this_test as *const i8,
                                c"fp".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    testset_fp();
                } else if unsafe {
                            strcmp(z_this_test as *const i8,
                                c"json".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    testset_json();
                } else if unsafe {
                            strcmp(z_this_test as *const i8,
                                c"trigger".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    testset_trigger();
                } else if unsafe {
                            strcmp(z_this_test as *const i8,
                                c"parsenumber".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    testset_parsenumber();
                } else if unsafe {
                            strcmp(z_this_test as *const i8,
                                c"rtree".as_ptr() as *mut i8 as *const i8)
                        } == 0 {
                    unsafe {
                        fatal_error(c"compile with -DSQLITE_ENABLE_RTREE to enable the R-Tree tests\n".as_ptr()
                                    as *mut i8 as *const i8)
                    };
                } else {
                    unsafe {
                        fatal_error(c"unknown testset: \"%s\"\nChoices: cte debug1 fp main orm rtree trigger\n".as_ptr()
                                    as *mut i8 as *const i8, z_this_test)
                    };
                }
                if unsafe { *z_t_set.offset(0 as isize) } != 0 {
                    let mut z_sql: *mut i8 = core::ptr::null_mut();
                    let mut z_obj: *mut i8 = core::ptr::null_mut();
                    unsafe {
                        speedtest1_begin_test(999,
                            c"Reset the database".as_ptr() as *mut i8 as *const i8)
                    };
                    loop {
                        z_obj =
                            unsafe {
                                speedtest1_once(c"SELECT name FROM main.sqlite_master WHERE sql LIKE \'CREATE %%TABLE%%\'".as_ptr()
                                            as *mut i8 as *const i8)
                            };
                        if z_obj == core::ptr::null_mut() { break; }
                        z_sql =
                            unsafe {
                                sqlite3_mprintf(c"DROP TABLE main.\"%w\"".as_ptr() as
                                            *mut i8 as *const i8, z_obj)
                            };
                        unsafe { speedtest1_exec(z_sql as *const i8) };
                        unsafe { sqlite3_free(z_sql as *mut ()) };
                        unsafe { sqlite3_free(z_obj as *mut ()) };
                    }
                    loop {
                        z_obj =
                            unsafe {
                                speedtest1_once(c"SELECT name FROM temp.sqlite_master WHERE sql LIKE \'CREATE %%TABLE%%\'".as_ptr()
                                            as *mut i8 as *const i8)
                            };
                        if z_obj == core::ptr::null_mut() { break; }
                        z_sql =
                            unsafe {
                                sqlite3_mprintf(c"DROP TABLE main.\"%w\"".as_ptr() as
                                            *mut i8 as *const i8, z_obj)
                            };
                        unsafe { speedtest1_exec(z_sql as *const i8) };
                        unsafe { sqlite3_free(z_sql as *mut ()) };
                        unsafe { sqlite3_free(z_obj as *mut ()) };
                    }
                    speedtest1_end_test();
                }
                break '__c67;
            }
            if !(unsafe { *z_t_set.offset(0 as isize) } != 0) {
                break '__b67;
            }
        }
        speedtest1_final();
        if show_stats != 0 {
            unsafe {
                sqlite3_exec(g.db,
                    c"PRAGMA compile_options".as_ptr() as *mut i8 as *const i8,
                    Some(x_compile_options), core::ptr::null_mut(),
                    core::ptr::null_mut())
            };
        }
        if show_stats != 0 {
            unsafe { sqlite3_db_status(g.db, 0, &mut i_cur, &mut i_hi, 0) };
            unsafe {
                printf(c"-- Lookaside Slots Used:        %d (max %d)\n".as_ptr()
                            as *mut i8 as *const i8, i_cur, i_hi)
            };
            unsafe { sqlite3_db_status(g.db, 4, &mut i_cur, &mut i_hi, 0) };
            unsafe {
                printf(c"-- Successful lookasides:       %d\n".as_ptr() as
                            *mut i8 as *const i8, i_hi)
            };
            unsafe { sqlite3_db_status(g.db, 5, &mut i_cur, &mut i_hi, 0) };
            unsafe {
                printf(c"-- Lookaside size faults:       %d\n".as_ptr() as
                            *mut i8 as *const i8, i_hi)
            };
            unsafe { sqlite3_db_status(g.db, 6, &mut i_cur, &mut i_hi, 0) };
            unsafe {
                printf(c"-- Lookaside OOM faults:        %d\n".as_ptr() as
                            *mut i8 as *const i8, i_hi)
            };
            unsafe { sqlite3_db_status(g.db, 1, &mut i_cur, &mut i_hi, 0) };
            unsafe {
                printf(c"-- Pager Heap Usage:            %d bytes\n".as_ptr()
                            as *mut i8 as *const i8, i_cur)
            };
            unsafe { sqlite3_db_status(g.db, 7, &mut i_cur, &mut i_hi, 1) };
            unsafe {
                printf(c"-- Page cache hits:             %d\n".as_ptr() as
                            *mut i8 as *const i8, i_cur)
            };
            unsafe { sqlite3_db_status(g.db, 8, &mut i_cur, &mut i_hi, 1) };
            unsafe {
                printf(c"-- Page cache misses:           %d\n".as_ptr() as
                            *mut i8 as *const i8, i_cur)
            };
            unsafe { sqlite3_db_status(g.db, 9, &mut i_cur, &mut i_hi, 1) };
            unsafe {
                printf(c"-- Page cache writes:           %d\n".as_ptr() as
                            *mut i8 as *const i8, i_cur)
            };
            unsafe { sqlite3_db_status(g.db, 2, &mut i_cur, &mut i_hi, 0) };
            unsafe {
                printf(c"-- Schema Heap Usage:           %d bytes\n".as_ptr()
                            as *mut i8 as *const i8, i_cur)
            };
            unsafe { sqlite3_db_status(g.db, 3, &mut i_cur, &mut i_hi, 0) };
            unsafe {
                printf(c"-- Statement Heap Usage:        %d bytes\n".as_ptr()
                            as *mut i8 as *const i8, i_cur)
            };
        }
        unsafe { sqlite3_close(g.db) };
        if show_stats != 0 {
            unsafe { sqlite3_status(0, &mut i_cur, &mut i_hi, 0) };
            unsafe {
                printf(c"-- Memory Used (bytes):         %d (max %d)\n".as_ptr()
                            as *mut i8 as *const i8, i_cur, i_hi)
            };
            unsafe { sqlite3_status(9, &mut i_cur, &mut i_hi, 0) };
            unsafe {
                printf(c"-- Outstanding Allocations:     %d (max %d)\n".as_ptr()
                            as *mut i8 as *const i8, i_cur, i_hi)
            };
            unsafe { sqlite3_status(2, &mut i_cur, &mut i_hi, 0) };
            unsafe {
                printf(c"-- Pcache Overflow Bytes:       %d (max %d)\n".as_ptr()
                            as *mut i8 as *const i8, i_cur, i_hi)
            };
            unsafe { sqlite3_status(5, &mut i_cur, &mut i_hi, 0) };
            unsafe {
                printf(c"-- Largest Allocation:          %d bytes\n".as_ptr()
                            as *mut i8 as *const i8, i_hi)
            };
            unsafe { sqlite3_status(7, &mut i_cur, &mut i_hi, 0) };
            unsafe {
                printf(c"-- Largest Pcache Allocation:   %d bytes\n".as_ptr()
                            as *mut i8 as *const i8, i_hi)
            };
        }
        if !(g.p_script).is_null() { unsafe { fclose(g.p_script) }; }
        unsafe { free(p_look) };
        unsafe { free(p_p_cache) };
        unsafe { free(p_heap) };
        return Ok(());
    }
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
static mut clock_vfs: *mut Sqlite3Vfs = core::ptr::null_mut();
static mut ones: [*const i8; 20] =
    [c"zero".as_ptr() as *const i8, c"one".as_ptr() as *const i8,
            c"two".as_ptr() as *const i8, c"three".as_ptr() as *const i8,
            c"four".as_ptr() as *const i8, c"five".as_ptr() as *const i8,
            c"six".as_ptr() as *const i8, c"seven".as_ptr() as *const i8,
            c"eight".as_ptr() as *const i8, c"nine".as_ptr() as *const i8,
            c"ten".as_ptr() as *const i8, c"eleven".as_ptr() as *const i8,
            c"twelve".as_ptr() as *const i8,
            c"thirteen".as_ptr() as *const i8,
            c"fourteen".as_ptr() as *const i8,
            c"fifteen".as_ptr() as *const i8,
            c"sixteen".as_ptr() as *const i8,
            c"seventeen".as_ptr() as *const i8,
            c"eighteen".as_ptr() as *const i8,
            c"nineteen".as_ptr() as *const i8];
static mut tens: [*const i8; 10] =
    [c"".as_ptr() as *const i8, c"ten".as_ptr() as *const i8,
            c"twenty".as_ptr() as *const i8, c"thirty".as_ptr() as *const i8,
            c"forty".as_ptr() as *const i8, c"fifty".as_ptr() as *const i8,
            c"sixty".as_ptr() as *const i8, c"seventy".as_ptr() as *const i8,
            c"eighty".as_ptr() as *const i8, c"ninety".as_ptr() as *const i8];
static mut az_puzzle: [*const i8; 3] =
    [c"534...9..67.195....98....6.8...6...34..8.3..1....2...6.6....28....419..5...28..79".as_ptr()
                as *const i8,
            c"53....9..6..195....98....6.8...6...34..8.3..1....2...6.6....28....419..5....8..79".as_ptr()
                as *const i8,
            c"53.......6..195....98....6.8...6...34..8.3..1....2...6.6....28....419..5....8..79".as_ptr()
                as *const i8];
static z_type: [i8; 120] =
    [73 as i8, 66 as i8, 66 as i8, 73 as i8, 73 as i8, 73 as i8, 84 as i8,
            73 as i8, 86 as i8, 86 as i8, 73 as i8, 84 as i8, 66 as i8,
            84 as i8, 66 as i8, 70 as i8, 66 as i8, 70 as i8, 73 as i8,
            84 as i8, 84 as i8, 70 as i8, 66 as i8, 84 as i8, 66 as i8,
            86 as i8, 66 as i8, 86 as i8, 73 as i8, 70 as i8, 84 as i8,
            66 as i8, 66 as i8, 70 as i8, 73 as i8, 84 as i8, 70 as i8,
            70 as i8, 86 as i8, 66 as i8, 73 as i8, 70 as i8, 73 as i8,
            86 as i8, 66 as i8, 86 as i8, 86 as i8, 86 as i8, 66 as i8,
            84 as i8, 86 as i8, 84 as i8, 73 as i8, 66 as i8, 66 as i8,
            70 as i8, 70 as i8, 73 as i8, 86 as i8, 73 as i8, 66 as i8,
            84 as i8, 66 as i8, 84 as i8, 86 as i8, 84 as i8, 84 as i8,
            70 as i8, 84 as i8, 86 as i8, 84 as i8, 86 as i8, 70 as i8,
            70 as i8, 73 as i8, 73 as i8, 84 as i8, 73 as i8, 70 as i8,
            66 as i8, 73 as i8, 84 as i8, 70 as i8, 84 as i8, 84 as i8,
            70 as i8, 70 as i8, 70 as i8, 86 as i8, 66 as i8, 73 as i8,
            73 as i8, 66 as i8, 84 as i8, 84 as i8, 73 as i8, 84 as i8,
            70 as i8, 84 as i8, 70 as i8, 70 as i8, 86 as i8, 86 as i8,
            86 as i8, 70 as i8, 73 as i8, 73 as i8, 73 as i8, 84 as i8,
            86 as i8, 66 as i8, 66 as i8, 86 as i8, 70 as i8, 70 as i8,
            84 as i8, 86 as i8, 86 as i8, 66 as i8, 0 as i8];
static mut z_mix1_tests: [i8; 62] =
    [109 as i8, 97 as i8, 105 as i8, 110 as i8, 44 as i8, 111 as i8,
            114 as i8, 109 as i8, 47 as i8, 50 as i8, 53 as i8, 44 as i8,
            99 as i8, 116 as i8, 101 as i8, 47 as i8, 50 as i8, 48 as i8,
            44 as i8, 106 as i8, 115 as i8, 111 as i8, 110 as i8, 44 as i8,
            102 as i8, 112 as i8, 47 as i8, 51 as i8, 44 as i8, 112 as i8,
            97 as i8, 114 as i8, 115 as i8, 101 as i8, 110 as i8, 117 as i8,
            109 as i8, 98 as i8, 101 as i8, 114 as i8, 47 as i8, 50 as i8,
            53 as i8, 44 as i8, 114 as i8, 116 as i8, 114 as i8, 101 as i8,
            101 as i8, 47 as i8, 49 as i8, 48 as i8, 44 as i8, 115 as i8,
            116 as i8, 97 as i8, 114 as i8, 44 as i8, 97 as i8, 112 as i8,
            112 as i8, 0 as i8];
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
    fn vfprintf(_: *mut FILE, _: *const i8, _: *mut i8)
    -> i32;
    fn exit(_: i32)
    -> ();
    fn fwrite(__ptr: *const (), __size: u64, __nitems: u64,
    __stream: *mut FILE)
    -> u64;
    fn isdigit(_c: i32)
    -> i32;
    fn strlen(__s: *const i8)
    -> u64;
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    fn printf(_: *const i8, ...)
    -> i32;
    fn fflush(_: *mut FILE)
    -> i32;
    fn isspace(_c: i32)
    -> i32;
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn fclose(_: *mut FILE)
    -> i32;
    fn memcpy(__dst: *mut (), __src: *const (), __n: u64)
    -> *mut ();
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn fopen(__filename: *const i8, __mode: *const i8)
    -> *mut FILE;
    fn atoi(_: *const i8)
    -> i32;
    fn strstr(__big: *const i8, __little: *const i8)
    -> *mut i8;
    fn malloc(__size: u64)
    -> *mut ();
    fn unlink(_: *const i8)
    -> i32;
    fn strchr(__s: *const i8, __c: i32)
    -> *mut i8;
    fn free(_: *mut ())
    -> ();
    static mut __stderrp: *mut FILE;
    static mut __stdoutp: *mut FILE;
    fn __builtin_va_start(_: &mut *mut i8, ...)
    -> ();
    fn __builtin_va_end(_: &mut *mut i8)
    -> ();
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}
type FILE = SFILE;