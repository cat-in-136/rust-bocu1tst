
/// State for BOCU-1 decoder function.
pub struct Boku1Rx {
    prev: i32,
    count: i32,
    diff: i32,
}

impl Boku1Rx {
    pub fn new() -> Boku1Rx {
        Boku1Rx {
            prev: 0,
            count: 0,
            diff: 0,
        }
    }

    pub fn decode_bocu1(&self, b: u8) -> i32 {
        // TODO convert
        b as i32
    }
}

