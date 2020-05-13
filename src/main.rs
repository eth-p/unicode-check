extern crate clap;
extern crate unicode_normalization;

use unicode_normalization::{is_nfc,is_nfkd,is_nfkc,is_nfd};
use std::str::from_utf8;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use clap::Clap;

#[derive(Clap)]
struct Opts {

    /// The file to check
    file: String,

    /// Show the result for each of the different normalization checks
    #[clap(long, short = "v")]
    verbose: bool

}

fn check(name: &String, verbose: bool) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(name)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let data_str = from_utf8(&data)?;

    // Check normalizations.
    let result_nfc = is_nfc(data_str);
    let result_nfd = is_nfd(data_str);
    let result_nfkd = is_nfkd(data_str);
    let result_nfkc = is_nfkc(data_str);

    if verbose {
        println!("NFC  Normalization: {:?}", result_nfc);
        println!("NFD  Normalization: {:?}", result_nfd);
        println!("NFKC Normalization: {:?}", result_nfkc);
        println!("NFKD Normalization: {:?}", result_nfkd);
        println!();
    }

    if result_nfc && result_nfd && result_nfkd && result_nfkc {
        println!("{} is normalized.", name);
    } else {
        println!("{} is NOT normalized.", name);
    }

    Ok(())
}

fn main()  {
    let opts: Opts = Opts::parse();

    let result = check(&opts.file, opts.verbose);
    if result.is_err() {
        eprintln!("{:}: {:}", opts.file, result.unwrap_err())
    }
}
