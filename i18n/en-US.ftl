no-merged-branch = No merged branches found on { $branch }.
found-merged-protected =
    { $count ->
        [one] Found { $count } merged but protected branch on { $branch }:
        *[other] Found { $count } merged but protected branches on { $branch }:
    }
branches-wont-be-deleted =
    { $count ->
        [one] This branch will not be deleted.
        *[other] These branches will not be deleted.
    }
found-merged =
    { $count ->
        [one] Found { $count } merged branch on { $branch }:
        *[other] Found { $count } merged branches on { $branch }:
    }
delete-selection = Delete [a]ll, [s]elected, [n]one:
no-branch-deleted = No branch deleted.
delete-branch-yes-no = Delete branch { $branch }? [y]es, [n]o:
branch-deleted = Branch { $branch } deleted.
branch-cannot-be-deleted = { $branch } cannot be deleted.
branch-has-not-been-deleted = { $branch } has not been deleted.
no-valid-branch-found = No valid branch found. Is the target folder a valid Git repository?
not-a-git-repository = Not a Git repository.
git-not-found = Git cannot be found. Please install it.