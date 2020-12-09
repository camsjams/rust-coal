use std::io::{Error, ErrorKind};
use pest::Parser;
use std::fs;
use pest::iterators::{Pairs};
use html_minifier::{HTMLMinifier, js::minify};

#[derive(Parser)]
#[grammar = "html.pest"]
struct Grammar;

fn get_tag(mut tag_pairs: Pairs<Rule>) -> [std::vec::Vec<&str>; 5] {
    let mut meta = vec![];
    let mut style = vec![];
    let mut children = vec![];
    let mut script = vec![];
    let mut title = vec![];

    if let Some(children_pair) = tag_pairs.next() {
        for p in children_pair.into_inner() {
            let token = match p.as_rule() {
                Rule::tag => {
                    let mut tag = p.clone().into_inner();
                    let tag_name = tag.next().expect("name").as_str();
                    tag.next()
                        .expect("attributes")
                        .into_inner();
                    match tag_name {
                        "style" => {
                            style.push(tag.as_str());
                            ""
                        }
                        "meta" => {
                            meta.push(p.as_str());
                            ""
                        }
                        "script" => {
                            script.push(p.as_str());
                            ""
                        }
                        "title" => {
                            title.push(tag.as_str());
                            ""
                        }
                        _ =>  p.as_str()
                    }
                }
                Rule::text => {
                    p.as_str()
                }
                _ => unreachable!(),
            };
            children.push(token);
        }
    }

    [
        meta,
        style,
        script,
        children,
        title
    ]
}

fn get_layout(layout_name: &str, source: &str) -> String {
    let layout_file = format!("{}{}.html", source, layout_name);
    fs::read_to_string(layout_file).expect("Coal Error: Failed to open layout file")
}

pub fn find_page(source: &str, page: &str, version: &str) -> Result<String, Error> {
    let filename = format!("{}pages/{}.html", source, page);
    let result = fs::read_to_string(filename);

    match result {
        Ok(contents) => {
            let mut pairs:Pairs<Rule> = Grammar::parse(Rule::view, &contents).unwrap();
            let mut tag_pairs = pairs.next().unwrap().into_inner();
            let layout_contents = get_layout(
                tag_pairs.next().expect("name").as_str(),
                source
            );

            tag_pairs.next()
                .expect("attributes")
                .into_inner();

            let [
                meta,
                style,
                script,
                children,
                title
            ] = get_tag(tag_pairs);

            let resolved_title = match title.last() {
                Some(child_title) => child_title.replace(">", ""),
                _ => page.to_string().to_uppercase()
            };

            let mut html_minifier = HTMLMinifier::new();
            html_minifier.digest(
                layout_contents
                .replace("{{meta}}",  &meta.join("\n").to_string())
                .replace("{{version}}", version)
                .replace("{{page}}", page)
                .replace("{{title}}", &resolved_title)
                .replace("{{style}}", &style
                    .into_iter()    
                    .map(|s| {
                        let css_options = &grass::Options::default().style(grass::OutputStyle::Compressed);
                        let compiled = grass::from_string(s.to_string(), css_options).unwrap();
                        format!("<style>{}</style>", compiled)
                    })
                    .collect::<Vec<String>>()
                    .join("\n")
                    .to_string()
                )
                .replace("{{content}}", &children.join("\n").to_string())
                .replace("{{script}}", &minify(&script.join("\n").to_string()))
            )
            .unwrap();
            Ok(std::str::from_utf8(html_minifier.get_html()).unwrap().to_string())
        },
        Err(_) => Err(Error::new(ErrorKind::Other, "Invalid Route"))
    }
}