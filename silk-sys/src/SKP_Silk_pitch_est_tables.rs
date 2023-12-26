pub static SKP_SILK_CB_LAGS_STAGE2: [[i16; 11]; 4] = [
    [
        0i16, 2i16, -1i16, -1i16, -1i16, 0i16, 0i16, 1i16, 1i16, 0i16, 1i16,
    ],
    [
        0i16, 1i16, 0i16, 0i16, 0i16, 0i16, 0i16, 1i16, 0i16, 0i16, 0i16,
    ],
    [
        0i16, 0i16, 1i16, 0i16, 0i16, 0i16, 1i16, 0i16, 0i16, 0i16, 0i16,
    ],
    [
        0i16, -1i16, 2i16, 1i16, 0i16, 1i16, 1i16, 0i16, 0i16, -1i16, -1i16,
    ],
];
pub static SKP_SILK_CB_LAGS_STAGE3: [[i16; 34]; 4] = [
    [
        -9i16, -7i16, -6i16, -5i16, -5i16, -4i16, -4i16, -3i16, -3i16, -2i16, -2i16, -2i16, -1i16,
        -1i16, -1i16, 0i16, 0i16, 0i16, 1i16, 1i16, 0i16, 1i16, 2i16, 2i16, 2i16, 3i16, 3i16, 4i16,
        4i16, 5i16, 6i16, 5i16, 6i16, 8i16,
    ],
    [
        -3i16, -2i16, -2i16, -2i16, -1i16, -1i16, -1i16, -1i16, -1i16, 0i16, 0i16, -1i16, 0i16,
        0i16, 0i16, 0i16, 0i16, 0i16, 1i16, 0i16, 0i16, 0i16, 1i16, 1i16, 0i16, 1i16, 1i16, 2i16,
        1i16, 2i16, 2i16, 2i16, 2i16, 3i16,
    ],
    [
        3i16, 3i16, 2i16, 2i16, 2i16, 2i16, 1i16, 2i16, 1i16, 1i16, 0i16, 1i16, 1i16, 0i16, 0i16,
        0i16, 1i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, -1i16, 0i16, 0i16, -1i16, -1i16, -1i16,
        -1i16, -1i16, -2i16, -2i16, -2i16,
    ],
    [
        9i16, 8i16, 6i16, 5i16, 6i16, 5i16, 4i16, 4i16, 3i16, 3i16, 2i16, 2i16, 2i16, 1i16, 0i16,
        1i16, 1i16, 0i16, 0i16, 0i16, -1i16, -1i16, -1i16, -2i16, -2i16, -2i16, -3i16, -3i16,
        -4i16, -4i16, -5i16, -5i16, -6i16, -7i16,
    ],
];
pub static SKP_SILK_LAG_RANGE_STAGE3: [[[i16; 2]; 4]; 3] = [
    [[-2i16, 6i16], [-1i16, 5i16], [-1i16, 5i16], [-2i16, 7i16]],
    [[-4i16, 8i16], [-1i16, 6i16], [-1i16, 6i16], [-4i16, 9i16]],
    [[-9i16, 12i16], [-3i16, 7i16], [-2i16, 7i16], [-7i16, 13i16]],
];
pub static SKP_SILK_CBK_SIZES_STAGE3: [i16; 3] = [16i16, 24i16, 34i16];
pub static SKP_SILK_CBK_OFFSETS_STAGE3: [i16; 3] =
    [(34 - 16 >> 1) as i16, (34 - 24 >> 1) as i16, 0i16];
