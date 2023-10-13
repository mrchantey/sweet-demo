use fantoccini::ClientBuilder;
use sweet::*;

sweet! {
    it nonSend "works" {
    let client = ClientBuilder::native()
        .connect("http://localhost:9515").await?;

    client.goto("https://example.com").await?;
    let url = client.current_url().await?;
    expect(url.as_ref()).to_contain("example.com")?;

    client.close().await?;
  }
}
