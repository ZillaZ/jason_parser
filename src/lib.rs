use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct Document;

pub fn get_pairs(json_data: &str) -> Result<Pairs<Rule>, ()> {
    let pairs = Document::parse(Rule::document, json_data);
    if let Ok(value) = pairs {
        Ok(value)
    }else{
        Err(())
    }
}