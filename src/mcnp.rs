extern crate regex;
use regex::Regex;
use std::path::Path;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;
use tera::{Tera, Context};
use chrono;


pub fn eddy_mcnp(filepath: &Path, content:&Vec<String>, scaling_factor:f64) {

    let name = filepath.file_name().expect("Error finding MCNP output filename.").to_str().expect("Failed to convert filename to str.");
    let filename:String = String::from(name);

    let location = filepath.parent().unwrap().display().to_string();
    // check if crit case
    let crit = check_if_crit(content);

    let (rundate, runtime) = get_rundate(content);

    let (ctme, nps) = get_run_length(content);

    let mcnp_input: Vec<String> = get_input(content);

    let parameters: HashMap<String, String> = get_parameters(&mcnp_input);

    let lost_particles:bool = check_lost_particles(content);


    //TODO: check_fatal_errors
    //TODO: get_warnings
    //TODO: get_comments
    //TODO: get duplicate surfaces
    //TODO: get_k_eff
    //TODO: get_active_cycles
    //TODO: get_cell_data
    //TODO: get_particle_populations

    // The sections below were implemeted following logic that makes sense for python
    // and may not necessarily be the best implementations for rust
    //TODO: create_cells ?
    //TODO: find_mcnp_particle_data ?
    //TODO: create_particle ?
    //TODO: get_tallies ?



    // Placeholder
    let particle_list = Vec::new();     // will need to be made mutable
    // particle_list.push("photon");








    // Get html from template using data
    let html = get_html(
        &filename,
        &scaling_factor,
        &crit,
        &rundate,
        &runtime,
        &ctme, 
        &nps,
        &parameters,
        &lost_particles,
        &mcnp_input,
        &particle_list,
    );
    // Write html string to file
    write_html(html, location, filename);

}


pub fn check_if_crit(content:&Vec<String>) -> bool {
    // Returns a bool:
    // true if this is crit case (if "kcode" is detected anywhere)
    // false if not crit case    
    // 
    // Arguments
    // * content - a &Vec<string> containing the full contents of the .out file
    // 

    for line in content {
        if line.contains("kcode") {
            return true;
        };
    };
    return false;
}


pub fn get_rundate(content:&Vec<String>) -> (String, String) {
    // Returns date and time this mcnp case was run
    // as a tuple of 2 Strings
    // 
    // Arguments
    // * content - a &Vec<string> containing the full contents of the .out file
    // 

    let re = Regex::new(r"1mcnp.*version\s\d.*\d\d/\d\d/\d\d\s*(\d\d/\d\d/\d\d)\s(\d\d:\d\d:\d\d)").unwrap();
    for line in content {
        match re.captures(line) {
            Some(caps) => {
                let date:Vec<&str> = caps[1].split("/").collect();
                let time:String = caps[2].to_string();
                // format date from mm/dd/yy to yyyy/mm/dd                
                let d:&str = date[1];
                let m:&str = date[0];
                let y:String = format!("20{}",date[2]);
                let rundate = format!("{y}/{m}/{d}");
                return (rundate, time);
            },
            None => continue,
        }
    }
    return (String::from("Not Found"), String::from("Not Found"));
}


pub fn get_run_length(content:&Vec<String>) -> (String, String) {
    // Returns a tuple of 2 Strings, one of which will be "N/A",
    // the other will be the run length in seconds or particles.
    // 
    // Arguments
    // * content - a &Vec<string> containing the full contents of the .out file
    // 

    let re_nps = Regex::new(r"^\s{6,}\d+-\s{7}(nps|NPS)\s+(\d+.*)\s*").unwrap();
    let re_ctme = Regex::new(r"^\s{6,}\d+-\s{7}(ctme|CTME)\s+(\d+).*").unwrap();
    for line in content {
        // find ctme card
        match re_ctme.captures(line) {
            Some(caps) => {
                let ctme:String = caps[2].to_string();
                let nps:String = String::from("N/A");
                return (ctme, nps);
            },
            None => (),
        };
        // in absence of ctme, find nps card
        match re_nps.captures(line) {
            Some(caps) => {
                let nps:String = caps[2].to_string();
                let ctme:String = String::from("N/A");
                return (ctme, nps);
            },
            None => (),
        }
    };
    // If neither ctme or nps found, return "Not found".
    return ("Not found".to_string(), "Not found".to_string());
}


pub fn get_input(content:&Vec<String>) -> Vec<String> {
    // Returns the section of the mcnp output file that mirrors the MCNP input
    //
    // Arguments
    // * content - a &Vec<string> containing the full contents of the .out file
    // 

    let re = Regex::new(r"^\s{6,}\d+-\s{7}(.*)").unwrap();
    let mut mcnp_input: Vec<String> = Vec::new();
    for line in content {
        if re.is_match(line) {
            mcnp_input.push(line.to_string());
        }    
    }
    return mcnp_input;

}


pub fn get_parameters(input: &Vec<String>) -> HashMap<String, String> {
    // Returns any parameters from the mcnp input
    //
    // Arguments
    // * input - a &Vec<string> containing the mcnp input section taken from the .out file
    // 

    // Initialize variables needed for this function
    let mut parameters: HashMap<String,String> = HashMap::new();
    let mut parameters_present: bool = false;
    let mut start:usize = 0;

    // find start of parameter section by looking for string "USING THE FOLLOWING VARIABLES"
    // and call the index of this line 'start', set variables_present to true
    for (index, line) in input.iter().enumerate() {
        if line.contains("USING THE FOLLOWING VARIABLES") {
            start = index;
            parameters_present = true;
            break;
        }
    }

    // if parameters have bgeen found,
    // loop from 'start+1' to end of input, 
    // add captures from each line matching regex as k:v pairs to parameters HashMap
    // break out of loop on first line that doesn't match regex 
    if parameters_present {
        let re  = Regex::new(r"\s*\d+-\s+c\s*(\S*)\s*=\s*(\S*)\s*").unwrap();
        for i in start+1..input.len() {
            match re.captures(&input[i]) {
                Some(caps) => {
                    parameters.insert(caps[1].to_string(), caps[2].to_string());
                },
            None => break,
            }          
        };
    }

    // for (k, v) in &parameters {
        // println!("{k} : {v}");
        // }
    return parameters
}


pub fn check_lost_particles(content:&Vec<String>) -> bool {
    // Check whether the run was terminated because 10 or more particles got lost
    // Returns   bool: true if run terminated due to lost particles, otherwise false
    //
    // * content - a &Vec<string> containing the full contents of the .out file
    // 

    for line in content {
        if line.contains("run terminated because") && line.contains("particles got lost") {
            return true;
        }
    } 
    return false;
}

pub fn get_html(
                    filename: &String,
                    scaling_factor:&f64,
                    crit: &bool,
                    rundate: &String,
                    runtime: &String,
                    ctme: &String ,
                    nps: &String,
                    parameters: &HashMap<String, String>,
                    lost_particles: &bool,
                    mcnp_input: &Vec<String>,
                    particle_list: &Vec<&str>,
                    ) -> String {


    // get today's date and time
    let (date, time) = get_now();

    // get tera template
    let tera = get_tera();

    // Create new context
    let mut context = Context::new();


    context.insert("filename", &filename);
    context.insert("scaling_factor", &scaling_factor);
    context.insert("crit", &crit);
    context.insert("rundate", &rundate);
    context.insert("runtime", &runtime);
    context.insert("date", &date);
    context.insert("time", &time);
    context.insert("ctme", &ctme);
    context.insert("nps", &nps);
    context.insert("parameters", &parameters);
    context.insert("lost_particles", &lost_particles);
    context.insert("mcnp_input", &mcnp_input);
    context.insert("particle_list", &particle_list);

    // Render template using context
    let html = tera.render("mcnp_template.html", &context).expect("Failed to render template.");

    return html
}


pub fn write_html(html:String, location:String, filename:String) {
        // create html file
        let mut html_path = PathBuf::new();
        html_path.push(&location);
        html_path.push(&filename);
        html_path.set_extension("html");
        let mut html_file = File::create(&html_path).expect("Unable to create html file.");
        println!("\nCreated file {}", &html_path.display());
        // Write html file
        html_file.write_all(html.as_bytes()).expect("Unable to write html to file.");
        println!("Wrote HTML to file {}\n", &html_path.display());
}


pub fn get_now() -> (String,String) {
    // Gets the current date and time
    // returns a tuple of 2 strings
    //
    // Arguments - None
    //

    let now = chrono::Utc::now();
    let date = now.date().to_string();
    let time = now.time().to_string();
    return (date, time);
}


pub fn get_tera() -> Tera {
    // Tera templating
    // Returns new Tera instance

    let tera = match Tera::new("src/templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    return tera;
}


//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// TESTS /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_if_crit() {
        let test_vec_crit = vec![
            String::from(""),
            String::from(" run terminated when     200 kcode cycles were done."),
            String::from(""),
            String::from(" computer time =    0.39 minutes"),        
        ];
        let test_vec_not_crit = vec![
            String::from("          Code Name & Version = MCNP_6.20, 6.2.0 "),
            String::from("  "),
            String::from("     _/      _/        _/_/_/       _/      _/       _/_/_/         _/_/_/ "),
            String::from("    _/_/  _/_/      _/             _/_/    _/       _/    _/     _/        "),
        ];
        assert!(check_if_crit(&test_vec_crit));
        assert!(!check_if_crit(&test_vec_not_crit));
    }

    #[test]
    fn test_get_rundate() {
        let test_input: Vec<String> = vec![
            String::from("              "),
            String::from("1mcnp     version 6     ld=02/20/18                     10/06/22 16:01:31 "),
            String::from("*************************************************************************                 probid =  10/06/22 16:01:31 "),
            String::from("inp=Cs_example_simple.mcnp out=Cs_example_simple.out name=Cs_example_simple. TAS"),
        ];
        let (rundate, runtime) = get_rundate(&test_input);
        assert_eq!(rundate, String::from("2022/10/06"));
        assert_eq!(runtime, String::from("16:01:31"))
    }

    #[test]
    fn test_get_run_length() {
        // ctme test
        let input = vec![
            String::from("        84-       c"),
            String::from("        85-       c"),
            String::from("        86-       CTME 1"),
            String::from("        87-       c"),
            String::from("        88-       c"),
        ];

        // 5333-       c RUN MCNP $$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$
        // 5334-       nps 3e6
        // 5335-       prdmp 3e5 3e5 1 10 3e5

        let (ctme, nps) = get_run_length(&input);
        assert_eq!(ctme, "1");
        assert_eq!(nps, "N/A");
    }

    #[test]
    fn test_check_lost_particles() {
        let input = vec![
            String::from("        1problem summary                               "),
            String::from("        run terminated because  13 particles got lost."),
            String::from("  +                                                                                                    01/29/21 10:15:36 "),
            String::from("   =====>      49.80 M histories/hr    (based on wall-clock time in mcrun)"),
        ];

        assert!(check_lost_particles(&input));
    }

    //TODO: check crit
    //TODO: check parameters?
    //TODO: check get_input ?


}