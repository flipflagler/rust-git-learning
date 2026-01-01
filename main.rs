fn main() {
    let s = String::from("rust");
    takes_ownership(&s);
    println!("{}", s);
}

fn takes_ownership(x: &String) {
    println!("{}", x);
}
