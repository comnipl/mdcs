use clap::{Parser, ValueEnum};

#[derive(Debug, Parser, PartialEq)]
pub struct Config{
    #[arg(long, env)]
    pub github_repository_url: String,
    #[arg(long, env)]
    pub github_username: String,
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
        let configs = Config::try_parse_from(&[
            "mdcs-controller",
            "--github-repository-url",
            "https://github.com/example_user/example_repo",
            "--github-username",
            "example_user",
            "--github-password",
            "example_password",
            "--server-name",
            "example_server",
            "--server-type",
            "paper",
            "--registry-path",
            "registry",
            "--server-path",
            "${HOME}/example_server",
            "--start-command",
            "java -Xms4G -Xmx4G -jar paper.jar --nogui",
            "--stop-command",
            "kill -s SIGTERM ${PID}",
        ]).unwrap();

        assert_eq!(
            configs,
            Config{
                github_repository_url: "https://github.com/example_user/example_repo".to_string(),
                github_username: "example_user".to_string(),
                github_password: "example_password".to_string(),
                server_name: "example_server".to_string(),
                server_type: ServerType::Paper,
                registry_path: "registry".to_string(),
                server_path: "${HOME}/example_server".to_string(),
                start_command: "java -Xms4G -Xmx4G -jar paper.jar --nogui".to_string(),
                stop_command: "kill -s SIGTERM ${PID}".to_string(),
            }
        );
    }
}
