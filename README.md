# RustPaint

This program is rust translation of mspaint.

## Instructions for Build and Use

Steps to build and/or run the software:

1. Compile the Rust code.
2. Run the ocmpiled rust program.

Instructions for using the software:

1. Launch the app.
2. Use the different tools to create art.
3. Save your work when you are done.

## Development Environment

To recreate the development environment, you need the following software and/or libraries with the specified versions:

* mspaint.exe
* Ghidra Decompiler (Decompile the compiled mspaint.exe file into C pseudocode)
* Open-AI Codex (Translate decompiled C pseudocode into usable Rust code)

## Useful Websites to Learn More

I found these websites useful in developing this software:

* ChatGPT.com (project ideas, design implementation, testing, and Google Android SDK setup and emulators.)
* Ghidra (https://github.com/NationalSecurityAgency/ghidra) needed for easily decompiling the original .exe file.

## Future Work

The following items I plan to fix, improve, and/or add to this project in the future:

* [1] The program as it stands is very buggy and bare bones. It would be nice to add more features to bring it more into conformity with the original mspaint.
* [2] Explore more about why the threshold for how much original code can actually end up being translated reacher near only 90%.
* [3] See if there are better alternative options for decompiling with an AI that runs locally on my own machine instead of using up all my usage on a cloud AI.