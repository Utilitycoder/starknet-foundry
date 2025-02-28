use crate::e2e::common::runner::{setup_package, test_runner};
use crate::{assert_stderr_contains, assert_stdout_contains};
use indoc::indoc;

#[test]
fn fuzzing() {
    let temp = setup_package("fuzzing");
    let snapbox = test_runner().arg("fuzzing");

    let output = snapbox.current_dir(&temp).assert().code(1);
    assert_stdout_contains!(
        output,
        indoc! {r"
        [..]Compiling[..]
        [..]Finished[..]


        Collected 13 test(s) from fuzzing package
        Running 13 test(s) from src/
        [PASS] fuzzing::tests::adding [..]
        [PASS] fuzzing::tests::fuzzed_argument (runs: 256, [..]
        [PASS] fuzzing::tests::fuzzed_both_arguments (runs: 256, [..]
        [PASS] fuzzing::tests::passing [..]
        [FAIL] fuzzing::tests::failing_fuzz (runs: 1, arguments: [[..], [..]])

        Failure data:
            original value: [593979512822486835600413552099926114], converted to a string: [result == a + b]

        [PASS] fuzzing::tests::custom_fuzzer_config (runs: 10, [..]
        [PASS] fuzzing::tests::uint8_arg (runs: 256, [..]
        [PASS] fuzzing::tests::fuzzed_loop (runs: 256, gas: {max: ~63, min: ~1, mean: ~33.00, std deviation: ~18.63})
        [PASS] fuzzing::tests::uint16_arg (runs: 256, [..]
        [PASS] fuzzing::tests::uint32_arg (runs: 256, [..]
        [PASS] fuzzing::tests::uint64_arg (runs: 256, [..]
        [PASS] fuzzing::tests::uint128_arg (runs: 256, [..]
        [PASS] fuzzing::tests::uint256_arg (runs: 256, [..]
        Running 0 test(s) from tests/
        Tests: 12 passed, 1 failed, 0 skipped, 0 ignored, 6 filtered out
        Fuzzer seed: [..]

        Failures:
            fuzzing::tests::failing_fuzz
        "}
    );
}

#[test]
fn fuzzing_set_runs() {
    let temp = setup_package("fuzzing");
    let snapbox = test_runner().arg("fuzzing");

    let output = snapbox
        .current_dir(&temp)
        .args(["--fuzzer-runs", "10"])
        .assert()
        .code(1);
    assert_stdout_contains!(
        output,
        indoc! {r"
        [..]Compiling[..]
        [..]Finished[..]
        
        
        Collected 13 test(s) from fuzzing package
        Running 13 test(s) from src/
        [PASS] fuzzing::tests::adding [..]
        [PASS] fuzzing::tests::fuzzed_argument (runs: 10, [..]
        [PASS] fuzzing::tests::fuzzed_both_arguments (runs: 10, [..]
        [PASS] fuzzing::tests::passing [..]
        [FAIL] fuzzing::tests::failing_fuzz (runs: 1, arguments: [[..], [..]])

        Failure data:
            original value: [593979512822486835600413552099926114], converted to a string: [result == a + b]

        [PASS] fuzzing::tests::custom_fuzzer_config (runs: 10, [..]
        [PASS] fuzzing::tests::uint8_arg (runs: 10, [..]
        [PASS] fuzzing::tests::fuzzed_loop (runs: 256, [..]
        [PASS] fuzzing::tests::uint16_arg (runs: 10, [..]
        [PASS] fuzzing::tests::uint32_arg (runs: 10, [..]
        [PASS] fuzzing::tests::uint64_arg (runs: 10, [..]
        [PASS] fuzzing::tests::uint128_arg (runs: 10, [..]
        [PASS] fuzzing::tests::uint256_arg (runs: 10, [..]
        Running 0 test(s) from tests/
        Tests: 12 passed, 1 failed, 0 skipped, 0 ignored, 6 filtered out
        Fuzzer seed: [..]

        Failures:
            fuzzing::tests::failing_fuzz
        "}
    );
}

#[test]
fn fuzzing_set_seed() {
    let temp = setup_package("fuzzing");
    let snapbox = test_runner().arg("fuzzing");

    let output = snapbox
        .current_dir(&temp)
        .args(["--fuzzer-seed", "1234"])
        .assert()
        .code(1);
    assert_stdout_contains!(
        output,
        indoc! {r"
        [..]Compiling[..]
        [..]Finished[..]
        
        
        Collected 13 test(s) from fuzzing package
        Running 13 test(s) from src/
        [PASS] fuzzing::tests::adding [..]
        [PASS] fuzzing::tests::fuzzed_argument (runs: 256, [..]
        [PASS] fuzzing::tests::fuzzed_both_arguments (runs: 256, [..]
        [PASS] fuzzing::tests::passing [..]
        [FAIL] fuzzing::tests::failing_fuzz (runs: 1, arguments: [[..], [..]])

        Failure data:
            original value: [..], converted to a string: [result == a + b]

        [PASS] fuzzing::tests::custom_fuzzer_config (runs: 10, [..]
        [PASS] fuzzing::tests::uint8_arg (runs: 256, [..]
        [PASS] fuzzing::tests::fuzzed_loop (runs: 256, [..]
        [PASS] fuzzing::tests::uint16_arg (runs: 256, [..]
        [PASS] fuzzing::tests::uint32_arg (runs: 256, [..]
        [PASS] fuzzing::tests::uint64_arg (runs: 256, [..]
        [PASS] fuzzing::tests::uint128_arg (runs: 256, [..]
        [PASS] fuzzing::tests::uint256_arg (runs: 256, [..]
        Running 0 test(s) from tests/
        Tests: 12 passed, 1 failed, 0 skipped, 0 ignored, 6 filtered out
        Fuzzer seed: 1234

        Failures:
            fuzzing::tests::failing_fuzz
        "}
    );
}

#[test]
fn fuzzing_incorrect_runs() {
    let temp = setup_package("fuzzing");
    let snapbox = test_runner();

    let output = snapbox
        .current_dir(&temp)
        .args(["--fuzzer-runs", "0"])
        .assert()
        .code(2);
    assert_stderr_contains!(
        output,
        indoc! {r"
        error: invalid value '0' for '--fuzzer-runs <FUZZER_RUNS>': Number of fuzzer runs must be greater than or equal to 3

        For more information, try '--help'.
        "}
    );
}

#[test]
fn fuzzing_incorrect_function_args() {
    let temp = setup_package("fuzzing");
    let snapbox = test_runner().arg("incorrect_args");

    let output = snapbox.current_dir(&temp).assert().code(2);
    assert_stdout_contains!(
        output,
        indoc! {r"
        [..]Compiling[..]
        [..]Finished[..]
        
        
        Collected 2 test(s) from fuzzing package
        Running 0 test(s) from src/
        Running 2 test(s) from tests/
        [ERROR] Tried to use incorrect type for fuzzing. Type = tests::incorrect_args::MyStruct is not supported
        "}
    );
}

#[test]
fn fuzzing_exit_first() {
    let temp = setup_package("fuzzing");
    let snapbox = test_runner().arg("exit_first_fuzz").arg("-x");

    let output = snapbox.current_dir(&temp).assert().code(1);
    assert_stdout_contains!(
        output,
        indoc! {r"
        [..]Compiling[..]
        [..]Finished[..]


        Collected 2 test(s) from fuzzing package
        Running 0 test(s) from src/
        Running 2 test(s) from tests/
        [FAIL] tests::exit_first_fuzz::exit_first_fails_test (runs: 1, arguments: [..])

        Failure data:
            original value: [..], converted to a string: [2 + b == 2 + b]

        Tests: 0 passed, 1 failed, 1 skipped, 0 ignored, 17 filtered out

        Fuzzer seed: [..]
        Failures:
            tests::exit_first_fuzz::exit_first_fails_test
        "}
    );
}

#[test]
fn fuzzing_exit_first_single_fail() {
    let temp = setup_package("fuzzing");
    let snapbox = test_runner().arg("exit_first_single_fail").arg("-x");

    let output = snapbox.current_dir(&temp).assert().code(1);
    assert_stdout_contains!(
        output,
        indoc! {r"
        [..]Compiling[..]
        [..]Finished[..]


        Collected 2 test(s) from fuzzing package
        Running 0 test(s) from src/
        Running 2 test(s) from tests/
        [FAIL] tests::exit_first_single_fail::exit_first_fails_test

        Failure data:
            original value: [..], converted to a string: [2 + b == 2 + b]

        Failures:
            tests::exit_first_single_fail::exit_first_fails_test

        Tests: 0 passed, 1 failed, 1 skipped, 0 ignored, 17 filtered out
        "}
    );
}
