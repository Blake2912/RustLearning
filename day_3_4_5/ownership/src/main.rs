fn main() {
    let reference = dangle();
    print!("{reference}");
}

fn dangle() -> String {
    let s = String::from("Hello");
    s
}
