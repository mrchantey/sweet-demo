use sweet::*;
use sweet_demo::add;

// have a play with some of these and make them fail!

#[sweet_test]
fn it_adds() -> Result<()> {
    expect(add(1, 2)).to_be(3)
}

sweet! {
  test "assertions" {
    assert!(add(1,2) == 3);
  }

  test "custom matchers"{
        expect(add(1,2)).to_be(3)?;
  }

  test "negation"{
        expect(add(1,2)).not().to_be(4)?;
  }
}
