mod document;
mod row;
mod editor;
mod terminal;
use editor::Editor;
pub use terminal::Terminal; // 这里的原理是在顶层重新导出Terminal结构，下面的模块可以直接 use crate::Terminal;
pub use editor::Position;
pub use row::Row;
pub use document::Document;

// TODO https://www.flenker.blog/hecto-chapter-4/ => Snap cursor to end of line

fn main() {
    let mut editor = Editor::default();
    editor.run();
}
