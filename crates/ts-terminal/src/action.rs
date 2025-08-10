use alloc::string::{String, ToString};
use std::io::{Write, stderr};

use ts_ansi::style::*;

/// Extension trait to update an action state based on the value of `self`.
pub trait ActionResult {
    /// Bind the final outcome of the action to the state of `self`.
    fn bind_action(self, action: Action) -> Self;

    /// Update the outcome of the action to error depending if `self` is considered an error.
    fn error_action(self, action: &mut Action) -> Self;
}

impl<T, E> ActionResult for Result<T, E> {
    fn bind_action(self, mut action: Action) -> Self {
        match &self {
            Ok(_) => action.report_success(),
            Err(_) => action.report_fail(),
        }
        self
    }

    fn error_action(self, action: &mut Action) -> Self {
        if self.is_err() {
            action.report_fail();
        }
        self
    }
}
impl<T> ActionResult for Option<T> {
    fn bind_action(self, mut action: Action) -> Self {
        match &self {
            Some(_) => action.report_success(),
            None => action.report_fail(),
        }
        self
    }

    fn error_action(self, action: &mut Action) -> Self {
        if self.is_none() {
            action.report_fail();
        }
        self
    }
}

/// Action State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
enum ActionState {
    /// The action is in progress.
    InProgress,
    /// The action was a success.
    Success,
    /// The action was an error.
    Fail,
}

/// Action progress reporter.
#[derive(Debug, Clone)]
pub struct Action {
    /// The current state of the action
    state: ActionState,
    /// Verb for the in progress action.
    actioning_verb: String,
    /// Verb for the completed action.
    actioned_verb: String,
    /// Details for the action.
    detail: String,
    /// Should the action erase the previous line when printing the next state.
    should_erase: bool,
}

impl Action {
    /// Create and report a new in progress action.
    ///
    /// ## Limitations
    /// * Anything else writing to the `stdout`/`stderr` will cause this to erase them unless
    ///   [`Self::dont_erase`] is called.
    /// * If the content is wrapped, this will erase part of it, keep details and verbs short.
    pub fn new<S1: ToString, S2: ToString, S3: ToString>(
        actioning_verb: S1,
        actioned_verb: S2,
        detail: S3,
    ) -> Self {
        let mut progress = Self {
            state: ActionState::InProgress,
            actioning_verb: actioning_verb.to_string(),
            actioned_verb: actioned_verb.to_string(),
            detail: detail.to_string(),
            should_erase: false,
        };

        progress.print();
        progress
    }

    /// Report the action as failed.
    pub fn report_fail(&mut self) {
        self.state = ActionState::Fail;
        self.print();
    }

    /// Report the action as a success.
    pub fn report_success(&mut self) {
        self.state = ActionState::Success;
        self.print();
    }

    /// Print the message for this action to `stderr`.
    ///
    /// All IO errors are ignored.
    pub fn print(&mut self) {
        #![expect(
            unused_must_use,
            reason = "displaying output is a non-critical part of the program, so this should not 
            panic, additionally, I don't want to have to think about the errors when calling this"
        )]

        let mut stderr = stderr().lock();

        if self.should_erase {
            stderr.write_all(ERASE_LINE_UP.as_bytes());
        }

        let actioning = &self.actioning_verb;
        let actioned = &self.actioned_verb;
        let detail = &self.detail;

        match self.state {
            ActionState::InProgress => {
                writeln!(stderr, "{CYAN}{BOLD}{actioning}{RESET} {detail}");
            }
            ActionState::Success => {
                writeln!(stderr, "{GREEN}{BOLD}{actioned}{RESET} {detail}");
            }
            ActionState::Fail => {
                writeln!(
                    stderr,
                    "{RED}{BOLD}{actioning}{RESET} {detail} {RED}{BOLD}failed{RESET}"
                );
            }
        };

        stderr.flush();

        self.should_erase = true;
    }

    /// Disable erasing the previous line on next print.
    pub fn dont_erase(&mut self) {
        self.should_erase = false;
    }
}
