
fn main() {
    let mut message = String::from("Name: Alfredo, Height: ");
    message.clear();
    let mut height = 190;
    println!("{}{}", message, height);
    height = 189;
    println!("{}{}", message, height);

}
