use std::str::FromStr;

use clap::Parser;
use clap::Subcommand;
use genpdf::Element;
use genpdf::{elements, Alignment, Document, style};

use crate::pdf_invoice::PdfInvoice;

mod pdf_expense_category;
mod pdf_line_item;
mod pdf_invoice;

#[derive(Parser, Debug)]
#[command(name = "", about = "", version = "1.0")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Generate { dir: String, invoice_name: String },
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

fn main() {
    let args = Args::parse();
    match args.command {
        Command::Generate { dir, invoice_name } => {
            let err = run_generate(dir, invoice_name);
            if err.is_some() {
                panic!("{}", err.unwrap());
            }
        }
    }
}
