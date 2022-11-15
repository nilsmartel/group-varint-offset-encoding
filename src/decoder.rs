pub(crate) fn decode_block(v: u8, data: &[u8]) -> (u32, u32, u32, usize) {
    match v & 0b11_11_11 {
        // GEN TABLE HERE
        0 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = data[2] as u32;
            (v0, v1, v2, 3)
        }
        1 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = data[3] as u32;
            (v0, v1, v2, 4)
        }
        2 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = data[4] as u32;
            (v0, v1, v2, 5)
        }
        3 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = data[5] as u32;
            (v0, v1, v2, 6)
        }
        4 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = data[3] as u32;
            (v0, v1, v2, 4)
        }
        5 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = data[4] as u32;
            (v0, v1, v2, 5)
        }
        6 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = data[5] as u32;
            (v0, v1, v2, 6)
        }
        7 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = data[6] as u32;
            (v0, v1, v2, 7)
        }
        8 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = data[4] as u32;
            (v0, v1, v2, 5)
        }
        9 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = data[5] as u32;
            (v0, v1, v2, 6)
        }
        10 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = data[6] as u32;
            (v0, v1, v2, 7)
        }
        11 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = data[7] as u32;
            (v0, v1, v2, 8)
        }
        12 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = data[5] as u32;
            (v0, v1, v2, 6)
        }
        13 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = data[6] as u32;
            (v0, v1, v2, 7)
        }
        14 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = data[7] as u32;
            (v0, v1, v2, 8)
        }
        15 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v2 = data[8] as u32;
            (v0, v1, v2, 9)
        }
        16 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = (data[2] as u32) | (data[3] as u32) << 8;
            (v0, v1, v2, 4)
        }
        17 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8;
            (v0, v1, v2, 5)
        }
        18 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8;
            (v0, v1, v2, 6)
        }
        19 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            (v0, v1, v2, 7)
        }
        20 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8;
            (v0, v1, v2, 5)
        }
        21 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8;
            (v0, v1, v2, 6)
        }
        22 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            (v0, v1, v2, 7)
        }
        23 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8;
            (v0, v1, v2, 8)
        }
        24 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8;
            (v0, v1, v2, 6)
        }
        25 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            (v0, v1, v2, 7)
        }
        26 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8;
            (v0, v1, v2, 8)
        }
        27 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8;
            (v0, v1, v2, 9)
        }
        28 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8;
            (v0, v1, v2, 7)
        }
        29 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8;
            (v0, v1, v2, 8)
        }
        30 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8;
            (v0, v1, v2, 9)
        }
        31 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v2 = (data[8] as u32) | (data[9] as u32) << 8;
            (v0, v1, v2, 10)
        }
        32 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            (v0, v1, v2, 5)
        }
        33 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            (v0, v1, v2, 6)
        }
        34 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            (v0, v1, v2, 7)
        }
        35 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            (v0, v1, v2, 8)
        }
        36 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            (v0, v1, v2, 6)
        }
        37 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            (v0, v1, v2, 7)
        }
        38 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            (v0, v1, v2, 8)
        }
        39 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            (v0, v1, v2, 9)
        }
        40 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            (v0, v1, v2, 7)
        }
        41 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            (v0, v1, v2, 8)
        }
        42 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            (v0, v1, v2, 9)
        }
        43 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            (v0, v1, v2, 10)
        }
        44 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = (data[5] as u32) | (data[6] as u32) << 8 | (data[7] as u32) << 16;
            (v0, v1, v2, 8)
        }
        45 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = (data[6] as u32) | (data[7] as u32) << 8 | (data[8] as u32) << 16;
            (v0, v1, v2, 9)
        }
        46 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = (data[7] as u32) | (data[8] as u32) << 8 | (data[9] as u32) << 16;
            (v0, v1, v2, 10)
        }
        47 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v2 = (data[8] as u32) | (data[9] as u32) << 8 | (data[10] as u32) << 16;
            (v0, v1, v2, 11)
        }
        48 => {
            let v0 = data[0] as u32;
            let v1 = data[1] as u32;
            let v2 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            (v0, v1, v2, 6)
        }
        49 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = data[2] as u32;
            let v2 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            (v0, v1, v2, 7)
        }
        50 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = data[3] as u32;
            let v2 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            (v0, v1, v2, 8)
        }
        51 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = data[4] as u32;
            let v2 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            (v0, v1, v2, 9)
        }
        52 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8;
            let v2 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            (v0, v1, v2, 7)
        }
        53 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8;
            let v2 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            (v0, v1, v2, 8)
        }
        54 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8;
            let v2 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            (v0, v1, v2, 9)
        }
        55 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8;
            let v2 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            (v0, v1, v2, 10)
        }
        56 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32) | (data[2] as u32) << 8 | (data[3] as u32) << 16;
            let v2 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            (v0, v1, v2, 8)
        }
        57 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32) | (data[3] as u32) << 8 | (data[4] as u32) << 16;
            let v2 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            (v0, v1, v2, 9)
        }
        58 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32) | (data[4] as u32) << 8 | (data[5] as u32) << 16;
            let v2 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            (v0, v1, v2, 10)
        }
        59 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32) | (data[5] as u32) << 8 | (data[6] as u32) << 16;
            let v2 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            (v0, v1, v2, 11)
        }
        60 => {
            let v0 = data[0] as u32;
            let v1 = (data[1] as u32)
                | (data[2] as u32) << 8
                | (data[3] as u32) << 16
                | (data[4] as u32) << 24;
            let v2 = (data[5] as u32)
                | (data[6] as u32) << 8
                | (data[7] as u32) << 16
                | (data[8] as u32) << 24;
            (v0, v1, v2, 9)
        }
        61 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8;
            let v1 = (data[2] as u32)
                | (data[3] as u32) << 8
                | (data[4] as u32) << 16
                | (data[5] as u32) << 24;
            let v2 = (data[6] as u32)
                | (data[7] as u32) << 8
                | (data[8] as u32) << 16
                | (data[9] as u32) << 24;
            (v0, v1, v2, 10)
        }
        62 => {
            let v0 = (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16;
            let v1 = (data[3] as u32)
                | (data[4] as u32) << 8
                | (data[5] as u32) << 16
                | (data[6] as u32) << 24;
            let v2 = (data[7] as u32)
                | (data[8] as u32) << 8
                | (data[9] as u32) << 16
                | (data[10] as u32) << 24;
            (v0, v1, v2, 11)
        }
        63 => {
            let v0 = (data[0] as u32)
                | (data[1] as u32) << 8
                | (data[2] as u32) << 16
                | (data[3] as u32) << 24;
            let v1 = (data[4] as u32)
                | (data[5] as u32) << 8
                | (data[6] as u32) << 16
                | (data[7] as u32) << 24;
            let v2 = (data[8] as u32)
                | (data[9] as u32) << 8
                | (data[10] as u32) << 16
                | (data[11] as u32) << 24;
            (v0, v1, v2, 12)
        } // END
        _ => unreachable!(),
    }
}
