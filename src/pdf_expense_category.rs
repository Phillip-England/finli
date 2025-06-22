use std::str::FromStr;

use rust_decimal::Decimal;

use crate::pdf_line_item::PdfLineItem;

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
