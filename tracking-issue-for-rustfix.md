
1. Add `println!("{{ 'name': '{:?}', 'applicability': '{:?} }}'", lint.name, applicability);` to
   the `span_lint_and_sugg` method
1. Run `cargo test`
1. Run `tests/ui/update-all-references.sh` to generate the `*.stdout` files
1. Concatenate the `*.stdout` files: `find tests/ui/ -type f -name '*.stdout' -exec cat {} + >> all.stdout`
1. Run `find tests/ui/ -type f -name '*.stdout' -exec cat {} + >> all.stdout | cat all.stdout | rg '^\{' | sort | uniq | rg MachineApplicable` | vim
1. Remove duplicates manually...
1. In vim select all lints -> `gu` to lowercase all
1. In vim: :%s/_/-/g
1. For each lint in the file 


Tracking Issue: Adding rustfix tests

1. Pick one of the files below that is not done, yet
1. Add `// run-rustfix` to the file, just below the license header, separated by 1 empty line
1. Run `cargo test`
1. Run `tests/ui/update-all-references.sh`
1. Run `cargo test` again
1. If it passes, open a PR with the changes, including the `*.fixed` file
1. If the output contains `error: failed to compile fixed code`, good! See 'Found a bug' below
1. If you have any other problems, just reply to this issue directly with your problem

### Found a bug

First, leave a comment below that you tested the file with rustfix and that it didn't work.

If you want to delegate the fixing to someone else, you're done here. Thanks!
If you want to fix the suggestion, have a look at the CONTRIBUTING.md. 


### Current List of lints that support auto-correct

TODO: don't make a tracking issue.

0. Create a rustfix label/un
1. Add run-rustfix to all the files below
2. Run cargo test

* [ ] tests/ui/get_unwrap.rs
* [ ] tests/ui/decimal_literal_representation.rs
* [ ] tests/ui/fn_to_numeric_cast.rs
* [ ] tests/ui/bool_comparison.rs
* [ ] tests/ui/println_empty_string.rs
* [ ] tests/ui/single_char_pattern.rs
* [ ] tests/ui/use_self.rs
* [ ] tests/ui/types.rs
* [ ] tests/ui/inconsistent_digit_grouping.rs
* [ ] tests/ui/writeln_empty_string.rs
* [ ] tests/ui/double_comparison.rs
* [ ] tests/ui/into_iter_on_ref.rs
* [ ] tests/ui/duration_subsec.rs
* [ ] tests/ui/map_clone.rs
* [ ] tests/ui/matches.rs
* [ ] tests/ui/bytecount.rs
* [ ] tests/ui/cast_lossless_integer.rs
* [ ] tests/ui/collapsible_if.rs
* [ ] tests/ui/strings.rs
* [ ] tests/ui/methods.rs
* [ ] tests/ui/while_loop.rs
* [ ] tests/ui/unnecessary_ref.rs
* [ ] tests/ui/needless_bool.rs
* [ ] tests/ui/excessive_precision.rs
* [ ] tests/ui/cast.rs
* [ ] tests/ui/for_loop.rs
* [ ] tests/ui/unreadable_literal.rs
* [ ] tests/ui/vec.rs
* [ ] tests/ui/useless_asref.rs
* [ ] tests/ui/precedence.rs
* [ ] tests/ui/cfg_attr_rustfmt.rs
* [ ] tests/ui/unnecessary_operation.rs
* [ ] tests/ui/replace_consts.rs
* [ ] tests/ui/unnecessary_fold.rs
* [ ] tests/ui/string_extend.rs
* [ ] tests/ui/len_zero.rs
* [ ] tests/ui/reference.rs
* [ ] tests/ui/ptr_offset_with_cast.rs
* [ ] tests/ui/large_digit_groups.rs
* [ ] tests/ui/cast_lossless_float.rs
* [ ] tests/ui/expect_fun_call.rs
* [ ] tests/ui/mem_replace.rs
* [ ] tests/ui/literals.rs
* [ ] tests/ui/redundant_field_names.rs
* [ ] tests/ui/infallible_destructuring_match.rs
* [ ] tests/ui/explicit_write.rs
* [ ] tests/ui/starts_ends_with.rs
* [ ] tests/ui/unit_arg.rs


failed:

* [ ] tests/ui/bytecount.rs
* [ ] tests/ui/cast.rs
* [ ] tests/ui/cfg_attr_rustfmt.rs
* [ ] tests/ui/collapsible_if.rs
* [ ] tests/ui/decimal_literal_representation.rs
* [ ] tests/ui/duration_subsec.rs
* [ ] tests/ui/excessive_precision.rs
* [ ] tests/ui/expect_fun_call.rs
* [ ] tests/ui/explicit_write.rs
* [ ] tests/ui/fn_to_numeric_cast.rs
* [ ] tests/ui/for_loop.rs
* [ ] tests/ui/get_unwrap.rs
* [ ] tests/ui/inconsistent_digit_grouping.rs
* [ ] tests/ui/infallible_destructuring_match.rs
* [ ] tests/ui/into_iter_on_ref.rs
* [ ] tests/ui/large_digit_groups.rs
* [ ] tests/ui/len_zero.rs
* [ ] tests/ui/literals.rs
* [ ] tests/ui/map_clone.rs
* [ ] tests/ui/matches.rs
* [ ] tests/ui/mem_replace.rs
* [ ] tests/ui/methods.rs
* [ ] tests/ui/needless_bool.rs
* [ ] tests/ui/precedence.rs
* [ ] tests/ui/redundant_field_names.rs
* [ ] tests/ui/reference.rs
* [ ] tests/ui/replace_consts.rs
* [ ] tests/ui/starts_ends_with.rs
* [ ] tests/ui/strings.rs
* [ ] tests/ui/types.rs
* [ ] tests/ui/unit_arg.rs
* [ ] tests/ui/unnecessary_fold.rs
* [ ] tests/ui/unnecessary_operation.rs
* [ ] tests/ui/use_self.rs
* [ ] tests/ui/useless_asref.rs
* [ ] tests/ui/while_loop.rs

the ones that work apparently:


* [ ] tests/ui/bool_comparison.rs
* [ ] tests/ui/cast_lossless_float.rs
* [ ] tests/ui/cast_lossless_integer.rs
* [ ] tests/ui/double_comparison.rs
* [ ] tests/ui/println_empty_string.rs
* [ ] tests/ui/ptr_offset_with_cast.rs
* [ ] tests/ui/single_char_pattern.rs
* [ ] tests/ui/string_extend.rs
* [ ] tests/ui/unnecessary_ref.rs
* [ ] tests/ui/unreadable_literal.rs
* [ ] tests/ui/vec.rs
* [ ] tests/ui/writeln_empty_string.rs
