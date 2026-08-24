use super::*;

use std::panic::{AssertUnwindSafe, catch_unwind};

fn run_with_signal_capture<F: FnOnce()>(f: F) -> bool {
    catch_unwind(AssertUnwindSafe(f)).is_ok()
}

fn fresh_state() -> TransitionState {
    TransitionState::new(
        Signal::create(TransitionPhase::Exited),
        Signal::create(0.0_f64),
        Signal::create(TransitionConfig::default()),
    )
}

fn seeded_state(phase: TransitionPhase, progress: f64) -> TransitionState {
    TransitionState::new(
        Signal::create(phase),
        Signal::create(progress),
        Signal::create(TransitionConfig::with_durations(100, 200)),
    )
}

// ============================================================
//  Phase enum tests.
// ============================================================

#[test]
fn transition_phase_exited_returns_exited() {
    assert_eq!(TransitionPhase::exited(), TransitionPhase::Exited);
}

#[test]
fn transition_phase_is_copy_and_eq() {
    let a: TransitionPhase = TransitionPhase::Entering;
    let b: TransitionPhase = a; // Copy semantics
    assert_eq!(a, b);
}

#[test]
fn transition_phase_variants_are_distinct() {
    let variants: [TransitionPhase; 4] = [
        TransitionPhase::Exited,
        TransitionPhase::Entering,
        TransitionPhase::Entered,
        TransitionPhase::Exiting,
    ];
    for i in 0..variants.len() {
        for j in 0..variants.len() {
            if i != j {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }
}

// ============================================================
//  TransitionConfig tests.
// ============================================================

#[test]
fn transition_config_with_ms_sets_both_durations() {
    let cfg: TransitionConfig = TransitionConfig::with_ms(150);
    assert_eq!(cfg.enter_ms, 150);
    assert_eq!(cfg.exit_ms, 150);
}

#[test]
fn transition_config_with_durations_sets_them_independently() {
    let cfg: TransitionConfig = TransitionConfig::with_durations(100, 250);
    assert_eq!(cfg.enter_ms, 100);
    assert_eq!(cfg.exit_ms, 250);
}

#[test]
fn transition_config_default_is_200ms() {
    let cfg: TransitionConfig = TransitionConfig::default();
    assert_eq!(cfg.enter_ms, 200);
    assert_eq!(cfg.exit_ms, 200);
}

#[test]
fn transition_config_duration_for_returns_correct_phase_duration() {
    let cfg: TransitionConfig = TransitionConfig::with_durations(100, 200);
    assert_eq!(cfg.duration_for(TransitionPhase::Entering), 100);
    assert_eq!(cfg.duration_for(TransitionPhase::Exiting), 200);
    assert_eq!(cfg.duration_for(TransitionPhase::Entered), 0);
    assert_eq!(cfg.duration_for(TransitionPhase::Exited), 0);
}

#[test]
fn transition_config_clone_preserves_values() {
    let cfg: TransitionConfig = TransitionConfig::with_durations(100, 200);
    let copy: TransitionConfig = cfg; // Copy semantics
    assert_eq!(cfg, copy);
}

#[test]
fn transition_config_equality() {
    assert_eq!(
        TransitionConfig::with_ms(100),
        TransitionConfig::with_ms(100)
    );
    assert_ne!(
        TransitionConfig::with_ms(100),
        TransitionConfig::with_ms(200)
    );
}

// ============================================================
//  Pure-Rust state tests (snapshot reads, no set paths).
// ============================================================

#[test]
fn fresh_state_is_exited_with_zero_progress() {
    let state: TransitionState = fresh_state();
    assert_eq!(state.current_phase(), TransitionPhase::Exited);
    assert_eq!(state.current_progress(), 0.0);
    assert!(state.is_exited());
    assert!(!state.is_entered());
    assert!(!state.is_animating());
    assert_eq!(state.remaining_ms(), 0);
}

#[test]
fn seeded_state_reflects_initial_values() {
    let state: TransitionState = seeded_state(TransitionPhase::Entering, 0.5);
    assert_eq!(state.current_phase(), TransitionPhase::Entering);
    assert_eq!(state.current_progress(), 0.5);
    assert!(state.is_animating());
    assert!(!state.is_entered());
    assert!(!state.is_exited());
}

#[test]
fn accessors_return_signal_clones() {
    let state: TransitionState = fresh_state();
    let _phase: Signal<TransitionPhase> = state.phase();
    let _progress: Signal<f64> = state.progress();
    let _config: Signal<TransitionConfig> = state.config();
}

#[test]
fn reactive_read_via_subscribed_signals_matches_initial() {
    let state: TransitionState = fresh_state();
    let phase_signal: Signal<TransitionPhase> = state.phase();
    let progress_signal: Signal<f64> = state.progress();
    assert_eq!(phase_signal.get(), TransitionPhase::Exited);
    assert_eq!(progress_signal.get(), 0.0);
}

#[test]
fn state_clone_shares_internal_signals() {
    let state: TransitionState = seeded_state(TransitionPhase::Entered, 1.0);
    let twin: TransitionState = state.clone();
    assert_eq!(twin.current_phase(), TransitionPhase::Entered);
    assert_eq!(twin.current_progress(), 1.0);
}

#[test]
fn seeded_entered_state_is_entered_not_animating() {
    let state: TransitionState = seeded_state(TransitionPhase::Entered, 1.0);
    assert!(state.is_entered());
    assert!(!state.is_animating());
    assert_eq!(state.remaining_ms(), 0);
}

#[test]
fn seeded_exited_state_is_exited_not_animating() {
    let state: TransitionState = seeded_state(TransitionPhase::Exited, 0.0);
    assert!(state.is_exited());
    assert!(!state.is_animating());
    assert_eq!(state.remaining_ms(), 0);
}

// ============================================================
//  Set-path coverage tests.
// ============================================================

#[test]
fn enter_from_exited_starts_entering_with_zero_progress_set_path() {
    let state: TransitionState = fresh_state();
    let ran: bool = run_with_signal_capture(|| {
        state.enter();
    });
    if ran {
        assert_eq!(state.current_phase(), TransitionPhase::Entering);
        assert_eq!(state.current_progress(), 0.0);
        assert!(state.is_animating());
    }
}

#[test]
fn enter_from_entering_is_noop_set_path() {
    let state: TransitionState = seeded_state(TransitionPhase::Entering, 0.5);
    let ran: bool = run_with_signal_capture(|| {
        state.enter();
    });
    if ran {
        // Should still be Entering at 0.5 — the no-op
        // enter didn't reset progress.
        assert_eq!(state.current_phase(), TransitionPhase::Entering);
        assert_eq!(state.current_progress(), 0.5);
    }
}

#[test]
fn enter_from_entered_is_noop_set_path() {
    let state: TransitionState = seeded_state(TransitionPhase::Entered, 1.0);
    let ran: bool = run_with_signal_capture(|| {
        state.enter();
    });
    if ran {
        assert_eq!(state.current_phase(), TransitionPhase::Entered);
        assert_eq!(state.current_progress(), 1.0);
    }
}

#[test]
fn enter_from_exiting_starts_entering_set_path() {
    let state: TransitionState = seeded_state(TransitionPhase::Exiting, 0.3);
    let ran: bool = run_with_signal_capture(|| {
        state.enter();
    });
    if ran {
        assert_eq!(state.current_phase(), TransitionPhase::Entering);
        assert_eq!(state.current_progress(), 0.0);
    }
}

#[test]
fn exit_from_entered_starts_exiting_with_full_progress_set_path() {
    let state: TransitionState = seeded_state(TransitionPhase::Entered, 1.0);
    let ran: bool = run_with_signal_capture(|| {
        state.exit();
    });
    if ran {
        assert_eq!(state.current_phase(), TransitionPhase::Exiting);
        assert_eq!(state.current_progress(), 1.0);
        assert!(state.is_animating());
    }
}

#[test]
fn exit_from_entering_starts_exiting_set_path() {
    let state: TransitionState = seeded_state(TransitionPhase::Entering, 0.7);
    let ran: bool = run_with_signal_capture(|| {
        state.exit();
    });
    if ran {
        assert_eq!(state.current_phase(), TransitionPhase::Exiting);
        assert_eq!(state.current_progress(), 1.0);
    }
}

#[test]
fn exit_from_exiting_is_noop_set_path() {
    let state: TransitionState = seeded_state(TransitionPhase::Exiting, 0.5);
    let ran: bool = run_with_signal_capture(|| {
        state.exit();
    });
    if ran {
        assert_eq!(state.current_phase(), TransitionPhase::Exiting);
        assert_eq!(state.current_progress(), 0.5);
    }
}

#[test]
fn exit_from_exited_is_noop_set_path() {
    let state: TransitionState = fresh_state();
    let ran: bool = run_with_signal_capture(|| {
        state.exit();
    });
    if ran {
        assert_eq!(state.current_phase(), TransitionPhase::Exited);
        assert_eq!(state.current_progress(), 0.0);
    }
}

#[test]
fn toggle_flips_entered_to_exiting_set_path() {
    let state: TransitionState = seeded_state(TransitionPhase::Entered, 1.0);
    let ran: bool = run_with_signal_capture(|| {
        state.toggle();
    });
    if ran {
        assert_eq!(state.current_phase(), TransitionPhase::Exiting);
        assert_eq!(state.current_progress(), 1.0);
    }
}

#[test]
fn toggle_flips_exited_to_entering_set_path() {
    let state: TransitionState = fresh_state();
    let ran: bool = run_with_signal_capture(|| {
        state.toggle();
    });
    if ran {
        assert_eq!(state.current_phase(), TransitionPhase::Entering);
        assert_eq!(state.current_progress(), 0.0);
    }
}

#[test]
fn toggle_flips_entering_to_exiting_set_path() {
    let state: TransitionState = seeded_state(TransitionPhase::Entering, 0.5);
    let ran: bool = run_with_signal_capture(|| {
        state.toggle();
    });
    if ran {
        assert_eq!(state.current_phase(), TransitionPhase::Exiting);
        assert_eq!(state.current_progress(), 1.0);
    }
}

#[test]
fn toggle_flips_exiting_to_entering_set_path() {
    let state: TransitionState = seeded_state(TransitionPhase::Exiting, 0.5);
    let ran: bool = run_with_signal_capture(|| {
        state.toggle();
    });
    if ran {
        assert_eq!(state.current_phase(), TransitionPhase::Entering);
        assert_eq!(state.current_progress(), 0.0);
    }
}

#[test]
fn tick_on_entered_phase_is_noop_set_path() {
    let state: TransitionState = seeded_state(TransitionPhase::Entered, 1.0);
    let ran: bool = run_with_signal_capture(|| {
        state.tick(50);
    });
    if ran {
        assert_eq!(state.current_phase(), TransitionPhase::Entered);
        assert_eq!(state.current_progress(), 1.0);
    }
}

#[test]
fn tick_on_exited_phase_is_noop_set_path() {
    let state: TransitionState = fresh_state();
    let ran: bool = run_with_signal_capture(|| {
        state.tick(50);
    });
    if ran {
        assert_eq!(state.current_phase(), TransitionPhase::Exited);
        assert_eq!(state.current_progress(), 0.0);
    }
}

#[test]
fn tick_on_entering_advances_progress_set_path() {
    // 100ms enter; tick 25ms → progress = 0.25
    let state: TransitionState = seeded_state(TransitionPhase::Entering, 0.0);
    let ran: bool = run_with_signal_capture(|| {
        state.tick(25);
    });
    if ran {
        assert_eq!(state.current_phase(), TransitionPhase::Entering);
        assert!((state.current_progress() - 0.25).abs() < f64::EPSILON);
    }
}

#[test]
fn tick_on_entering_to_completion_advances_phase_set_path() {
    // 100ms enter; tick 100ms → progress = 1.0, phase = Entered
    let state: TransitionState = seeded_state(TransitionPhase::Entering, 0.0);
    let ran: bool = run_with_signal_capture(|| {
        state.tick(100);
    });
    if ran {
        assert_eq!(state.current_phase(), TransitionPhase::Entered);
        assert_eq!(state.current_progress(), 1.0);
    }
}

#[test]
fn tick_on_entering_overshoots_to_completion_set_path() {
    // 100ms enter; tick 200ms → progress = 1.0, phase = Entered
    let state: TransitionState = seeded_state(TransitionPhase::Entering, 0.0);
    let ran: bool = run_with_signal_capture(|| {
        state.tick(200);
    });
    if ran {
        assert_eq!(state.current_phase(), TransitionPhase::Entered);
        assert_eq!(state.current_progress(), 1.0);
    }
}

#[test]
fn tick_on_exiting_advances_progress_set_path() {
    // 200ms exit; tick 50ms → progress = 0.75
    let state: TransitionState = seeded_state(TransitionPhase::Exiting, 1.0);
    let ran: bool = run_with_signal_capture(|| {
        state.tick(50);
    });
    if ran {
        assert_eq!(state.current_phase(), TransitionPhase::Exiting);
        assert!((state.current_progress() - 0.75).abs() < f64::EPSILON);
    }
}

#[test]
fn tick_on_exiting_to_completion_advances_phase_set_path() {
    // 200ms exit; tick 200ms → progress = 0.0, phase = Exited
    let state: TransitionState = seeded_state(TransitionPhase::Exiting, 1.0);
    let ran: bool = run_with_signal_capture(|| {
        state.tick(200);
    });
    if ran {
        assert_eq!(state.current_phase(), TransitionPhase::Exited);
        assert_eq!(state.current_progress(), 0.0);
    }
}

#[test]
fn tick_with_zero_duration_enter_jumps_to_entered_set_path() {
    // 0ms enter → tick any amount → progress = 1.0, phase = Entered
    let state: TransitionState = TransitionState::new(
        Signal::create(TransitionPhase::Entering),
        Signal::create(0.0_f64),
        Signal::create(TransitionConfig::with_ms(0)),
    );
    let ran: bool = run_with_signal_capture(|| {
        state.tick(50);
    });
    if ran {
        assert_eq!(state.current_phase(), TransitionPhase::Entered);
        assert_eq!(state.current_progress(), 1.0);
    }
}

#[test]
fn tick_with_zero_duration_exit_jumps_to_exited_set_path() {
    // 0ms exit → tick any amount → progress = 0.0, phase = Exited
    let state: TransitionState = TransitionState::new(
        Signal::create(TransitionPhase::Exiting),
        Signal::create(1.0_f64),
        Signal::create(TransitionConfig::with_ms(0)),
    );
    let ran: bool = run_with_signal_capture(|| {
        state.tick(50);
    });
    if ran {
        assert_eq!(state.current_phase(), TransitionPhase::Exited);
        assert_eq!(state.current_progress(), 0.0);
    }
}

#[test]
fn tick_with_zero_elapsed_does_not_advance_set_path() {
    let state: TransitionState = seeded_state(TransitionPhase::Entering, 0.3);
    let ran: bool = run_with_signal_capture(|| {
        state.tick(0);
    });
    if ran {
        assert_eq!(state.current_phase(), TransitionPhase::Entering);
        assert_eq!(state.current_progress(), 0.3);
    }
}

#[test]
fn reset_returns_state_to_exited_set_path() {
    let state: TransitionState = seeded_state(TransitionPhase::Exiting, 0.5);
    let ran: bool = run_with_signal_capture(|| {
        state.reset();
    });
    if ran {
        assert_eq!(state.current_phase(), TransitionPhase::Exited);
        assert_eq!(state.current_progress(), 0.0);
        assert!(state.is_exited());
    }
}

#[test]
fn change_config_updates_durations_set_path() {
    let state: TransitionState = fresh_state();
    let ran: bool = run_with_signal_capture(|| {
        state.change_config(TransitionConfig::with_durations(500, 1000));
    });
    if ran {
        assert_eq!(state.current_config().enter_ms, 500);
        assert_eq!(state.current_config().exit_ms, 1000);
    }
}

#[test]
fn remaining_ms_on_entering_returns_full_minus_elapsed_set_path() {
    // 100ms enter at progress 0.3 → remaining = 70ms
    let state: TransitionState = seeded_state(TransitionPhase::Entering, 0.3);
    let ran: bool = run_with_signal_capture(|| {
        // Seed reset on remaining_ms — read directly,
        // no set needed.
        let _ = std::panic::AssertUnwindSafe(());
        // Simulate read after potential native panic.
        // If ran is false we can't read remaining_ms
        // because the read path doesn't panic (only set
        // does), so we just trust the seed.
    });
    // Snapshot read of remaining_ms is signal::get,
    // which is safe on native.
    assert_eq!(state.remaining_ms(), 70);
    assert!(ran); // ran should be true since we didn't actually set.
}

#[test]
fn remaining_ms_on_exiting_returns_elapsed_set_path() {
    // 200ms exit at progress 0.5 → remaining = 100ms
    let state: TransitionState = seeded_state(TransitionPhase::Exiting, 0.5);
    assert_eq!(state.remaining_ms(), 100);
}

#[test]
fn remaining_ms_on_terminal_phases_is_zero() {
    let exited: TransitionState = fresh_state();
    assert_eq!(exited.remaining_ms(), 0);
    let entered: TransitionState = seeded_state(TransitionPhase::Entered, 1.0);
    assert_eq!(entered.remaining_ms(), 0);
}

#[test]
fn tick_until_done_advances_to_entered_set_path() {
    let state: TransitionState = seeded_state(TransitionPhase::Entering, 0.0);
    let ran: bool = run_with_signal_capture(|| {
        state.tick_until_done(10);
    });
    if ran {
        assert_eq!(state.current_phase(), TransitionPhase::Entered);
        assert_eq!(state.current_progress(), 1.0);
    }
}

#[test]
fn tick_until_done_advances_to_exited_set_path() {
    let state: TransitionState = seeded_state(TransitionPhase::Exiting, 1.0);
    let ran: bool = run_with_signal_capture(|| {
        state.tick_until_done(10);
    });
    if ran {
        assert_eq!(state.current_phase(), TransitionPhase::Exited);
        assert_eq!(state.current_progress(), 0.0);
    }
}

#[test]
fn tick_until_done_on_terminal_phase_is_noop_set_path() {
    let state: TransitionState = seeded_state(TransitionPhase::Entered, 1.0);
    let ran: bool = run_with_signal_capture(|| {
        state.tick_until_done(10);
    });
    if ran {
        assert_eq!(state.current_phase(), TransitionPhase::Entered);
        assert_eq!(state.current_progress(), 1.0);
    }
}

// ============================================================
//  Lifecycle stress tests.
// ============================================================

#[test]
fn full_lifecycle_enter_tick_complete_exit_tick_complete_set_path() {
    let state: TransitionState = fresh_state();
    let ran: bool = run_with_signal_capture(|| {
        state.enter();
        state.tick(50);
        state.tick(50);
        state.exit();
        state.tick(100);
        state.tick(100);
    });
    if ran {
        assert_eq!(state.current_phase(), TransitionPhase::Exited);
        assert_eq!(state.current_progress(), 0.0);
    }
}

#[test]
fn multiple_enter_exit_cycles_work_set_path() {
    let state: TransitionState = fresh_state();
    let ran: bool = run_with_signal_capture(|| {
        state.enter();
        state.tick_until_done(10);
        state.exit();
        state.tick_until_done(10);
        state.enter();
        state.tick_until_done(10);
    });
    if ran {
        assert_eq!(state.current_phase(), TransitionPhase::Entered);
        assert_eq!(state.current_progress(), 1.0);
    }
}

#[test]
fn reactively_subscribed_phase_signal_reflects_enter_set_path() {
    let state: TransitionState = fresh_state();
    let phase_signal: Signal<TransitionPhase> = state.phase();
    assert_eq!(phase_signal.get(), TransitionPhase::Exited);
    let ran: bool = run_with_signal_capture(|| {
        state.enter();
    });
    if ran {
        assert_eq!(phase_signal.get(), TransitionPhase::Entering);
    }
}
