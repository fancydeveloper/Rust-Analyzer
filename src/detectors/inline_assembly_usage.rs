use crate::{
    error::Error,
    project::Project,
    report::Severity,
    visitor::{AsmBlockContext, AstVisitor},
};
use sway_types::Spanned;

#[derive(Default)]
pub struct InlineAssemblyUsageVisitor;

impl AstVisitor for InlineAssemblyUsageVisitor {
    fn visit_asm_block(&mut self, context: &AsmBlockContext, project: &mut Project) -> Result<(), Error> {
        project.report.borrow_mut().add_entry(
            context.path,
            project.span_to_line(context.path, &context.asm.span())?,
            Severity::Medium,
            format!(
                "The `{}` function contains inline assembly usage.",
                if let Some(item_impl) = context.item_impl.as_ref() {
                    format!(
                        "{}::{}",
                        item_impl.ty.span().as_str(),
                        context.item_fn.fn_signature.name.as_str(),
                    )
                } else {
                    format!(
                        "{}",
                        context.item_fn.fn_signature.name.as_str(),
                    )
                }
            ),
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_inline_assembly_usage() {
        crate::tests::test_detector("inline_assembly_usage")
    }
}
