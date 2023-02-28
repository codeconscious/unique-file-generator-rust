# Unique File Generator (Rust Version)

I've largely rewritten my [C# unique file generator](https://github.com/codeconscious/unique-file-generator) in Rust as a way to get more familiar with the language. Most of the features have been merged, and it's now functional!

This command line tool allows you to quickly and easily create an arbitrary number of unique (by name and content) files on your computer. Each filename contains a random collection of characters to differentiate them. You can also supply optional parameters to customize files according to your needs.

## Usage
At the minimum, you must specify the number of files you want to generate using `-c` or `--count`. This should be a single positive integer.

### Argument Flags
Each argument flag is optional. You must supply at least one value for each argument used. If you supply multiple values for an argument, they will be concatenated into one string divided by spaces.

Flag | Description
---- | :----
-p | Add a filename prefix.
-e | File extension of the generated files. The opening period is optional. If not specified, no extension is added.
-s | Desired size of each file in bytes. If specified, files will be populated with random alphanumeric characters; otherwise, each file will only contain its own name.
-o | The output subfolder, which will be created, if needed. Otherwise, a default is used.
-d | A delay in milliseconds to be applied between each file's creation. Defaults to 0 if unspecified.
-h | See the instructions.
