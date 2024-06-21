mod tests;

fn main() {
    let mut trellis = cucumber_trellis::CucumberTrellis::new(None);

    trellis.add_test::<tests::signals::Signals>();
    trellis.add_test::<tests::memo::MemoFunctions>();
    trellis.add_test::<tests::actions::Actions>();
    trellis.add_test::<tests::css::CssClasses>();

    trellis.run_tests();
}
