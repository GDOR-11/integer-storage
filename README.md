how to add new number:

- commit all your changes (no need to push)
- mkdir numbers
- create the files you want to create inside the numbers folder
- git add numbers --sparse
- git commit -m "automatic updates"
- git rebase --onto origin/main HEAD~1 HEAD
- git push origin HEAD:main
- git checkout main
- git reset HEAD~


<!-- - git pull origin main -->
<!-- - mkdir numbers -->
<!-- - create the files you want to create inside numbers -->
<!-- - git add numbers --sparse -->
<!-- - git commit -m "automatic updates" -->
<!-- - git push origin main -->
<!-- - git sparse-checkout reapply -->
