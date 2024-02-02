use reactive_graph::{
    signal::ArcRwSignal,
    traits::{
        Get, Readable, Set, Update, UpdateUntracked, With, WithUntracked,
        Writeable,
    },
};

#[test]
fn create_signal() {
    let a = ArcRwSignal::new(0);
    assert_eq!(*a.read(), 0);
    assert_eq!(a.get(), 0);
    assert_eq!(a.with_untracked(|n| n + 1), 1);
    assert_eq!(a.with(|n| n + 1), 1);
}

#[test]
fn update_signal() {
    let a = ArcRwSignal::new(0);
    *a.write() += 1;
    assert_eq!(a.get(), 1);
    a.update(|n| *n += 1);
    assert_eq!(a.get(), 2);
    a.update_untracked(|n| *n += 1);
    assert_eq!(a.get(), 3);
    a.set(4);
    assert_eq!(a.get(), 4);
}
