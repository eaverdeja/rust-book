// 1a.
// The API client is expected to provide a single WidgetImpl enum
// representing all possible widgets,
// and implement the Widget trait for WidgetImpl.

// The main difference here to option 2 is that we can only provide
// a single implementation of Widget for each enum we have defined.
// This limits the possibility of cross-components that option 2 has.
// For ex., a MyWidgetImpl::Row can't render a WidgetImpl::Text as
// because its impl block defines the return type of render as
// type Children = MyWidgetImpl

// This is still adequate for the requirements to 1a though,
// since it states that the API client will provide a *single*
// enum representing all the possible widgets.

// --- end 1a.

// 1b.
// The API client is expected to provide a unique struct for each
// possible widget, and implement the Widget trait for each struct.
// Each widget can return a vector containing widgets of any possible type.

// Option 3 would be very restrictive for 1b as each trait implementation
// would only have a single associated type. This means that a widget
// could only have a single type of child. Also, what should be the type of Children
// for leaf nodes? Since Children is bound by Widget, leaf nodes become a conundrum -
// they can return empty vectors, but what is their return type for render?

// So I say it is not appropriate for 1b.

// --- end 1b.

// 1c.
// Only types that implement the Widget trait should
// be returned from render.

// Option 3: children are an associated type
// 1a. ✅
// 1b. ❌
// 1c. ✅
pub trait Widget {
    type Children: Widget;
    fn render(&self) -> Vec<Self::Children>;
}

// 1a.
#[derive(Debug)]
enum WidgetImpl {
    Button {
        button_type: String,
        content: String,
    },
    Text(String),
}

// 1a.
impl Widget for WidgetImpl {
    type Children = WidgetImpl;
    fn render(&self) -> Vec<Self::Children> {
        match self {
            Self::Button {
                button_type,
                content,
            } => vec![Self::Text(content.clone())],
            Self::Text(_) => vec![],
        }
    }
}

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

// 1a.
#[derive(Debug)]
enum MyWidgetImpl {
    Row(Row),
    Column(Column),
    Grid(Grid),
}

// 1a.
impl Widget for MyWidgetImpl {
    type Children = MyWidgetImpl;
    fn render(&self) -> Vec<Self::Children> {
        match self {
            Self::Grid(grid) => grid
                .columns
                .iter()
                .map(|c| Self::Column(c.clone()))
                .collect(),
            Self::Column(column) => column.rows.iter().map(|r| Self::Row(r.clone())).collect(),
            Self::Row(_) => vec![],
        }
    }
}

// 1b.
impl Widget for Row {
    // What should be the type of Children for leaf nodes??
    type Children = String;
    //     error[E0277]: the trait bound `String: Widget` is not satisfied
    //    --> src/bin/widget3.rs:103:21
    //     |
    // 103 |     type Children = String; // What should be the type of Children for leaf nodes??
    //     |                     ^^^^^^ the trait `Widget` is not implemented for `String`
    //     |
    //     = help: the following other types implement trait `Widget`:
    //               MyWidgetImpl
    //               Row
    //               WidgetImpl
    // note: required by a bound in `Widget::Children`
    //    --> src/bin/widget3.rs:36:20
    //     |
    // 36  |     type Children: Widget;
    //     |                    ^^^^^^ required by this bound in `Widget::Children`
    fn render(&self) -> Vec<Self::Children> {
        vec!["foobar".to_string()]
    }
}

fn main() {
    let button = WidgetImpl::Button {
        button_type: "submit".to_string(),
        content: "Click me!".to_string(),
    };
    println!("{:?}", button.render());
    // [Text("Click me!")]

    let row1 = Row {
        content: "not the greatest API :p".to_string(),
    };
    let row2 = Row {
        content: "but we'll go with it!".to_string(),
    };
    let column = Column {
        rows: vec![row1, row2],
    };
    let grid = Grid {
        columns: vec![column],
    };
    let grid_widget = MyWidgetImpl::Grid(grid);

    println!("{:?}", grid_widget);
}
