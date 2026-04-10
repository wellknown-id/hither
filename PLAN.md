# Hither

hither is a command line program that can conjure up other command line programs. The apps themselves are WASM modules.

A quick prototype was written in Go using wazero as the WASM runtime, and has since been ported to Rust with wasmtime.

The original prompt that created it was this:

```
write a go app called hither that installs a command line program 'h' that looks in the current directory or user home directory (all platforms) for a .hither directory containing .wasm modules and runs those modules such that should the user run 'h google.com search eggs' it will attempt to locate ./.hither/google.com/search.wasm or fall back to ~/.hither/google.com/search.wasm or print an error if it is not found. Use wazero as the wasm runtime. Make sure tests are written. If google.com is problematic to use because there is no public open access search API, use some other widely known public example online API and write wasm modules for it.
```

## Plan

It was quick and it was fun, but then we realised, oh wait this is cool! We bought hithered.to and hithered.com on the spur of the moment and now we are going to make this a bit more useful for more people.

## Workstream 1

- [x] Port everything to Rust and use wasmtime as the runtime.
- [x] Build for Windows, Mac, Linux.
- [x] Make "hither" build as "hither" instead of "h" and add an "--install" command to copy the "hither" executable to a user bin directory (all platforms).
- [x] Update the "--install" command so it accepts an optional "--alias=h" argument that installs an alias "h" for "hither".

## Workstream 2

- [x] Rewrite the README.md.

## Workstream 3

- [x] Refactor so that example guest "echo" is copied to "./.hither/echo.wasm" instead of "./.hither/example.com/echo.wasm" such that "h echo hello" works.
- [x] Add a "to" example like "echo" copied to "./.hither/to.wasm" that works in the following way:
   - [x] When a user runs "\[hither\] to" without any arguments it prints a message like:
        
        > What would you like to encant? Tell me your wishes.
        > Do you wish for pictures of cats? Then say it, and I shall make it so!
        > 
        > Run `hither to pictures of cats` and your wish is my command!
        >
        > Run `hither to financial news today` and I will do as you bid!

        Note: The "to" command *just* prints in the interation!
- [x] Add a "help" example like "echo" copied to "./.hither/help.wasm" that prints a message like:

        > Hither! If you don't know what to do, you can say "hither list" and I will tell you what you can do!

- [x] Add a "list" example like "echo" copied to "./.hither/list.wasm" that prints a list of the users hither modules.

## Workstream 4

- [x] Research and identify candidate libraries/sdks that can work with OpenAI API compatible inference providers (local AI included, critical must-have) and save the findings to new file ./research/llms.md
- [x] Choose an path from ./research/llms.md and design a host interface to expose WASM guests in a new file ./ai.wit (don't integrate yet!)

## Workstream 5

- [ ] Research and identify small and fast vector search that we can bundle, sqlite maybe does this well already.
- [ ] Find or train a small embedding model.

## Workstream 6

- [ ] Research and identify candidate libraries/sdks that could be used to build a Rust based OpenAI API compatible gateway.