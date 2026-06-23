use week1::{dany, ethan, farzin};

macro_rules! test_skip_list {
    ($module:ident) => {{
        let mut list = $module::SkipList::new();

        assert!(!list.contains(10));

        list.insert(10);
        list.insert(5);
        list.insert(20);

        assert!(list.contains(10));
        assert!(list.contains(5));
        assert!(list.contains(20));
        assert!(!list.contains(99));

        assert!(list.remove(10));
        assert!(!list.contains(10));

        assert!(!list.remove(99));
    }};
}

#[test]
fn dany_skip_list_works() {
    test_skip_list!(dany);
}

#[test]
fn ethan_skip_list_works() {
    test_skip_list!(ethan);
}

#[test]
fn farzin_skip_list_works() {
    test_skip_list!(farzin);
}