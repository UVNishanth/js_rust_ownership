fn main(){
    let x = String::from("hello");
    let y = &x;
    y = 4;
    println!("{}", y);
}