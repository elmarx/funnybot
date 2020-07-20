[![Rust build](https://github.com/elmarx/funnybot/workflows/Rust/badge.svg)](https://github.com/elmarx/funnybot/actions?query=workflow%3ARust) [![crates.io badge](https://img.shields.io/crates/v/funnybot.svg)](https://crates.io/crates/funnybot) [![docs.rs badge](https://docs.rs/funnybot/badge.svg)](https://docs.rs/funnybot)

# Funnybot

Simple/naive helper for custom mocking: record arguments, return pre-recorded values.

Since recording arguments requires (inner) mutation, funnybot's main job is to hide that behind `RwLock`, and generally to
take out verbosity out of manual mocking.

## Example

```rust

struct MockRepository<'a> {
    // funnybot-instance to hold recorded arguments (here: `String`) and return-values (here: list of String)
    pub group_members: FunnyBot<'a, String, Vec<String>>
}

impl<'a> Repository for MockRepository<'a> {
    async fn list_group_members(&self, group_id: &str) -> Vec<String> {
        self.group_members.call(group_id.to_owned())
    }
}

#[test]
fn test_something() {
    let mock_repository = MockRepositry {
        group_members: FunnyBot::repeat(vec!["stan", "kyle", "eric", "kenny"])
    };

    let subject = Subject::new(&mock_repository);

    let actual = subject.my_function();

    assert_eq!(mock_repository.group_members.args(), vec["main-characters"]);
    assert_eq!(actual, …);
}

```
