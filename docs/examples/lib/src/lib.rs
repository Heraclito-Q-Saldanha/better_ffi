use better_ffi::*;

#[safe_ffi]
struct Humam {
	name: FFIString,
	age: u8,
}

#[safe_ffi]
fn hello(humam: Humam) {
	println!("hello {} {}", humam.name, humam.age);
}
