use std::env;
use std::path::Path;
use std::fs;
use std::io::{prelude::*, BufReader};


fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = args[1].to_string();
    let scaling_factor = args[2].parse::<f64>().unwrap();
    println!("{scaling_factor}");
    assert!(check_file_exists(&filepath), "Input file {} not found.", &filepath);

    let data = read_file(&filepath);
    let code:String = determine_code(&data);

}


fn check_file_exists(filepath: &String) -> bool {
    let result = Path::new(filepath).exists();
    return result;
}


fn read_file(filepath: &String) -> Vec<String> {
    let file = fs::File::open(filepath).expect("Problem finding file.");
    let reader = BufReader::new(file);
    let mut content = Vec::new();
    for line in reader.lines() {
        content.push(line.expect("Problem reading line from input file."));
    };
    return content;
} 


fn determine_code(file:&Vec<String>) -> String {
    //TODO: make this work even with files < 4 lines long - how do try/except work in Rust?
    if file[0].contains("MCNP") {
        return String::from("MCNP");
    } else if file[2].contains("SCALE") {
        return String::from("SCALE");
    } else {
        panic!("File not identified as SCALE or MCNP output.")
    };


}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checks_file_exists() {
        let file = String::from("src/test_file/Cs_example_simple.out");
        assert!(check_file_exists(&file) == true);
    }

    #[test]
    fn check_reads_file() {
        // file should be read in a vector of Strings
        let filepath = String::from("src/test_file/poem.txt");
        let file = read_file(&filepath);
        assert_eq!(&file[0], "The Zen of Rust.");
        assert_eq!(&file[1], "");
        assert_eq!(&file[3], "Go back to Python.");
    }

    #[test]
    fn test_determines_mcnp() {
        let test_vec = vec![
            String::from("          Code Name & Version = MCNP_6.20, 6.2.0 "),
            String::from("  "),
            String::from("     _/      _/        _/_/_/       _/      _/       _/_/_/         _/_/_/ "),
            String::from("    _/_/  _/_/      _/             _/_/    _/       _/    _/     _/        "),
            String::from("   _/  _/  _/      _/             _/  _/  _/       _/_/_/       _/_/_/     "),
            String::from("  _/      _/      _/             _/    _/_/       _/           _/    _/    "),
            String::from(" _/      _/        _/_/_/       _/      _/       _/             _/_/       "),
        ];
        assert_eq!(determine_code(&test_vec), "MCNP");
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
    }

    #[test]
    fn test_determines_mcnp_null() {
        let test_vec = vec![String::from("SCALE"), String::from("SCALE"), String::from("SCALE")];
        assert_ne!(determine_code(&test_vec), "MCNP");

    }

    #[test]
    fn test_determines_scale_null() {
        let test_vec = vec![
            String::from("          Code Name & Version = MCNP_6.20, 6.2.0 "),
            String::from("  "),
            String::from("     _/      _/        _/_/_/       _/      _/       _/_/_/         _/_/_/ "),
            String::from("    _/_/  _/_/      _/             _/_/    _/       _/    _/     _/        "),
            String::from("   _/  _/  _/      _/             _/  _/  _/       _/_/_/       _/_/_/     "),
            String::from("  _/      _/      _/             _/    _/_/       _/           _/    _/    "),
            String::from(" _/      _/        _/_/_/       _/      _/       _/             _/_/       "),
        ];
        assert_ne!(determine_code(&test_vec), "SCALE");

    }
}

