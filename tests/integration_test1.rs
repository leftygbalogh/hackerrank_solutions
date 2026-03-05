use untitled::proj_test::printer;
//cargo test
#[test]
fn oiyu(){
	println!("oiyu");
	assert_eq!(printer(42), 42);
}