use criterion::{criterion_group, criterion_main, Criterion};
use ratatui_mergearea::MergeArea;
use ratatui_mergearea_bench::{dummy_terminal, TerminalExt, LOREM};

#[inline]
fn run(pat: &str, mut textarea: MergeArea<'_>, forward: bool) {
    let mut term = dummy_terminal();
    textarea.set_search_pattern(pat).unwrap();
    term.draw_textarea(&textarea);
    for _ in 0..100 {
        if forward {
            textarea.search_forward(false);
        } else {
            textarea.search_back(false);
        }
        term.draw_textarea(&textarea);
    }
    textarea.set_search_pattern(r"").unwrap();
    term.draw_textarea(&textarea);
}

fn short(c: &mut Criterion) {
    let textarea = MergeArea::with_value(LOREM.join("\n"));
    c.bench_function("search::forward_short", |b| {
        b.iter(|| run(r"\w*i\w*", textarea.clone(), true))
    });
    c.bench_function("search::back_short", |b| {
        b.iter(|| run(r"\w*i\w*", textarea.clone(), false))
    });
}

fn long(c: &mut Criterion) {
    let mut lines = vec![];
    for _ in 0..10 {
        lines.push(LOREM.join("\n"));
    }
    let textarea = MergeArea::with_value(lines.join("\n"));
    c.bench_function("search::forward_long", |b| {
        b.iter(|| run(r"[A-Z]\w*", textarea.clone(), true))
    });
    c.bench_function("search::back_long", |b| {
        b.iter(|| run(r"[A-Z]\w*", textarea.clone(), false))
    });
}

criterion_group!(search, short, long);
criterion_main!(search);
