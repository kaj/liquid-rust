use Renderable;
use context::Context;
use filters::size;

pub struct Template {
    pub elements: Vec<Box<Renderable>>
}

impl Renderable for Template {
    fn render (&self, context: &mut Context) -> Option<String>{
        context.filters.insert("size".to_string(), Box::new(size));

        Some(self.elements.iter().fold(String::new(), |fold, val| {
                                  match val.render(context)  {
                                      Some(x) => fold + &x,
                                      _ => fold
                                  }
                                 }))
    }
}

impl Template {
    pub fn new(elements: Vec<Box<Renderable>>) -> Template {
        Template{elements: elements}
    }
}

