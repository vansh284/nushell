use crate::commands::WholeStreamCommand;
use crate::errors::ShellError;
use crate::parser::registry::{CommandRegistry, Signature};
use crate::prelude::*;
use futures::stream::StreamExt;

pub struct Compact;

#[derive(Deserialize)]
pub struct CompactArgs {
    rest: Vec<Tagged<String>>,
}

impl WholeStreamCommand for Compact {
    fn name(&self) -> &str {
        "compact"
    }

    fn signature(&self) -> Signature {
        Signature::build("compact").rest(SyntaxShape::Any, "the columns to compact from the table")
    }

    fn usage(&self) -> &str {
        "Creates a table with non-empty rows"
    }

    fn run(
        &self,
        args: CommandArgs,
        registry: &CommandRegistry,
    ) -> Result<OutputStream, ShellError> {
        args.process(registry, compact)?.run()
    }
}

pub fn compact(
    CompactArgs { rest: columns }: CompactArgs,
    RunnableContext { input, .. }: RunnableContext,
) -> Result<OutputStream, ShellError> {
    let objects = input.values.take_while(move |item| {
        let keep = if columns.is_empty() {
            item.is_some()
        } else {
            columns
                .iter()
                .all(|field| item.get_data(field).borrow().is_some())
        };

        futures::future::ready(keep)
    });

    Ok(objects.from_input_stream())
}
