use crate::create::CreateArguments;

use super::ArgumentNames;

#[derive(Debug, clap::Args)]
#[group(requires_all = ["name", "account"], multiple = true)]
pub struct CreateCommand {
    /// The name to use for the new graph
    #[arg(short, long)]
    pub name: Option<String>,
    /// The slug of the account in which the new graph should be created
    #[arg(short, long, value_name = "SLUG")]
    pub account: Option<String>,
    /// Adds an environment variable to the graph
    #[clap(short = 'e', long = "env", value_parser, num_args = 0..)]
    environment_variables: Vec<String>,
}

impl CreateCommand {
    pub fn create_arguments(&self) -> Option<CreateArguments<'_>> {
        self.name
            .as_deref()
            .zip(self.account.as_deref())
            .map(|(name, account_slug)| CreateArguments {
                account_slug,
                name,
                env_vars: self.environment_variables().collect(),
            })
    }

    pub fn environment_variables(&self) -> impl Iterator<Item = (&str, &str)> {
        self.environment_variables
            .iter()
            .filter_map(|s| super::split_env_var(s))
    }
}

impl ArgumentNames for CreateCommand {
    fn argument_names(&self) -> Option<Vec<&'static str>> {
        let arguments = [(self.name.is_some(), vec!["name", "account"])]
            .iter()
            .filter(|arguments| arguments.0)
            .flat_map(|arguments| arguments.1.clone())
            .collect::<Vec<_>>();
        if arguments.is_empty() {
            None
        } else {
            Some(arguments)
        }
    }
}
