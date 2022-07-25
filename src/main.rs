extern crate regex;
use regex::Regex;

fn main() {
    //let re = Regex::new(r"(\w{5})").unwrap();
    let re = Regex::new(r"dcode").unwrap();
    let text = "dcodeA1";

    match re.captures(text) {
        //Some(caps) => println!("Found match: {}", caps.get(0).unwrap().as_str()),
        Some(caps) => println!("Found match: {}", &caps[0]),
        None => println!("Could not find match..."),
    }
}
