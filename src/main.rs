use std::env::args;

fn main() {
    let mut args = args();
    _ = args.next();
    let file_path = args
        .next()
        .unwrap_or("./test_asm/risc_test.hex".to_string());

    let file = std::fs::read_to_string(file_path);

    println!("{:?}", file);
}
