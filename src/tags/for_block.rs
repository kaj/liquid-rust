use Renderable;
use Block;
use context::Context;
use LiquidOptions;
use lexer::Token;
use lexer::Element;
use tags::ForBlock;
use parser::parse;
use template::Template;
use lexer::Token::Identifier;
use value::Value;

#[cfg(test)]
use std::default::Default;
#[cfg(test)]
use lexer::tokenize;

struct For {
    var_name: String,
    array_id: String,
    inner: Template
}

fn get_array(context: &mut Context, array_id: &str) -> Vec<Value> {
    match context.get_val(array_id) {
        Some(&Value::Array(ref x)) => x.clone(),
        _ => panic!("TODO")
    }
}

impl Renderable for For {
    fn render(&self, context: &mut Context) -> Option<String>{
        let arr = get_array(context, &self.array_id);
        let mut ret = String::new();
        for i in arr {
            context.set_val(&self.var_name, i);
            ret = ret + &self.inner.render(context).unwrap();
        }
        Some(ret)
    }
}


impl Block for ForBlock{
    fn initialize(&self, _tag_name: &str, arguments: &[Token], tokens: Vec<Element>, options : &LiquidOptions) -> Result<Box<Renderable>, String>{ let mut args = arguments.iter();

        let inner = try!(parse(&tokens, options));

        let var_name = match args.next() {
            Some(&Identifier(ref x)) => x.clone(),
            x => return Err(format!("Expected an identifier, found {:?}", x))
        };

        match args.next() {
            Some(&Identifier(ref x)) if x == "in" => (),
            x => return Err(format!("Expected 'in', found {:?}", x))
        };

        // TODO implement ranges
        let array_id = match args.next() {
            Some(&Identifier(ref x)) => x.clone(),
            x => return Err(format!("Expected an identifier, found {:?}", x))
        };

        Ok(Box::new(For{
            var_name: var_name,
            array_id: array_id,
            inner: Template::new(inner)
        }) as Box<Renderable>)
    }
}

#[test]
fn test_for() {
    let block = ForBlock;
    let options : LiquidOptions = Default::default();
    let for_tag = block.initialize("for", &vec![Identifier("name".to_string()), Identifier("in".to_string()), Identifier("array".to_string())], tokenize("test {{name}} "), &options);

    let mut data : Context = Default::default();
    data.set_val("array", Value::Array(vec![Value::Num(22f32), Value::Num(23f32), Value::Num(24f32), Value::Str("wat".to_string())]));
    assert_eq!(for_tag.unwrap().render(&mut data).unwrap(), "test 22 test 23 test 24 test wat ".to_string());
}

