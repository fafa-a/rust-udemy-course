pub fn hello() {
    let a = String::from("Hello");
    let clos = || {
        println!("{}", a);
    };
    clos();
    clos();
    clos();
    clos();
}
