

fn main() {
    let thing = std::fs::read_dir(r"C:\Users\Iknit\Documents").unwrap();
    for path in thing {
        println!("{:?}", path.unwrap().path());
    }
}
