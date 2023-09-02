//--- модуль md_person ------------------------------------------------------------------------


#[derive(Debug)]
pub struct StPerson{
	n_main: String,
	n_first: String,
	n_padre: String,
	bd_y: i16,
	bd_m: i8,
	bd_d: i8,
}

impl StPerson {
	pub fn new(	main: &'static str, 
				first: &'static str,
				padre: &'static str, 
				bdy: i16,
				bdm: i8,
				bdd: i8,				
			  ) -> StPerson {
			  
			  Self{ n_main: main.to_string(), n_first: first.to_string(), 
				    n_padre: padre.to_string(), bd_y: bdy, bd_m: bdm, bd_d: bdd,	
			  }
	}
	
	pub fn out(&self) {
		println!("\t--- Person (test) -------------------------------------------------------");
		println!("\tFull name:");
		println!("\t    {}",self.n_main);
		println!("\t    {}",self.n_first);
		println!("\t    {}",self.n_padre);
		println!("\tBD: {}/{}/{}", self.bd_d, self.bd_m, self.bd_y);
		println!("\t-------------------------------------------------------------------------");
	}

	
	pub fn tostring(&self) -> String {

	    let mut s = String::with_capacity(256);
	    s.push_str("Structure StPerson");
	    
	    s.push_str("\nFull name:");
	    s.push_str("\n\t");
	    s.push_str(&self.n_main);
	    s.push_str("\n\t");
	    s.push_str(&self.n_first);
	    s.push_str("\n\t");
	    s.push_str(&self.n_padre);
	    s.push_str("\nBD: ");
	    s.push_str(&self.bd_d.to_string());
	    s.push('/');
	    s.push_str(&self.bd_m.to_string());
	    s.push('/');
	    s.push_str(&self.bd_y.to_string());

	    s
	}
										  
}


