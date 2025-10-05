use clap::{Parser, ValueEnum};

#[derive(Debug, Parser, PartialEq)]
pub struct Config{
    #[arg(long, env)]
    pub github_username: String,
    #[arg(long, env)]
    pub github_repository_owner_name: String,
    #[arg(long, env)]
    pub github_repository_name: String,
    #[arg(long, env)]
    pub github_branch_name: Option<String>,
    #[arg(long, env, hide_env_values = true)]
    pub github_password: String,
    #[arg(long, env)]
    pub server_name: String,
    #[arg(long, env)]
    pub server_type: ServerType,
    #[arg(long, env)]
    pub registry_path: String,
    #[arg(long, env)]
    pub server_path: String,
    #[arg(long, env)]
    pub start_command: String,
    #[arg(long, env)]
    pub stop_command: String,
}

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum ServerType{
    Paper,
    Velocity,
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn parse_arguments(){
        let configs = Config::try_parse_from([
            "mdcs-controller",
            "--github-username",
            "example_user",
            "--github-repository-owner-name",
            "example_user",
            "--github-repository-name",
            "example_repo",
            "--github-branch-name",
            "main",
            "--github-password",
            "example_password",
            "--server-name",
            "example_server",
            "--server-type",
            "paper",
            "--registry-path",
            "${HOME}/registry",
            "--server-path",
            "${HOME}/example_server",
            "--start-command",
            "./start.sh",
            "--stop-command",
            "./stop.sh",
        ]).unwrap();

        assert_eq!(
            configs,
            Config{
                github_username: "example_user".to_string(),
                github_repository_owner_name: "example_user".to_string(),
                github_repository_name: "example_repo".to_string(),
                github_branch_name: Some("main".to_string()),
                github_password: "example_password".to_string(),
                server_name: "example_server".to_string(),
                server_type: ServerType::Paper,
                registry_path: "${HOME}/registry".to_string(),
                server_path: "${HOME}/example_server".to_string(),
                start_command: "./start.sh".to_string(),
                stop_command: "./stop.sh".to_string(),
            }
        );
    }
}
