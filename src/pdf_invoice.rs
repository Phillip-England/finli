use std::str::FromStr;

use crate::pdf_expense_category::PdfExpenseCategory;
use crate::pdf_line_item::PdfLineItem;
use rust_decimal::Decimal;
use genpdf::{elements, Alignment, Document, Element, style};



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

        // prepare the pdf
        let font_family = genpdf::fonts::from_files("./fonts", "LiberationSans", None);
        if font_family.is_err() {
            let err = font_family.err().unwrap();
            println!("{:?}", err); // third-party error
            return Some(format!("THIRD PARTY FAILURE: failed to load the font family for pdf generation"));
        }
        let font_family = font_family.unwrap();
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
                let item_title = format!("[{}] [{}] [{}]", item.date, item.description, item.cost);
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