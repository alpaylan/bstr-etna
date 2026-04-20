// ETNA workload runner for bstr.
//
// Usage: cargo run --release --bin etna -- <tool> <property>
//   tool:     etna | proptest | quickcheck | crabcheck | hegel
//   property: SplitnBounded | CowPartialEqBstr | DebugValidFfFdPreserved
//             | DebugAsciiControlHex | DebugHexLowercase | All
//
// Each invocation emits exactly one JSON line to stdout and exits 0 unless
// called with bad arguments (exit 2). Etna's `log_process_output` parses the
// JSON line; non-zero exit would produce a status:aborted record.

use bstr::etna::{
    property_cow_partialeq_bstr, property_debug_ascii_control_hex,
    property_debug_hex_lowercase, property_debug_valid_fffd_preserved,
    property_splitn_bounded, PropertyResult,
};
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

#[derive(Default, Clone, Copy)]
struct Metrics {
    inputs: u64,
    elapsed_us: u128,
}

impl Metrics {
    fn combine(self, other: Metrics) -> Metrics {
        Metrics {
            inputs: self.inputs + other.inputs,
            elapsed_us: self.elapsed_us + other.elapsed_us,
        }
    }
}

type Outcome = (Result<(), String>, Metrics);

fn to_err(r: PropertyResult) -> Result<(), String> {
    match r {
        PropertyResult::Pass | PropertyResult::Discard => Ok(()),
        PropertyResult::Fail(m) => Err(m),
    }
}

const ALL_PROPERTIES: &[&str] = &[
    "SplitnBounded",
    "CowPartialEqBstr",
    "DebugValidFfFdPreserved",
    "DebugAsciiControlHex",
    "DebugHexLowercase",
];

fn run_all<F: FnMut(&str) -> Outcome>(mut f: F) -> Outcome {
    let mut total = Metrics::default();
    let mut final_status: Result<(), String> = Ok(());
    for p in ALL_PROPERTIES {
        let (r, m) = f(p);
        total = total.combine(m);
        if r.is_err() && final_status.is_ok() {
            final_status = r;
        }
    }
    (final_status, total)
}

// ---------- shared newtypes ----------
//
// The quickcheck fork requires `Arbitrary + Debug + Display` for every
// function argument when the `etna` feature is enabled (see
// tester.rs:705). `Vec<u8>` has `Debug` but not `Display`, so we wrap it
// in a small newtype that forwards `Display` to `Debug`. The same wrapper
// is used as the crabcheck `Arbitrary<R>` carrier for consistency.

#[derive(Clone)]
struct Bytes(Vec<u8>);

impl fmt::Debug for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl quickcheck::Arbitrary for Bytes {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let len = g.random_range(0usize..=32);
        Bytes((0..len).map(|_| u8::arbitrary(g)).collect())
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(self.0.shrink().map(Bytes))
    }
}

impl<R: rand::Rng> crabcheck::quickcheck::Arbitrary<R> for Bytes {
    fn generate(rng: &mut R, _n: usize) -> Self {
        let len = rng.random_range(0usize..=32);
        Bytes((0..len).map(|_| rng.random_range(0u16..=255) as u8).collect())
    }
}

// ---------- etna (concrete witness replay) ----------

fn run_etna_property(property: &str) -> Outcome {
    if property == "All" {
        return run_all(run_etna_property);
    }
    let t0 = Instant::now();
    let result = match property {
        "SplitnBounded" => {
            to_err(property_splitn_bounded(b"ab".to_vec(), b":".to_vec(), 2))
        }
        "CowPartialEqBstr" => to_err(property_cow_partialeq_bstr(
            b"hello".to_vec(),
            b"goodbye".to_vec(),
        )),
        "DebugValidFfFdPreserved" => {
            to_err(property_debug_valid_fffd_preserved(Vec::new(), Vec::new()))
        }
        "DebugAsciiControlHex" => to_err(property_debug_ascii_control_hex(0x1c)),
        "DebugHexLowercase" => to_err(property_debug_hex_lowercase(0xff)),
        _ => {
            return (
                Err(format!("Unknown property for etna: {property}")),
                Metrics::default(),
            )
        }
    };
    let elapsed_us = t0.elapsed().as_micros();
    (
        result,
        Metrics {
            inputs: 1,
            elapsed_us,
        },
    )
}

// ---------- proptest ----------

fn bytes_strategy() -> proptest::collection::VecStrategy<proptest::num::u8::Any> {
    use proptest::prelude::*;
    proptest::collection::vec(any::<u8>(), 0..32)
}

fn run_proptest_property(property: &str) -> Outcome {
    use proptest::prelude::*;
    use proptest::test_runner::{Config, TestCaseError, TestError, TestRunner};
    use std::panic::AssertUnwindSafe;
    if property == "All" {
        return run_all(run_proptest_property);
    }
    let counter = Arc::new(AtomicU64::new(0));
    let t0 = Instant::now();
    let mut runner = TestRunner::new(Config::default());
    let result: Result<(), String> = match property {
        "SplitnBounded" => {
            let c = counter.clone();
            let strat = (
                bytes_strategy(),
                proptest::collection::vec(any::<u8>(), 1..4),
                0u32..8,
            );
            runner
                .run(&strat, move |(h, n, k)| {
                    c.fetch_add(1, Ordering::Relaxed);
                    let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                        property_splitn_bounded(h.clone(), n.clone(), k)
                    }));
                    match res {
                        Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                        Ok(PropertyResult::Fail(_)) | Err(_) => {
                            Err(TestCaseError::fail(format!("({:?} {:?} {})", h, n, k)))
                        }
                    }
                })
                .map_err(|e| match e {
                    TestError::Fail(reason, _) => reason.to_string(),
                    other => other.to_string(),
                })
        }
        "CowPartialEqBstr" => {
            let c = counter.clone();
            let strat = (bytes_strategy(), bytes_strategy());
            runner
                .run(&strat, move |(a, b)| {
                    c.fetch_add(1, Ordering::Relaxed);
                    let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                        property_cow_partialeq_bstr(a.clone(), b.clone())
                    }));
                    match res {
                        Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                        Ok(PropertyResult::Fail(_)) | Err(_) => {
                            Err(TestCaseError::fail(format!("({:?} {:?})", a, b)))
                        }
                    }
                })
                .map_err(|e| match e {
                    TestError::Fail(reason, _) => reason.to_string(),
                    other => other.to_string(),
                })
        }
        "DebugValidFfFdPreserved" => {
            let c = counter.clone();
            let strat = (bytes_strategy(), bytes_strategy());
            runner
                .run(&strat, move |(p, s)| {
                    c.fetch_add(1, Ordering::Relaxed);
                    let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                        property_debug_valid_fffd_preserved(p.clone(), s.clone())
                    }));
                    match res {
                        Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                        Ok(PropertyResult::Fail(_)) | Err(_) => {
                            Err(TestCaseError::fail(format!("({:?} {:?})", p, s)))
                        }
                    }
                })
                .map_err(|e| match e {
                    TestError::Fail(reason, _) => reason.to_string(),
                    other => other.to_string(),
                })
        }
        "DebugAsciiControlHex" => {
            let c = counter.clone();
            runner
                .run(&any::<u8>(), move |byte| {
                    c.fetch_add(1, Ordering::Relaxed);
                    let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                        property_debug_ascii_control_hex(byte)
                    }));
                    match res {
                        Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                        Ok(PropertyResult::Fail(_)) | Err(_) => {
                            Err(TestCaseError::fail(format!("({})", byte)))
                        }
                    }
                })
                .map_err(|e| match e {
                    TestError::Fail(reason, _) => reason.to_string(),
                    other => other.to_string(),
                })
        }
        "DebugHexLowercase" => {
            let c = counter.clone();
            runner
                .run(&any::<u8>(), move |byte| {
                    c.fetch_add(1, Ordering::Relaxed);
                    let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                        property_debug_hex_lowercase(byte)
                    }));
                    match res {
                        Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                        Ok(PropertyResult::Fail(_)) | Err(_) => {
                            Err(TestCaseError::fail(format!("({})", byte)))
                        }
                    }
                })
                .map_err(|e| match e {
                    TestError::Fail(reason, _) => reason.to_string(),
                    other => other.to_string(),
                })
        }
        _ => {
            return (
                Err(format!("Unknown property for proptest: {property}")),
                Metrics::default(),
            )
        }
    };
    let elapsed_us = t0.elapsed().as_micros();
    let inputs = counter.load(Ordering::Relaxed);
    (result, Metrics { inputs, elapsed_us })
}

// ---------- quickcheck ----------

static QC_COUNTER: AtomicU64 = AtomicU64::new(0);

fn qc_splitn_bounded(h: Bytes, n: Bytes, k: u32) -> quickcheck::TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_splitn_bounded(h.0, n.0, k % 8) {
        PropertyResult::Pass => quickcheck::TestResult::passed(),
        PropertyResult::Discard => quickcheck::TestResult::discard(),
        PropertyResult::Fail(_) => quickcheck::TestResult::failed(),
    }
}

fn qc_cow_partialeq_bstr(a: Bytes, b: Bytes) -> quickcheck::TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_cow_partialeq_bstr(a.0, b.0) {
        PropertyResult::Pass => quickcheck::TestResult::passed(),
        PropertyResult::Discard => quickcheck::TestResult::discard(),
        PropertyResult::Fail(_) => quickcheck::TestResult::failed(),
    }
}

fn qc_debug_valid_fffd_preserved(p: Bytes, s: Bytes) -> quickcheck::TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_debug_valid_fffd_preserved(p.0, s.0) {
        PropertyResult::Pass => quickcheck::TestResult::passed(),
        PropertyResult::Discard => quickcheck::TestResult::discard(),
        PropertyResult::Fail(_) => quickcheck::TestResult::failed(),
    }
}

fn qc_debug_ascii_control_hex(byte: u8) -> quickcheck::TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_debug_ascii_control_hex(byte) {
        PropertyResult::Pass => quickcheck::TestResult::passed(),
        PropertyResult::Discard => quickcheck::TestResult::discard(),
        PropertyResult::Fail(_) => quickcheck::TestResult::failed(),
    }
}

fn qc_debug_hex_lowercase(byte: u8) -> quickcheck::TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_debug_hex_lowercase(byte) {
        PropertyResult::Pass => quickcheck::TestResult::passed(),
        PropertyResult::Discard => quickcheck::TestResult::discard(),
        PropertyResult::Fail(_) => quickcheck::TestResult::failed(),
    }
}

fn run_quickcheck_property(property: &str) -> Outcome {
    use quickcheck::{QuickCheck, ResultStatus};
    use std::time::Duration;
    if property == "All" {
        return run_all(run_quickcheck_property);
    }
    QC_COUNTER.store(0, Ordering::Relaxed);
    let t0 = Instant::now();
    let qc = || {
        QuickCheck::new()
            .tests(200)
            .max_tests(1000)
            .max_time(Duration::from_secs(86_400))
    };
    let result = match property {
        "SplitnBounded" => qc().quicktest(
            qc_splitn_bounded as fn(Bytes, Bytes, u32) -> quickcheck::TestResult,
        ),
        "CowPartialEqBstr" => qc().quicktest(
            qc_cow_partialeq_bstr as fn(Bytes, Bytes) -> quickcheck::TestResult,
        ),
        "DebugValidFfFdPreserved" => qc().quicktest(
            qc_debug_valid_fffd_preserved
                as fn(Bytes, Bytes) -> quickcheck::TestResult,
        ),
        "DebugAsciiControlHex" => qc().quicktest(
            qc_debug_ascii_control_hex as fn(u8) -> quickcheck::TestResult,
        ),
        "DebugHexLowercase" => qc().quicktest(
            qc_debug_hex_lowercase as fn(u8) -> quickcheck::TestResult,
        ),
        _ => {
            return (
                Err(format!("Unknown property for quickcheck: {property}")),
                Metrics::default(),
            )
        }
    };
    let elapsed_us = t0.elapsed().as_micros();
    let inputs = QC_COUNTER.load(Ordering::Relaxed);
    let status = match result.status {
        ResultStatus::Finished => Ok(()),
        ResultStatus::Failed { arguments } => Err(format!("({})", arguments.join(" "))),
        ResultStatus::Aborted { err } => Err(format!("aborted: {err:?}")),
        ResultStatus::TimedOut => Err("timed out".to_string()),
        ResultStatus::GaveUp => Err(format!(
            "gave up after {} tests, {} discarded",
            result.n_tests_passed, result.n_tests_discarded
        )),
    };
    (status, Metrics { inputs, elapsed_us })
}

// ---------- crabcheck ----------

static CC_COUNTER: AtomicU64 = AtomicU64::new(0);

fn cc_splitn_bounded((h, n, k): (Bytes, Bytes, u32)) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_splitn_bounded(h.0, n.0, k % 8) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn cc_cow_partialeq_bstr((a, b): (Bytes, Bytes)) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_cow_partialeq_bstr(a.0, b.0) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn cc_debug_valid_fffd_preserved((p, s): (Bytes, Bytes)) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_debug_valid_fffd_preserved(p.0, s.0) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn cc_debug_ascii_control_hex(byte: u8) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_debug_ascii_control_hex(byte) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn cc_debug_hex_lowercase(byte: u8) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_debug_hex_lowercase(byte) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn run_crabcheck_property(property: &str) -> Outcome {
    use crabcheck::quickcheck as cc;
    if property == "All" {
        return run_all(run_crabcheck_property);
    }
    CC_COUNTER.store(0, Ordering::Relaxed);
    let t0 = Instant::now();
    let cfg = cc::Config { tests: 200 };
    let result = match property {
        "SplitnBounded" => cc::quickcheck_with_config(
            cfg,
            cc_splitn_bounded as fn((Bytes, Bytes, u32)) -> Option<bool>,
        ),
        "CowPartialEqBstr" => cc::quickcheck_with_config(
            cfg,
            cc_cow_partialeq_bstr as fn((Bytes, Bytes)) -> Option<bool>,
        ),
        "DebugValidFfFdPreserved" => cc::quickcheck_with_config(
            cfg,
            cc_debug_valid_fffd_preserved as fn((Bytes, Bytes)) -> Option<bool>,
        ),
        "DebugAsciiControlHex" => cc::quickcheck_with_config(
            cfg,
            cc_debug_ascii_control_hex as fn(u8) -> Option<bool>,
        ),
        "DebugHexLowercase" => cc::quickcheck_with_config(
            cfg,
            cc_debug_hex_lowercase as fn(u8) -> Option<bool>,
        ),
        _ => {
            return (
                Err(format!("Unknown property for crabcheck: {property}")),
                Metrics::default(),
            )
        }
    };
    let elapsed_us = t0.elapsed().as_micros();
    let inputs = CC_COUNTER.load(Ordering::Relaxed);
    let status = match result.status {
        cc::ResultStatus::Finished => Ok(()),
        cc::ResultStatus::Failed { arguments } => Err(format!("({})", arguments.join(" "))),
        cc::ResultStatus::TimedOut => Err("timed out".to_string()),
        cc::ResultStatus::GaveUp => Err(format!(
            "gave up: passed={}, discarded={}",
            result.passed, result.discarded
        )),
        cc::ResultStatus::Aborted { error } => Err(format!("aborted: {error}")),
    };
    (status, Metrics { inputs, elapsed_us })
}

// ---------- hegel ----------

static HG_COUNTER: AtomicU64 = AtomicU64::new(0);

fn hegel_settings() -> hegel::Settings {
    use hegel::HealthCheck;
    hegel::Settings::new()
        .test_cases(200)
        .suppress_health_check(HealthCheck::all())
}

fn draw_bytes(tc: &hegel::TestCase, max_len: usize) -> Vec<u8> {
    use hegel::generators as hgen;
    let len = tc.draw(
        hgen::integers::<usize>()
            .min_value(0)
            .max_value(max_len),
    );
    let mut out = Vec::with_capacity(len);
    for _ in 0..len {
        out.push(tc.draw(hgen::integers::<u8>()));
    }
    out
}

fn run_hegel_property(property: &str) -> Outcome {
    use hegel::{generators as hgen, Hegel, TestCase};
    use std::panic::AssertUnwindSafe;
    if property == "All" {
        return run_all(run_hegel_property);
    }
    HG_COUNTER.store(0, Ordering::Relaxed);
    let t0 = Instant::now();
    let settings = hegel_settings();
    let run_result = std::panic::catch_unwind(AssertUnwindSafe(|| match property {
        "SplitnBounded" => {
            Hegel::new(|tc: TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let haystack = draw_bytes(&tc, 24);
                let mut needle = draw_bytes(&tc, 4);
                if needle.is_empty() {
                    needle.push(b':');
                }
                let k: u32 = tc.draw(hgen::integers::<u32>().min_value(0).max_value(6));
                let cex = format!("({:?} {:?} {})", haystack, needle, k);
                let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    property_splitn_bounded(haystack.clone(), needle.clone(), k)
                }));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("{cex}"),
                }
            })
            .settings(settings.clone())
            .run();
        }
        "CowPartialEqBstr" => {
            Hegel::new(|tc: TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let a = draw_bytes(&tc, 24);
                let b = draw_bytes(&tc, 24);
                let cex = format!("({:?} {:?})", a, b);
                let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    property_cow_partialeq_bstr(a.clone(), b.clone())
                }));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("{cex}"),
                }
            })
            .settings(settings.clone())
            .run();
        }
        "DebugValidFfFdPreserved" => {
            Hegel::new(|tc: TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let p = draw_bytes(&tc, 12);
                let s = draw_bytes(&tc, 12);
                let cex = format!("({:?} {:?})", p, s);
                let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    property_debug_valid_fffd_preserved(p.clone(), s.clone())
                }));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("{cex}"),
                }
            })
            .settings(settings.clone())
            .run();
        }
        "DebugAsciiControlHex" => {
            Hegel::new(|tc: TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let byte: u8 = tc.draw(hgen::integers::<u8>());
                let cex = format!("({})", byte);
                let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    property_debug_ascii_control_hex(byte)
                }));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("{cex}"),
                }
            })
            .settings(settings.clone())
            .run();
        }
        "DebugHexLowercase" => {
            Hegel::new(|tc: TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let byte: u8 = tc.draw(hgen::integers::<u8>());
                let cex = format!("({})", byte);
                let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    property_debug_hex_lowercase(byte)
                }));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("{cex}"),
                }
            })
            .settings(settings.clone())
            .run();
        }
        _ => panic!("__unknown_property:{property}"),
    }));
    let elapsed_us = t0.elapsed().as_micros();
    let inputs = HG_COUNTER.load(Ordering::Relaxed);
    let status = match run_result {
        Ok(()) => Ok(()),
        Err(e) => {
            let msg = if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "hegel panicked with non-string payload".to_string()
            };
            if let Some(rest) = msg.strip_prefix("__unknown_property:") {
                return (
                    Err(format!("Unknown property for hegel: {rest}")),
                    Metrics::default(),
                );
            }
            Err(msg
                .strip_prefix("Property test failed: ")
                .unwrap_or(&msg)
                .to_string())
        }
    };
    (status, Metrics { inputs, elapsed_us })
}

// ---------- dispatch + main ----------

fn run(tool: &str, property: &str) -> Outcome {
    match tool {
        "etna" => run_etna_property(property),
        "proptest" => run_proptest_property(property),
        "quickcheck" => run_quickcheck_property(property),
        "crabcheck" => run_crabcheck_property(property),
        "hegel" => run_hegel_property(property),
        _ => (
            Err(format!("Unknown tool: {tool}")),
            Metrics::default(),
        ),
    }
}

fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn emit_json(
    tool: &str,
    property: &str,
    status: &str,
    metrics: Metrics,
    counterexample: Option<&str>,
    error: Option<&str>,
) {
    let cex = counterexample.map_or("null".to_string(), json_str);
    let err = error.map_or("null".to_string(), json_str);
    println!(
        "{{\"status\":{},\"tests\":{},\"discards\":0,\"time\":{},\"counterexample\":{},\"error\":{},\"tool\":{},\"property\":{}}}",
        json_str(status),
        metrics.inputs,
        json_str(&format!("{}us", metrics.elapsed_us)),
        cex,
        err,
        json_str(tool),
        json_str(property),
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <tool> <property>", args[0]);
        eprintln!("Tools: etna | proptest | quickcheck | crabcheck | hegel");
        eprintln!(
            "Properties: SplitnBounded | CowPartialEqBstr | DebugValidFfFdPreserved | DebugAsciiControlHex | DebugHexLowercase | All"
        );
        std::process::exit(2);
    }
    let (tool, property) = (args[1].as_str(), args[2].as_str());

    // Silence library-under-test panic noise (frameworks catch panics, but
    // the default hook still prints "thread 'main' panicked at ..." to stderr).
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let caught =
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run(tool, property)));
    std::panic::set_hook(previous_hook);

    let (result, metrics) = match caught {
        Ok(outcome) => outcome,
        Err(payload) => {
            let msg = if let Some(s) = payload.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = payload.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "panic with non-string payload".to_string()
            };
            emit_json(
                tool,
                property,
                "aborted",
                Metrics::default(),
                None,
                Some(&format!("adapter panic: {msg}")),
            );
            return;
        }
    };

    match result {
        Ok(()) => emit_json(tool, property, "passed", metrics, None, None),
        Err(msg) => emit_json(tool, property, "failed", metrics, Some(&msg), None),
    }
}
