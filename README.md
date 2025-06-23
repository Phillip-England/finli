# About
A cli used for sorting .pdf receipts and generating invoices

## Installation
```bash
cargo install finli
```

## Pdf Naming Conventions
`.pdf` files must be named as follows:
```bash
[DATE][VENDOR][COST][DESCRIPTION][CATEGORY][LOCATION]
```

Here is a valid name: `010125-target-10.95-pants-uniforms-southroads.pdf`

## Invoice Generation
Creates an invoice from a directory full of `.pdf` files:
```bash
finli generate ./some_dir "INVOICE TITLE"
```

## Splitting Receipts
Takes a directory full of invoices, scans for 'split' receipts, duplicates them over 'southroads' and 'utica', then sorts each receipt by location into subdirectories.
```bash
finli sort ./some_dir ./some_destination
```