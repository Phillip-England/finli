
use std::path::Path;
use std::fs;

use crate::pdf_invoice::PdfLineItem;

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

        println!("{:?}", split_line_items);

        let sorted_dir = PdfSortedDir {
            dir_root: out_path.to_str().unwrap().to_owned(), // cannot fail
            dir_southroads: southroads_out.to_str().unwrap().to_owned(), // cannot fail
            dir_utica: utica_out.to_str().unwrap().to_owned(), // cannot fail
        };
        return Ok(sorted_dir);
    }

}


pub struct PdfSplitAllocator {

}

impl PdfSplitAllocator {
    
    pub fn new_from_line_item(line_item: PdfLineItem) -> Result<PdfSplitAllocator, String> {
        if !line_item.location.contains("split") {
            return Err(format!("INVALID LINE ITEM PROVDIED: only line items marked 'split' can be used to generate an allocator\n{:?}", line_item));
        }
        let allocator = PdfSplitAllocator {

        };
        return Ok(allocator);
    }


}