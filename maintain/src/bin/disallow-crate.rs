//! This should be a custom linter/compiler plugin, as it's full of bugs right now.
//! - Imports can hide in macros, which won't be traversed here.
//! - We're never sure of the actual _full_ path of the use.

use anyhow::{bail, Context as _};
use clap::Parser as _;
use globset::{Glob, GlobSetBuilder};
use proc_macro2::LineColumn;
use syn::{
    spanned::Spanned, visit::Visit, ExprPath, ItemUse, PathArguments, PathSegment, TypePath,
    UseGroup, UseName, UsePath, UseRename, UseTree,
};
use tracing::{debug, error};
use tracing_subscriber::{filter::LevelFilter, EnvFilter};
use walkdir::WalkDir;

#[derive(Debug, clap::Parser)]
struct Args {
    /// The crate to disallow
    #[arg(name = "crate")]
    krate: String,
    /// The files to check.
    ///
    /// May be specified multiple times.
    #[arg(short, long)]
    glob: Vec<Glob>,
}

struct DisallowCrateVisitor<'a> {
    disallowed_crate: &'a str,
    use_violations: Vec<ItemUse>,
    path_violations: Vec<TypePath>,
    expr_violations: Vec<ExprPath>,
}

impl<'a> DisallowCrateVisitor<'a> {
    fn new(disallowed_crate: &'a str) -> Self {
        let r = std::mem::take;
        r(&mut ());
        Self {
            disallowed_crate,
            use_violations: vec![],
            path_violations: vec![],
            expr_violations: vec![],
        }
    }
}

impl Visit<'_> for DisallowCrateVisitor<'_> {
    /// Examine all `use` statements
    fn visit_item_use(&mut self, i: &'_ ItemUse) {
        match &i.tree {
            UseTree::Path(UsePath { ident, .. })
            | UseTree::Name(UseName { ident })
            | UseTree::Rename(UseRename { ident, .. })
                if ident == self.disallowed_crate =>
            {
                self.use_violations.push(i.clone())
            }
            // `use {fvm, fvm3}; is legal rust, but shouldn't be allowed by rustfmt, so don't handle for now
            UseTree::Group(UseGroup { items: _, .. }) => todo!("handle unconventional grouping"),
            _ => {}
        };
        syn::visit::visit_item_use(self, i)
    }
    fn visit_type_path(&mut self, i: &'_ TypePath) {
        // We only know for sure unless the path is absolute.
        // We've got no good way of handling for now, so don't use this information.
        // let _is_absolute = i.path.leading_colon.is_some();
        match i.path.segments.first().expect("empty type path is illegal") {
            PathSegment {
                ident,
                arguments: PathArguments::None,
            } if ident == self.disallowed_crate => self.path_violations.push(i.clone()),
            _ => {}
        }
        syn::visit::visit_type_path(self, i)
    }
    fn visit_expr_path(&mut self, i: &'_ ExprPath) {
        match i.path.segments.first().expect("empty expr_path is illegal") {
            PathSegment { ident, .. } if ident == self.disallowed_crate => {
                self.expr_violations.push(i.clone())
            }
            _ => {}
        }
        syn::visit::visit_expr_path(self, i);
    }
}

fn lint_file(
    disallowed_crate: &str,
    file: syn::File,
) -> (Vec<ItemUse>, Vec<TypePath>, Vec<ExprPath>) {
    let mut visitor = DisallowCrateVisitor::new(disallowed_crate);
    visitor.visit_file(&file);
    (
        visitor.use_violations,
        visitor.path_violations,
        visitor.expr_violations,
    )
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with_target(false)
        .without_time()
        .with_writer(std::io::stderr)
        .init();

    let args = Args::parse();
    let mut globset = GlobSetBuilder::new();
    for glob in args.glob {
        globset.add(glob);
    }
    let globset = globset.build().context("couldn't build globset")?;
    let mut num_errors = 0;
    let mut num_files = 0;
    for maybe_entry in WalkDir::new(".") {
        let entry = maybe_entry.context("couldn't walk directory")?;
        if entry.file_type().is_file() && globset.is_match(entry.path()) {
            num_files += 1;
            let path = entry.path().display();
            debug!(%path, "matched file");
            let file = syn::parse_file(
                &std::fs::read_to_string(entry.path())
                    .context(format!("couldn't read file {path}"))?,
            )
            .context(format!("couldn't parse file {path}"))?;
            let (use_violations, path_violations, expr_violations) = lint_file(&args.krate, file);
            num_errors += use_violations.len() + path_violations.len() + expr_violations.len();
            for (LineColumn { line, column }, source) in use_violations
                .into_iter()
                .map(line_column_and_source_text)
                .chain(path_violations.into_iter().map(line_column_and_source_text))
                .chain(expr_violations.into_iter().map(line_column_and_source_text))
            {
                error!("use in {path}:{line}:{column}");
                error!("{source}")
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

fn line_column_and_source_text(s: impl Spanned) -> (LineColumn, String) {
    let span = s.span();
    (
        span.start(),
        span.source_text()
            .expect("proc-macro2 built with span-locations"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_counts(
        code: &str,
        disallowed_crate: &str,
        expected_use_violations: usize,
        expected_path_violations: usize,
        expected_expr_violations: usize,
    ) {
        let file = syn::parse_file(code).unwrap();
        let (use_violations, path_violations, expr_violations) = lint_file(disallowed_crate, file);
        assert_eq!(expected_use_violations, use_violations.len());
        assert_eq!(expected_path_violations, path_violations.len());
        assert_eq!(expected_expr_violations, expr_violations.len());
    }

    #[test]
    fn test() {
        test_counts("use fvm;", "fvm", 1, 0, 0);
        test_counts("fn foo(_: fvm::Foo) {}", "fvm", 0, 1, 0);
        test_counts("fn foo() { fvm::bar() }", "fvm", 0, 0, 1);
    }
}
