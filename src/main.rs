use autocxx::prelude::*; // use all the main autocxx functions

include_cpp! {
    // C++ headers we want to include.
    #include "libecpint/api.hpp"
    #include "run.h"
    // Safety policy. We are marking that this whole C++ inclusion is unsafe
    // which means the functions themselves do not need to be marked
    // as unsafe. Other policies are possible.
    safety!(unsafe)
    generate!("test_ecpint")
    generate!("ECPIntWrapper")    

}

#[cxx::bridge]
mod ffi2 {
    unsafe extern "C++" {
        include!("libecpint/api.hpp");
    }
}

fn main() {
    let mut ecpint = ffi::ECPIntWrapper::new("/usr/share/libecpint").within_box();
    let ints = ecpint.as_mut().get_integrals();
    dbg!(ints.len());
    let derivs = ecpint.as_mut().get_first_derivs();
    dbg!(derivs.len());

}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn read_basis_file_from(
    filename: &Path,
    exps: &mut Vec<f64>,
    coeffs: &mut Vec<f64>,
    coords: &mut Vec<f64>,
    ams: &mut Vec<i32>,
    lens: &mut Vec<i32>,
    atom: &[f64; 3],
) -> io::Result<()> {
    let input_file = File::open(filename)?;
    let reader = io::BufReader::new(input_file);

    for line in reader.lines() {
        let line = line?;
        let mut tokens = line.split(';');
        let mut len = -1;
        let mut am = 0;

        for token in tokens {
            if len == -1 {
                am = i32::from_str(token.trim()).unwrap();
                len += 1;
            } else {
                let x_c: Vec<&str> = token.split(',').collect();
                if x_c.len() == 2 {
                    exps.push(f64::from_str(x_c[0].trim()).unwrap());
                    coeffs.push(f64::from_str(x_c[1].trim()).unwrap());
                    len += 1;
                }
            }
        }

        if len > 0 {
            ams.push(am);
            lens.push(len);
            coords.extend_from_slice(atom);
        }
    }

    Ok(())
}
