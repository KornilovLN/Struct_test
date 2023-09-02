//use std::env;
//use std::{thread, time}; 
//extern crate ansi_term;
//use ansi_term::Colour;

use std::fs::File;
use std::io;
use std::io::prelude::*;

use std::error::Error;

mod md_person; 
mod md_about;


//=== Секция Config ===========================================================

pub struct Config {
	pub filename: String,
	pub worktype: String,
}

impl Config {
	pub fn new(args: &[String]) -> Result<Config, &'static str> {
	    let len = args.len();	    
	    if len < 2 {
	    	let _ = help(0,args);
		}

		let mut workt = String::new();
		let mut filen = String::new();

		match len {			
			3 => { //--- передано имя аргумента и значение				
				if args[1] == "-p" {			//--- ключ -p (print)
					if args[2] == "A" {			//--- выведет About в файл a.txt	
						workt.push_str("A");
						filen.push_str("a.txt");
					}else if args[2] == "P" {	//--- выведет Person в файл p.txt
						workt.push_str("P");
						filen.push_str("p.txt");
					}else{
						let _ = help(2,args);
					}
					
				}else{
					let _ = help(1,args);
				}
			},					
			_ => {	//--- остальные случаи - не рассматриваются			
				let _ = help(4,args);
			}, 
		}

		let filename = filen.clone();
		let worktype = workt.clone();

		Ok(Config { filename, worktype })
	}
}

impl Config {
	pub fn out(&self) {
		println!("\n\t-------------------------------------------------------------------------");
		println!("\t--- Будет создана запись Person и занесена в файл \t{}",self.filename);
		println!("\t-------------------------------------------------------------------------\n");
	}
}

pub fn help(n: u32, args: &[String]) ->  Result<(), Box<dyn Error>>  {
	print!("Config: App: {} -> ", args[0]);
	match n {
		0 => {			
	    	println!("Config: недостаточно аргументов args.len() == {}", args.len());
		},
		1 => {
			println!("Config: Неверный первый аргумент args[1] == {}", args[1]);
		},
		2 => {
			println!("Config: Неверный второй аргумент args[2] == {}", args[2]);
		},
		3 => {
			println!("Config: Ошибка в аргументе файл");
		},
		4 => {
			println!("Config: Неверное число аргументов args.len {}", args.len());
		},
		_ => {
			todo!();
		}
	}
	
	Ok(())
	
}

//=== Секция Run ==============================================================

fn write_out(f: String, st: &str) -> io::Result<()> {
	let mut out = File::create(f)?;
	write!(out,"{}",st)?;
	Ok(())
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let about = md_about::StAbout::new(
    	"Leonid", 
		"Nikolaevich", 
		"Kornilov",
		"Kornilov LN (Starmark)", 
		"https://github.com/KornilovLN/tut_prj_struct.git",
		"ln.KornilovStar@gmail.com",
		"30.08.2023 19:24:00",
    );	
	about.out();
	about.waiter(2);

    let prs = md_person::StPerson::new("Starmark", "Leon", "Nik", 1953, 7, 3);
    prs.out();	//println!("\n'Format debug printing' {:#?}", prs);	
	
	if	config.worktype == "A" || config.worktype == "P" {
	    config.out();
	    
	    if config.worktype == "A" {
			write_out(config.filename, &(about.tostring()));
	    }else if config.worktype == "P" {
			write_out(config.filename, &(prs.tostring()));
		}
	}  		

	Ok(())	
}

