fn main() {
    println!("Hello RustFi {:?}!", hello_rustfi());
}

fn say_hello(x: u32) {
    x.to_string()
}

fn hello_rustfi() -> String {
    let x = 2022u8;
    say_hello(x)
}