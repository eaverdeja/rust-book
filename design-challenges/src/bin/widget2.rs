// 1a.
// The API client is expected to provide a single WidgetImpl enum
// representing all possible widgets,
// and implement the Widget trait for WidgetImpl.

// Option 2 would work for 1a. as it is open for extension,
// in contrast to option 1. The API client can create MyWidgetImpl
// and have it implement the Widget trait to fulfill the requirements.

// This option also has some of the benefits of option 1 such as low overhead due
// to lack of heap allocations and no dynamic dispatch.

// The tradeoff is a not-so-nice API for defining the custom enum
// that uses generics quite heavily.
// --- end 1a.

// 1b.
// The API client is expected to provide a unique struct for each
// possible widget, and implement the Widget trait for each struct.
// Each widget can return a vector containing widgets of any possible type.

// Option 2 won't work for 1b. because when we implement the trait
// for the struct, we need to define the child type.
// So a widget wouldn't be able to return a vector containing widgets of any possible type.

// -- end 1b.

// 1c.
// Only types that implement the Widget trait should
// be returned from render.

// Option 2: children are a trait parameter
// 1a. ✅
// 1b. ❌
// 1c. ✅
pub trait Widget<Children> {
    fn render(&self) -> Vec<Children>;
}

// 1a.
#[derive(Debug)]
// Default WidgetImpl
enum WidgetImpl {
    Button {
        button_type: String,
        content: String,
    },
    Text(String),
}

// 1a.
impl Widget<WidgetImpl> for WidgetImpl {
    fn render(&self) -> Vec<WidgetImpl> {
        match self {
            Self::Button {
                button_type,
                content,
            } => {
                vec![Self::Text(content.to_owned())]
            }
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
// User-defined WidgetImpl
enum MyWidgetImpl {
    Row(Row),
    Column(Column),
    Grid(Grid),
}

// 1a.
impl Widget<MyWidgetImpl> for MyWidgetImpl {
    fn render(&self) -> Vec<MyWidgetImpl> {
        match self {
            MyWidgetImpl::Grid(grid) => grid
                .columns
                .iter()
                .map(|c| MyWidgetImpl::Column(c.clone()))
                .collect(),
            MyWidgetImpl::Column(column) => column
                .rows
                .iter()
                .map(|r| MyWidgetImpl::Row(r.clone()))
                .collect(),
            MyWidgetImpl::Row(_) => vec![],
        }
    }
}

// 1a.
impl Widget<WidgetImpl> for Row {
    fn render(&self) -> Vec<WidgetImpl> {
        vec![WidgetImpl::Text(self.content.clone())]
    }
}

// 1b.
impl Widget<Column> for Grid {
    fn render(&self) -> Vec<Column> {
        self.columns.iter().map(|c| c.clone()).collect()
    }
}

// 1b.
impl Widget<Row> for Column {
    fn render(&self) -> Vec<Row> {
        self.rows.iter().map(|r| r.clone()).collect()
    }
}

fn one_a() {
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
    // Grid(Grid { columns: [Column { rows: [Row { content: "not the greatest API :p" }, Row { content: "but we'll go with it!" }] }] })

    println!("\nTraversing the grid:");
    let columns = grid_widget.render();
    println!("Grid renders {} columns", columns.len());

    for (i, column) in columns.iter().enumerate() {
        println!("Column {}: {:?}", i, column);

        let rows = column.render();
        println!("  Column {} renders {} rows", i, rows.len());

        for (j, row) in rows.iter().enumerate() {
            println!("    Row {}: {:?}", j, row);

            // Now show that rows can render standard WidgetImpl elements
            let standard_widgets = row.render();
            println!(
                "      Row {} renders standard widgets: {:?}",
                j, standard_widgets
            );
        }
    }
}

fn one_b() {
    let row1 = Row {
        content: "first row".to_string(),
    };
    let row2 = Row {
        content: "second row!".to_string(),
    };
    let column = Column {
        rows: vec![row1, row2],
    };
    let grid = Grid {
        columns: vec![column],
    };

    println!("{:?}", grid.render());
    // [Column { rows: [Row { content: "first row" }, Row { content: "second row!" }] }]
}

fn main() {
    // println!("1a.");
    // one_a();
    // println!();

    println!("1b.");
    one_b();
    println!();
}
