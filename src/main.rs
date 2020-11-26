mod screenshot;

fn main() {
    let s =  screenshot::get_screenshot(0).unwrap();
    
    println!("{}", s.height());
}
