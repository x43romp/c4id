mod c4;

fn main() {

    let args: Vec<String> = std::env::args().collect();
    if let Some(filename) = args.get(1) {
        let result = c4::c4_file::c4_file(filename);
        println!("{}", result);
    } else {
        eprintln!("Please provide a filename as the first argument.");
    }
}
