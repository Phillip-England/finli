
use std::path::Path;
use std::fs;

use rust_decimal::Decimal;

use crate::pdf_invoice::PdfLineItem;

#[derive(Debug)]
pub struct PdfSortedDir {
    dir_root: String,
    dir_southroads: String,
    dir_utica: String,
}

impl PdfSortedDir {

    pub fn new(dir: &str, out: &str) -> Result<PdfSortedDir, String> {

        // getting the line items
        let line_items = PdfLineItem::new_from_dir(&dir)?;

        // generating out paths to create
        let out_path = Path::new(out);
        let southroads_out = out_path.join("southroads");
        let southroads_out = &southroads_out.as_path();
        let utica_out = out_path.join("utica");
        let utica_out = &utica_out.as_path();
        let out_dirs: Vec<&Path> = vec![out_path, southroads_out, utica_out];

        // creating all the out dirs
        for inner_dir in out_dirs {
            let result = fs::create_dir_all(inner_dir);
            if result.is_err() {
                println!("{:?}", result.err().unwrap());
                return Err(format!("FAILED TO CREATE DIR: the following dir was not created {}", out));
            }
        }

        // getting all of the split line items out
        let mut split_line_items: Vec<PdfLineItem> = vec![];
        for item in &line_items {
            if item.location.contains("split") {
                split_line_items.push(item.clone());
            }
        }

        // duplicating our split pdfs
        for item in split_line_items {

            // getting the cost in cents and determining if we are even
            let cost_in_cents = (item.cost * Decimal::from(100)).round();
            let item_is_even = cost_in_cents % Decimal::from(2) == Decimal::from(0);

            // setting up the cost
            let mut utica_cost: Decimal;
            let mut southroads_cost: Decimal;
            let mut half_cost_in_cents: Decimal;

            // manging even / odd costs
            if item_is_even {
                half_cost_in_cents = (cost_in_cents) / Decimal::from(2);
                southroads_cost = half_cost_in_cents / Decimal::from(100);
                utica_cost  = half_cost_in_cents / Decimal::from(100);
            } else {
                half_cost_in_cents = (cost_in_cents - Decimal::from(1)) / Decimal::from(2);
                southroads_cost = half_cost_in_cents / Decimal::from(100);
                utica_cost  = (half_cost_in_cents + Decimal::from(1)) / Decimal::from(100);
            }

            // sanity check
            if (utica_cost + southroads_cost) != item.cost {
                return Err(format!("PDF SPLIT ERROR: when splitting a PdfLineItem, the cost of the two pdfs does not equal the total cost of the original\n{:?}", item));
            }

            // cloning line item into two and updating costs
            let mut utica_line_item = item.clone();
            utica_line_item.set_cost(utica_cost);
            let err = utica_line_item.set_location("utica.pdf".to_owned());
            if err.is_some() {
                return Err(err.unwrap());
            }
            let mut southroads_line_item = item.clone().to_owned();
            southroads_line_item.set_cost(southroads_cost);
            let err = southroads_line_item.set_location("southroads.pdf".to_owned());
            if err.is_some() {
                return Err(err.unwrap())
            }

            // setting up new paths for output files
            utica_line_item.set_source_dir(&(out.to_owned() + "/utica"));
            southroads_line_item.set_source_dir(&(out.to_owned() + "/southroads"));

            // getting the details for copying/pasting
            let source_dest = item.path;
            let utica_out = utica_line_item.path;
            let southroads_out = southroads_line_item.path;

            // copying the files
            let utica_file = fs::copy(source_dest.clone(), utica_out.clone());
            if utica_file.is_err() {
                let err = utica_file.err().unwrap();
                println!("{}", err);
                return Err(format!("FILE COPY FAILURE: failed to copy {} to {}", source_dest, utica_out))
            }
            let southroads_file = fs::copy(source_dest.clone(), southroads_out.clone());
            if southroads_file.is_err() {
                let err = southroads_file.err().unwrap();
                println!("{}", err);
                return Err(format!("FILE COPY FAILURE: failed to copy {} to {}", source_dest, southroads_out))
            }




        };

        // sorting our non-split pdfs
        for mut item in line_items {
            
            // skipping split locatons
            let location = item.location.clone();
            if location.contains("split") {
                continue;
            }
            
            // getting copy/paste destinations
            let source_dest = item.path.clone();
            
            // copy/pasting for southroads
            if location.contains("southroads") {
                item.set_source_dir(&(out.to_owned() + "/southroads"));
                let file = fs::copy(source_dest.clone(), item.path.clone());
                if file.is_err() {
                    let err = file.err().unwrap();
                    println!("{}", err);
                    return Err(format!("FILE COPY FAILURE: failed to copy {} to {}", source_dest, item.path))
                }
                continue;
            }

            // copy/pasting for utica
            if location.contains("utica") {
                item.set_source_dir(&(out.to_owned() + "/utica"));
                let file = fs::copy(source_dest.clone(), item.path.clone());
                if file.is_err() {
                    let err = file.err().unwrap();
                    println!("{}", err);
                    return Err(format!("FILE COPY FAILURE: failed to copy {} to {}", source_dest, item.path))
                }
            }

        }



        let sorted_dir = PdfSortedDir {
            dir_root: out_path.to_str().unwrap().to_owned(), // cannot fail
            dir_southroads: southroads_out.to_str().unwrap().to_owned(), // cannot fail
            dir_utica: utica_out.to_str().unwrap().to_owned(), // cannot fail
        };
        return Ok(sorted_dir);
    }

}
