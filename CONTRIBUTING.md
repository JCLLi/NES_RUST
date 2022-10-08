# Contributing to the project

A part of the assignment is managing the project, i.e. properly using git and formatting the code in a proper way. Before contributing, some standards need to be upheld.

## Before you branch
* Make an issue in GitLab detailing what need to be done or what feature is necessary. A branch should relate to at least one issue.
* Decide what you want this branch to be. Don't add too many features together in one big branch, split it up into fixes/features.
* Decide where to branch from. Sometimes it is easy to branch from another feature branch as it might have added features you want to use. But if you want to merge this branch later on you will have to wait until the other branch is also merged.
* Give a proper branch name:
	* the name is short and descriptive;
	* if it is a feature (an addition to the code) name it <code>feature/<branch_name></code>, if it is a bug fix call it <code>bugfix/<branch_name></code>;

## Before you commit
* See if you staged only the proper content to a commit.
* A commit is a working version:
	* all new functions should have a test;
	* all tests pass;
	* code is formatted properly (use <code>$ cargo fmt</code>);
	* code passes the clippy tests (use <code>$ cargo clippy</code> to see what changes need to be made).
* A commit should have a function:
	* do not commit because you finished for the day, commit because you added something substantial;
	* do not overdo your commits. One new function is not always reason for a commit, try to group function/tests together for one better version;
* Write a good commit message:
	* use [present tense](https://stackoverflow.com/questions/13861318/why-is-it-considered-good-practice-to-describe-git-commits-in-the-present-tense);
	* make a detailed description, but keep it short.

### I committed but didn't follow the standards, is is salvageable? 

Yes, git allows to [rewrite the commit history](https://stackoverflow.com/questions/1884474/change-old-commit-message-using-git-rebase). This can mess up your history. If you don't know what you're doing ask for help.
	

## Before you push
* Check if the previous points were followed.

### I pushed but didn't follow the standards, is it salvageable?

Yes, git not only lets you rewrite commits, you can push those as well with the --force flag. This can ***REALLY*** mess up your history. If you don't know what you're doing you should ***REALLY*** ask for help! 

## Before you merge
* Check if the previous points were followed.
* Create a merge request:
	* check if your source and destination branch are correct;
	* check if no merge conflict are present;
	* assign yourself;
	* describe what is changed and why;
	* in the description mention which issues it relates to and which is closes. Using keyword "Closes" (e.g. <code>Closes #<issue_number1>, #<issue_number2></code>) automatically closes the issues when the merge is made. Keyword "Relates to" automatically mentions the issue as related to this merge request;  
	* wait for 2 reviewers to test your new commit;
* Change the code according to the reviewers (or argue why it shouldn't be changed). Easiest way to do this is to make a new commit and push, but sometimes it can be better to rewrite your commit history (as mentioned in Before you commit and Before you push), but be very careful with this. 
* If the reviewing process is done you can merge. 
* Check if all issues were closed properly, close the ones that did not automatically close.

## Reviewing
* Check if no merge conflict are present.
* See if the commit/branch/push/merge standards were followed correctly. Most importantly:
	* Does it compile?
	* Does it work as intended?
	* Does cargo fmt alter the code?
	* Do the clippy tests pass?
	* Are functions tested thoroughly?
* If needed, describe what changes need to be done (and where).
* If the code can be merged, give your approval.
