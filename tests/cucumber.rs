mod tests;

#[cucumber_trellis::cucumber_test(use_tokio)]
fn cucumber(trellis: &mut cucumber_trellis::CucumberTrellis) {
    trellis.add_test::<tests::signals::Signals>();
    trellis.add_test::<tests::memo::MemoFunctions>();
    trellis.add_test::<tests::actions::Actions>();
    trellis.add_test::<tests::css::CssClasses>();
}
