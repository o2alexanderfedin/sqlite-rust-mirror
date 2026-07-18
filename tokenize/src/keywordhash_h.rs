use super::*;

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_keyword_count() -> i32 { return 147; }

pub(crate) static z_kw_text: [i8; 666] =
    ['R' as i32 as i8, 'E' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8,
            'D' as i32 as i8, 'E' as i32 as i8, 'X' as i32 as i8,
            'E' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8,
            'S' as i32 as i8, 'C' as i32 as i8, 'A' as i32 as i8,
            'P' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8,
            'C' as i32 as i8, 'H' as i32 as i8, 'E' as i32 as i8,
            'C' as i32 as i8, 'K' as i32 as i8, 'E' as i32 as i8,
            'Y' as i32 as i8, 'B' as i32 as i8, 'E' as i32 as i8,
            'F' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8,
            'E' as i32 as i8, 'I' as i32 as i8, 'G' as i32 as i8,
            'N' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8,
            'E' as i32 as i8, 'G' as i32 as i8, 'E' as i32 as i8,
            'X' as i32 as i8, 'P' as i32 as i8, 'L' as i32 as i8,
            'A' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8,
            'S' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8,
            'A' as i32 as i8, 'D' as i32 as i8, 'D' as i32 as i8,
            'A' as i32 as i8, 'T' as i32 as i8, 'A' as i32 as i8,
            'B' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8,
            'E' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8,
            'C' as i32 as i8, 'T' as i32 as i8, 'A' as i32 as i8,
            'B' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8,
            'F' as i32 as i8, 'T' as i32 as i8, 'H' as i32 as i8,
            'E' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8,
            'E' as i32 as i8, 'F' as i32 as i8, 'E' as i32 as i8,
            'R' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8,
            'B' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8,
            'L' as i32 as i8, 'S' as i32 as i8, 'E' as i32 as i8,
            'X' as i32 as i8, 'C' as i32 as i8, 'L' as i32 as i8,
            'U' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8,
            'L' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8,
            'E' as i32 as i8, 'M' as i32 as i8, 'P' as i32 as i8,
            'O' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8,
            'R' as i32 as i8, 'Y' as i32 as i8, 'I' as i32 as i8,
            'S' as i32 as i8, 'N' as i32 as i8, 'U' as i32 as i8,
            'L' as i32 as i8, 'L' as i32 as i8, 'S' as i32 as i8,
            'A' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8,
            'P' as i32 as i8, 'O' as i32 as i8, 'I' as i32 as i8,
            'N' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8,
            'R' as i32 as i8, 'S' as i32 as i8, 'E' as i32 as i8,
            'C' as i32 as i8, 'T' as i32 as i8, 'I' as i32 as i8,
            'E' as i32 as i8, 'S' as i32 as i8, 'N' as i32 as i8,
            'O' as i32 as i8, 'T' as i32 as i8, 'N' as i32 as i8,
            'U' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8,
            'I' as i32 as i8, 'K' as i32 as i8, 'E' as i32 as i8,
            'X' as i32 as i8, 'C' as i32 as i8, 'E' as i32 as i8,
            'P' as i32 as i8, 'T' as i32 as i8, 'R' as i32 as i8,
            'A' as i32 as i8, 'N' as i32 as i8, 'S' as i32 as i8,
            'A' as i32 as i8, 'C' as i32 as i8, 'T' as i32 as i8,
            'I' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8,
            'A' as i32 as i8, 'T' as i32 as i8, 'U' as i32 as i8,
            'R' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8,
            'T' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8,
            'A' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8,
            'E' as i32 as i8, 'X' as i32 as i8, 'C' as i32 as i8,
            'L' as i32 as i8, 'U' as i32 as i8, 'S' as i32 as i8,
            'I' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8,
            'X' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8,
            'T' as i32 as i8, 'S' as i32 as i8, 'C' as i32 as i8,
            'O' as i32 as i8, 'N' as i32 as i8, 'S' as i32 as i8,
            'T' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8,
            'I' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8,
            'O' as i32 as i8, 'F' as i32 as i8, 'F' as i32 as i8,
            'S' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8,
            'R' as i32 as i8, 'I' as i32 as i8, 'G' as i32 as i8,
            'G' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8,
            'A' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8,
            'E' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8,
            'R' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8,
            'E' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8,
            'T' as i32 as i8, 'A' as i32 as i8, 'C' as i32 as i8,
            'H' as i32 as i8, 'A' as i32 as i8, 'V' as i32 as i8,
            'I' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8,
            'L' as i32 as i8, 'O' as i32 as i8, 'B' as i32 as i8,
            'E' as i32 as i8, 'G' as i32 as i8, 'I' as i32 as i8,
            'N' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8,
            'R' as i32 as i8, 'E' as i32 as i8, 'F' as i32 as i8,
            'E' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8,
            'N' as i32 as i8, 'C' as i32 as i8, 'E' as i32 as i8,
            'S' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8,
            'I' as i32 as i8, 'Q' as i32 as i8, 'U' as i32 as i8,
            'E' as i32 as i8, 'R' as i32 as i8, 'Y' as i32 as i8,
            'W' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8,
            'H' as i32 as i8, 'O' as i32 as i8, 'U' as i32 as i8,
            'T' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8,
            'E' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8,
            'A' as i32 as i8, 'S' as i32 as i8, 'E' as i32 as i8,
            'A' as i32 as i8, 'T' as i32 as i8, 'T' as i32 as i8,
            'A' as i32 as i8, 'C' as i32 as i8, 'H' as i32 as i8,
            'B' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8,
            'W' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8,
            'N' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8,
            'H' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8,
            'G' as i32 as i8, 'R' as i32 as i8, 'O' as i32 as i8,
            'U' as i32 as i8, 'P' as i32 as i8, 'S' as i32 as i8,
            'C' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8,
            'C' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8,
            'E' as i32 as i8, 'F' as i32 as i8, 'A' as i32 as i8,
            'U' as i32 as i8, 'L' as i32 as i8, 'T' as i32 as i8,
            'C' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8,
            'E' as i32 as i8, 'C' as i32 as i8, 'O' as i32 as i8,
            'L' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8,
            'T' as i32 as i8, 'E' as i32 as i8, 'C' as i32 as i8,
            'R' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8,
            'T' as i32 as i8, 'E' as i32 as i8, 'C' as i32 as i8,
            'U' as i32 as i8, 'R' as i32 as i8, 'R' as i32 as i8,
            'E' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8,
            '_' as i32 as i8, 'D' as i32 as i8, 'A' as i32 as i8,
            'T' as i32 as i8, 'E' as i32 as i8, 'I' as i32 as i8,
            'M' as i32 as i8, 'M' as i32 as i8, 'E' as i32 as i8,
            'D' as i32 as i8, 'I' as i32 as i8, 'A' as i32 as i8,
            'T' as i32 as i8, 'E' as i32 as i8, 'J' as i32 as i8,
            'O' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8,
            'S' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8,
            'T' as i32 as i8, 'M' as i32 as i8, 'A' as i32 as i8,
            'T' as i32 as i8, 'C' as i32 as i8, 'H' as i32 as i8,
            'P' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8,
            'N' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8,
            'Y' as i32 as i8, 'Z' as i32 as i8, 'E' as i32 as i8,
            'P' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8,
            'G' as i32 as i8, 'M' as i32 as i8, 'A' as i32 as i8,
            'T' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8,
            'I' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8,
            'I' as i32 as i8, 'Z' as i32 as i8, 'E' as i32 as i8,
            'D' as i32 as i8, 'E' as i32 as i8, 'F' as i32 as i8,
            'E' as i32 as i8, 'R' as i32 as i8, 'R' as i32 as i8,
            'E' as i32 as i8, 'D' as i32 as i8, 'I' as i32 as i8,
            'S' as i32 as i8, 'T' as i32 as i8, 'I' as i32 as i8,
            'N' as i32 as i8, 'C' as i32 as i8, 'T' as i32 as i8,
            'U' as i32 as i8, 'P' as i32 as i8, 'D' as i32 as i8,
            'A' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8,
            'V' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8,
            'U' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8,
            'V' as i32 as i8, 'I' as i32 as i8, 'R' as i32 as i8,
            'T' as i32 as i8, 'U' as i32 as i8, 'A' as i32 as i8,
            'L' as i32 as i8, 'W' as i32 as i8, 'A' as i32 as i8,
            'Y' as i32 as i8, 'S' as i32 as i8, 'W' as i32 as i8,
            'H' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8,
            'W' as i32 as i8, 'H' as i32 as i8, 'E' as i32 as i8,
            'R' as i32 as i8, 'E' as i32 as i8, 'C' as i32 as i8,
            'U' as i32 as i8, 'R' as i32 as i8, 'S' as i32 as i8,
            'I' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8,
            'A' as i32 as i8, 'B' as i32 as i8, 'O' as i32 as i8,
            'R' as i32 as i8, 'T' as i32 as i8, 'A' as i32 as i8,
            'F' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8,
            'R' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8,
            'A' as i32 as i8, 'M' as i32 as i8, 'E' as i32 as i8,
            'A' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8,
            'R' as i32 as i8, 'O' as i32 as i8, 'P' as i32 as i8,
            'A' as i32 as i8, 'R' as i32 as i8, 'T' as i32 as i8,
            'I' as i32 as i8, 'T' as i32 as i8, 'I' as i32 as i8,
            'O' as i32 as i8, 'N' as i32 as i8, 'A' as i32 as i8,
            'U' as i32 as i8, 'T' as i32 as i8, 'O' as i32 as i8,
            'I' as i32 as i8, 'N' as i32 as i8, 'C' as i32 as i8,
            'R' as i32 as i8, 'E' as i32 as i8, 'M' as i32 as i8,
            'E' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8,
            'C' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8,
            'T' as i32 as i8, 'C' as i32 as i8, 'O' as i32 as i8,
            'L' as i32 as i8, 'U' as i32 as i8, 'M' as i32 as i8,
            'N' as i32 as i8, 'C' as i32 as i8, 'O' as i32 as i8,
            'M' as i32 as i8, 'M' as i32 as i8, 'I' as i32 as i8,
            'T' as i32 as i8, 'C' as i32 as i8, 'O' as i32 as i8,
            'N' as i32 as i8, 'F' as i32 as i8, 'L' as i32 as i8,
            'I' as i32 as i8, 'C' as i32 as i8, 'T' as i32 as i8,
            'C' as i32 as i8, 'R' as i32 as i8, 'O' as i32 as i8,
            'S' as i32 as i8, 'S' as i32 as i8, 'C' as i32 as i8,
            'U' as i32 as i8, 'R' as i32 as i8, 'R' as i32 as i8,
            'E' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8,
            '_' as i32 as i8, 'T' as i32 as i8, 'I' as i32 as i8,
            'M' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8,
            'T' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8,
            'P' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8,
            'C' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8,
            'I' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8,
            'F' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8,
            'L' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8,
            'T' as i32 as i8, 'F' as i32 as i8, 'I' as i32 as i8,
            'L' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8,
            'R' as i32 as i8, 'E' as i32 as i8, 'P' as i32 as i8,
            'L' as i32 as i8, 'A' as i32 as i8, 'C' as i32 as i8,
            'E' as i32 as i8, 'F' as i32 as i8, 'I' as i32 as i8,
            'R' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8,
            'F' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8,
            'L' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8,
            'I' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8,
            'F' as i32 as i8, 'R' as i32 as i8, 'O' as i32 as i8,
            'M' as i32 as i8, 'F' as i32 as i8, 'U' as i32 as i8,
            'L' as i32 as i8, 'L' as i32 as i8, 'I' as i32 as i8,
            'M' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8,
            'I' as i32 as i8, 'F' as i32 as i8, 'O' as i32 as i8,
            'R' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8,
            'R' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8,
            'T' as i32 as i8, 'R' as i32 as i8, 'I' as i32 as i8,
            'C' as i32 as i8, 'T' as i32 as i8, 'O' as i32 as i8,
            'T' as i32 as i8, 'H' as i32 as i8, 'E' as i32 as i8,
            'R' as i32 as i8, 'S' as i32 as i8, 'O' as i32 as i8,
            'V' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8,
            'E' as i32 as i8, 'T' as i32 as i8, 'U' as i32 as i8,
            'R' as i32 as i8, 'N' as i32 as i8, 'I' as i32 as i8,
            'N' as i32 as i8, 'G' as i32 as i8, 'R' as i32 as i8,
            'I' as i32 as i8, 'G' as i32 as i8, 'H' as i32 as i8,
            'T' as i32 as i8, 'R' as i32 as i8, 'O' as i32 as i8,
            'L' as i32 as i8, 'L' as i32 as i8, 'B' as i32 as i8,
            'A' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8,
            'R' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8,
            'S' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8,
            'B' as i32 as i8, 'O' as i32 as i8, 'U' as i32 as i8,
            'N' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8,
            'D' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8,
            'I' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8,
            'U' as i32 as i8, 'S' as i32 as i8, 'I' as i32 as i8,
            'N' as i32 as i8, 'G' as i32 as i8, 'V' as i32 as i8,
            'A' as i32 as i8, 'C' as i32 as i8, 'U' as i32 as i8,
            'U' as i32 as i8, 'M' as i32 as i8, 'V' as i32 as i8,
            'I' as i32 as i8, 'E' as i32 as i8, 'W' as i32 as i8,
            'I' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8,
            'O' as i32 as i8, 'W' as i32 as i8, 'B' as i32 as i8,
            'Y' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8,
            'I' as i32 as i8, 'T' as i32 as i8, 'I' as i32 as i8,
            'A' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8,
            'Y' as i32 as i8, 'P' as i32 as i8, 'R' as i32 as i8,
            'I' as i32 as i8, 'M' as i32 as i8, 'A' as i32 as i8,
            'R' as i32 as i8, 'Y' as i32 as i8];

pub(crate) static a_kw_offset: [u16; 148] =
    [0 as u16, 0 as u16, 2 as u16, 2 as u16, 8 as u16, 9 as u16, 14 as u16,
            16 as u16, 20 as u16, 23 as u16, 25 as u16, 25 as u16, 29 as u16,
            33 as u16, 36 as u16, 41 as u16, 46 as u16, 48 as u16, 53 as u16,
            54 as u16, 59 as u16, 62 as u16, 65 as u16, 67 as u16, 69 as u16,
            78 as u16, 81 as u16, 86 as u16, 90 as u16, 90 as u16, 94 as u16,
            99 as u16, 101 as u16, 105 as u16, 111 as u16, 119 as u16,
            123 as u16, 123 as u16, 123 as u16, 126 as u16, 129 as u16,
            132 as u16, 137 as u16, 142 as u16, 146 as u16, 147 as u16,
            152 as u16, 156 as u16, 160 as u16, 168 as u16, 174 as u16,
            181 as u16, 184 as u16, 184 as u16, 187 as u16, 189 as u16,
            195 as u16, 198 as u16, 206 as u16, 211 as u16, 216 as u16,
            219 as u16, 222 as u16, 226 as u16, 236 as u16, 239 as u16,
            244 as u16, 244 as u16, 248 as u16, 252 as u16, 259 as u16,
            265 as u16, 271 as u16, 277 as u16, 277 as u16, 283 as u16,
            284 as u16, 288 as u16, 295 as u16, 299 as u16, 306 as u16,
            312 as u16, 324 as u16, 333 as u16, 335 as u16, 341 as u16,
            346 as u16, 348 as u16, 355 as u16, 359 as u16, 370 as u16,
            377 as u16, 378 as u16, 385 as u16, 391 as u16, 397 as u16,
            402 as u16, 408 as u16, 412 as u16, 415 as u16, 424 as u16,
            429 as u16, 433 as u16, 439 as u16, 441 as u16, 444 as u16,
            453 as u16, 455 as u16, 457 as u16, 466 as u16, 470 as u16,
            476 as u16, 482 as u16, 490 as u16, 495 as u16, 495 as u16,
            495 as u16, 511 as u16, 520 as u16, 523 as u16, 527 as u16,
            532 as u16, 539 as u16, 544 as u16, 553 as u16, 557 as u16,
            560 as u16, 565 as u16, 567 as u16, 571 as u16, 579 as u16,
            585 as u16, 588 as u16, 597 as u16, 602 as u16, 610 as u16,
            610 as u16, 614 as u16, 623 as u16, 628 as u16, 633 as u16,
            639 as u16, 642 as u16, 645 as u16, 648 as u16, 650 as u16,
            655 as u16, 659 as u16];

pub(crate) static a_kw_len: [u8; 148] =
    [0 as u8, 7 as u8, 7 as u8, 5 as u8, 4 as u8, 6 as u8, 4 as u8, 5 as u8,
            3 as u8, 6 as u8, 7 as u8, 3 as u8, 6 as u8, 6 as u8, 7 as u8,
            7 as u8, 3 as u8, 8 as u8, 2 as u8, 6 as u8, 5 as u8, 4 as u8,
            4 as u8, 3 as u8, 10 as u8, 4 as u8, 7 as u8, 6 as u8, 9 as u8,
            4 as u8, 2 as u8, 6 as u8, 5 as u8, 9 as u8, 9 as u8, 4 as u8,
            7 as u8, 3 as u8, 2 as u8, 4 as u8, 4 as u8, 6 as u8, 11 as u8,
            6 as u8, 2 as u8, 7 as u8, 5 as u8, 5 as u8, 9 as u8, 6 as u8,
            10 as u8, 4 as u8, 6 as u8, 2 as u8, 3 as u8, 7 as u8, 5 as u8,
            9 as u8, 6 as u8, 6 as u8, 4 as u8, 5 as u8, 5 as u8, 10 as u8,
            6 as u8, 5 as u8, 7 as u8, 4 as u8, 5 as u8, 7 as u8, 6 as u8,
            7 as u8, 7 as u8, 6 as u8, 5 as u8, 7 as u8, 3 as u8, 7 as u8,
            4 as u8, 7 as u8, 6 as u8, 12 as u8, 9 as u8, 4 as u8, 6 as u8,
            5 as u8, 4 as u8, 7 as u8, 6 as u8, 12 as u8, 8 as u8, 8 as u8,
            2 as u8, 6 as u8, 6 as u8, 7 as u8, 6 as u8, 4 as u8, 5 as u8,
            9 as u8, 5 as u8, 5 as u8, 6 as u8, 3 as u8, 4 as u8, 9 as u8,
            13 as u8, 2 as u8, 2 as u8, 4 as u8, 6 as u8, 6 as u8, 8 as u8,
            5 as u8, 17 as u8, 12 as u8, 7 as u8, 9 as u8, 4 as u8, 4 as u8,
            6 as u8, 7 as u8, 5 as u8, 9 as u8, 4 as u8, 4 as u8, 5 as u8,
            2 as u8, 5 as u8, 8 as u8, 6 as u8, 4 as u8, 9 as u8, 5 as u8,
            8 as u8, 4 as u8, 3 as u8, 9 as u8, 5 as u8, 5 as u8, 6 as u8,
            4 as u8, 6 as u8, 2 as u8, 2 as u8, 9 as u8, 3 as u8, 7 as u8];

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_keyword_name(mut i: i32, pz_name: &mut *const i8,
    pn_name: &mut i32) -> i32 {
    if i < 0 || i >= 147 { return 1; }
    { let __p = &mut i; let __t = *__p; *__p += 1; __t };
    *pz_name =
        unsafe {
            (&raw const z_kw_text[0 as usize] as
                    *const i8).add(a_kw_offset[i as usize] as usize)
        };
    *pn_name = a_kw_len[i as usize] as i32;
    return 0;
}

pub(crate) static a_kw_hash: [u8; 127] =
    [84 as u8, 92 as u8, 134 as u8, 82 as u8, 105 as u8, 29 as u8, 0 as u8,
            0 as u8, 94 as u8, 0 as u8, 85 as u8, 72 as u8, 0 as u8, 53 as u8,
            35 as u8, 86 as u8, 15 as u8, 0 as u8, 42 as u8, 97 as u8,
            54 as u8, 89 as u8, 135 as u8, 19 as u8, 0 as u8, 0 as u8,
            140 as u8, 0 as u8, 40 as u8, 129 as u8, 0 as u8, 22 as u8,
            107 as u8, 0 as u8, 9 as u8, 0 as u8, 0 as u8, 123 as u8,
            80 as u8, 0 as u8, 78 as u8, 6 as u8, 0 as u8, 65 as u8,
            103 as u8, 147 as u8, 0 as u8, 136 as u8, 115 as u8, 0 as u8,
            0 as u8, 48 as u8, 0 as u8, 90 as u8, 24 as u8, 0 as u8, 17 as u8,
            0 as u8, 27 as u8, 70 as u8, 23 as u8, 26 as u8, 5 as u8,
            60 as u8, 142 as u8, 110 as u8, 122 as u8, 0 as u8, 73 as u8,
            91 as u8, 71 as u8, 145 as u8, 61 as u8, 120 as u8, 74 as u8,
            0 as u8, 49 as u8, 0 as u8, 11 as u8, 41 as u8, 0 as u8,
            113 as u8, 0 as u8, 0 as u8, 0 as u8, 109 as u8, 10 as u8,
            111 as u8, 116 as u8, 125 as u8, 14 as u8, 50 as u8, 124 as u8,
            0 as u8, 100 as u8, 0 as u8, 18 as u8, 121 as u8, 144 as u8,
            56 as u8, 130 as u8, 139 as u8, 88 as u8, 83 as u8, 37 as u8,
            30 as u8, 126 as u8, 0 as u8, 0 as u8, 108 as u8, 51 as u8,
            131 as u8, 128 as u8, 0 as u8, 34 as u8, 0 as u8, 0 as u8,
            132 as u8, 0 as u8, 98 as u8, 38 as u8, 39 as u8, 0 as u8,
            20 as u8, 45 as u8, 117 as u8, 93 as u8];

pub(crate) static a_kw_next: [u8; 148] =
    [0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 4 as u8, 0 as u8, 43 as u8,
            0 as u8, 0 as u8, 106 as u8, 114 as u8, 0 as u8, 0 as u8, 0 as u8,
            2 as u8, 0 as u8, 0 as u8, 143 as u8, 0 as u8, 0 as u8, 0 as u8,
            13 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 141 as u8, 0 as u8,
            0 as u8, 119 as u8, 52 as u8, 0 as u8, 0 as u8, 137 as u8,
            12 as u8, 0 as u8, 0 as u8, 62 as u8, 0 as u8, 138 as u8, 0 as u8,
            133 as u8, 0 as u8, 0 as u8, 36 as u8, 0 as u8, 0 as u8, 28 as u8,
            77 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 59 as u8, 0 as u8,
            47 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
            0 as u8, 0 as u8, 0 as u8, 0 as u8, 69 as u8, 0 as u8, 0 as u8,
            0 as u8, 0 as u8, 0 as u8, 146 as u8, 3 as u8, 0 as u8, 58 as u8,
            0 as u8, 1 as u8, 75 as u8, 0 as u8, 0 as u8, 0 as u8, 31 as u8,
            0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 127 as u8, 0 as u8,
            104 as u8, 0 as u8, 64 as u8, 66 as u8, 63 as u8, 0 as u8,
            0 as u8, 0 as u8, 0 as u8, 0 as u8, 46 as u8, 0 as u8, 16 as u8,
            8 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8, 0 as u8,
            0 as u8, 0 as u8, 0 as u8, 0 as u8, 81 as u8, 101 as u8, 0 as u8,
            112 as u8, 21 as u8, 7 as u8, 67 as u8, 0 as u8, 79 as u8,
            96 as u8, 118 as u8, 0 as u8, 0 as u8, 68 as u8, 0 as u8, 0 as u8,
            99 as u8, 44 as u8, 0 as u8, 55 as u8, 0 as u8, 76 as u8, 0 as u8,
            95 as u8, 32 as u8, 33 as u8, 57 as u8, 25 as u8, 0 as u8,
            102 as u8, 0 as u8, 0 as u8, 87 as u8];

pub(crate) static a_kw_code: [u8; 148] =
    [0 as u8, 99 as u8, 117 as u8, 162 as u8, 39 as u8, 59 as u8, 41 as u8,
            125 as u8, 68 as u8, 33 as u8, 133 as u8, 63 as u8, 64 as u8,
            48 as u8, 2 as u8, 66 as u8, 164 as u8, 38 as u8, 24 as u8,
            139 as u8, 16 as u8, 119 as u8, 160 as u8, 11 as u8, 132 as u8,
            161 as u8, 92 as u8, 129 as u8, 21 as u8, 21 as u8, 43 as u8,
            51 as u8, 83 as u8, 13 as u8, 138 as u8, 95 as u8, 52 as u8,
            19 as u8, 67 as u8, 122 as u8, 48 as u8, 137 as u8, 6 as u8,
            28 as u8, 116 as u8, 119 as u8, 163 as u8, 72 as u8, 9 as u8,
            20 as u8, 120 as u8, 152 as u8, 70 as u8, 69 as u8, 131 as u8,
            78 as u8, 90 as u8, 96 as u8, 40 as u8, 148 as u8, 48 as u8,
            5 as u8, 119 as u8, 126 as u8, 124 as u8, 3 as u8, 26 as u8,
            82 as u8, 119 as u8, 14 as u8, 32 as u8, 49 as u8, 153 as u8,
            93 as u8, 147 as u8, 35 as u8, 31 as u8, 121 as u8, 158 as u8,
            114 as u8, 17 as u8, 101 as u8, 8 as u8, 144 as u8, 128 as u8,
            47 as u8, 4 as u8, 30 as u8, 71 as u8, 98 as u8, 7 as u8,
            141 as u8, 45 as u8, 130 as u8, 140 as u8, 81 as u8, 97 as u8,
            159 as u8, 150 as u8, 73 as u8, 27 as u8, 29 as u8, 100 as u8,
            44 as u8, 134 as u8, 88 as u8, 127 as u8, 15 as u8, 50 as u8,
            36 as u8, 61 as u8, 10 as u8, 37 as u8, 119 as u8, 101 as u8,
            101 as u8, 86 as u8, 89 as u8, 42 as u8, 85 as u8, 167 as u8,
            74 as u8, 84 as u8, 87 as u8, 143 as u8, 119 as u8, 149 as u8,
            18 as u8, 146 as u8, 75 as u8, 94 as u8, 166 as u8, 151 as u8,
            119 as u8, 12 as u8, 77 as u8, 76 as u8, 91 as u8, 135 as u8,
            145 as u8, 79 as u8, 80 as u8, 165 as u8, 62 as u8, 34 as u8,
            65 as u8, 136 as u8, 123 as u8];

pub(crate) extern "C" fn keyword_code(z: *const i8, n: i64,
    p_type_1: &mut i32) -> i64 {
    unsafe {
        let mut i: i64 = 0 as i64;
        let mut j: i64 = 0 as i64;
        let mut z_kw: *const i8 = core::ptr::null();
        { let _ = 0; };
        i =
            ((unsafe {
                                        *(sqlite3_upper_to_lower.as_ptr() as
                                                    *const u8).add(unsafe { *z.offset(0 as isize) } as u8 as
                                                    usize)
                                    } as i32 * 4 ^
                            unsafe {
                                        *(sqlite3_upper_to_lower.as_ptr() as
                                                    *const u8).add(unsafe { *z.offset((n - 1 as i64) as isize) }
                                                        as u8 as usize)
                                    } as i32 * 3) as i64 ^ n * 1 as i64) % 127 as i64;
        {
            i = a_kw_hash[i as usize] as i32 as i64;
            '__b0: loop {
                if !(i > 0 as i64) { break '__b0; }
                '__c0: loop {
                    if a_kw_len[i as usize] as i64 != n { break '__c0; }
                    z_kw = &z_kw_text[a_kw_offset[i as usize] as usize];
                    if unsafe { *z.offset(0 as isize) } as i32 & !32 !=
                            unsafe { *z_kw.offset(0 as isize) } as i32 {
                        break '__c0;
                    }
                    if unsafe { *z.offset(1 as isize) } as i32 & !32 !=
                            unsafe { *z_kw.offset(1 as isize) } as i32 {
                        break '__c0;
                    }
                    j = 2 as i64;
                    while j < n &&
                            unsafe { *z.offset(j as isize) } as i32 & !32 ==
                                unsafe { *z_kw.offset(j as isize) } as i32 {
                        { let __p = &mut j; let __t = *__p; *__p += 1; __t };
                    }
                    if j < n { break '__c0; }
                    *p_type_1 = a_kw_code[i as usize] as i32;
                    break '__b0;
                    break '__c0;
                }
                i = a_kw_next[i as usize] as i64;
            }
        }
        return n;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_keyword_code(z: *const u8, n: i32) -> i32 {
    let mut id: i32 = 60;
    if n >= 2 { keyword_code(z as *mut i8 as *const i8, n as i64, &mut id); }
    return id;
}

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_keyword_check(z_name: *const i8, n_name: i32)
    -> i32 {
    return (60 != sqlite3_keyword_code(z_name as *const u8, n_name)) as i32;
}
