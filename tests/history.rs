use ratatui_mergearea::TextArea;

#[test]
fn disable_history() {
    let mut t = TextArea::default();
    t.set_max_histories(0);
    assert!(t.insert_str("hello"));
    assert_eq!(t.text().as_str(), "hello");
}
