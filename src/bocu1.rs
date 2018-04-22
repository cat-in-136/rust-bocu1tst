/// initial value for "prev": middle of the ASCII range
const BOCU1_ASCII_PREV: i32 = 0x40;

// bounding byte values for differences
const BOCU1_MIN: i32 = 0x21;
const BOCU1_MIDDLE: i32 = 0x90;
const BOCU1_MAX_LEAD: i32 = 0xfe;
const BOCU1_MAX_TRAIL: i32 = 0xff;
const BOCU1_RESET: i32 = 0xff;

/// number of lead bytes
const BOCU1_COUNT: i32 = (BOCU1_MAX_LEAD - BOCU1_MIN + 1);

/// adjust trail byte counts for the use of some C0 control byte values
const BOCU1_TRAIL_CONTROLS_COUNT: i32 = 20;
const BOCU1_TRAIL_BYTE_OFFSET: i32 = (BOCU1_MIN - BOCU1_TRAIL_CONTROLS_COUNT);

/// number of trail bytes
const BOCU1_TRAIL_COUNT: i32 = ((BOCU1_MAX_TRAIL - BOCU1_MIN + 1) + BOCU1_TRAIL_CONTROLS_COUNT);

/// number of positive and negative single-byte codes
/// (counting 0==BOCU1_MIDDLE among the positive ones)
const BOCU1_SINGLE: i32 = 64;

// number of lead bytes for positive and negative 2/3/4-byte sequences
const BOCU1_LEAD_2: i32 = 43;
const BOCU1_LEAD_3: i32 = 3;
const BOCU1_LEAD_4: i32 = 1;

// The difference value range for single-byters.
const BOCU1_REACH_POS_1: i32 = (BOCU1_SINGLE - 1);
const BOCU1_REACH_NEG_1: i32 = (-BOCU1_SINGLE);

// The difference value range for double-byters.
const BOCU1_REACH_POS_2: i32 = (BOCU1_REACH_POS_1 + BOCU1_LEAD_2 * BOCU1_TRAIL_COUNT);
const BOCU1_REACH_NEG_2: i32 = (BOCU1_REACH_NEG_1 - BOCU1_LEAD_2 * BOCU1_TRAIL_COUNT);

// The difference value range for 3-byters.
const BOCU1_REACH_POS_3: i32 =
    (BOCU1_REACH_POS_2 + BOCU1_LEAD_3 * BOCU1_TRAIL_COUNT * BOCU1_TRAIL_COUNT);
const BOCU1_REACH_NEG_3: i32 =
    (BOCU1_REACH_NEG_2 - BOCU1_LEAD_3 * BOCU1_TRAIL_COUNT * BOCU1_TRAIL_COUNT);

// The lead byte start values.
const BOCU1_START_POS_2: i32 = (BOCU1_MIDDLE + BOCU1_REACH_POS_1 + 1);
const BOCU1_START_POS_3: i32 = (BOCU1_START_POS_2 + BOCU1_LEAD_2);
const BOCU1_START_POS_4: i32 = (BOCU1_START_POS_3 + BOCU1_LEAD_3);

const BOCU1_START_NEG_2: i32 = (BOCU1_MIDDLE + BOCU1_REACH_NEG_1);
const BOCU1_START_NEG_3: i32 = (BOCU1_START_NEG_2 - BOCU1_LEAD_2);
const BOCU1_START_NEG_4: i32 = (BOCU1_START_NEG_3 - BOCU1_LEAD_3);

/// Byte value map for control codes,
/// from external byte values 0x00..0x20
/// to trail byte values 0..19 (0..0x13) as used in the difference calculation.
/// External byte values that are illegal as trail bytes are mapped to -1.
#[cfg_attr(rustfmt, rustfmt_skip)]
const BOCU1_BYTE_TO_TRAIL: [i8; BOCU1_MIN as usize] = [
/*  0     1     2     3     4     5     6     7    */
    -1,   0x00, 0x01, 0x02, 0x03, 0x04, 0x05, -1,
/*  8     9     a     b     c     d     e     f    */
    -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,
/*  10    11    12    13    14    15    16    17   */
    0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
/*  18    19    1a    1b    1c    1d    1e    1f   */
    0x0e, 0x0f, -1,   -1,   0x10, 0x11, 0x12, 0x13,
/*  20   */
    -1,
];

/// State for BOCU-1 decoder function.
pub struct Bocu1Rx {
    prev: i32,
    count: i32,
    diff: i32,
}

impl Bocu1Rx {
    pub fn new() -> Bocu1Rx {
        Bocu1Rx {
            prev: 0,
            count: 0,
            diff: 0,
        }
    }

    fn decode_bocu1_lead_byte(&mut self, b: i32) -> i32 {
        let (c, count) = if b >= BOCU1_START_NEG_2 {
            /* positive difference */
            if b < BOCU1_START_POS_3 {
                /* two bytes */
                (
                    (b - BOCU1_START_POS_2) * BOCU1_TRAIL_COUNT + BOCU1_REACH_POS_1 + 1,
                    1,
                )
            } else if b < BOCU1_START_POS_4 {
                /* three bytes */
                (
                    (b - BOCU1_START_POS_3) * BOCU1_TRAIL_COUNT * BOCU1_TRAIL_COUNT
                        + BOCU1_REACH_POS_2 + 1,
                    2,
                )
            } else {
                /* four bytes */
                (BOCU1_REACH_POS_3 + 1, 3)
            }
        } else {
            /* negative difference */
            if b >= BOCU1_START_NEG_3 {
                /* two bytes */
                (
                    (b - BOCU1_START_NEG_2) * BOCU1_TRAIL_COUNT + BOCU1_REACH_NEG_1,
                    1,
                )
            } else if b > BOCU1_MIN {
                /* three bytes */
                (
                    (b - BOCU1_START_NEG_3) * BOCU1_TRAIL_COUNT * BOCU1_TRAIL_COUNT
                        + BOCU1_REACH_NEG_2,
                    2,
                )
            } else {
                (
                    -BOCU1_TRAIL_COUNT * BOCU1_TRAIL_COUNT * BOCU1_TRAIL_COUNT + BOCU1_REACH_NEG_3,
                    3,
                )
            }
        };

        self.diff = c;
        self.count = count;
        -1
    }

    fn decode_bocu1_trail_byte(&mut self, b: i32) -> i32 {
        let t: i32;

        if b <= 0x20 {
            /* skip some C0 controls and make the trail byte range contiguous */
            t = BOCU1_BYTE_TO_TRAIL[b as usize] as i32;
            if t < 0 {
                /* illegal trail byte value */
                self.prev = BOCU1_ASCII_PREV;
                self.count = 0;
                return -99;
            }
        } else if (BOCU1_MAX_TRAIL < 0xff) && (b > BOCU1_MAX_TRAIL) {
            return -99;
        } else {
            t = b - BOCU1_TRAIL_BYTE_OFFSET;
        }

        /* add trail byte into difference and decrement count */
        let c = self.diff;
        let count = self.count;

        if count == 1 {
            /* final trail byte, deliver a code point */
            let c = self.prev + c + t;
            if 0 <= c && c <= 0x10ffff {
                /* valid code point result */
                self.prev = bocu1_prev(c);
                self.count = 0;
                return c;
            } else {
                /* illegal code point result */
                self.prev = BOCU1_ASCII_PREV;
                self.count = 0;
                return -99;
            }
        }

        /* intermediate trail byte */
        if count == 2 {
            self.diff = c + t * BOCU1_TRAIL_COUNT;
        } else
        /* count==3 */
        {
            self.diff = c + t * BOCU1_TRAIL_COUNT * BOCU1_TRAIL_COUNT;
        }
        self.count = count - 1;

        -1
    }

    pub fn decode_bocu1(&mut self, b: u8) -> i32 {
        let b = b as i32;
        let mut prev = self.prev;
        let count;

        if prev == 0 {
            /* lenient handling of initial 0 values */
            prev = BOCU1_ASCII_PREV;
            self.prev = BOCU1_ASCII_PREV;
            count = 0;
            self.count = 0;
        } else {
            count = self.count;
        }

        if count == 0 {
            if b <= 0x20 {
                if b != 0x20 {
                    self.prev = BOCU1_ASCII_PREV;
                }
                b
            } else if (b >= BOCU1_START_NEG_2) && (b < BOCU1_START_POS_2) {
                let c = prev + (b - BOCU1_MIDDLE);
                self.prev = bocu1_prev(c);
                c
            } else if b == BOCU1_RESET {
                self.prev = BOCU1_ASCII_PREV;
                -1
            } else {
                self.decode_bocu1_lead_byte(b)
            }
        } else {
            self.decode_bocu1_trail_byte(b)
        }
    }
}

fn bocu1_prev(c: i32) -> i32 {
    /* compute new prev */
    if (0x3040 <= c) && (c <= 0x309f) {
        0x3070
    } else if (0x4e00 <= c) && (c <= 0x9fa5) {
        0x4e00 - BOCU1_REACH_NEG_2
    } else if (0xac00 <= c) && (c <= 0xd7a3) {
        /* Korean Hangul */
        (0xd7a3 + 0xac00) / 2
    } else {
        /* mostly small scripts */
        (c & (!0x7fi32)) + BOCU1_ASCII_PREV
    }
}
