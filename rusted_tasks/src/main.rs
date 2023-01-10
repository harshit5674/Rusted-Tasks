use std::fs::File;
use std::io::Read;
use std::ptr::null;
use colored::Colorize;
use std::time::SystemTime;
use chrono;
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
		let mut parsed = json::parse(&data).unwrap();
		let mut i=0;
		
//		sys_time=sys_time.to_string()
		let mut sys_time=chrono::offset::Local::now();
		let timestamp_str = sys_time.format("%Y-%m-%dT%H:%M:%S.%f").to_string();
		let mut day=String::new();
		let mut month=String::new();
		let mut year=String::new();
		let mut ok=false;
		let mut ok1=false;
		for c in timestamp_str.chars(){
			if c=='T' {
				break;
			}
			if c=='-' && ok==false{
				ok=true;
				continue;
			}
			if c=='-' && ok1==false{
				ok1=true;
				continue;
			}
			if ok==true{
				if ok1==true{
					day.push(c);
				}
				else{
					month.push(c);
				}
			}
			else{
				year.push(c);
			}
		}
		day.push('/');
		month.push('/');
		day.push_str(&month);
		day.push_str(&year);
		loop{
			if parsed[i]["task"].is_null() {
				break;
			}	
			if parsed[i]["date"]!=day.trim() {
				i=i+1;
				continue;
			}
			if parsed[i]["code_red"]=="yes" {
				println!("{}{}",format!("{}. ",i+1).red().bold(), format!("{}",parsed[i]["task"]).red().bold());
				i=i+1;
				continue;
			}
			println!("{}{}",format!("{}. ",i+1).white(), format!("{}",parsed[i]["task"]).white().bold());
			i=i+1;
		}
		println!("");
		println!("{}",format!("-----------------------Completed Tasks").bold());
		println!("");
		data=String::new();
		f=File::open("/Users/harshit/Programs/Rusted-Tasks/rusted_tasks/src/rusted_done.json").expect("error while opening files");
		f.read_to_string(&mut data).expect("Unable to read string");
		data.as_str();
		parsed = json::parse(&data).unwrap();
		i=0;
		loop{
			if parsed[i]["task"].is_null() {
				break;
			}	
			println!("{}{}",format!("{}. ",i+1).green(), format!("{}",parsed[i]["task"]).green().bold());
			i=i+1;
		}
	}
}

