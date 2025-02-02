in order to add a new number to the repository, follow these instructions:

- commit all your changes (no need to push)
- mkdir numbers
- create the files you want to create inside the numbers folder
- git add numbers --sparse
- git commit -m "automatic updates"
- git rebase --onto origin/main HEAD~1 HEAD
- git push origin HEAD:main
- git checkout main
- git reset HEAD~


the numbers are stored in a big-endian BCD-like format, using groups of 10 bits to encode 3 digits instead of groups of 4 bits to encode 1 digit.
