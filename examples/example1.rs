extern crate libc;
use libc::c_int;
use std::ptr;
use tulip_rs::{ti_sma, ti_sma_start, TI_OKAY};

fn print_array(p: &[f64]) {
    for (i, &item) in p.iter().enumerate() {
        if i != 0 {
            print!(", ");
        }
        print!("{:.1}", item);
    }
    println!();
}

fn main() {
    let data_in: [f64; 10] = [5.0, 8.0, 12.0, 11.0, 9.0, 8.0, 7.0, 10.0, 11.0, 13.0];
    let input_length = data_in.len();

    println!("We have {} bars of input data.", input_length);
    print_array(&data_in);

    let options: [f64; 1] = [3.0];
    println!("Our option array is: ");
    print_array(&options);

    let start: c_int = unsafe { ti_sma_start(options.as_ptr()) };
    println!("The start amount is: {}", start);

    let output_length = input_length - start as usize;
    let mut data_out = vec![0.0; output_length];
    println!("The output length is: {}", output_length);

    let all_inputs: [*const f64; 1] = [data_in.as_ptr()];
    let mut all_outputs: [*mut f64; 1] = [data_out.as_mut_ptr()];
    let error: c_int = unsafe {
        ti_sma(
            input_length as c_int,
            all_inputs.as_ptr(),
            options.as_ptr(),
            all_outputs.as_mut_ptr(),
        )
    };
    assert_eq!(error, TI_OKAY as i32);

    println!("The output data is: ");
    print_array(&data_out);
}
