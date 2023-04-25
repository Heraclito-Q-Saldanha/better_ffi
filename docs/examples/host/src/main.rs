use better_ffi::*;

#[safe_ffi]
struct Humam {
	name: FFIString,
	age: u8,
}

fn main() {
	let lib = unsafe { Library::load("../lib/target/debug/liblib.so") }.unwrap();
	let hello: Symbol<extern "C" fn(humam: Humam)> = unsafe { lib.get(b"hello\0") }.unwrap();
	hello(Humam {
		name: "anarcus".into(),
		age: 150,
	});
}
