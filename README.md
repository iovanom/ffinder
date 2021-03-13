# Fast Finder

## The Goal

The main goal is to learn rust with some real application that I need.
I use this app with [fzf](https://github.com/junegunn/fzf) to `Alt+c` command to `cd` to specific directory.

The standard find command is not useful for me because it search recursively in deep but I need to search
lineary level by level.
For example if we have a directory with structure:

```
  /var/
    /var/dir1/
      /var/dir1/subdir1
      /var/dir1/subdir2
    /var/dir2/
      /var/dir2/subdir3
      /var/dir2/subdir4
    /var/dir3

```

The result for this should be:

```

/var/
/var/dir1/
/var/dir2/
/var/dir3/
/var/dir1/subdir1/
/var/dir1/subdir2/
/var/dir2/subdir3/
/var/dir2/subdir4/

```


## Features

- [x] print all directories in deep
- [ ] search directory by name
- [ ] the same logic for files

