extern crate regex;
use regex::Regex;
use std::path::Path;


pub fn eddy_mcnp(filepath: &Path, content:&Vec<String>, scaling_factor:f64) {

    let filename = filepath.file_name().expect("Error finding MCNP output filename.");
    // check if crit case
    let crit = check_if_crit(content);

    //TODO: get rundate, runtime
    let (ctme, nps) = get_runtime(content);


}


pub fn check_if_crit(file:&Vec<String>) -> bool {
    for line in file {
        if line.contains("kcode") {
            return true;
        };
    };
    return false;
}

pub fn get_runtime(content:&Vec<String>) -> (String, String) {
    let mut rundate:String;
    let mut runtime:String;

    //         # date_time = {'date': f"{y}/{m}/{d}", 'time': time}
    //         rundate = f"{y}/{m}/{d}"
    //         runtime = time
    //         return rundate, runtime
    for line in content {
        if line.starts_with("1mcnp") {  // better regex for this line is "1mcnp.*version.*\d\d/\d\d/\d\d.*"
            let split_line:Vec<&str> = line.split_whitespace().collect();
            // split line on spaces
            runtime = String::from(split_line[5]);
            // get rundate
            let date:Vec<&str> = split_line[4].split("/").collect();
            let d:&str = date[1];
            let m:&str = date[0];
            let y:String = format!("{}{}", "20", &date[2]);
            let rundate= format!("{y}/{m}/{d}");
            return (rundate, runtime);
        } 
    }
    return (String::from("Not Found"), String::from("Not Found"));

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
            String::from("         2 warning messages so far."),            
            String::from(""),
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
}