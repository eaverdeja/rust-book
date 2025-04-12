// 1a.
// The API client is expected to provide a single WidgetImpl enum
// representing all possible widgets,
// and implement the Widget trait for WidgetImpl.

// Honestly, I don't even know how to implement this one!
// I tried but I don't know how to construct a vector of &dyn Widget.
// The book says "We create a trait object by specifying some sort of pointer, such as a & reference or a Box<T> smart pointer",
// so this should be valid, but I don't know how.

// For now I say this is not an appropriate solution.

// --- end 1a.

// 1b.
// The API client is expected to provide a unique struct for each
// possible widget, and implement the Widget trait for each struct.
// Each widget can return a vector containing widgets of any possible type.

// I don't even know how Option 4 should work lol.

// So I say it is not appropriate for 1b.

// --- end 1b.

// 1c.
// Only types that implement the Widget trait should
// be returned from render.

// Option 4: children are a reference trait object
// 1a. ❌
// 1b. ✅
// 1c. ✅
pub trait Widget {
    fn render(&self) -> Vec<&dyn Widget>;
}

// We'd need to add a lifetime parameter to deal with references
// pub trait Widget {
//     // Each widget returns references to other widgets it owns/contains
//     fn render<'a>(&'a self) -> Vec<&'a dyn Widget>;
// }
//
// This makes option 4 not viable since we'd have to change the signature on Widget::render

struct Button {
    button_type: String,
    content: String,
}
struct Text {
    content: String,
}
enum WidgetImpl {
    Button(Button),
    Text(Text),
}

impl Widget for Text {
    fn render(&self) -> Vec<&dyn Widget> {
        vec![]
    }
}

impl Widget for Button {
    fn render(&self) -> Vec<&dyn Widget> {
        let text = Text {
            content: self.content.clone(),
        };
        vec![&text]
        //         error[E0515]: cannot return value referencing local variable `text`
        //   --> src/bin/widget4.rs:54:9
        //    |
        // 54 |         vec![&text]
        //    |         ^^^^^-----^
        //    |         |    |
        //    |         |    `text` is borrowed here
        //    |         returns a value referencing data owned by the current function
        //    |
        //    = note: this error originates in the macro `vec` (in Nightly builds, run with -Z macro-backtrace for more info)
    }
}

impl Widget for WidgetImpl {
    fn render(&self) -> Vec<&dyn Widget> {
        match self {
            Self::Button(button) => button.render(),
            Self::Text(text) => text.render(),
        }
    }
}

fn main() {}
