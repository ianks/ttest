# `ttest` (VERY WIP)

It's like [`vim-test`][vim-test], but for the terminal. The `ttest` CLI makes provides a unified interface for running tests.

```sh
$ ttest test/some_test.rb:18 # Ruby MiniTest
$ ttest test/some_spec.rb:18 # Ruby RSpect
$ ttest some_search_string # Runs tests across all languages
```

[vim-test]: https://github.com/vim-test/vim-test
