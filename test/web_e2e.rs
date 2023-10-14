use sweet::*;

#[sweet_test(e2e)]
pub async fn works() -> Result<()> {
    let page = visit("http://example.com").await?;
    expect(page)
        .poll(|p| p.to_contain_text("for use in illustrative examples"))
        .await
}
