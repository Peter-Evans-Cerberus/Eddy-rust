
use std::env;
use std::path::Path;
mod initial_checks;
mod mcnp;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = Path::new(&args[1]);
    let scaling_factor = args[2].parse::<f64>().unwrap();
    //TODO: remove println below
    println!("{scaling_factor}");
    assert!(initial_checks::check_file_exists(filepath), "Input file {:?} not found.", filepath.to_str());

    // read in data as vec<String>
    let data = initial_checks::read_file(filepath);
    //TODO: Sanitize input string
    // check if case is MCNP or SCALE
    let code:&str = initial_checks::determine_code(&data);   


    if code == "MCNP" {
        mcnp::eddy_mcnp(filepath, &data, scaling_factor);
    } else if code == "SCALE" {
        println!("SCALE case recognised. Implementation not yet complete.")
    }

}
