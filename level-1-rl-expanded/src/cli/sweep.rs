//! The sweep experiment: score all eleven grid policies and report the best.
//!
//! Question: which boldness, from 0.0 to 1.0 in steps of 0.1, earns the most?
//! Example: with the default horizon 5 and discount 0.95, boldness 0.3 wins with 3.103947.
//! Predict: at discount 0 all eleven scores tie. Which boldness is reported, and why?

use std::io::{self, Write};

use crate::model::DatingWorld;
use crate::planning::{best_policy, evaluate_policy_grid};

pub(super) fn print(
    out: &mut impl Write,
    world: &DatingWorld,
    horizon: usize,
    discount: f64,
) -> io::Result<()> {
    writeln!(out, "Horizon: {horizon} transitions | Discount: {discount}")?;
    writeln!(out, "boldness   expected return")?;
    let evaluations = evaluate_policy_grid(world, horizon, discount);
    for evaluation in &evaluations {
        let boldness = evaluation.policy.boldness();
        let score = evaluation.expected_return;
        writeln!(out, "{boldness:>8.1}   {score:>15.6}")?;
    }
    if let Some(best) = best_policy(&evaluations) {
        let boldness = best.policy.boldness();
        let score = best.expected_return;
        writeln!(
            out,
            "Best on this grid: boldness={boldness:.1}, expected return={score:.6}"
        )?;
    }
    writeln!(out, "Scores within 1e-12 tie and keep the lower boldness. This searches only the 11 displayed candidates.")?;
    writeln!(
        out,
        "Next experiment: compare --boldness 0.3, or use --help for smaller steps."
    )?;
    Ok(())
}
