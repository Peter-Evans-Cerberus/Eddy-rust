
use std::env;
use std::fs;
use std::path::Path;
use std::io::{prelude::*, BufReader};
use tera::{Tera, Context};
mod mcnp;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath: &Path = Path::new(&args[1]);
    let scaling_factor: f64 = args[2].parse::<f64>().unwrap();
    assert!(filepath.exists(), "Input file {} not found", filepath.to_str().expect("Unable to convert provided filepath argument to String."));

    // read in data as vec<String>
    let data: Vec<String> = read_file(filepath);
    //TODO: Sanitize input string
    // check if case is MCNP or SCALE
    let code:&str = determine_code(&data);   


    if code == "MCNP" {
        mcnp::eddy_mcnp(filepath, &data, scaling_factor);
    } else if code == "SCALE" {
        println!("SCALE case recognised. Implementation not yet complete.")
    }

    // Tera templating
// Use globbing
    let mut tera = match Tera::new("src/templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    let mut context = Context::new();
    context.insert("name", &"World");

    let html = tera.render("hello.html", &context).expect("Hnggg");
    println!("{html}");


}


pub fn read_file(filepath: &Path) -> Vec<String> {
    let file = fs::File::open(filepath).expect("Problem finding file.");
    let reader = BufReader::new(file);
    let mut content = Vec::new();
    for line in reader.lines() {
        content.push(line.expect("Problem reading line from input file."));
    };
    return content;
} 


pub fn determine_code(file:&Vec<String>) -> &str {
    //TODO: make this work even with files < 4 lines long - how do try/except work in Rust?
    if file[0].contains("Code Name & Version = MCNP") || file[0].contains("1mcnp     version") {
        println!("Case identified as MCNP output.");
        return "MCNP";
    } else if file[2].contains("SCALE") {
        println!("Case identified as SCALE output.");
        return "SCALE";
    } else {
        panic!("File not identified as SCALE or MCNP output.")
    };
}



//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// TESTS /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_reads_file() {
        // file should be read in a vector of Strings
        let filepath = Path::new("src/test_file/poem.txt");
        let file = read_file(filepath);
        assert_eq!(&file[0], "The Zen of Rust.");
        assert_eq!(&file[1], "");
        assert_eq!(&file[3], "Go back to Python.");
    }

    #[test]
    fn test_determines_mcnp() {
        let mcnp6_vec = vec![
            String::from("          Code Name & Version = MCNP_6.20, 6.2.0 "),
            String::from("  "),
            String::from("     _/      _/        _/_/_/       _/      _/       _/_/_/         _/_/_/ "),
            String::from("    _/_/  _/_/      _/             _/_/    _/       _/    _/     _/        "),
            String::from("   _/  _/  _/      _/             _/  _/  _/       _/_/_/       _/_/_/     "),
            String::from("  _/      _/      _/             _/    _/_/       _/           _/    _/    "),
            String::from(" _/      _/        _/_/_/       _/      _/       _/             _/_/       "),
        ];
        let mcnp4c_vec = vec![
            String::from("1mcnp     version 4c2   ld=01/20/01                      07/12/22 10:10:20"),
            String::from(" *************************************************************************                 probid =   07/12/22 10:10:20"),
            String::from(" inp=wall.ip outp=outb.txt                                                       "),       
        ];
        assert_eq!(determine_code(&mcnp6_vec), "MCNP");
        assert_ne!(determine_code(&mcnp6_vec), "SCALE");
        assert_eq!(determine_code(&mcnp4c_vec), "MCNP");
    }

    #[test]
    fn test_determines_scale() {
        let test_vec = vec![
            String::from(" ******************************************************************************* "),
            String::from(" *                                                                             * "),
            String::from(" *                                 SCALE 6.2.4                                 * "),
            String::from(" *                                -------------                                * "),
            String::from(" *                                 August 2022                                 * "),
            String::from(" *                                                                             * "),
            String::from(" *           SCALE:  A Comprehensive Modeling and Simulation Suite             * "),
            String::from(" *                   for Nuclear Safety Analysis and Design                    * "),
            String::from(" *                                                                             * "),
            String::from(" *                      Reactor and Nuclear Systems Division                   * "),
            String::from(" *                        Oak Ridge National Laboratory                        * "),
            String::from(" *                                                                             * "),
            String::from(" *                           http://scale.ornl.gov                             * "),
            String::from(" *                            scalehelp@ornl.gov                               * "),
            String::from(" *                                                                             * "),
            String::from(" ******************************************************************************* "),
            String::from(" ******************************************************************************* "),
        ];
        assert_eq!(determine_code(&test_vec), "SCALE");
        assert_ne!(determine_code(&test_vec), "MCNP");
    }

}




