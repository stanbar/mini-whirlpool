use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input = if args.len() > 1 {
        args.get(1).unwrap().clone()
    } else {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        buffer
    };

    let hash = whirlpool::core::hash(input.as_bytes().into());
    print_result(hash);
    Ok(())
}

fn print_result(result: [u8; 16]) {
    result.iter().for_each(|x| print!("{:x}", x));
    println!();
}
