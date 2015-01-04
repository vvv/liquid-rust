use Renderable;
use Value;
use Block;
use Context;
use LiquidOptions;
use tags::RawBlock;
use lexer::Token;
use lexer::Element;
use lexer::Element::{Expression, Tag, Raw};
use std::collections::HashMap;
use std::default::Default;

struct RawT{
    content : String
}

impl Renderable for RawT{
    fn render(&self, context: &Context) -> Option<String>{
        Some(self.content.to_string())
    }
}

impl Block for RawBlock{
    fn initialize(&self, tag_name: &str, arguments: &[Token], tokens: Vec<Element>, options : &LiquidOptions) -> Box<Renderable>{
        let content = tokens.iter().fold("".to_string(), |a, b|
                                         match b  {
                                            &Expression(_, ref text) => text,
                                            &Tag(_, ref text) => text,
                                            &Raw(ref text) => text
                                         }.to_string() + a.as_slice()
                                        );
        box RawT{content: content} as Box<Renderable>
    }
}

#[test]
fn test_raw() {
    let block = RawBlock;
    let options : LiquidOptions = Default::default();
    let raw = block.initialize("raw", vec![][0..], vec![Expression(vec![], "This is a test".to_string())], &options);
    assert_eq!(raw.render(&Default::default()).unwrap(), "This is a test".to_string());
}
