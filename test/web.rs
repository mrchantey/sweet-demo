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

  test e2e "an e2e test"{
      let page = visit("http://example.com").await?;
    expect(page)
      .poll(|p|p.to_contain_text("This domain is for use in illustrative examples")).await?;
  }
}
