use crate::{cli_input::PublishCommand, errors::CliError, output::report};
use std::{
    fs,
    io::{IsTerminal, Read},
};

#[tokio::main]
pub(crate) async fn publish(
    PublishCommand {
        subgraph_name,
        graph_ref,
        url,
        schema_path,
        message,
        ..
    }: PublishCommand,
) -> Result<(), CliError> {
    let schema = match schema_path {
        Some(path) => fs::read_to_string(path).map_err(CliError::SchemaReadError)?,
        None if std::io::stdin().is_terminal() => {
            return Err(CliError::MissingArgument("--schema or a schema piped through stdin"))
        }
        None => {
            let mut schema = String::new();

            std::io::stdin()
                .read_to_string(&mut schema)
                .map_err(CliError::SchemaReadError)?;

            schema
        }
    };

    report::publishing();

    let outcome = backend::api::publish::publish(
        graph_ref.account(),
        graph_ref.graph(),
        graph_ref.branch(),
        &subgraph_name,
        url.as_str(),
        &schema,
        message.as_deref(),
    )
    .await
    .map_err(CliError::BackendApiError)?;

    match &outcome {
        backend::api::publish::PublishOutcome::Success { composition_errors } if composition_errors.is_empty() => {
            report::publish_command_success(&subgraph_name);
        }
        backend::api::publish::PublishOutcome::Success { composition_errors } => {
            report::publish_command_composition_failure(composition_errors);
        }
        backend::api::publish::PublishOutcome::GraphDoesNotExist {
            account_slug,
            graph_slug,
        } => report::publish_graph_does_not_exist(account_slug, graph_slug),
    };

    Ok(())
}