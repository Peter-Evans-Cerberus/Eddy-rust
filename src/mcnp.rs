extern crate regex;
use regex::Regex;
use std::path::Path;


pub fn eddy_mcnp(filepath: &Path, content:&Vec<String>, scaling_factor:f64) {

    let filename = filepath.file_name().expect("Error finding MCNP output filename.");
    // check if crit case
    let crit = check_if_crit(content);

    //TODO: get rundate, runtime
    let (rundate, runtime) = get_rundate(content);
    println!("{rundate} {runtime}");
    //let (ctme, nps) = 


}


pub fn check_if_crit(file:&Vec<String>) -> bool {
    for line in file {
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

}