use sweet::*;
use web_sys::window;

sweet! {
  test "a component test" {
    window().unwrap()
      .document().unwrap()
      .body().unwrap()
      .set_inner_html("<h1>This is a heading</h1>");
    expect(window()).get("h1")?
      .to_contain_text("This is a heading")?;
  }
}
