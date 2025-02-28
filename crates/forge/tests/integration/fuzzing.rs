use forge_runner::test_case_summary::{AnyTestCaseSummary, TestCaseSummary};
use indoc::indoc;
use test_utils::runner::TestCase;
use test_utils::running_tests::run_test_case;
use test_utils::{assert_passed, test_case};

#[test]
fn fuzzed_argument() {
    let test = test_case!(indoc!(
        r"
        fn adder(a: felt252, b: felt252) -> felt252 {
            a + b
        }

        #[test]
        fn fuzzed_argument(b: felt252) {
            let result = adder(2, b);
            assert(result == 2 + b, '2 + b == 2 + b');
        }
    "
    ));

    let result = run_test_case(&test);

    assert_passed!(result);
}

#[test]
fn fuzzer_different_types() {
    let test = test_case!(indoc!(
        r"
        #[test]
        fn fuzzer_different_types(a: u256) {
            if a <= 5_u256 {
                assert(2 == 2, '2 == 2');
            } else {
                let x = a - 5_u256;
                assert(x == a - 5_u256, 'x != a - 5');
            }
        }
    "
    ));

    let result = run_test_case(&test);

    assert_passed!(result);
}

#[test]
fn fuzzed_loop() {
    let test = test_case!(indoc!(
        r"
        #[test]
        #[fuzzer(runs: 256, seed: 100)]
        fn fuzzed_loop(a: u8) {
            let mut i: u8 = 0;
            loop {
                if (i == a) {
                    break;
                }

                i += 1;
            };

            assert(1 == 1, '');
        }
    "
    ));

    let result = run_test_case(&test);

    let crate_summary = TestCase::find_test_result(&result);
    let AnyTestCaseSummary::Fuzzing(TestCaseSummary::Passed { gas_info, .. }) =
        &crate_summary.test_case_summaries[0]
    else {
        panic!()
    };

    assert_eq!(gas_info.min, 1);
    assert_eq!(gas_info.max, 63);
    assert!((gas_info.mean - 33.).abs() < f64::EPSILON);
    assert!((gas_info.std_deviation - 18.626).abs() < 0.01);
}
