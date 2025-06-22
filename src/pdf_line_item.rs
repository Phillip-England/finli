use std::path::Path;
use std::str::FromStr;

use rust_decimal::Decimal;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct PdfLineItem {
    pub source_dir: String,
    pub path: String,
    pub trimmed_path: String,
    pub parts: Vec<String>,
    pub date: String,
    pub vendor: String,
    pub cost: Decimal,
    pub description: String,
    pub category: String,
    pub location: String,
}

impl Clone for PdfLineItem {
    fn clone(&self) -> PdfLineItem {
        PdfLineItem {
            source_dir: self.source_dir.clone(),
            path: self.path.clone(),
            trimmed_path: self.trimmed_path.clone(),
            parts: self.parts.clone(),
            date: self.date.clone(),
            vendor: self.vendor.clone(),
            cost: self.cost, // Decimal implements Copy, so you can copy it directly
            description: self.description.clone(),
            category: self.category.clone(),
            location: self.location.clone(),
        }
    }
}

impl PdfLineItem {
    pub fn new(source_dir: &str, path: &str) -> Result<PdfLineItem, String> {
        // ensuring our pdf file has 6 lines
        let trimmed_path = &path[(source_dir.len() + 1)..path.len()];
        let parts: Vec<String> = trimmed_path.split("-").map(|s| s.to_string()).collect();
        if parts.len() != 6 {
            return Err(format!("INVALID FILE NAME: PdfLineItem must consist of 6 distinct parts but you provided {}\n{}", parts.len(), trimmed_path).to_owned());
        }

        // ensuring we have a valid date
        let date_str = parts[0].to_owned();
        let date_num = date_str.parse::<i32>();
        if !date_num.is_ok() {
            return Err(format!(
                "INVALID DATE: PdfLineItem 'date' field should be a valid number\n{}",
                trimmed_path
            ));
        }
        if date_str.len() != 6 {
            return Err(format!(
				"INVALID DATE: PdfLineItem 'date' field should only consist of 6 digits like '010125'\n{}",
				trimmed_path,
			));
        }

        // extracting vendor
        let vendor = parts[1].to_owned();

        // ensuring we have a cost that is a valid floating-point number
        let cost = parts[2].to_owned();
        let cost_as_float = cost.parse::<f64>();
        if cost_as_float.is_err() {
            return Err(format!(
                "INVALID COST: PdfLineItem 'cost' failed to convert to a floating point number\n{}",
                trimmed_path
            )
            .to_owned());
        }

        // converting the cost (as a String) into a Decimal
        let cost_as_decimal = Decimal::from_str(&cost);
        if cost_as_decimal.is_err() {
            let err = cost_as_decimal.unwrap();
            println!("{:?}", err); // third party library error
            return Err(format!("CONVERSION FAILURE: failed to convert the 'cost' field into a Decimal fit for accurate financial math\n{}", trimmed_path));
        }
        let cost_as_decimal = cost_as_decimal.unwrap();

        // extracting the description and category
        let description = parts[3].to_owned();
        let category = parts[4].to_owned();

        // extracting the location and ensuring we have a valid name
        let valid_locations = vec!["southroads.pdf", "utica.pdf", "split.pdf"];
        let location = parts[5].to_owned();
        let location = location.to_lowercase();
        if !valid_locations.contains(&location.as_str()) {
            return Err(format!("INVALID LOCATION: the 'location' field must be 'southroads', 'utica', or 'split'\n{}", trimmed_path));
        }

        let line_item = PdfLineItem {
            source_dir: source_dir.to_owned(),
            path: path.to_owned(),
            trimmed_path: trimmed_path.to_owned(),
            parts: parts,
            date: date_str.to_owned(),
            vendor: vendor,
            cost: cost_as_decimal,
            description: description,
            category: category,
            location: location,
        };
        return Ok(line_item);
    }

    pub fn new_from_dir(source_dir: &str) -> Result<Vec<PdfLineItem>, String> {
        // ensure we have a valid source dir
        let dir_path = Path::new(&source_dir);
        if !dir_path.exists() {
            return Err(format!(
                "MISSING DIR: this dir does not exist: {:?}",
                dir_path
            ));
        }
        if !dir_path.is_dir() {
            return Err(format!(
                "INVALID DIR: provided a file, not a dir: {:?}",
                dir_path
            ));
        }

        // extract all the file paths within
        let mut file_paths: Vec<String> = vec![];
        for entry in WalkDir::new(dir_path) {
            if entry.is_err() {
                println!("{:?}", entry.err()); // print third party error
                return Err(
                    "WALKDIR FAILURE: an error was encountered when walking the provided dir path"
                        .to_owned(),
                );
            }
            let entry = entry.unwrap();
            let path = entry.path();
            let path_str = path.to_str();
            if path_str.is_none() {
                return Err("CONVERSION ERROR: failed to convert the filepath to a &str".to_owned());
            }
            let path_str = path_str.unwrap().to_owned();
            if path_str == source_dir {
                // skip the provided dir
                continue;
            }
            if path.is_dir() {
                return Err("INVALID DIR CONTENTS: the provided file path must not contain any subdirectories".to_owned());
            }
            let ext = path.extension();
            if ext.is_none() {
                return Err(
                    "INVALID DIR CONTENT: the dir must contain files with valid extensions"
                        .to_owned(),
                );
            }
            let ext = ext.unwrap();
            if ext != "pdf" {
                return Err(
                    "INVALID FILE EXTENSION: the dir must contain only .pdf files".to_owned(),
                );
            }
            file_paths.push(path_str);
        }

        // take each file path and create a PdfLineItem for each
        let mut line_items: Vec<PdfLineItem> = vec![];
        for path in file_paths {
            let line_item = PdfLineItem::new(&source_dir, &path);
            if line_item.is_err() {
                return Err(line_item.err().unwrap());
            }
            let line_item = line_item.unwrap();
            line_items.push(line_item);
        }

        return Ok(line_items);
    }

}
