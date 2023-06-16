//! This should be a custom linter/compiler plugin, as it's pretty trivial to fool right now
//! - Imports can hide in macros, which won't be traversed here.
//! - We're never sure of the actual _full_ path of the use.
//! - You can just assign e.g a function to a variable, and call it later.
//! - etc...
//!
//! (But should be good enough for the most part).

use std::{fmt::Display, iter::repeat};

use anyhow::{bail, Context as _};
use clap::Parser as _;
use globset::{Glob, GlobSetBuilder};
use ignore::Walk;
use indoc::indoc;
use owo_colors::Stream::Stderr;
use proc_macro2::{LineColumn, Span};
use syn::{
    spanned::Spanned, visit::Visit, Expr, ExprCall, ExprPath, ExprStruct, ItemUse, UseGroup,
    UseName, UsePath, UseRename, UseTree,
};

#[derive(Debug, clap::Parser)]
#[command(
    about = indoc! {
        "Traverse files for use of CRATE in rust source code.
        Returns an error if it finds any."
    },
    long_about = indoc! {"
    Interprets the given files as rust source, raising errors if any of the following are found:

    - import statements like `use CRATE;`.
    - function calls like `CRATE::foo()`.
    - struct construction like `CRATE::Foo { .. }`.

    This performs a best-guess, but it's pretty trivial to introduce false positives and negatives."
    }
)]
struct Args {
    /// The crate to disallow.
    #[arg(name = "crate")]
    krate: String,
    /// The file(s) to check.
    #[arg(num_args(1..), required = true)]
    glob: Vec<Glob>,
}

/// Traverses the rust AST, collecting the occurences we want to lint for
struct Visitor<'a> {
    disallowed_crate: &'a str,
    violations: Vec<Span>,
}

impl<'a> Visitor<'a> {
    fn new(disallowed_crate: &'a str) -> Self {
        Self {
            disallowed_crate,
            violations: vec![],
        }
    }
}

impl Visit<'_> for Visitor<'_> {
    /// Catch `use foo;`
    fn visit_item_use(&mut self, i: &'_ ItemUse) {
        match &i.tree {
            UseTree::Path(UsePath { ident, .. })
            | UseTree::Name(UseName { ident })
            | UseTree::Rename(UseRename { ident, .. })
                if ident == self.disallowed_crate =>
            {
                self.violations.push(ident.span())
            }
            // `use {fvm, fvm3}; is legal rust, but shouldn't be allowed by rustfmt, so don't handle for now
            UseTree::Group(UseGroup { items: _, .. }) => todo!("handle unconventional grouping"),
            _ => {}
        };
        syn::visit::visit_item_use(self, i)
    }
    /// Catch `foo::bar();`
    fn visit_expr_call(&mut self, i: &'_ ExprCall) {
        match &*i.func {
            Expr::Path(ExprPath { path, .. })
                if path
                    .segments
                    .first()
                    .is_some_and(|it| it.ident == self.disallowed_crate) =>
            {
                self.violations.push(path.span())
            }
            _ => {}
        }
        syn::visit::visit_expr_call(self, i)
    }
    /// Catch `foo::Bar { .. }`
    fn visit_expr_struct(&mut self, i: &'_ ExprStruct) {
        if i.path
            .segments
            .first()
            .is_some_and(|it| it.ident == self.disallowed_crate)
        {
            self.violations.push(i.span())
        }
        syn::visit::visit_expr_struct(self, i)
    }
}

fn lint_file(disallowed_crate: &str, file: syn::File) -> Vec<Span> {
    let mut visitor = Visitor::new(disallowed_crate);
    visitor.visit_file(&file);
    visitor.violations
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut globset = GlobSetBuilder::new();
    for glob in args.glob {
        globset.add(glob);
    }
    let globset = globset.build().context("couldn't build globset")?;
    let mut num_errors = 0;
    let mut num_files = 0;
    for maybe_entry in Walk::new(".") {
        let entry = maybe_entry.context("couldn't walk directory")?;
        if entry.file_type().expect("this isn't STDIN").is_file() && globset.is_match(entry.path())
        {
            num_files += 1;
            let path = entry.path().display();
            let file = std::fs::read_to_string(entry.path())
                .context(format!("couldn't read file {path}"))?;
            let ast = syn::parse_file(&file).context(format!("couldn't parse file {path}"))?;
            let violations = lint_file(&args.krate, ast);
            num_errors += violations.len();
            let lines = file.lines().collect::<Vec<_>>();
            for LineColumn { line, column } in violations.into_iter().map(|it| it.start()) {
                let line = line - 1; // LineColumn is 1-index, we're 0-indexed;
                eprintln!("{}", format!("Error in {path}:{line}:{column}").red());

                if let Some(before) = line.checked_sub(1).and_then(|ix| lines.get(ix)) {
                    if !before.is_empty() {
                        eprintln!("{}", before.dimmed())
                    }
                }
                let actual = lines[line];
                eprintln!("{actual}");
                let diag = repeat(" ")
                    .take(column)
                    .chain(["^^^ disallowed crate"])
                    .collect::<String>();
                eprintln!("{}", diag.blue());

                if let Some(after) = lines.get(line + 1) {
                    if !after.is_empty() {
                        eprintln!("{}", after.dimmed());
                    }
                }
                println!()
            }
        }
    }
    match num_errors {
        0 => {
            eprintln!("Found no errors in {num_files} files");
            Ok(())
        }
        nonzero => bail!("Found {nonzero} errors in {num_files} files"),
    }
}

/// Conditional colored printing to stderr
trait ColorHelper: owo_colors::OwoColorize + Display {
    fn red(&self) -> Box<dyn Display + '_> {
        Box::new(self.if_supports_color(Stderr, owo_colors::OwoColorize::red))
    }
    fn dimmed(&self) -> Box<dyn Display + '_> {
        Box::new(self.if_supports_color(Stderr, owo_colors::OwoColorize::dimmed))
    }
    fn blue(&self) -> Box<dyn Display + '_> {
        Box::new(self.if_supports_color(Stderr, owo_colors::OwoColorize::blue))
    }
}

impl<T> ColorHelper for T where T: owo_colors::OwoColorize + Display {}

#[cfg(test)]
mod tests {
    use super::*;
    #[track_caller]
    fn test_counts(code: &str, krate: &str, n: usize) {
        let file = syn::parse_file(code).unwrap();
        let violations = lint_file(krate, file);
        assert_eq!(n, violations.len());
    }

    #[test]
    fn test() {
        test_counts("use fvm;", "fvm", 1);
        test_counts("fn foo(_: fvm::Bar) { }", "fvm", 0);
        test_counts("fn foo() { fvm::bar() }", "fvm", 1);
        test_counts("const _: fvm::Foo = fvm::Foo { };", "fvm", 1);
    }
}
