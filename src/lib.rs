#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*; // Import the bindings/functions from your main module
    use libc::c_int;

    #[test]
    fn test_ti_sma_start() {
        let options = [3.0];
        let start_value: c_int = unsafe { ti_sma_start(options.as_ptr()) };

        // Assuming ti_sma_start should return a positive value (or some expected value)
        assert!(start_value >= 0);
        // You can also check against expected specific values:
        // assert_eq!(start_value, expected_value);
    }
}
