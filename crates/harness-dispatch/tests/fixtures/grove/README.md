# Grove task-file fixtures

Task files in the shape of Grove leaves, one for each convention the
`harness-dispatch/grove` adapter interprets. `tests/grove.rs` runs each one
through the command seam, under the kinds its table names, and fails if a
fixture here has no row there. `@RUN@` stands for a run ID the test records
before it reads the fixture.
