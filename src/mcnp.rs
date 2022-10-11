extern crate regex;
use regex::Regex;
use std::path::Path;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
mod style;


pub fn eddy_mcnp(filepath: &Path, content:&Vec<String>, scaling_factor:f64) {

    let filename = filepath.file_name().expect("Error finding MCNP output filename.");
    let location = filepath.parent().unwrap();
    // check if crit case
    let crit = check_if_crit(content);

    let (rundate, runtime) = get_rundate(content);
    println!("{rundate} {runtime}");

    let (ctme, nps) = get_run_length(content);

    // create html file
    let mut html_path = PathBuf::new();
    html_path.push(location.to_str().expect("Error determining path."));
    html_path.push(filename);
    html_path.set_extension("html");
    let mut html_file = File::create(html_path).expect("Unable to create html file.");

    // get css and write to html file
    let css = style::get_css();
    html_file.write_all(css.as_bytes()).expect("Unable to write css to file.");
}


pub fn check_if_crit(content:&Vec<String>) -> bool {
    for line in content {
        if line.contains("kcode") {
            return true;
        };
    };
    return false;
}


pub fn get_rundate(content:&Vec<String>) -> (String, String) {

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

}