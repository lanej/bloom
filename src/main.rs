use clap::Parser;
use std::io::BufRead;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = 16 * 1024)]
    bitmap_bytes: usize,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 16 * 1024)]
    count: usize,
}

fn main() {
    let args = Args::parse();
    let mut bloom: bloomfilter::Bloom<str> = bloomfilter::Bloom::new(args.bitmap_bytes, args.count);
    std::io::stdin().lock().lines().for_each(|l| {
        let line = l.unwrap();
        if !bloom.check_and_set(&line) {
            println!("{}", line)
        }
    });
}
