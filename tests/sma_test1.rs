use std::ffi::c_int;

use tulip_rs::ti_sma_start;

#[test]
fn sanity_sma_test() {
    let data = vec![1.0, 1.0, 1.0, 1.0];
    let sma_options = 1.0;
    let start: c_int = unsafe { ti_sma_start(&sma_options as *const f64) };
    let mut data_out: Vec<f64> = vec![0.0; data.len() - start as usize];
    let all_inputs: [*const f64; 1] = [data.as_ptr()];
    let mut all_outputs: [*mut f64; 1] = [data_out.as_mut_ptr()];

    let error: c_int = unsafe {
        tulip_rs::ti_sma(
            data.len() as c_int,
            all_inputs.as_ptr(),
            &sma_options as *const f64,
            all_outputs.as_mut_ptr(),
        )
    };
    assert_eq!(error, tulip_rs::TI_OKAY as i32);
    // Make sure the output is correct
    assert_eq!(data_out, vec![1.0, 1.0, 1.0, 1.0]);
}
