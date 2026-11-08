//! The `payment` suite: one test per scenario in `features/suites/payment.feature`.
//!
//! The proofs are checked by the Lean kernel when `just vv` runs
//! `prismpm verify` on `app/`. What these tests hold fixed is the claim each
//! proof is about: the property's exact condition and the theorem that states
//! no request breaks it. A weakened condition would still verify, and would no
//! longer be the claim the row makes.

use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/conformance is two below the root")
        .to_path_buf()
}

fn proofs() -> String {
    std::fs::read_to_string(root().join("app/src/PaymentApprovalProofs.lex.tex"))
        .expect("the proofs module")
}

/// The body of the top-level declaration named `name`, up to its closing brace.
fn declaration<'a>(model: &'a str, name: &str) -> &'a str {
    let start = model
        .find(&format!("\n  name: {name},\n"))
        .unwrap_or_else(|| panic!("{name} is not declared"));
    let end = model[start..]
        .find("\n}\n")
        .unwrap_or_else(|| panic!("{name} is not closed"));
    &model[start..start + end]
}

/// `PA-01`: The app never approves a clerk's payment whose amount exceeds the limit
#[test]
fn payment_pa_01() {
    let model = proofs();
    let breaks = declaration(&model, "breaksInv001");
    assert!(
        breaks.contains("((project request role) =? (PaymentApproval::roleClerk))")
            && breaks.contains("((project request limit) <? (project request amount))")
            && breaks.contains(
                "((PaymentApproval::judge request) =? (PaymentApproval::answerApproved))"
            ),
        "PA-01's condition changed:\n{breaks}"
    );
    let holds = declaration(&model, "inv001Holds");
    assert!(
        holds.contains("statement: ((breaksInv001 request) = false),"),
        "PA-01 must state that no request breaks it:\n{holds}"
    );
}

/// `PA-02`: The app always approves a manager's payment
#[test]
fn payment_pa_02() {
    let model = proofs();
    let breaks = declaration(&model, "breaksInv002");
    assert!(
        breaks.contains("((project request role) =? (PaymentApproval::roleManager))")
            && breaks.contains(
                "(not ((PaymentApproval::judge request) =? (PaymentApproval::answerApproved)))"
            ),
        "PA-02's condition changed:\n{breaks}"
    );
    let holds = declaration(&model, "inv002Holds");
    assert!(
        holds.contains("statement: ((breaksInv002 request) = false),"),
        "PA-02 must state that no request breaks it:\n{holds}"
    );
}
