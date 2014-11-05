#![crate_name = "Mi"]
#![feature(non_ascii_idents)]

extern crate getopts;

use std::os;
use std::io::{print};

use wrappers::{λ, ξ};

pub mod wrappers;

fn main() {
    let args = os::args();
    let opts = [
        getopts::optflag("h", "help", "display this help and exit"),
        getopts::optflag("v", "version", "output version information and exit"),
    ];
    let matches = match getopts::getopts(args.tail(), opts) {
        Ok(m) => m,
        Err(f) => panic!("Invalid options\n{}", f)
    };
    if matches.opt_present("help") {
        println!("Mi 1.0.0");
        println!("");
        println!("Usage:");
        print(getopts::usage("", opts).as_slice());
        println!("");
    }
    if matches.opt_present("version") {
        println!("Mi 1.0.0");
    }
    
    λ(||{
        ξ::<()> (|| {
                println!("Please, kill me ");
            }
        );
    });
    λ(||{
        ξ::<()> (|| { loop {
                (|r:|s:|t:|||||{r(|t:|||{t()})})
                (|s:|t:||||{s(||{()})})
            }}
        );
    });
}
