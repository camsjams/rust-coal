use std::io::{Error};
use pest::Parser;
use std::fs;
use pest::iterators::{Pairs};

#[derive(Parser)]
#[grammar = "html.pest"]
struct Grammar;

pub fn find_page(source: &str, page: &str) -> Result<String, Error> {
    let filename = format!("{}pages/{}.html", source, page);
    let contents = fs::read_to_string(filename).expect("Failed to open content file");
    
    println!("With text:\n{}", contents);
    let mut pairs:Pairs<Rule> = Grammar::parse(Rule::view, &contents).unwrap();
    let mut outer = pairs.next().unwrap().into_inner();
    let layout = outer.next().expect("name").as_str();

    let children = "children here";
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("token:    {:?}", pair);
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());
        println!("Text:    {}", pair.clone().into_inner());

        // // A pair can be converted to an iterator of the tokens which make it up:
        // for inner_pair in pair.into_inner() {
        //     match inner_pair.as_rule() {
        //         Rule::alpha => println!("Letter:  {}", inner_pair.as_str()),
        //         Rule::digit => println!("Digit:   {}", inner_pair.as_str()),
        //         _ => unreachable!()
        //     };
        // }
    }

    let layout_file = format!("{}{}.html", source, layout);
    let layout_contents = fs::read_to_string(layout_file).expect("Failed to open layout file");

    Ok(layout_contents
        .replace("{{meta}}", "")
        .replace("{{version}}", env!("CARGO_PKG_VERSION"))
        .replace("{{title}}", page)
        .replace("{{style}}", "")
        .replace("{{content}}", children)
        .replace("{{script}}", "")
    )
}