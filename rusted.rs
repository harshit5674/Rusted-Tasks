use std::fs::File;
use std::io::Read;
use crate::json;

fn main(){
	let mut data=String::new();
	let mut f=File::open("/rusted_todone.json").expect("Error while opening files");
	f.read_to_string(&mut data).expect("Unable to read string");
	let parsed = json::parse(data);
	println!("{}", data);
}
