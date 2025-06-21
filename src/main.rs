use std::path::Path;

use clap::Parser;
use walkdir::WalkDir;

mod pdf_line_item;

use crate::pdf_line_item::PdfLineItem;

#[derive(Parser, Debug)]
#[command(name = "", about = "", version = "1.0")]
struct Args {
    #[arg(short, long)]
    run: String,
    dir: String,
}

fn run_sort(args: Args) -> Option<String> {
    let line_items = PdfLineItem::new_from_dir(&args.dir);
    if line_items.is_err() {
        let err = line_items.err().unwrap();
        return Some(err);
    }
    let line_items = line_items.unwrap();
    for item in line_items {
        println!("{:?}", item);
    }

    return None;
}

fn main() {
    let args = Args::parse();

    if args.run == "sort" {
        let err = run_sort(args);
        if err.is_some() {
            panic!("{}", err.unwrap());
        }
        return;
    }

}
