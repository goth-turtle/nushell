use crate::errors::ShellError;
use crate::prelude::*;

// TODO: "Amount remaining" wrapper

pub fn first(args: CommandArgs) -> Result<OutputStream, ShellError> {
    if args.len() == 0 {
        if let Some(span) = args.name_span {
            return Err(ShellError::labeled_error(
                "First requires an amount",
                "needs parameter",
                span,
            ));
        } else {
            return Err(ShellError::string("first requires an amount."));
        }
    }

    let amount = args.expect_nth(0)?.as_i64();

    let amount = match amount {
        Ok(o) => o,
        Err(_) => {
            return Err(ShellError::labeled_error(
                "Value is not a number",
                "expected integer",
                args.expect_nth(0)?.span,
            ))
        }
    };

    let input = args.input;

    Ok(input
        .take(amount as u64)
        .map(|v| ReturnValue::Value(v))
        .boxed())
}
