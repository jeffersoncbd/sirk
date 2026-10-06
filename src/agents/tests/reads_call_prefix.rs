use super::super::Agent;

#[test]
fn reads_call_prefix_as_one_token_or_an_argument_list() {
    let single = Agent::parse(
        "containerized",
        "---\nadapter: codex\ncall_prefix: wrapper\n---\nReview.",
    )
    .unwrap();
    assert_eq!(single.call_prefix, ["wrapper"]);

    let docker = Agent::parse(
        "containerized",
        "---\nadapter: codex\ncall_prefix: [docker, exec, -i, harness]\n---\nReview.",
    )
    .unwrap();
    assert_eq!(docker.call_prefix, ["docker", "exec", "-i", "harness"]);
    assert!(
        serde_yaml::to_string(&docker)
            .unwrap()
            .contains("call_prefix:\n- docker\n- exec")
    );
}
