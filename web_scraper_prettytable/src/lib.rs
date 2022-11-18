use reqwest;
use select::document::Document;
use select::predicate::{Class, Name, Predicate, Text};
use prettytable::{Table, Row, Cell, Attr, color};
use std::fs::File;

const ROW: &str = "css-1cxc880";
const PERCENTAGE_CELL: &str = "css-1b7j986";
const PRICE_CELL: &str = "css-1vyy4qg";
const NAME_CELL: &str = "css-1sem0fc";

pub fn crypto_com_crawler(url: &str) {

    let resp = reqwest::blocking::get(url).unwrap(); 
    assert!(resp.status().is_success());
    
    let document = Document::from_read(resp).unwrap();
    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("#")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::BRIGHT_GREEN)),
        Cell::new("Name")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::BRIGHT_GREEN)),
        Cell::new("Price")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::BRIGHT_GREEN)),
        Cell::new("24H Change")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::BRIGHT_GREEN)),
    ]));

    let mut id = 1..51;

    for node in document.find(Class(ROW)) {
        let cell = node.find(Class(NAME_CELL)
            .descendant(Class("css-70qvj9")
            .descendant(Class("css-ttxvk0"))))
            .next()
            .unwrap();
        
        let name = cell.find(Name("a")).next().unwrap().text();
        let name_short = cell.find(Name("span")).next().unwrap().text();
        
        let price = node.find(Class(PRICE_CELL)
            .descendant(Class("css-b1ilzc")
            .child(Text)))
            .next()
            .unwrap()
            .text();

        let perc = node.find(Class(PERCENTAGE_CELL)
            .descendant(Name("p")))
            .next()
            .unwrap()
            .text();
            
        table.add_row(Row::new(vec![
            Cell::new(id.next().unwrap().to_string().as_str())
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::BRIGHT_WHITE)),
            Cell::new(format!("{} [{}]",name, name_short).as_str())
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::BRIGHT_CYAN)),
            Cell::new(price.as_str())
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::BRIGHT_BLUE)),
            Cell::new(perc.as_str())
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(if perc.contains("+") {
                        color::GREEN
                    }else {
                        color::RED
                    }))
        ]));
    }

    table.add_row(Row::new(vec![
        Cell::new("#")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::BRIGHT_YELLOW)),
        Cell::new("Crypto.com Exchange data")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::BRIGHT_YELLOW)),
        Cell::new("MUCH WOW")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::BRIGHT_YELLOW)),
        Cell::new("WOW")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::BRIGHT_YELLOW)),
    ]));
    
    //Export table as CSV
    let out = File::create("output_csv.txt").unwrap();
    table.to_csv(out).unwrap();

    table.printstd();
}

    
