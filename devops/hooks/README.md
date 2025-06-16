# Git hooks

## `pre-commit`

This is required to avoid spamming commits that are not passing the basic `cargo` checks. To use it, run `cp devops/hooks/pre-commit .git/hooks/pre-commit` and make sure that the file is executable. If you are not sure, call `chmod +x .git/hooks/pre-commit` and it will work automatically when calling `git commit`.
