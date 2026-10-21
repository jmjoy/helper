macro_rules! map_tests {
    ($map:ident) => {
        mod $map {
            #[test]
            fn paths_and_generic_arguments() {
                let values = helper::$map! {
                    u8::MAX: Result::<u16, u32>::Ok(u16::MAX),
                    u8::MIN: Result::<u16, u32>::Err(u32::MAX),
                };
                assert_eq!(values[&u8::MAX], Ok(u16::MAX));
                assert_eq!(values[&u8::MIN], Err(u32::MAX));

                let values = helper::$map! {
                    <u8 as Into<u16>>::into(7): Vec::<(u8, u16)>::new(),
                };
                assert_eq!(values[&7], []);
            }

            #[test]
            fn closures_with_typed_parameters() {
                let operations = helper::$map! {"add": |a: i32, b: i32| a + b};
                assert_eq!(operations["add"](2, 3), 5);
            }

            #[test]
            fn does_not_shadow_outer_map() {
                let map = [10, 20, 30];
                let values = helper::$map! {map.len(): map[0], 0: map.len()};
                assert_eq!(values[&3], 10);
                assert_eq!(values[&0], 3);
            }

            #[test]
            fn evaluates_once_in_order_and_keeps_last_duplicate_value() {
                let mut events = Vec::new();
                let values = helper::$map! {
                    { events.push("key1"); 7 }: { events.push("value1"); 10 },
                    { events.push("key2"); 7 }: { events.push("value2"); 20 },
                };

                assert_eq!(events, ["key1", "value1", "key2", "value2"]);
                assert_eq!(values.len(), 1);
                assert_eq!(values[&7], 20);
            }

            #[test]
            fn propagates_errors_from_keys_and_values() {
                fn build(
                    key: Result<u8, &'static str>, value: Result<u8, &'static str>,
                ) -> Result<u8, &'static str> {
                    let values = helper::$map! {key?: value?};
                    Ok(values[&1])
                }

                assert_eq!(build(Ok(1), Ok(2)), Ok(2));
                assert_eq!(build(Err("key"), Ok(2)), Err("key"));
                assert_eq!(build(Ok(1), Err("value")), Err("value"));
            }
        }
    };
}

map_tests!(hmap);
map_tests!(btmap);

macro_rules! set_tests {
    ($set:ident) => {
        mod $set {
            #[test]
            fn paths_and_generic_arguments() {
                let values = helper::$set! {
                    Result::<u16, u32>::Ok(u16::MAX),
                    Result::<u16, u32>::Err(u32::MAX),
                };
                assert!(values.contains(&Ok(u16::MAX)));
                assert!(values.contains(&Err(u32::MAX)));
            }

            #[test]
            fn does_not_shadow_outer_seq() {
                let seq = [10, 20, 30];
                let values = helper::$set! {seq.len(), seq[0]};
                assert!(values.contains(&3));
                assert!(values.contains(&10));
            }

            #[test]
            fn evaluates_once_in_order_and_deduplicates() {
                let mut events = Vec::new();
                let values = helper::$set! {
                    { events.push(1); 7 },
                    { events.push(2); 7 },
                    { events.push(3); 8 },
                };

                assert_eq!(events, [1, 2, 3]);
                assert_eq!(values.len(), 2);
                assert!(values.contains(&7));
                assert!(values.contains(&8));
            }

            #[test]
            fn propagates_errors() {
                fn build(value: Result<u8, &'static str>) -> Result<bool, &'static str> {
                    let values = helper::$set! {value?};
                    Ok(values.contains(&1))
                }

                assert_eq!(build(Ok(1)), Ok(true));
                assert_eq!(build(Err("value")), Err("value"));
            }
        }
    };
}

set_tests!(hset);
set_tests!(btset);
