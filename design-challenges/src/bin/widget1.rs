// 1a.
// The API client is expected to provide a single WidgetImpl enum
// representing all possible widgets,
// and implement the Widget trait for WidgetImpl.

// Option 1 won't work for 1a. because it requires WidgetImpl to be known
// at compile time. It can't have different Widget implementations in
// the same tree. It is closed for extension - not open.
// So the API client wouldn't be able to provide their own WidgetImpl enum.

// The above is not false, but since the API client provides a SINGLE enum in advance,
// that is not an issue.
// --- end 1a.

// 1b.
// The API client is expected to provide a unique struct for each
// possible widget, and implement the Widget trait for each struct.
// Each widget can return a vector containing widgets of any possible type.

// Option 1 won't work for 1b. because it wouldn't allow for widgets
// to return a vector containing widgets of any possible type, only Self.
// -- end 1b.

// 1c.
// Only types that implement the Widget trait should
// be returned from render.

// Option 1: children must be Self
// 1a. ✅
// 1b. ❌
// 1c. ✅
pub trait Widget: Sized {
    fn render(&self) -> Vec<Self>;
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
    fn render(&self) -> Vec<Self> {
        match self {
            Self::Button {
                button_type,
                content,
            } => {
                // content as a Text child widget
                vec![WidgetImpl::Text(content.clone())]
            }
            Self::Text(_) => {
                // Text nodes are leaf nodes with no children
                vec![]
            }
        }
    }
}

// 1b.
struct Text {}
struct Button {}
impl Widget for Button {
    fn render(&self) -> Vec<Self> {
        vec![Text {}]
        //         error[E0308]: mismatched types
        //   --> src/bin/widget1.rs:60:14
        //    |
        // 60 |         vec![Text {}]
        //    |              ^^^^^^^ expected `Button`, found `Text`
    }
}

fn main() {
    // 1a.
    let button = WidgetImpl::Button {
        button_type: "submit".to_string(),
        content: "Click me".to_string(),
    };
    println!("{:?}", button.render())
}
