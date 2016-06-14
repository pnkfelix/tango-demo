extern crate tango;
extern crate pandoc;
// extern crate simple_json;
// extern crate serde_json;
// extern crate json_io;
// extern crate json_flex;
extern crate rustc_serialize;

fn main() {
    tango::process_root().unwrap();
    run_pandoc();
}

fn run_pandoc() {
    let mut pandoc = pandoc::new();
    pandoc.add_input("src/slides.md");
    pandoc.set_output("slides.html");
    pandoc.add_option(pandoc::PandocOption::Css("stripes.css".to_string()));
    pandoc.set_output_format(pandoc::OutputFormat::Revealjs);
    pandoc.add_option(pandoc::PandocOption::Standalone);
    pandoc.add_option(pandoc::PandocOption::SlideLevel(2));
    pandoc.set_variable("theme", "mozilla-sandstone");
    pandoc.set_variable("center", "false");
    pandoc.add_filter(run_dot_filter);
    pandoc.execute().unwrap();
}
use rustc_serialize::json::{Json, Object, Array};

fn run_dot_filter(input: String) -> String {
    println!("Input:    `{}`", input);
    let mut input: Json = Json::from_str(&input).unwrap();
    replace_dot_blocks(&mut input).unwrap();
    // let ast: Ast = json::decode(&input).unwrap();
    println!("Output:    `{}`", input);
    format!("{}", input)
}

#[derive(Debug)]
enum Error {

}

// {"t":"CodeBlock","c":[["",["dot"],[]],"digraph demo_dot {\n    A -> B\n}"]}
fn replace_dot_blocks(json: &mut Json) -> Result<(), Error> {
    let new_content = match *json {
        Json::Array(ref mut jsons) => {
            for json in jsons {
                try!(replace_dot_blocks(json));
            }
            None
        }
        Json::Object(ref mut key_vals) => {
            if key_vals.get("t") == Some(&Json::String("CodeBlock".to_string()))
            {
                let mut block: &mut Array = key_vals
                    .get_mut("c").unwrap()
                    .as_array_mut().unwrap();
                let attr = {
                    let attr = block[0].as_array().unwrap();
                    if attr[1].as_array().unwrap().iter().all(|c| c.as_string() != Some("dot")) {
                        return Ok(());
                    }
                    attr.clone()
                };
                let content: &Json = &mut block[1];
                let content = content.as_string().unwrap();
                println!("saw attr: {:?} content: {}", attr, content);
                let new_text: String = {
                    use std::process::{Command, Stdio};
                    use std::io::{Read, Write};
                    let mut child = Command::new("dot")
                        .arg("-Tsvg")
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .spawn()
                        .unwrap_or_else(|e| panic!("spawn of dot failed: {:?}", e));
                    {
                        let mut stdin = child.stdin.as_mut()
                            .unwrap_or_else(|| panic!("unwrap of child stdin failed"));
                        write!(stdin, "{}", content)
                            .unwrap_or_else(|e| panic!("write of content to stdin failed: {:?}", e));
                    }
                    let _ecode = child.wait()
                        .unwrap_or_else(|e| panic!("wait for child failed: {:?}", e));
                    let mut output = String::new();
                    let mut stdout = child.stdout
                        .as_mut()
                        .unwrap_or_else(|| panic!("unwrap of child stdout failed"));
                    stdout.read_to_string(&mut output).unwrap();
                    let mut chars = output.chars();

                    // Skip the `<?xml ...>` directive.
                    skip_line(&mut chars);
                    // Skip the (two-line) <!DOCTYPE ...> dtd directive.
                    skip_line(&mut chars);
                    skip_line(&mut chars);

                    let svg_output: String = chars.collect();
                    println!("child svg_output: {}", svg_output);
                    svg_output
                };
                Some(new_text)
            } else {
                None
            }
        }
        _ => None,
    };
    if let Some(new_string) = new_content {
        let mut para: Object = Default::default();
        para.insert("t".to_string(), Json::String("Para".to_string()));
        para.insert("c".to_string(), {
            let mut raw: Object = Default::default();
            raw.insert("t".to_string(), Json::String("RawInline".to_string()));
            raw.insert("c".to_string(), Json::Array(vec![Json::String("html".to_string()), Json::String(new_string)]));
            Json::Array(vec![Json::Object(raw)])
        });
        *json = Json::Object(para);
    }
    
    Ok(())
}

fn skip_line<I>(chars: &mut I) where I: Iterator<Item=char> {
    chars.any(|c| c == '\n');
}

#[cfg(does_not_work_with_pandoc_encoding)]
mod json_ast {
    //! Transcribed ffom
    //! http://hackage.haskell.org/package/pandoc-types-1.16.1/docs/Text-Pandoc-Definition.html

    #![allow(non_snake_case)]

    use std::collections::HashMap;

    pub type Ast = (Meta, Vec<Block>,);
    #[derive(RustcEncodable, RustcDecodable, Default)]
    pub struct Meta {
        unMeta: UnMeta,
    }
    pub type UnMeta = HashMap<String, MetaValue>;
    #[derive(RustcEncodable, RustcDecodable)]
    pub enum MetaValue {
        MetaMap(HashMap<String, MetaValue>),
        MetaList(Vec<MetaValue>),
        MetaBool(bool),
        MetaString(String),
        MetaInlines(Vec<Inline>),
        MetaBlocks(Vec<Block>),
    }
    #[derive(RustcEncodable, RustcDecodable)]
    pub struct Block {
        pub t: BlockType,
        pub c: (),
    }
    #[derive(RustcEncodable, RustcDecodable)]
    pub enum BlockType {
        Plain(Vec<Inline>),
        Para(Vec<Inline>),
        CodeBlock(Attr, String),
        RawBlock(Format, String),
        BlockQuote(Vec<Block>),
        OrderedList(ListAttributes, Vec<Vec<Block>>),
        BulletList(Vec<Vec<Block>>),
        DefinitionList(Vec<(Vec<Inline>, Vec<Vec<Block>>)>),
        Headerr(Int, Attr, Vec<Inline>),
        HorizontalRule,
        Table(Vec<Inline>, Vec<Alignment>, Vec<Double>, Vec<TableCell>, Vec<Vec<TableCell>>),
        Div(Attr, Vec<Block>),
        Null,
    }
    #[derive(RustcEncodable, RustcDecodable)]
    pub enum Inline {
        
    }
    #[derive(RustcEncodable, RustcDecodable)]
    pub enum Alignment {

    }
    pub type ListAttributes = (Int, ListNumberStyle, ListNumberDelim);
    #[derive(RustcEncodable, RustcDecodable)]
    pub enum ListNumberStyle {

    }
    #[derive(RustcEncodable, RustcDecodable)]
    pub enum ListNumberDelim {

    }
    pub type Format = String;
    pub type Attr = (String, Vec<String>, Vec<(String, String)>);
    pub type TableCell = Vec<Block>;
    #[derive(RustcEncodable, RustcDecodable)]
    pub enum QuoteType {

    }
    pub type Target = (String, String);
    #[derive(RustcEncodable, RustcDecodable)]
    pub enum MathType {

    }
    #[derive(RustcEncodable, RustcDecodable)]
    pub struct Citation {

    }
    #[derive(RustcEncodable, RustcDecodable)]
    pub enum CitationMode {

    }
    pub type Double = f64;
    pub type Int = i32;
}
