# test-with-tokio

This crate provides a single attribute macro that allows you to provide a guard
expression, and is otherwise vaguely like `#[tokio::test]`. It enables you to
run most of your tests in parallel, but to have a few that cannot be run
concurrently.

It also enables you to create several case variants of the same test.