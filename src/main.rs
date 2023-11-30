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

mod io {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;
    use std::str::FromStr;

    pub fn read_basis_file(
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
            // ignore comment lines
            if line.starts_with("#") {
                continue;
            }

            let mut tokens = line.split(';');
            let mut len = -1;
            let mut am = 0;

            for token in tokens {
                // First bit is L
                if len == -1 {
                    am = i32::from_str(token.trim()).unwrap();
                    len += 1;
                } else {
                    // subsequent bits are x,c
                    let x_c: Vec<&str> = token.split(',').collect();
                    if x_c.len() == 2 {
                        exps.push(f64::from_str(x_c[0].trim()).unwrap());
                        coeffs.push(f64::from_str(x_c[1].trim()).unwrap());
                        len += 1;
                    }
                }
            }

            if len > 0 {
                // non-empty shell found
                ams.push(am);
                lens.push(len);
                coords.extend_from_slice(atom);
            }
        }

        Ok(())
    }
}

fn main() {
    let mut g_exps = vec![];
    let mut g_coeffs = vec![];
    let mut g_ams = vec![];
    let mut g_lens = vec![];
    let mut g_coords = vec![];
    let H_pos = [0.0; 3];
    let I_pos = [0.0, 0.0, 3.0];

    // hydrogen cc-pVDZ
    self::io::read_basis_file(
        "hydrogen.bas".as_ref(),
        &mut g_exps,
        &mut g_coeffs,
        &mut g_coords,
        &mut g_ams,
        &mut g_lens,
        &H_pos,
    );

    // iodine cc-pVDZ-PP
    self::io::read_basis_file(
        "iodine.bas".as_ref(),
        &mut g_exps,
        &mut g_coeffs,
        &mut g_coords,
        &mut g_ams,
        &mut g_lens,
        &I_pos,
    );
    // check sizes, should read 'Basis read: 44, 12, 36'
    assert_eq!(g_exps.len(), 44);
    assert_eq!(g_ams.len(), 12);
    assert_eq!(g_coords.len(), 36);

    let mut ecpint = ffi::ECPIntWrapper::new("/usr/share/libecpint").within_box();
    let ints = ecpint.as_mut().get_integrals();
    dbg!(ints.len());
    let derivs = ecpint.as_mut().get_first_derivs();
    dbg!(derivs.len());
}
