use std::str::FromStr;
use std::path::Path;

use walkdir::WalkDir;
use rust_decimal::Decimal;
use genpdf::{elements, fonts::{self, FontData, FontFamily}, style, Alignment, Document, Element};



pub struct PdfInvoice {
    pub expense_categories: Vec<PdfExpenseCategory>,
    pub total_cost: Decimal,
    pub file_name: String,
    pub name: String,
}

impl PdfInvoice {

    pub fn new_from_dir(dir: &str, invoice_name: &str) -> Result<PdfInvoice, String> {

        // extract the line items
        let line_items = PdfLineItem::new_from_dir(&dir);
        if line_items.is_err() {
            let err = line_items.err().unwrap();
            return Err(err);
        }
        let line_items = line_items.unwrap();

        // sort into categories
        let expense_categories = PdfExpenseCategory::new_from_line_items(line_items);

        // geting the total
        let mut invoice_total = Decimal::from_str("0").unwrap(); // cannot fail
        for category in &expense_categories {
            invoice_total += category.total_cost;
        }

        // creating invoice
        let pdf_invoice = PdfInvoice {
            name: invoice_name.to_string(),
            expense_categories: expense_categories,
            total_cost: invoice_total,
            file_name:  format!("{}.pdf", invoice_name).to_lowercase().replace(" ", "_"),
        };

        return Ok(pdf_invoice);

    }

    pub fn generate(&self) -> Option<String> {

		// loading in our fonts
		let regular = include_bytes!("../fonts/LiberationSans-Regular.ttf") as &[u8];
		let bold = include_bytes!("../fonts/LiberationSans-Bold.ttf");
		let italic = include_bytes!("../fonts/LiberationSans-Italic.ttf");
		let bold_italic = include_bytes!("../fonts/LiberationSans-BoldItalic.ttf");

		let regular_data = genpdf::fonts::FontData::new(regular.to_vec(), None).expect("failed to load in the regular font");
		let bold_data = genpdf::fonts::FontData::new(regular.to_vec(), None).expect("failed to load in the bold font");
		let italic_data = genpdf::fonts::FontData::new(regular.to_vec(), None).expect("failed to load in the italic font");
		let bold_italic_data = genpdf::fonts::FontData::new(regular.to_vec(), None).expect("failed to load in the bold-italic font");


		let font_family = FontFamily {
			regular: regular_data,
			bold: bold_data,
			italic: italic_data,
			bold_italic: bold_italic_data,
		};

        // prepare the pdf
        // let font_family = genpdf::fonts::from_files("./fonts", "LiberationSans", None);
        // if font_family.is_err() {
        //     let err = font_family.err().unwrap();
        //     println!("{:?}", err); // third-party error
        //     return Some(format!("THIRD PARTY FAILURE: failed to load the font family for pdf generation"));
        // }
        // let font_family = font_family.unwrap();
        let mut doc = Document::new(font_family);
        doc.set_title(self.name.clone());
        let mut decorator = genpdf::SimplePageDecorator::new();
        decorator.set_margins(10);
        doc.set_page_decorator(decorator);
        doc.set_font_size(12);

        // write title header to invoice pdf
        let invoice_title = format!("{}: {}", self.name, self.total_cost);
        let header = elements::Paragraph::new(invoice_title).aligned(Alignment::Left);
        let header = header.clone().styled(style::Style::new().bold().with_font_size(20));
        doc.push(header);
        let empty_paragraph = elements::Paragraph::new("").aligned(Alignment::Left);
        doc.push(empty_paragraph);
        let empty_paragraph = elements::Paragraph::new("").aligned(Alignment::Left);
        doc.push(empty_paragraph);
        let empty_paragraph = elements::Paragraph::new("").aligned(Alignment::Left);
        doc.push(empty_paragraph);

        // writing the categories
        for category in &self.expense_categories {
            let category_title = format!("{} => {}", category.name, category.total_cost);
            let header = elements::Paragraph::new(category_title).aligned(Alignment::Left);
            let header = header.clone().styled(style::Style::new().bold().with_font_size(16));
            doc.push(header.clone());
            let empty_paragraph = elements::Paragraph::new("").aligned(Alignment::Left);
            let empty_paragraph = empty_paragraph.clone().styled(style::Style::new().with_font_size(4));
            doc.push(empty_paragraph);
            for item in &category.line_items {
                let item_title = format!("[{}] [{}] [{}] [{}]", item.date, item.description, item.vendor, item.cost);
                let item_paragraph = elements::Paragraph::new(item_title).aligned(Alignment::Left);
                doc.push(item_paragraph);
                let empty_paragraph = elements::Paragraph::new("").aligned(Alignment::Left);
                let empty_paragraph = empty_paragraph.clone().styled(style::Style::new().with_font_size(4));
                doc.push(empty_paragraph);
            }
            let empty_paragraph = elements::Paragraph::new("").aligned(Alignment::Left);
            doc.push(empty_paragraph);
        }

        // writing output
        let output_file = doc.render_to_file(self.file_name.clone());
        if output_file.is_err() {
            let err = output_file.err().unwrap();
            println!("{:?}", err); // third-party error
            return Some(format!("PDF RENDER FAILURE: failed to render output pdf file: {}", self.file_name));
        }

        return None;
    }

}

#[derive(Debug)]
pub struct PdfExpenseCategory {
    pub name: String,
    pub line_items: Vec<PdfLineItem>,
    pub total_cost: Decimal,
}

impl PdfExpenseCategory {
    pub fn new_from_line_items(line_items: Vec<PdfLineItem>) -> Vec<PdfExpenseCategory> {
        let mut expense_categories: Vec<PdfExpenseCategory> = vec![];

        // extract each type of category
        let mut all_categories: Vec<String> = vec![];
        for item in &line_items {
            if all_categories.contains(&item.category) {
                continue;
            }
            all_categories.push(item.category.clone());
        }

        for category in &all_categories {
            let mut total = Decimal::from_str("0").unwrap(); // no error possible
            let mut matching_line_items: Vec<PdfLineItem> = vec![];
            for item in &line_items {
                if *category != item.category {
                    continue;
                }
                total = total + item.cost;
                matching_line_items.push(item.clone());
            }
            let expense_category = PdfExpenseCategory {
                name: category.clone(),
                line_items: matching_line_items.clone(),
                total_cost: total,
            };
            expense_categories.push(expense_category);
        }

        return expense_categories;
    }

}

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

    pub fn set_cost(&mut self, new_cost: Decimal) {
        let old_cost = self.cost.to_string();
        let original_path = self.path.clone();
        self.cost = new_cost;
        self.path = original_path.replace(&format!("-{}-", old_cost), &format!("-{}-", new_cost));
        self.trimmed_path = original_path.replace(&format!("-{}-", old_cost), &format!("-{}-", &new_cost.to_string()));
    }


    pub fn set_location(&mut self, new_location: String) -> Option<String> {
        let new_location = new_location.to_lowercase();
        let valid_locations = vec!["southroads.pdf", "utica.pdf", "split.pdf"];
        if !valid_locations.contains(&new_location.as_str()) {
            return Some(format!(
                "INVALID LOCATION: the 'location' field must be 'southroads', 'utica', or 'split'\n{}",
                self.trimmed_path
            ));
        }
        let old_location = self.location.clone();
        let original_path = self.path.clone();
        self.location = new_location.clone();
        self.path = original_path.replace(&format!("-{}", old_location), &format!("-{}", new_location));
        self.trimmed_path = original_path.replace(&format!("-{}", old_location), &format!("-{}", new_location));
        return None
    }

    pub fn set_source_dir(&mut self, new_source: &str) {
        let old_source = self.source_dir.clone();
        let old_path = self.path.clone();
        self.source_dir = new_source.to_owned();
        self.path = self.source_dir.clone() + &old_path[old_source.len()..old_path.len()].to_owned();
    }


}
