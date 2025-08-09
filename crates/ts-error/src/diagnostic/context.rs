use crate::diagnostic::Span;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

#[derive(Debug, Clone)]
/// Context for a diagnostic.
pub struct Context {
    /// The context for the diagnostic, sequential lines of the source where the last string is the
    /// relevant line for the diagnostic. Each line is at most 100 characters wide
    pub context: Vec<String>,
    /// The span of the context relevant to the diagnostic.
    pub span: Span,
    /// The label for the span.
    pub label: Option<String>,
    /// How indented into the context the span starts.
    pub span_indent: usize,
}
impl Context {
    /// Create the context for a diagnostic from a span and the source file.
    pub fn new(source: &str, span: Span) -> Self {
        const MAX_LENGTH: usize = 100;

        let context_end = span.column.saturating_sub(1) + span.length.min(MAX_LENGTH);
        let context_start = span.column.saturating_sub(1);

        let span_start = context_start
            .saturating_sub(MAX_LENGTH.saturating_sub(context_end.saturating_sub(context_start)));
        let span_end = span_start + MAX_LENGTH;

        let mut context = Vec::with_capacity(3);
        let lines: Vec<&str> = source.lines().collect();
        for i in (1..4).rev() {
            if let Some(index) = span.line.checked_sub(i)
                && let Some(line) = lines.get(index)
            {
                let line_context = line
                    .get(span_start..span_end.min(line.len()))
                    .unwrap_or_default();
                context.push(line_context.to_string());
            }
        }

        let span_indent = context_start.saturating_sub(span_start);

        Self {
            context,
            span,
            label: None,
            span_indent,
        }
    }

    /// Sets the label of the context.
    pub fn label<S: ToString>(mut self, label: S) -> Self {
        self.label = Some(label.to_string());
        self
    }
}

#[cfg(test)]
mod test {
    use alloc::{string::String, vec, vec::Vec};

    use crate::diagnostic::{Context, Span};

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
    fn handles_context() {
        let span = Span::default().line(7).column(12).length(6);
        let context = Context::new(SOURCE, span);
        assert_eq!(
            vec![
                r#""#,
                r#"/// An error report, displays the error stack of some error."#,
                r#"pub struct Report<'e> {"#
            ],
            context.context
        );

        let span = Span::default().line(36);
        let context = Context::new(SOURCE, span);
        assert_eq!(
            vec![
                r#"        while let Some(error) = current_error {"#,
                r#"            writeln!(f, " {BOLD}{RED}{count}{DEFAULT}.{RESET} {error}")?;"#,
                r#""#
            ],
            context.context
        );

        let span = Span::default().line(999);
        let context = Context::new(SOURCE, span);
        assert_eq!(Vec::<String>::new(), context.context);

        let span = Span::default().line(35).column(999).length(999);
        let context = Context::new(SOURCE, span);
        assert_eq!(vec![r#""#, r#""#, r#""#], context.context);

        let span = Span::default().line(1).column(200).length(50);
        let context = Context::new(MINIFIED_SOURCE, span);
        assert_eq!(
            vec![
                r#"ontents;action;constructor(t,e){this.element=ht(`${t}/error`,HTMLElement),this.contents=ht(`${t}/err"#
            ],
            context.context
        );
    }
}
