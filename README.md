# cob

`cob` is a small cli program to checkout git branches conveniently.

## Usage

```bash
# examples:
#   $ cob                                                     # queries github and gives you a list of issues to pick from
#   ? Select issue to create branch from ›
#   ❯ 3: test issue number 3
#     2: test issue number 2
#     1: test issue
#
#   $ cob 3                                                    # queries github and checks out a branch with the title of the issue
#     Switched to a new branch 'test_issue_number_3'
#
#   $ cob "my string with #hashes and, other: . punctuations" # instant cob
#     Switched to a new branch 'my_string_with_#hashes_and_other_punctuations'
#
#   $ cob some sentence without special characters            # instant cob
#   Switched to a new branch 'some_sentence_without_special_characters'
```
