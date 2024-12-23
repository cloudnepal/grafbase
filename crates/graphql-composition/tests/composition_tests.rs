use std::{fs, path::Path, sync::OnceLock};

use graphql_composition::FederatedGraph;

fn update_expect() -> bool {
    static UPDATE_EXPECT: OnceLock<bool> = OnceLock::new();
    *UPDATE_EXPECT.get_or_init(|| std::env::var("UPDATE_EXPECT").is_ok())
}

fn run_test(federated_graph_path: &Path) -> datatest_stable::Result<()> {
    miette_run_test(federated_graph_path).map_err(Into::into)
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("{0}")]
    Miette(miette::Report),
    #[error(transparent)]
    IO(#[from] std::io::Error),
}

impl From<miette::Report> for Error {
    fn from(report: miette::Report) -> Self {
        Error::Miette(report)
    }
}

fn miette_run_test(federated_graph_path: &Path) -> Result<(), Error> {
    if cfg!(windows) {
        return Ok(()); // newlines
    }

    let subgraphs_dir = federated_graph_path.with_file_name("").join("subgraphs");
    let api_sdl_path = federated_graph_path.with_file_name("api.graphql");

    if !subgraphs_dir.is_dir() {
        return Err(miette::miette!("{} is not a directory.", subgraphs_dir.display()).into());
    }

    let mut subgraphs_sdl = fs::read_dir(subgraphs_dir)?
        .filter_map(Result::ok)
        .filter(|file| file.file_name() != ".gitignore")
        .map(|file| fs::read_to_string(file.path()).map(|contents| (contents, file.path())))
        .collect::<Result<Vec<_>, _>>()?;

    // [fs::read_dir()] doesn't guarantee ordering. Sort to make tests deterministic
    // (inconsistencies observed in CI).
    subgraphs_sdl.sort_by_key(|(_, path)| path.file_name().unwrap().to_owned());

    let mut subgraphs = graphql_composition::Subgraphs::default();

    for (sdl, path) in subgraphs_sdl {
        let name = path.file_stem().unwrap().to_str().unwrap().replace('_', "-");

        subgraphs
            .ingest_str(&sdl, &name, &format!("http://example.com/{name}"))
            .map_err(|err| miette::miette!("Error parsing {}: \n{err:#}", path.display()))?;
    }

    let expected_federated_sdl = fs::read_to_string(federated_graph_path)
        .map_err(|err| miette::miette!("Error trying to read federated.graphql: {}", err))?;
    let expected_api_sdl = fs::read_to_string(&api_sdl_path)
        .map_err(|err| miette::miette!("Error trying to read api.graphql: {}", err))
        .ok();
    let (actual_federated_sdl, actual_api_sdl) = match graphql_composition::compose(&subgraphs).into_result() {
        Ok(federated_graph) => (
            graphql_federated_graph::render_federated_sdl(&federated_graph).expect("rendering federated SDL"),
            Some(graphql_federated_graph::render_api_sdl(&federated_graph)),
        ),
        Err(diagnostics) => (
            format!(
                "{}\n",
                diagnostics
                    .iter_messages()
                    .map(|msg| format!("# {}", msg.lines().collect::<Vec<_>>().join("\\n")))
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
            None,
        ),
    };

    if update_expect() {
        if let Some(sdl) = actual_api_sdl {
            fs::write(api_sdl_path, sdl).unwrap();
        }

        fs::write(federated_graph_path, actual_federated_sdl)?;
        return Ok(());
    }

    if expected_federated_sdl != actual_federated_sdl {
        return Err(miette::miette!(
            "{}\n\n\n=== Hint: run the tests again with UPDATE_EXPECT=1 to update the snapshot. ===",
            similar::udiff::unified_diff(
                similar::Algorithm::default(),
                &expected_federated_sdl,
                &actual_federated_sdl,
                5,
                Some(("Expected", "Actual"))
            )
        )
        .into());
    }

    match (expected_api_sdl, actual_api_sdl) {
        (None, None) => Ok(()),
        (Some(_), None) => Err(miette::miette!("Expected no API SDL, but there is an api.graphql expectation.").into()),
        (None, Some(_)) => Err(miette::miette!("Expected an api.graphql, but found none.").into()),
        (Some(a), Some(b)) if a == b => Ok(()),
        (Some(a), Some(b)) => Err(miette::miette!(
            "{}\n\n\n=== Hint: run the tests again with UPDATE_EXPECT=1 to update the snapshot. ===",
            similar::udiff::unified_diff(similar::Algorithm::default(), &a, &b, 5, Some(("Expected", "Actual")))
        )
        .into()),
    }
}

fn test_sdl_roundtrip(federated_graph_path: &Path) -> datatest_stable::Result<()> {
    if cfg!(windows) {
        return Ok(()); // newlines
    }

    let sdl = fs::read_to_string(federated_graph_path)
        .map_err(|err| miette::miette!("Error trying to read federated.graphql: {}", err))?;

    // Exclude tests with an empty schema. This is the case for composition error tests.
    if sdl.lines().all(|line| line.is_empty() || line.starts_with('#')) {
        return Ok(());
    }

    let roundtripped = graphql_federated_graph::render_federated_sdl(
        &FederatedGraph::from_sdl(&sdl).map_err(|err| miette::miette!("Error ingesting SDL: {err}\n\nSDL:\n{sdl}"))?,
    )?;

    if roundtripped == sdl {
        return Ok(());
    }

    Err(miette::miette!(
        "{}\n\n\n=== Hint: run the tests again with UPDATE_EXPECT=1 to update the snapshot. ===",
        similar::udiff::unified_diff(
            similar::Algorithm::default(),
            &sdl,
            &roundtripped,
            5,
            Some(("Expected", "Actual"))
        )
    )
    .into())
}

datatest_stable::harness! {
    run_test, "./tests/composition", r"^.*federated.graphql$",
    test_sdl_roundtrip, "./tests/composition", r"^.*federated.graphql$",
}
