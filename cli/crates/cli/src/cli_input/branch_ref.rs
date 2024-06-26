use std::{fmt, str};

/// Parsed project reference. A project reference is a string of the form `account/project@branch`.
#[derive(Clone, Debug)]
pub struct BranchRef {
    account: String,
    project: String,
    branch: String,
}

impl BranchRef {
    pub(crate) const ARG_DESCRIPTION: &'static str =
        r#"Branch reference following the format "account/project@branch""#;

    pub(crate) fn account(&self) -> &str {
        self.account.as_ref()
    }

    pub(crate) fn project(&self) -> &str {
        self.project.as_ref()
    }

    pub(crate) fn branch(&self) -> &str {
        self.branch.as_ref()
    }
}

impl str::FromStr for BranchRef {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const GENERIC_ERR: &str = r#"Invalid branch reference. The branch reference argument must follow the format: "account/project@branch""#;

        let Some((account, rest)) = s.split_once('/') else {
            return Err(GENERIC_ERR);
        };

        if account.is_empty() {
            return Err("The account name is missing before '/'.");
        }

        let Some((project, branch)) = rest.split_once('@') else {
            return Err(GENERIC_ERR);
        };

        if project.is_empty() {
            return Err("The project name is missing.");
        }

        if branch.is_empty() {
            return Err("The branch name is missing.");
        }

        Ok(BranchRef {
            account: account.to_owned(),
            project: project.to_owned(),
            branch: branch.to_owned(),
        })
    }
}

impl fmt::Display for BranchRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.account)?;
        f.write_str("/")?;
        f.write_str(&self.project)?;
        f.write_str("@")?;
        f.write_str(&self.branch)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn branch_ref_ok() {
        let cases = [
            "microsoft/windows@main",
            "test/project@master",
            "__my__/_____project-with-things@branch-here",
            "1/2@3",
        ];

        for case in cases {
            assert_eq!(case, case.parse::<BranchRef>().unwrap().to_string());
        }
    }
}
