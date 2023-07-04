use stackalloc::AllocatedRegion;

#[test]
fn simple() {
    let region = AllocatedRegion::new(0, 10);

    assert_eq!(region.size(), 10);
    assert_eq!(region.start(), 0);
    assert_eq!(region.end(), 9);
    assert!(region.contains(0));
    assert!(!region.contains(10));
    assert!(region.contains(5));
}

#[test]
fn other() {
    let region = AllocatedRegion::new(45, 11);

    assert_eq!(region.size(), 11);
    assert_eq!(region.start(), 45);
    assert_eq!(region.end(), 45 + 11 - 1);
    assert!(!region.contains(0));
    assert!(!region.contains(10));
    assert!(region.contains(50));
}
