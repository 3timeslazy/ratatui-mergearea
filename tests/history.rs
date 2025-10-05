use ratatui_mergearea::MergeArea;

#[test]
fn disable_history() {
    let mut t = MergeArea::default();
    t.set_max_histories(0);
    assert!(t.insert_str("hello"));
    assert_eq!(t.text().as_str(), "hello");
}
