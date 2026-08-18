//! Transcendental functions that give the same bits on every machine.
//!
//! # Why this file exists
//!
//! Determinism is the first iron rule, and both determinism tests build their
//! two worlds *in one process against one libm*. Neither can ever see the
//! failure that actually threatens us: the same seed producing a different
//! history on a different machine. The endgame in SPEC section 13 is Ridge
//! developing on Windows while a Proxmox box runs the suite nightly, and that
//! is precisely the arrangement where a libm difference surfaces as a red test
//! nobody can reproduce locally.
//!
//! IEEE 754 pins `+`, `-`, `*`, `/` and `sqrt` to the last bit. It says nothing
//! whatever about `exp`, `ln` or `pow`. Those are library routines, and glibc,
//! musl, Apple's libm and Microsoft's UCRT do not agree on them — glibc does not
//! even agree with *itself* across versions, because its `exp` was rewritten in
//! 2.27 and again in 2.28. A one-ulp difference in a single `exp` on month three
//! is a different unlock order by month forty and a different century.
//!
//! So the sim does not call the platform's transcendentals at all. Everything
//! below is built from IEEE-exact primitives only: `+`, `-`, `*`, `/`,
//! comparisons, and reinterpreting the bits of an `f64`. Every one of those is
//! specified to the bit by the standard, so the results here are identical on
//! any conforming target.
//!
//! Two things could still break that, and both are already true of this build:
//!
//!  * **x87 excess precision.** A 32-bit x86 target without SSE2 computes `f64`
//!    in 80-bit registers and rounds unpredictably. Every target we build for
//!    (`x86_64`, `aarch64`) uses SSE2/NEON, where `f64` arithmetic is genuinely
//!    64-bit.
//!  * **Fused multiply-add contraction.** An FMA computes `a*b+c` with one
//!    rounding instead of two and so gives different bits. Rust does not permit
//!    the backend to contract `a*b+c` into an FMA — unlike C compilers at
//!    `-ffp-contract=fast` — so the expressions below round exactly where they
//!    are written.
//!
//! # Why not just replace the shapes with rational functions
//!
//! That was the other option, and it is the one PLAN 1.3 suggests: `saturate`
//! is a soft-cap *shape*, not a physical law, and `x/(1 + x/cap)` has the same
//! limit and the same slope at the origin. It would have been shorter than this
//! file.
//!
//! It was rejected because it costs calibration for nothing. Any replacement
//! moves the golden hash — the sim is chaotic, so even a half-ulp difference in
//! month one becomes a different timeline by 2025 — which means "the hash moves"
//! is not a cost that distinguishes the options. What distinguishes them is how
//! far the *model* moves. A Padé-style rational differs from `1 - e^(-u)` by
//! more than a tenth at `u = 3`, so every soft cap in the tech tree would need
//! re-tuning and every calibration threshold re-argued. The routines here agree
//! with a correct libm to within an ulp or two, so the model is unchanged and
//! only the seed-level noise moves. Exactness at no design cost beats exactness
//! bought with a retune.
//!
//! # Accuracy
//!
//! Not the point, but worth stating: `exp` and `ln` are within a couple of ulps
//! of correctly rounded across the range the sim uses, and `powf` within a few,
//! being `exp(y * ln x)`. The tests at the bottom check that against the
//! platform libm — as a sanity check on the maths, never as the definition. The
//! *definition* is the pinned bit patterns, which are what a future change to
//! these functions has to trip over.

// Clippy wants `f64::consts::LOG2_E` and `SQRT_2` here, and the literals
// shortened. Both are refused, in this file only. The constants below are not
// mathematical constants that happen to be written out — they are the
// implementation of `exp` and `ln`, chosen for their bit patterns: `LN2_HI` is
// split precisely so its low 20 significand bits are clear, and the test table
// pairs each literal with the exact `u64` it must produce, down to recording
// where we sit one ulp from libm. The module header says it in one line: "The
// *definition* is the pinned bit patterns." Spelling those as named constants
// or trimming their digits is a style change to the one file where the digits
// are the specification, and determinism is the first iron rule.
#![allow(clippy::approx_constant, clippy::excessive_precision)]

/// `ln(2)` split so that `LN2_HI` has its low 20 significand bits clear. That
/// makes `k * LN2_HI` exact for every `k` we can produce, so the argument
/// reduction below loses nothing to rounding.
const LN2_HI: f64 = 6.931_471_803_691_238_164_9e-1;
/// The rest of `ln(2)`, to about 1e-27 in total.
const LN2_LO: f64 = 1.908_214_929_270_587_700_02e-10;
/// `1 / ln(2)`.
const INV_LN2: f64 = 1.442_695_040_888_963_387e0;

/// Above this `exp` overflows to infinity: `ln(f64::MAX)`.
const EXP_MAX: f64 = 709.782_712_893_384_0;
/// Below this `exp` underflows to zero: a little past `ln(f64::MIN_POSITIVE)`
/// less 53 doublings.
const EXP_MIN: f64 = -745.133_219_101_941_2;

/// `e^x`, identically on every platform.
///
/// Range reduction `x = k·ln2 + r` with `|r| ≤ ln2/2`, then a degree-13 Taylor
/// series for `e^r` — the first omitted term is `r^14/14!`, under 5e-18 relative
/// — then scaling by `2^k`, which is exact.
pub fn exp(x: f64) -> f64 {
    if x.is_nan() {
        return x;
    }
    if x >= EXP_MAX {
        return f64::INFINITY;
    }
    if x <= EXP_MIN {
        return 0.0;
    }

    // k = round-to-nearest(x / ln2). Written as a truncating cast of x±0.5
    // rather than `.round()` so it is plain arithmetic; the magnitude is at most
    // 1075 so the cast cannot trap or saturate.
    let k = (x * INV_LN2 + if x >= 0.0 { 0.5 } else { -0.5 }) as i32;
    let kf = k as f64;
    // Two-step subtraction: the first product is exact, so all the rounding
    // error lives in the tiny second term.
    let r = (x - kf * LN2_HI) - kf * LN2_LO;

    let er = exp_small(r);
    // The sim never comes near either extreme — every `exp` it asks for is a
    // soft cap on a small negative number — but an edge case that is merely
    // unlikely is still an edge case that has to be deterministic, so the
    // scaling is split rather than allowed to run off the end of the exponent
    // field in a single step.
    if k > 1023 {
        return er * pow2i(k - 64) * pow2i(64);
    }
    if k < -1022 {
        // Gradual underflow: split so the only rounding that can lose bits is
        // the last one.
        return er * pow2i(k + 106) * pow2i(-106);
    }
    er * pow2i(k)
}

/// `e^r` for `|r| ≤ ln(2)/2`, by Horner on the Taylor series.
fn exp_small(r: f64) -> f64 {
    // 1 + r(1 + r/2(1 + r/3(1 + ...))) — written as literal reciprocals of the
    // factorials so the constants can be read off by eye. Every one of these
    // divisions is const-folded by rustc's soft-float evaluator, which is
    // exactly-rounded and platform-independent, so `1.0/6.0` here is the same
    // bit pattern everywhere.
    let mut s = 1.0 / 13.0;
    s = 1.0 / 12.0 * (1.0 + r * s);
    s = 1.0 / 11.0 * (1.0 + r * s);
    s = 1.0 / 10.0 * (1.0 + r * s);
    s = 1.0 / 9.0 * (1.0 + r * s);
    s = 1.0 / 8.0 * (1.0 + r * s);
    s = 1.0 / 7.0 * (1.0 + r * s);
    s = 1.0 / 6.0 * (1.0 + r * s);
    s = 1.0 / 5.0 * (1.0 + r * s);
    s = 1.0 / 4.0 * (1.0 + r * s);
    s = 1.0 / 3.0 * (1.0 + r * s);
    s = 1.0 / 2.0 * (1.0 + r * s);
    1.0 + r * (1.0 + r * s)
}

/// `2^k` for an integer `k`, built straight out of the exponent field. Exact,
/// and the only place in this file that touches bits for arithmetic rather than
/// for decomposition.
fn pow2i(k: i32) -> f64 {
    if k > 1023 {
        return f64::INFINITY;
    }
    if k >= -1022 {
        // Normal: bias the exponent and leave the significand at 1.0.
        return f64::from_bits(((k + 1023) as u64) << 52);
    }
    if k < -1074 {
        return 0.0;
    }
    // Subnormal reach. 2^-1074 is the smallest subnormal — bit pattern 1 — so
    // every power below the normal floor is a single set bit in the significand.
    f64::from_bits(1u64 << (k + 1074))
}

/// `sqrt(2)`, the split point for `ln`'s range reduction.
const SQRT2: f64 = 1.414_213_562_373_095_1;

/// Natural log, identically on every platform.
///
/// Decompose `x = m · 2^e` with `m ∈ [√2/2, √2)`, then
/// `ln(m) = 2·atanh(z)` with `z = (m-1)/(m+1)`, `|z| ≤ 0.1716`. The series in
/// `z²` converges by a factor of ~34 a term, so eleven terms take the error
/// below an ulp.
pub fn ln(x: f64) -> f64 {
    if x.is_nan() {
        return x;
    }
    if x < 0.0 {
        return f64::NAN;
    }
    if x == 0.0 {
        return f64::NEG_INFINITY;
    }
    if x == f64::INFINITY {
        return x;
    }

    let mut bits = x.to_bits();
    let mut e = ((bits >> 52) & 0x7ff) as i32;
    if e == 0 {
        // Subnormal. Scale by 2^54 into the normal range and pay it back after.
        let scaled = x * f64::from_bits(0x4350_0000_0000_0000); // 2^54
        bits = scaled.to_bits();
        e = ((bits >> 52) & 0x7ff) as i32 - 54;
    }
    e -= 1023;

    // Significand as a value in [1, 2).
    let mut m = f64::from_bits((bits & 0x000f_ffff_ffff_ffff) | 0x3ff0_0000_0000_0000);
    if m > SQRT2 {
        m *= 0.5;
        e += 1;
    }

    let z = (m - 1.0) / (m + 1.0);
    let z2 = z * z;
    // 2z(1 + z²/3 + z⁴/5 + ... + z²⁰/21)
    let mut s = 1.0 / 21.0;
    s = 1.0 / 19.0 + z2 * s;
    s = 1.0 / 17.0 + z2 * s;
    s = 1.0 / 15.0 + z2 * s;
    s = 1.0 / 13.0 + z2 * s;
    s = 1.0 / 11.0 + z2 * s;
    s = 1.0 / 9.0 + z2 * s;
    s = 1.0 / 7.0 + z2 * s;
    s = 1.0 / 5.0 + z2 * s;
    s = 1.0 / 3.0 + z2 * s;
    let ln_m = 2.0 * z * (1.0 + z2 * s);

    // The high/low split of ln2 again, so the exponent term does not swamp the
    // significand term's accuracy when |e| is large.
    let ef = e as f64;
    ef * LN2_HI + (ef * LN2_LO + ln_m)
}

/// `x^y`, identically on every platform, as `exp(y · ln x)`.
///
/// Only the cases the sim actually reaches are given exact answers: `x ≥ 0` and
/// a finite `y`. A negative base returns NaN rather than pretending to know what
/// a fractional power of it means.
pub fn powf(x: f64, y: f64) -> f64 {
    if y == 0.0 {
        return 1.0;
    }
    if x == 1.0 {
        return 1.0;
    }
    if x.is_nan() || y.is_nan() {
        return f64::NAN;
    }
    if x < 0.0 {
        return f64::NAN;
    }
    if x == 0.0 {
        return if y > 0.0 { 0.0 } else { f64::INFINITY };
    }
    exp(y * ln(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Regenerates the pinned tables below. Not a test — an instrument, for the
    /// day one of these functions is legitimately rewritten.
    ///
    ///     cargo test -p spheres-sim --lib dump_pins -- --ignored --nocapture
    #[test]
    #[ignore]
    fn dump_pins() {
        println!("--- exp");
        for x in [0.0f64, 1.0, -1.0, 0.5, -0.34657359027997264, -2.0, -0.75, -7.5, 20.0, -30.0, 709.0, -745.2] {
            println!("({:?}, {:#018x}), // libm {:#018x}", x, exp(x).to_bits(), x.exp().to_bits());
        }
        println!("--- ln");
        for x in [1.0f64, 2.0, 0.5, 1.4142135623730951, 1.5, 0.7, 0.005, 10.0, 1e-9, 1e300] {
            println!("({:?}, {:#018x}), // libm {:#018x}", x, ln(x).to_bits(), x.ln().to_bits());
        }
        println!("--- powf");
        for (x, y) in [(0.20f64, 0.70f64), (0.85, 0.70), (1.45, 0.55), (0.30, 1.35), (1.0, 0.70), (0.0, 0.70)] {
            println!("({:?}, {:?}, {:#018x}), // libm {:#018x}", x, y, powf(x, y).to_bits(), x.powf(y).to_bits());
        }
    }

    /// The pin. These are the bit patterns the sim's timeline is built on, at
    /// inputs chosen to cover every branch above: the identities, both sides of
    /// the `exp` range reduction, the `√2` split in `ln`, the arguments the tech
    /// tree and the war model actually pass, and the extremes.
    ///
    /// If one of these changes, the golden hash changed with it and it was not
    /// an accident of the model — it was these functions. Re-pin both together
    /// or neither.
    #[test]
    fn transcendentals_are_pinned_to_the_bit() {
        // The trailing comment on each line is what this machine's libm returned
        // when the pin was taken, so the size of the disagreement is on the
        // record: never more than one ulp, and identical in most cases.
        let exp_cases: [(f64, u64); 12] = [
            (0.0, 0x3ff0000000000000),                  // libm 0x3ff0000000000000
            (1.0, 0x4005bf0a8b14576a),                  // libm 0x4005bf0a8b145769
            (-1.0, 0x3fd78b56362cef38),                 // libm 0x3fd78b56362cef38
            (0.5, 0x3ffa61298e1e069c),                  // libm 0x3ffa61298e1e069c
            (-0.34657359027997264, 0x3fe6a09e667f3bcc), // libm 0x3fe6a09e667f3bcd
            (-2.0, 0x3fc152aaa3bf81cc),                 // libm 0x3fc152aaa3bf81cc
            (-0.75, 0x3fde3b40ebefcd7e),                // libm 0x3fde3b40ebefcd7e
            (-7.5, 0x3f421f9ba40f31d5),                 // libm 0x3f421f9ba40f31d5
            (20.0, 0x41bceb088b68e804),                 // libm 0x41bceb088b68e804
            (-30.0, 0x3d3a56e0c2ac7f75),                // libm 0x3d3a56e0c2ac7f75
            (709.0, 0x7fdd422d2be5dc9b),                // libm 0x7fdd422d2be5dc9b
            (-745.2, 0x0000000000000000),               // libm 0x0000000000000000
        ];
        for (x, bits) in exp_cases {
            assert_eq!(
                exp(x).to_bits(),
                bits,
                "exp({}) = {:#018x}, expected {:#018x}",
                x,
                exp(x).to_bits(),
                bits
            );
        }

        let ln_cases: [(f64, u64); 10] = [
            (1.0, 0x0000000000000000),                  // libm 0x0000000000000000
            (2.0, 0x3fe62e42fefa39ef),                  // libm 0x3fe62e42fefa39ef
            (0.5, 0xbfe62e42fefa39ef),                  // libm 0xbfe62e42fefa39ef
            (1.4142135623730951, 0x3fd62e42fefa39f1),   // the √2 split point; libm 0x3fd62e42fefa39f0
            (1.5, 0x3fd9f323ecbf984c),                  // libm 0x3fd9f323ecbf984c
            (0.7, 0xbfd6d3c324e13f4f),                  // libm 0xbfd6d3c324e13f50
            (0.005, 0xc015317a1b949c53),                // libm 0xc015317a1b949c53
            (10.0, 0x40026bb1bbb55516),                 // libm 0x40026bb1bbb55516
            (1e-9, 0xc034b927f32bffb8),                 // libm 0xc034b927f32bffb8, subnormal-free path
            (1e300, 0x4085963447f87fb5),                // libm 0x4085963447f87fb5
        ];
        for (x, bits) in ln_cases {
            assert_eq!(
                ln(x).to_bits(),
                bits,
                "ln({}) = {:#018x}, expected {:#018x}",
                x,
                ln(x).to_bits(),
                bits
            );
        }

        // 0.70 is the diffusion-reach exponent in `tech::effective_cost`, 0.55
        // the investment-intensity exponent in `economy::tick`, and 1.35 the
        // tacit-knowledge exponent `tech::TACIT`. These are the three the sim
        // actually asks for.
        let powf_cases: [(f64, f64, u64); 6] = [
            (0.20, 0.70, 0x3fd4be914a0ac752), // libm 0x3fd4be914a0ac751
            (0.85, 0.70, 0x3fec8f1b9d5a3883), // libm 0x3fec8f1b9d5a3883
            (1.45, 0.55, 0x3ff3a0b9c05b72b6), // libm 0x3ff3a0b9c05b72b6
            (0.30, 1.35, 0x3fc9320ee1c79807), // libm 0x3fc9320ee1c79807
            (1.0, 0.70, 0x3ff0000000000000),  // libm 0x3ff0000000000000
            (0.0, 0.70, 0x0000000000000000),  // libm 0x0000000000000000
        ];
        for (x, y, bits) in powf_cases {
            assert_eq!(
                powf(x, y).to_bits(),
                bits,
                "powf({}, {}) = {:#018x}, expected {:#018x}",
                x,
                y,
                powf(x, y).to_bits(),
                bits
            );
        }
    }

    /// The guard that keeps this module from being quietly bypassed.
    ///
    /// Everything above is worth nothing if the next person to touch the growth
    /// model writes `x.exp()` because it is shorter. Nothing else in the crate
    /// can catch that: the sim would still be perfectly deterministic on the
    /// machine it was written on, and would stay that way until the Proxmox box
    /// disagreed with Windows about a timeline neither could reproduce for the
    /// other.
    ///
    /// So this reads the source. Crude, and it earns its crudeness — it is the
    /// only thing standing between a one-character convenience and a class of
    /// bug that takes days to find. `sqrt` is absent from the list on purpose:
    /// IEEE 754 specifies it to the last bit, so it is safe.
    ///
    /// `exact.rs` itself is not scanned, since it is where the platform libm is
    /// legitimately called — by the tests, as a sanity check on the maths.
    #[test]
    fn nothing_in_the_sim_calls_the_platform_transcendentals() {
        const BANNED: [&str; 12] = [
            ".exp()", ".ln()", ".powf(", ".log(", ".log2()", ".log10()", ".exp2()",
            ".exp_m1()", ".ln_1p()", ".cbrt()", ".sin()", ".cos()",
        ];
        // The file list used to be eight hand-written include_str! entries,
        // which is a guard that quietly stops guarding: government.rs,
        // stratagems.rs, data/mod.rs and the eight tech domain files were all
        // outside it, and with ~80 nations about to arrive across ten parallel
        // sessions the list would have drifted further every week. It now walks
        // the source tree instead, so a module cannot be added without being
        // scanned. Reading the directory is fine here — this is a test, it
        // touches no sim state, and determinism is a property of the tick loop
        // rather than of the test harness.
        fn rust_files(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
            let entries = std::fs::read_dir(dir).expect("the crate source must be readable");
            let mut paths: Vec<std::path::PathBuf> =
                entries.map(|e| e.expect("readable entry").path()).collect();
            // Sorted so a failure names the same file every run.
            paths.sort();
            for p in paths {
                if p.is_dir() {
                    rust_files(&p, out);
                } else if p.extension().is_some_and(|e| e == "rs") {
                    out.push(p);
                }
            }
        }
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut files = vec![];
        rust_files(&root, &mut files);
        assert!(
            files.len() >= 12,
            "the source scan found only {} files under {} — it is not looking where it thinks",
            files.len(),
            root.display()
        );
        let sources: Vec<(String, String)> = files
            .iter()
            // exact.rs is where the platform libm is legitimately called, by the
            // tests below, as a sanity check on the maths.
            .filter(|p| p.file_name().is_none_or(|n| n != "exact.rs"))
            .map(|p| {
                let name = p
                    .strip_prefix(&root)
                    .unwrap_or(p)
                    .to_string_lossy()
                    .replace('\\', "/");
                (name, std::fs::read_to_string(p).expect("readable source"))
            })
            .collect();

        for (name, src) in &sources {
            for (i, line) in src.lines().enumerate() {
                for pat in BANNED {
                    assert!(
                        !line.contains(pat),
                        "{}:{} calls the platform libm — `{}` in `{}`. \
                         Use crate::exact instead; see the header of exact.rs.",
                        name,
                        i + 1,
                        pat,
                        line.trim()
                    );
                }
            }
        }
    }

    /// The constants the range reductions stand on. Each one is stated in the
    /// source as a decimal literal, and a mistyped digit would show up as a
    /// quiet accuracy loss rather than a failure, so they are checked against
    /// the values they are supposed to be.
    #[test]
    fn the_range_reduction_constants_are_what_they_claim() {
        // The split of ln2 has to be exact when the halves are added back, or
        // the argument reduction leaks error into every `exp`.
        assert_eq!(LN2_HI + LN2_LO, std::f64::consts::LN_2);
        // ...and the high half must have its low bits clear, which is what makes
        // `k * LN2_HI` exact for every k the reduction can produce.
        assert_eq!(LN2_HI.to_bits() & 0x1f_ffff, 0);
        assert_eq!(INV_LN2, std::f64::consts::LOG2_E);
        assert_eq!(SQRT2, std::f64::consts::SQRT_2);
        // The overflow and underflow thresholds must sit exactly where f64 runs
        // out, or `exp` either returns infinity for a representable answer or
        // grinds through the series for one that cannot exist.
        assert_eq!(EXP_MAX, f64::MAX.ln());
        assert!(exp(709.0).is_finite());
        assert!(exp(710.0).is_infinite());
        // Gradual underflow: these are subnormal results, and they must come out
        // as subnormals rather than as zero.
        assert_eq!(exp(-745.0), 5e-324);
        assert_eq!(exp(-744.0), 1e-323);
        assert!(exp(-746.0) == 0.0);
        // And `ln` has to survive being handed one back.
        assert!((ln(5e-324) - (5e-324f64).ln()).abs() < 1e-12);
    }

    /// A sanity check on the maths, not a definition. The platform libm is
    /// allowed to disagree in the last couple of bits — that disagreement is the
    /// entire reason this module exists — but a bug in the series above would
    /// show up here as a gross error rather than an ulp.
    #[test]
    fn agrees_with_the_platform_libm_to_a_few_ulps() {
        let mut x = -60.0f64;
        while x < 60.0 {
            let a = exp(x);
            let b = x.exp();
            assert!(
                (a - b).abs() <= 4.0 * b * f64::EPSILON,
                "exp({}) ours {:e} libm {:e}",
                x,
                a,
                b
            );
            x += 0.013;
        }

        let mut v = 1e-9f64;
        while v < 1e9 {
            let a = ln(v);
            let b = v.ln();
            assert!(
                (a - b).abs() <= 4.0 * b.abs().max(1.0) * f64::EPSILON,
                "ln({}) ours {:e} libm {:e}",
                v,
                a,
                b
            );
            v *= 1.7;
        }

        for &(x, y) in &[
            (0.0001, 0.70),
            (0.5, 0.70),
            (0.999, 0.70),
            (1.6, 0.55),
            (0.4, 1.35),
            (3.0, 0.55),
        ] {
            let a = powf(x, y);
            let b = x.powf(y);
            assert!(
                (a - b).abs() <= 8.0 * b * f64::EPSILON,
                "powf({}, {}) ours {:e} libm {:e}",
                x,
                y,
                a,
                b
            );
        }
    }

    /// The identities the sim leans on, stated as equalities rather than
    /// tolerances because they must hold exactly for the model to behave.
    #[test]
    fn the_identities_are_exact() {
        assert_eq!(exp(0.0), 1.0);
        assert_eq!(ln(1.0), 0.0);
        assert_eq!(powf(1.0, 0.7), 1.0);
        assert_eq!(powf(0.0, 0.7), 0.0);
        // saturate(0) must be exactly 0, or every nation gains a rounding
        // artefact of a bonus it has not earned.
        assert_eq!(1.0 - exp(-0.0), 0.0);
        // Monotone: the soft caps are only meaningful if more input is never
        // worth less output.
        let mut prev = f64::NEG_INFINITY;
        let mut u = 0.0f64;
        while u < 12.0 {
            let v = 1.0 - exp(-u);
            assert!(v >= prev, "saturate lost monotonicity at u={}", u);
            prev = v;
            u += 0.001;
        }
    }
}
