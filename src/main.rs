use autocxx::prelude::*; // use all the main autocxx functions

include_cpp! {
    // C++ headers we want to include.
    #include "libecpint/api.hpp"
    #include "run.h"
    // Safety policy. We are marking that this whole C++ inclusion is unsafe
    // which means the functions themselves do not need to be marked
    // as unsafe. Other policies are possible.
    safety!(unsafe)
    // generate!("run_ecpint")
    generate!("test_ecpint")
}

#[cxx::bridge]
mod ffi2 {
    unsafe extern "C++" {
        include!("libecpint/api.hpp");
    }
}

fn main() {
    ffi::test_ecpint();
}
