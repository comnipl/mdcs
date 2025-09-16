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
                github_repository_url: "https://github.com/example_user/example_repo",
                github_username: "example_user",
                github_password: "example_password",
                server_name: "example_server",
                server_type: ServerType::Paper,
                registry_path: "registry",
                server_path: "${HOME}/example_server",
                start_command: "java -Xms4G -Xmx4G -jar paper.jar --nogui",
                stop_command: "kill - s SIGTERM ${PID}",
            }
        );
    }
}
