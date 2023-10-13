use sweet::*;

sweet! {
  test e2e "an e2e test"{
    let page = visit("http://example.com").await?;
    expect(page)
      .poll(|p|p.to_contain_text("for use in illustrative examples")).await?;
  }
}
