how to add new number:

use git rebase to only push the commit we want?

<!-- - git branch fact -->
<!-- - git checkout fact -->

- mkdir numbers
- create the files you want to create inside numbers
- git add numbers --sparse
- git commit -m "automatic updates"

- git rebase --onto origin/main HEAD~1 HEAD

<!-- - git rebase --onto origin/main main fact -->
<!-- - git push origin fact -->
<!-- - git branch -d fact -->
<!-- - git checkout main -->
<!-- - git sparse-checkout reapply -->

<!-- - git checkout main -->
<!-- - git rebase fact -->
<!-- - git push origin fact -->
<!-- - git branch -d fact -->

- git pull origin main
- mkdir numbers
- create the files you want to create inside numbers
- git add numbers --sparse
- git commit -m "automatic updates"
- git push origin main
- git sparse-checkout reapply
