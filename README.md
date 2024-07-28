Fun looks up the file tree for a .fun directory and executes a file with the name of the first argument and executes it with the other arguments becoming the child's arguments and directory is set to the parent of the .fun directory.
That sounds a bit complicated so let's look at an example

let there be `../fun/test` then running `fun test 1 2` will execute `../fun/test 1 2` with current directory for test `..`

## WHY?
I use it to have some scripts handy. Like Creating/Deploying to a test enviroment or building a project.
Some people might use make for this but I don't like it's single file approach and custom syntax with dependencies.
So when I'm not in C land I use this.
