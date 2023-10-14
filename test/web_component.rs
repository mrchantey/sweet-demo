use sweet::*;
use web_sys::window;

use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
    window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .set_inner_html("<h1>This is a heading</h1>");
    expect(window())
        .get("h1")?
        .to_contain_text("This is a heading")
}
