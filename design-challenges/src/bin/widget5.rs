// 1a.
// The API client is expected to provide a single WidgetImpl enum
// representing all possible widgets,
// and implement the Widget trait for WidgetImpl.

// I'm not too sure about this one. I managed to implement WidgetImpl,
// but I can't print out the leaf nodes.

// For now I say this is not an appropriate solution

// --- end 1a.

// 1b.
// The API client is expected to provide a unique struct for each
// possible widget, and implement the Widget trait for each struct.
// Each widget can return a vector containing widgets of any possible type.

// Same thing as before. I managed to implement the Widget trait
// for each struct, but I can't print out the content for leaf nodes

// For now I say this is not an appropriate solution

// --- end 1b.

// 1c.
// Only types that implement the Widget trait should
// be returned from render.

// Option 5: children are a boxed trait object
// 1a. ✅
// 1b. ✅
// 1c. ✅
pub trait Widget {
    fn render(&self) -> Vec<Box<dyn Widget>>;
}

// 1b. ---
#[derive(Debug, Clone)]
struct Row {
    content: String,
}
#[derive(Debug, Clone)]
struct Column {
    rows: Vec<Row>,
}
#[derive(Debug)]
struct Grid {
    columns: Vec<Column>,
}

impl Widget for String {
    fn render(&self) -> Vec<Box<dyn Widget>> {
        vec![]
    }
}

impl Widget for Row {
    fn render(&self) -> Vec<Box<dyn Widget>> {
        vec![Box::new(self.content.clone())]
    }
}

impl Widget for Column {
    fn render(&self) -> Vec<Box<dyn Widget>> {
        self.rows
            .iter()
            .map(|r| Box::new(r.clone()) as Box<dyn Widget>)
            .collect()
    }
}

impl Widget for Grid {
    fn render(&self) -> Vec<Box<dyn Widget>> {
        self.columns
            .iter()
            .map(|c| Box::new(c.clone()) as Box<dyn Widget>)
            .collect()
    }
}
// --- 1b.

// 1a. ---
#[derive(Debug)]
enum WidgetImpl {
    Button {
        button_type: String,
        content: String,
    },
    Text {
        content: String,
    },
}

impl Widget for WidgetImpl {
    fn render(&self) -> Vec<Box<dyn Widget>> {
        match self {
            Self::Button {
                button_type,
                content,
            } => vec![Box::new(WidgetImpl::Text {
                content: content.clone(),
            })],
            Self::Text { content } => vec![Box::new(content.clone())],
        }
    }
}
// --- 1a.

fn main() {
    let button = WidgetImpl::Button {
        button_type: "submit".to_string(),
        content: "Click me!".to_string(),
    };
    println!("{:?}", button.render());
    // [[[]]]

    let grid = Grid {
        columns: vec![Column {
            rows: vec![
                Row {
                    content: "foo".to_string(),
                },
                Row {
                    content: "bar".to_string(),
                },
            ],
        }],
    };
    println!("{:?}", grid.render())
    // [[[[]], [[]]]]
}
