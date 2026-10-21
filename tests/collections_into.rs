#![forbid(unsafe_code)]

macro_rules! conversion_tests {
    ($module:ident, $map_type:ident, $set_type:ident, $map_macro:ident, $set_macro:ident) => {
        mod $module {
            use std::{
                cell::RefCell,
                collections::{$map_type as Map, $set_type as Set},
            };

            use helper::{$map_macro as make_map, $set_macro as make_set};

            #[test]
            fn converts_mixed_borrowed_and_owned_strings() {
                let values: Map<String, String> = make_map! {
                    "name": "alice",
                    String::from("role"): String::from("admin"),
                };
                assert_eq!(values["name"], "alice");
                assert_eq!(values["role"], "admin");

                let names: Set<String> = make_set! {"alice", String::from("bob"), "alice",};
                assert_eq!(names.len(), 2);
                assert!(names.contains("alice"));
                assert!(names.contains("bob"));
            }

            #[test]
            fn converts_numeric_types_and_supports_generic_expressions() {
                let values: Map<u16, u64> = make_map! {u8::MAX: u32::MAX, 0u8: 7u16};
                assert_eq!(values[&255], u64::from(u32::MAX));
                assert_eq!(values[&0], 7);

                let results: Set<Result<u16, u32>> = make_set! {
                    Result::<u16, u32>::Ok(7),
                    Result::<u16, u32>::Err(9),
                };
                assert!(results.contains(&Ok(7)));
                assert!(results.contains(&Err(9)));
            }

            #[test]
            fn supports_empty_collections_and_trailing_commas() {
                let values: Map<String, String> = make_map! {};
                let names: Set<String> = make_set! {};
                assert!(values.is_empty());
                assert!(names.is_empty());

                let values: Map<String, String> = make_map! {"key": "value",};
                assert_eq!(values["key"], "value");
            }

            struct Input<'a> {
                value: &'static str,
                converted: &'static str,
                events: &'a RefCell<Vec<&'static str>>,
            }

            impl From<Input<'_>> for String {
                fn from(input: Input<'_>) -> Self {
                    input.events.borrow_mut().push(input.converted);
                    Self::from(input.value)
                }
            }

            fn input<'a>(
                events: &'a RefCell<Vec<&'static str>>, evaluated: &'static str,
                converted: &'static str, value: &'static str,
            ) -> Input<'a> {
                events.borrow_mut().push(evaluated);
                Input {
                    value,
                    converted,
                    events,
                }
            }

            #[test]
            fn map_evaluation_and_conversion_happen_once_in_input_order() {
                let events = RefCell::new(Vec::new());
                let values: Map<String, String> = make_map! {
                    input(&events, "key1", "convert key1", "key"):
                        input(&events, "value1", "convert value1", "old"),
                    input(&events, "key2", "convert key2", "key"):
                        input(&events, "value2", "convert value2", "new"),
                };

                assert_eq!(
                    *events.borrow(),
                    [
                        "key1",
                        "convert key1",
                        "value1",
                        "convert value1",
                        "key2",
                        "convert key2",
                        "value2",
                        "convert value2",
                    ]
                );
                assert_eq!(values.len(), 1);
                assert_eq!(values["key"], "new");
            }

            #[test]
            fn set_evaluation_and_conversion_happen_once_before_deduplication() {
                let events = RefCell::new(Vec::new());
                let names: Set<String> = make_set! {
                    input(&events, "first", "convert first", "alice"),
                    input(&events, "second", "convert second", "alice"),
                };

                assert_eq!(
                    *events.borrow(),
                    ["first", "convert first", "second", "convert second"]
                );
                assert_eq!(names.len(), 1);
                assert!(names.contains("alice"));
            }

            #[test]
            fn propagates_errors_in_the_calling_function() {
                fn map(
                    key: Result<u8, &'static str>, value: Result<u8, &'static str>,
                ) -> Result<Map<u16, u32>, &'static str> {
                    Ok(make_map! {key?: value?})
                }

                assert_eq!(map(Ok(1), Ok(2)).unwrap()[&1], 2);
                assert_eq!(map(Err("key"), Ok(2)), Err("key"));
                assert_eq!(map(Ok(1), Err("value")), Err("value"));

                fn set(value: Option<u8>) -> Option<Set<u16>> {
                    Some(make_set! {value?})
                }

                assert!(set(Some(1)).unwrap().contains(&1));
                assert_eq!(set(None), None);
            }

            #[test]
            fn preserves_bindings_and_precedence_forwarded_by_macro_rules() {
                macro_rules! map {
                    ($value: expr) => {{
                        let value = 99u8;
                        let _ = &value;
                        make_map! {$value * 2: $value * 3}
                    }};
                }
                macro_rules! set {
                    ($value: expr) => {{
                        let value = 99u8;
                        let _ = &value;
                        make_set! {$value * 2}
                    }};
                }

                let value = 1u8;
                let values: Map<u16, u32> = map!(value + 2);
                let names: Set<u16> = set!(value + 2);
                assert_eq!(values[&6], 9);
                assert!(names.contains(&6));
            }

            #[test]
            fn generated_bindings_and_trait_paths_are_hygienic() {
                struct Into;
                let _ = Into;
                let map = [1u8, 2];
                let seq = [3u8, 4];

                let values: Map<u16, u32> = make_map! {map[0]: map[1]};
                let names: Set<u16> = make_set! {seq[0], seq[1]};
                assert_eq!(values[&1], 2);
                assert_eq!(names.len(), 2);
                assert!(names.contains(&3));
                assert!(names.contains(&4));
            }
        }
    };
}

conversion_tests!(hash, HashMap, HashSet, hmap_into, hset_into);
conversion_tests!(btree, BTreeMap, BTreeSet, btmap_into, btset_into);

#[test]
fn btree_collections_iterate_in_sorted_order_after_conversion() {
    use std::collections::{BTreeMap, BTreeSet};

    let values: BTreeMap<String, String> = helper::btmap_into! {
        "z": "last", "a": "first", "m": "middle",
    };
    let names: BTreeSet<String> = helper::btset_into! {"z", "a", "m", "a"};

    assert_eq!(
        values.keys().map(String::as_str).collect::<Vec<_>>(),
        ["a", "m", "z"]
    );
    assert_eq!(names.into_iter().collect::<Vec<_>>(), ["a", "m", "z"]);
}
