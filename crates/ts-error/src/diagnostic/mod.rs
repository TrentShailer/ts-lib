//! # Diagnostic
//!
//! A diagnostic over some source file.

mod context;
mod span;

use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::fmt::Write;

use ts_ansi::{
    format_error, format_warning,
    style::{BOLD, CYAN, DEFAULT, RED, RESET, YELLOW},
};

pub use context::Context;
pub use span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
/// A diagnostic severity.
pub enum Severity {
    /// An error.
    Error,
    /// A warning.
    Warning,
}
impl Severity {
    pub(crate) fn colour(self) -> &'static str {
        match &self {
            Self::Error => RED,
            Self::Warning => YELLOW,
        }
    }

    pub(crate) fn word(self) -> &'static str {
        match &self {
            Self::Error => "error",
            Self::Warning => "warning",
        }
    }
}

#[derive(Debug)]
/// A collection of diagnostics
pub struct Diagnostics {
    /// The problems.
    pub problems: Vec<Diagnostic>,
    /// The context.
    pub context: String,
}
impl Diagnostics {
    /// Create a new collection of diagnostics.
    pub fn new<S: ToString>(context: S) -> Self {
        Self {
            problems: vec![],
            context: context.to_string(),
        }
    }

    /// Returns if there are no diagnostics.
    pub fn is_empty(&self) -> bool {
        self.problems.is_empty()
    }

    /// Push a diagnostic into this collection.
    pub fn push(&mut self, diagnostic: Diagnostic) {
        self.problems.push(diagnostic);
    }

    /// Returns an iterator over the error diagnostics.
    pub fn errors(&self) -> impl Iterator<Item = &Diagnostic> {
        self.problems
            .iter()
            .filter(|problem| problem.severity == Severity::Error)
    }

    /// Returns an iterator over the warning diagnostics.
    pub fn warnings(&self) -> impl Iterator<Item = &Diagnostic> {
        self.problems
            .iter()
            .filter(|problem| problem.severity == Severity::Warning)
    }
}
impl core::fmt::Display for Diagnostics {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let warnings: Vec<_> = self.warnings().collect();
        let errors: Vec<_> = self.errors().collect();

        for error in &errors {
            writeln!(f, "{error}")?;
        }
        for warning in &warnings {
            writeln!(f, "{warning}")?;
        }

        if !errors.is_empty() {
            writeln!(
                f,
                "{}",
                format_error!("{} generated {} errors", self.context, errors.len())
            )?;
        }
        if !warnings.is_empty() {
            writeln!(
                f,
                "{}",
                format_warning!("{} generated {} warnings", self.context, warnings.len())
            )?;
        }

        Ok(())
    }
}
impl core::error::Error for Diagnostic {}

#[derive(Debug)]
/// A diagnostic over some source file.
pub struct Diagnostic {
    /// The diagnostic severity.
    pub severity: Severity,
    /// The diagnostic headline.
    pub headline: String,
    /// The diagnostic filepath.
    pub file_path: Option<String>,
    /// The diagnostic context.
    pub context: Option<Context>,
    /// The nodes.
    pub notes: Vec<String>,
}

impl Diagnostic {
    /// Create a new diagnostic.
    pub fn new<S: ToString>(severity: Severity, headling: S) -> Self {
        Self {
            severity,
            headline: headling.to_string(),
            file_path: None,
            context: None,
            notes: Vec::new(),
        }
    }

    /// Create an error diagnostic.
    pub fn error<S: ToString>(headling: S) -> Self {
        Self {
            severity: Severity::Error,
            headline: headling.to_string(),
            file_path: None,
            context: None,
            notes: Vec::new(),
        }
    }

    /// Create a warning diagnostic.
    pub fn warning<S: ToString>(headling: S) -> Self {
        Self {
            severity: Severity::Warning,
            headline: headling.to_string(),
            file_path: None,
            context: None,
            notes: Vec::new(),
        }
    }

    /// Set the file path of the diagnostic.
    pub fn file_path<S: ToString>(mut self, path: S) -> Self {
        self.file_path = Some(path.to_string());
        self
    }

    /// Add a note to the diagnostic.
    pub fn add_note<S: ToString>(mut self, note: S) -> Self {
        self.notes.push(note.to_string());
        self
    }

    /// Set the context of the diagnostic.
    pub fn context(mut self, context: Context) -> Self {
        self.context = Some(context);
        self
    }
}

impl core::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let colour = self.severity.colour();
        let severity = self.severity.word();

        // Write headling:
        // error: some headline here
        writeln!(
            f,
            "{BOLD}{colour}{severity}{DEFAULT}: {}{RESET}",
            self.headline
        )?;

        let line_number_size = self
            .context
            .as_ref()
            .map_or(1, |context| context.span.line.to_string().len());
        let indent = " ".repeat(line_number_size);

        // Write file path:
        // ` --> some/path/to/a.file:12:2`
        if let Some(file_path) = &self.file_path {
            write!(f, "{indent}{CYAN}{BOLD}-->{RESET} {file_path}",)?;

            // Write file location
            if let Some(context) = &self.context {
                write!(f, ":{}:{}", context.span.line, context.span.column)?;
            }
            f.write_char('\n')?;
        }
        // Otherwide write line and column:
        // `  | line 12, column 2`
        else if let Some(context) = &self.context {
            writeln!(
                f,
                "{indent}{CYAN}{BOLD}-->{RESET} line {}, column {}",
                context.span.line, context.span.column
            )?;
        }
        // Write spacer
        writeln!(f, "{indent}{CYAN}{BOLD} | {RESET}")?;

        // Write context
        if let Some(context) = &self.context {
            // Write source lines:
            // `98  | some source code here`
            // `99  | some source code here`
            // `100 | some source code here`
            for (index, line) in context.context.iter().enumerate() {
                let line_number = (context.span.line.saturating_sub(
                    context
                        .context
                        .len()
                        .saturating_sub(index)
                        .saturating_sub(1),
                ))
                .to_string();
                let padding = " ".repeat(line_number_size - line_number.len());
                writeln!(f, "{CYAN}{BOLD}{line_number}{padding} | {RESET}{line}",)?;
            }

            // Write span highlighter:
            // `    |      ^^^^^^`
            write!(
                f,
                "{indent}{CYAN}{BOLD} | {RESET}{}{colour}{BOLD}{}",
                " ".repeat(context.span_indent),
                "^".repeat(context.span.length)
            )?;
            // Write label
            if let Some(label) = &context.label {
                f.write_char(' ')?;
                f.write_str(label)?;
            }
            writeln!(f, "{RESET}")?;
        }

        // Write notes
        if !self.notes.is_empty() {
            writeln!(f, "{indent}{CYAN}{BOLD} | {RESET}")?;
            for note in &self.notes {
                writeln!(f, "{indent}{CYAN}{BOLD} = {DEFAULT}note{RESET}: {note}")?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    extern crate std;

    use std::io::{Write, stderr, stdout};

    use alloc::string::ToString;

    use crate::diagnostic::{Context, Diagnostic, Diagnostics, Span};

    const SOURCE: &str = r#"use alloc::boxed::Box;
use core::{error::Error, fmt};

use ts_ansi::style::{BOLD, DEFAULT, RED, RESET};

/// An error report, displays the error stack of some error.
pub struct Report<'e> {
    /// The error for this report.
    pub source: Box<dyn Error + 'e>,
}
impl<'e> Report<'e> {
    /// Create a new error report.
    pub fn new<E: Error + 'e>(source: E) -> Self {
        Self {
            source: Box::new(source),
        }
    }
}
impl Error for Report<'static> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())
    }
}
impl fmt::Debug for Report<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}
impl fmt::Display for Report<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current_error = Some(self.source.as_ref());
        let mut count = 1;

        while let Some(error) = current_error {
            writeln!(f, " {BOLD}{RED}{count}{DEFAULT}.{RESET} {error}")?;

            count += 1;
            current_error = error.source();
        }

        Ok(())
    }
}"#;

    const MINIFIED_SOURCE: &str = r#"async function Ui(n){return location.href=n,await mu()}function mu(){let n=t=>{setTimeout(()=>n(t),400)};return new Promise(n)}var br=class{element;contents;action;constructor(t,e){this.element=ht(`${t}/error`,HTMLElement),this.contents=ht(`${t}/error/content`,HTMLElement),this.action=e}clearError(){this.element.classList.add("collapse"),this.element.ariaHidden="true",this.contents.textContent=""}addError(t){if(this.contents.textContent===""){this.element.classList.remove("collapse"),this.element.ariaHidden="false",this.contents.textContent=`Could not ${this.action}: ${t}`;return}this.contents.textContent+=`, ${t}`}setSomethingWentWrong(){this.element.classList.remove("collapse"),this.element.ariaHidden="false",this.contents.textContent=`Something went wrong while trying to ${this.action}. Try again later.`}},Nr=class{input;error;constructor(t,e){this.input=ht(`${t}${e}/input`,HTMLInputElement),this.error=ht(`${t}${e}/error`,HTMLElement),this.input.addEventListener("input",()=>{this.input.setCustomValidity("")})}getValue(){return this.input.type==="checkbox"?this.input.checked?"checked":"unchecked":this.input.value}setLock(t){this.input.disabled=t}clearError(){this.input.setCustomValidity(""),this.error.classList.add("hidden"),this.error.ariaHidden="true",this.error.textContent="!"}addError(t){if(this.error.textContent==="!"){this.input.setCustomValidity(t),this.error.classList.remove("hidden"),this.error.ariaHidden="false",this.error.textContent=`Invalid value: ${t}`;return}this.error.textContent+=`, ${t}`,this.input.setCustomValidity(this.error.textContent??"Invalid value")}},ge=class{form;formError;submitButton;inputs;constructor(t,e,r){this.form=ht(t,HTMLFormElement),this.formError=new br(t,r),this.submitButton=ht(`${t}/submit`,HTMLButtonElement);let o=new Map;for(let i of e)o.set(i,new Nr(t,i));this.inputs=o}clearErrors(){this.formError.clearError();for(let t of this.inputs.values())t.clearError()}setLock(t){this.submitButton.disabled=t;for(let e of this.inputs.values())e.setLock(t)}setInputErrors(t){if(!t||t.length===0){this.formError.addError("an unknown field is invalid");return}for(let e of t){let r=this.inputs.get(e.pointer)??null;r?r.addError(e.detail):this.formError.addError(`field ${e.pointer} ${e.detail}`)}}getValues(){let t=new Map;for(let[e,r]of this.inputs)t.set(e,r.getValue());return t}};"#;

    #[test]
    fn show_output() {
        let _stdout = stdout().lock();
        let mut stderr = stderr().lock();

        let warning = Diagnostic::warning("struct `Report` is never used")
            .file_path("crates/ts-error/src/report.rs")
            .context(Context::new(
                SOURCE,
                Span::default().line(7).column(12).length(6),
            ))
            .add_note("`#[warn(dead_code)]` on by default");

        let error = Diagnostic::error("struct `Report` is never used")
            .context(
                Context::new(SOURCE, Span::default().line(7).column(12).length(6))
                    .label("this is unused"),
            )
            .add_note("`#[warn(dead_code)]` on by default");

        let minified_error = Diagnostic::error("some headline here")
            .context(
                Context::new(
                    MINIFIED_SOURCE,
                    Span::default().line(1).column(200).length(50),
                )
                .label("some label here"),
            )
            .add_note("some note here")
            .add_note("this code is trimmed");

        stderr
            .write_all(error.to_string().as_bytes())
            .expect("writing to stderr should not fail");
        stderr
            .write_all(b"\n")
            .expect("writing to stderr should not fail");
        stderr
            .write_all(minified_error.to_string().as_bytes())
            .expect("writing to stderr should not fail");
        stderr
            .write_all(b"\n")
            .expect("writing to stderr should not fail");
        stderr
            .write_all(warning.to_string().as_bytes())
            .expect("writing to stderr should not fail");

        stderr
            .write_all(b"\n-----\n")
            .expect("writing to stderr should not fail");

        let mut diagnostics = Diagnostics::new("test");
        diagnostics.push(warning);
        diagnostics.push(minified_error);
        diagnostics.push(error);
        stderr
            .write_all(diagnostics.to_string().as_bytes())
            .expect("writing to stderr should not fail");

        stderr.flush().expect("flusing stderr should not fail");
    }
}
