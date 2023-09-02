
//--- библиотека lib.rs содержит: Config, run(); Секция Tests
use tut_prj_struct::{Config};
//--- понадобится функция std::env::args из ст. библиотеки
use std::env;
//--- для выхода из программы с ненулевым сообщением системе
use std::process;


fn main() {

	//--- Получить и разместить в векторе аргументы строки запуска ПО
	let args: Vec<String> = env::args().collect();
	

	//--- создать структуру данных на основе аргументов строки
	let config = Config::new(&args).unwrap_or_else(|err| {
		eprintln!("Main: Проблема при разборе аргументов: {}", err);
		process::exit(1);
	});
	
	//--- Основная работа ПО
	if let Err(e) = tut_prj_struct::run(config) {
		eprintln!("Main: Ошибка в приложении: {}", e);		
		process::exit(1);
	}	

}


