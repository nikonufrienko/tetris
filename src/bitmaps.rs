use rand::rngs::ThreadRng;
use rand::Rng;

pub struct TetraminoBitmap {
    h: u8,
    w: u8,
    color: (u8, u8, u8),
    bitmaps: &'static [&'static [u8]; 4],
}

#[rustfmt::skip]
pub const I_TETR: TetraminoBitmap = TetraminoBitmap {
    h: 4,
    w: 4,
    color : (20, 162, 236),
    bitmaps: &[
        &[
            0b0100,
            0b0100,
            0b0100,
            0b0100
        ],
        &[
            0b0000,
            0b1111,
            0b0000,
            0b0000
        ],
        &[
            0b0010,
            0b0010,
            0b0010,
            0b0010
        ],
        &[
            0b0000,
            0b0000,
            0b1111,
            0b0000
        ]
    ],
};

#[rustfmt::skip]
const L_TETR:TetraminoBitmap = TetraminoBitmap {
    h: 3,
    w: 3,
    color: (62, 68, 206),
    bitmaps: &[
        &[
            0b010,
            0b010,
            0b011
        ],
        &[
            0b000,
            0b111,
            0b100
        ],
        &[
            0b110,
            0b010,
            0b010
        ],
        &[
            0b001,
            0b111,
            0b000
        ]
    ]
};

#[rustfmt::skip]
const J_TETR:TetraminoBitmap = TetraminoBitmap {
    h: 3,
    w: 3,
    color: (255, 0, 255),
    bitmaps: &[
        &[
            0b010,
            0b010,
            0b110
        ],
        &[
            0b100,
            0b111,
            0b000
        ],
        &[
            0b011,
            0b010,
            0b010
        ],
        &[
            0b000,
            0b111,
            0b001
        ]
    ]
};

#[rustfmt::skip]
const O_TETR:TetraminoBitmap = TetraminoBitmap {
    h: 2,
    w: 2,
    color: (254, 199, 18),
    bitmaps: &[
        &[
            0b11,
            0b11
        ]; 4
    ]
};

#[rustfmt::skip]
pub const S_TETR:TetraminoBitmap = TetraminoBitmap {
    h: 3,
    w: 3,
    color: (36, 176, 77),
    bitmaps: &[
        &[
            0b000,
            0b011,
            0b110
        ],
        &[
            0b010,
            0b011,
            0b001
        ],
        &[
            0b000,
            0b011,
            0b110
        ],
        &[
            0b010,
            0b011,
            0b001
        ]
    ]
};

#[rustfmt::skip]
const T_TETR:TetraminoBitmap = TetraminoBitmap {
    h: 3,
    w: 3,
    color: (162,71,164),
    bitmaps: &[
        &[
            0b010,
            0b111,
            0b000
        ],
        &[
            0b010,
            0b011,
            0b010
        ],
        &[
            0b000,
            0b111,
            0b010
        ],
        &[
            0b010,
            0b110,
            0b010
        ]
    ]
};

#[rustfmt::skip]
const Z_TETR:TetraminoBitmap = TetraminoBitmap {
    h: 3,
    w: 3,
    color: (238, 32, 36),
    bitmaps: &[
        &[
            0b110,
            0b011,
            0b000,
        ],
        &[
            0b001,
            0b011,
            0b010
        ],
        &[
            0b110,
            0b011,
            0b000,
        ],
        &[
            0b001,
            0b011,
            0b010
        ]
    ]
};

const TETR_LIST: &[&'static TetraminoBitmap] = &[
    &I_TETR, &L_TETR, &J_TETR, &O_TETR, &S_TETR, &T_TETR, &Z_TETR,
];

impl TetraminoBitmap {
    pub fn get_color(&self) -> (u8, u8, u8) {
        self.color
    }

    pub fn get_dimension(&self, rot: u8) -> (u8, u8) {
        match rot {
            0 | 2 => (self.w, self.h),
            1 | 3 => (self.h, self.w),
            _ => (0, 0),
        }
    }

    pub fn is_empty_cell(&self, x: u8, y: u8, rot: u8) -> bool {
        let (w, _) = self.get_dimension(rot);
        self.bitmaps[rot as usize][y as usize] & (1 << (w - x - 1)) == 0
    }
}

pub fn get_random(rng: &mut ThreadRng) -> &'static TetraminoBitmap {
    return TETR_LIST[rng.gen_range(0..TETR_LIST.len())];
}
