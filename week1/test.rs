use std::collections::BTreeSet;

use week1::{dany, ethan, farzin};

const GENERATED_SCENARIOS: usize = 500;

trait SkipListOps: Sized {
    fn new() -> Self;
    fn insert(&mut self, value: i32);
    fn contains(&self, value: i32) -> bool;
    fn remove(&mut self, value: i32) -> bool;
}

macro_rules! impl_skip_list_ops {
    ($module:ident) => {
        impl SkipListOps for $module::SkipList {
            fn new() -> Self {
                $module::SkipList::new()
            }

            fn insert(&mut self, value: i32) {
                $module::SkipList::insert(self, value);
            }

            fn contains(&self, value: i32) -> bool {
                $module::SkipList::contains(self, value)
            }

            fn remove(&mut self, value: i32) -> bool {
                $module::SkipList::remove(self, value)
            }
        }
    };
}

impl_skip_list_ops!(dany);
impl_skip_list_ops!(ethan);
impl_skip_list_ops!(farzin);

fn assert_matches_model<L: SkipListOps>(
    list: &L,
    expected: &BTreeSet<i32>,
    probes: &[i32],
    context: &str,
) {
    for value in probes {
        assert_eq!(
            list.contains(*value),
            expected.contains(value),
            "{context}: contains({value}) did not match the model"
        );
    }
}

fn insert_model<L: SkipListOps>(
    list: &mut L,
    expected: &mut BTreeSet<i32>,
    value: i32,
    context: &str,
) {
    list.insert(value);
    expected.insert(value);
    assert_matches_model(list, expected, &[value], context);
}

fn remove_model<L: SkipListOps>(
    list: &mut L,
    expected: &mut BTreeSet<i32>,
    value: i32,
    context: &str,
) {
    let expected_removed = expected.remove(&value);
    assert_eq!(
        list.remove(value),
        expected_removed,
        "{context}: remove({value}) returned the wrong result"
    );
    assert_matches_model(list, expected, &[value], context);
}

fn value_for(scenario: usize, step: usize) -> i32 {
    let raw = (scenario as i64 * 1_103_515_245 + step as i64 * 12_345 + 97_531) % 200_001;
    raw as i32 - 100_000
}

fn probe_values(scenario: usize, step: usize) -> [i32; 12] {
    [
        value_for(scenario, step),
        value_for(scenario, step + 1),
        value_for(scenario, step + 7),
        value_for(scenario.wrapping_add(11), step + 13),
        -(value_for(scenario, step + 3)),
        0,
        1,
        -1,
        i32::MIN,
        i32::MAX,
        scenario as i32,
        -(scenario as i32),
    ]
}

fn run_empty_list_cases<L: SkipListOps>(suite_name: &str) {
    let mut list = L::new();
    let mut expected = BTreeSet::new();
    let probes = [0, 1, -1, 10, -10, i32::MIN, i32::MAX];

    assert_matches_model(&list, &expected, &probes, suite_name);

    for value in probes {
        remove_model(
            &mut list,
            &mut expected,
            value,
            &format!("{suite_name} empty-list removal"),
        );
    }

    assert_matches_model(&list, &expected, &probes, suite_name);
}

fn run_basic_ordering_cases<L: SkipListOps>(suite_name: &str) {
    let mut list = L::new();
    let mut expected = BTreeSet::new();
    let values = [10, 5, 20, -4, 0, 15, -30, 7, i32::MIN, i32::MAX];

    for value in values {
        insert_model(
            &mut list,
            &mut expected,
            value,
            &format!("{suite_name} basic unordered insert"),
        );
    }

    assert_matches_model(&list, &expected, &values, suite_name);

    for value in [10, -30, i32::MIN, i32::MAX, 123_456] {
        remove_model(
            &mut list,
            &mut expected,
            value,
            &format!("{suite_name} basic removal"),
        );
    }

    assert_matches_model(&list, &expected, &values, suite_name);
}

fn run_duplicate_cases<L: SkipListOps>(suite_name: &str) {
    let mut list = L::new();
    let mut expected = BTreeSet::new();
    let values = [42, -42, 0, i32::MIN, i32::MAX];

    for value in values {
        for _ in 0..12 {
            insert_model(
                &mut list,
                &mut expected,
                value,
                &format!("{suite_name} duplicate insert"),
            );
        }
    }

    for value in values {
        remove_model(
            &mut list,
            &mut expected,
            value,
            &format!("{suite_name} first duplicate removal"),
        );
        remove_model(
            &mut list,
            &mut expected,
            value,
            &format!("{suite_name} second duplicate removal"),
        );
    }
}

fn run_reinsert_cases<L: SkipListOps>(suite_name: &str) {
    let mut list = L::new();
    let mut expected = BTreeSet::new();

    for round in 0..50 {
        let value = value_for(round, round * 3);
        insert_model(
            &mut list,
            &mut expected,
            value,
            &format!("{suite_name} reinsert round {round}"),
        );
        remove_model(
            &mut list,
            &mut expected,
            value,
            &format!("{suite_name} reinsert round {round}"),
        );
        insert_model(
            &mut list,
            &mut expected,
            value,
            &format!("{suite_name} reinsert round {round}"),
        );
    }

    for round in (0..50).rev() {
        let value = value_for(round, round * 3);
        remove_model(
            &mut list,
            &mut expected,
            value,
            &format!("{suite_name} reverse cleanup round {round}"),
        );
    }
}

fn run_generated_scenario<L: SkipListOps>(suite_name: &str, scenario: usize) {
    let mut list = L::new();
    let mut expected = BTreeSet::new();

    for step in 0..160 {
        let context = format!("{suite_name} generated scenario {scenario}, step {step}");
        let value = value_for(scenario, step);

        match (scenario + step) % 6 {
            0 | 1 | 2 => insert_model(&mut list, &mut expected, value, &context),
            3 => remove_model(&mut list, &mut expected, value, &context),
            4 => {
                insert_model(&mut list, &mut expected, value, &context);
                insert_model(&mut list, &mut expected, value, &context);
            }
            _ => {
                remove_model(&mut list, &mut expected, value, &context);
                insert_model(&mut list, &mut expected, value, &context);
            }
        }

        if step % 8 == 0 {
            let probes = probe_values(scenario, step);
            assert_matches_model(&list, &expected, &probes, &context);
        }
    }

    for step in (0..160).step_by(3) {
        let context = format!("{suite_name} generated scenario {scenario}, cleanup {step}");
        remove_model(
            &mut list,
            &mut expected,
            value_for(scenario, step),
            &context,
        );
    }

    for step in 0..160 {
        let context = format!("{suite_name} generated scenario {scenario}, final check {step}");
        assert_matches_model(
            &list,
            &expected,
            &[value_for(scenario, step)],
            &context,
        );
    }
}

fn run_full_skip_list_suite<L: SkipListOps>(suite_name: &str) {
    run_empty_list_cases::<L>(suite_name);
    run_basic_ordering_cases::<L>(suite_name);
    run_duplicate_cases::<L>(suite_name);
    run_reinsert_cases::<L>(suite_name);

    for scenario in 0..GENERATED_SCENARIOS {
        run_generated_scenario::<L>(suite_name, scenario);
    }
}

#[test]
fn dany_full_skip_list_suite() {
    run_full_skip_list_suite::<dany::SkipList>("dany");
}

#[test]
fn ethan_full_skip_list_suite() {
    run_full_skip_list_suite::<ethan::SkipList>("ethan");
}

#[test]
fn farzin_full_skip_list_suite() {
    run_full_skip_list_suite::<farzin::SkipList>("farzin");
}
