use clap::Parser;
use std::{borrow::Cow, io::BufRead};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 16 * 1024)]
    bitmap_bytes: usize,

    #[arg(short, long, default_value_t = 16 * 1024)]
    count: usize,

    #[arg(short, long)]
    delimiter: Option<String>,

    #[arg(short, long)]
    index: Option<usize>,
}

fn main() {
    let args = Args::parse();
    let mut bloom: bloomfilter::Bloom<str> = bloomfilter::Bloom::new(args.bitmap_bytes, args.count);

    std::io::stdin().lock().lines().for_each(|l| {
        let line = l.unwrap();
        let mut input = Cow::Borrowed(&line);
        if let (Some(d), Some(i)) = (&args.delimiter, &args.index) {
            input = Cow::Owned(
                line.split(d)
                    .nth(*i)
                    .unwrap_or_else(|| panic!("'{}': has no {} value separated by {}", line, i, d))
                    .to_owned(),
            );
        }
        if !bloom.check_and_set(&input) {
            println!("{}", line)
        }
    });
}
