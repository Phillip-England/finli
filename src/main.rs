use std::fs;

use clap::Parser;
use clap::Subcommand;

use crate::pdf_invoice::PdfInvoice;
use crate::pdf_sorted_dir::PdfSortedDir;

mod pdf_invoice;
mod pdf_sorted_dir;

#[derive(Parser, Debug)]
#[command(name = "", about = "", version = "1.0")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Generate { dir: String, invoice_name: String },
    Sort { dir: String, out: String },
}

fn run_generate(dir: String, invoice_name: String) -> Option<String> {
    let invoice = PdfInvoice::new_from_dir(&dir, &invoice_name);
    if invoice.is_err() {
        let err = invoice.err().unwrap();
        return Some(err);
    }
    let invoice = invoice.unwrap();
    let err = invoice.generate();
    if err.is_some() {
        let err = err.unwrap();
        return Some(err);
    }
    return None;
}

fn run_sort(dir: String, out: String) -> Result<(), String> {
    let sorted_dir = PdfSortedDir::new(&dir, &out)?;
    return Ok(());
}

fn main() {
    let args = Args::parse();
    match args.command {
        Command::Generate { dir, invoice_name } => {
            let err = run_generate(dir, invoice_name);
            if err.is_some() {
                panic!("{}", err.unwrap());
            }
        },
        Command::Sort { dir, out } => {
            let err = run_sort(dir, out);
            if err.is_err() {
                panic!("{:?}", err.unwrap());
            }
        }
    }
}
