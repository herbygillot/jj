// Copyright 2022 The Jujutsu Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common::TestEnvironment;
use itertools::Itertools;
use regex::Regex;

pub mod common;

#[test]
fn test_show() {
    let test_env = TestEnvironment::default();
    test_env.jj_cmd_success(test_env.env_root(), &["init", "repo", "--git"]);
    let repo_path = test_env.env_root().join("repo");

    let stdout = test_env.jj_cmd_success(&repo_path, &["show"]);
    let stdout = stdout.lines().skip(2).join("\n");

    insta::assert_snapshot!(stdout, @r###"
    Author: Test User <test.user@example.com> (2001-02-03 04:05:07.000 +07:00)
    Committer: Test User <test.user@example.com> (2001-02-03 04:05:07.000 +07:00)

    (no description set)
    "###);
}

#[test]
fn test_show_relative_timestamps() {
    let test_env = TestEnvironment::default();
    test_env.jj_cmd_success(test_env.env_root(), &["init", "repo", "--git"]);
    let repo_path = test_env.env_root().join("repo");

    test_env.add_config(
        br#"[ui]
        relative-timestamps = true
        "#,
    );

    let stdout = test_env.jj_cmd_success(&repo_path, &["show"]);
    let timestamp_re = Regex::new(r"\([0-9]+ years ago\)").unwrap();
    let stdout = stdout
        .lines()
        .skip(2)
        .map(|x| timestamp_re.replace_all(x, "(...timestamp...)"))
        .join("\n");

    insta::assert_snapshot!(stdout, @r###"
    Author: Test User <test.user@example.com> (...timestamp...)
    Committer: Test User <test.user@example.com> (...timestamp...)

    (no description set)
    "###);
}
