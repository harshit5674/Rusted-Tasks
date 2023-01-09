use std::fs::File;
use std::io::Read;
use std::ptr::null;
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main(){
	let action=std::env::args().nth(1).expect("No argument given");
	if action=="default" {
		let mut data=String::new();
		let mut f=File::open("/Users/harshit/Programs/Rusted-Tasks/rusted_tasks/src/rusted_todone.json").expect("error while opening files");
		f.read_to_string(&mut data).expect("Unable to read string");
		data.as_str();
		let parsed = json::parse(&data).unwrap();
		let mut i=0;
		loop{
			if parsed[i]["task"].is_null() {
				break;
			}	
			println!("{}", parsed[i]["task"]);
			i=i+1;
		}
	}
}

