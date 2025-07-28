# Intel 8080 Emulator

Emulating the Intel 8080 for fun

This is a work in progress. The end goal is to have a disassembler, an emulator,
and a few different visual frontends.

## Background

This is solely a learning project for myself. I'm not doing any innovation in
the emulation field or aiming to have the most efficient, clean, or accurate
emulator! I'm just here to build a system :P

I'm following Emulator101.com as a guideline. Unfortunately the
domain itself has expired, but you can find the guide [on the Internet
Archive][emulator101.com] and the [source code on Github][emulator101 source].

## Install

You must have [Rust] installed on your system.

Once you have Rust, you can build the binaries by running

```bash
cargo build
```
You can optionally install [Just], which allows you to use the [Justfile] in
this repo to run small project-specific scripts (for example, `just build`). You
can also look in the Justfile to see the commands and run them yourself, so it's
only for convienence.

## Usage

### Disassembler

Given an 8080 assembly program `program.e`, you can disassemble it with

```bash
cargo run -p disassembler program.e
```

The disassembly will be printed on stdout. You can provide multiple files and
they will be disassembled one after the other.

Quoting a warning from [emulator101.com][disassembler warning]:

> There is a problem here. If you have an arbitrary chunk of memory that
> includes 8080 code, it probably has data interleaved in it.
>
> For example, the sprites for the characters in a game may be mixed in with
> the code. When your disassembler hits that chunk of memory, it is going to
> think it's code and continue to chew on it. Unless it gets lucky, any code
> disassembled after that chunk of data may or may not be right.
>
> For now, there isn't a whole lot you can do about this. Just be aware the
> problem exists. If you see things like:
>
> - a jump from known good code to an instruction that doesn't exist in your
>   disassembly listing
>
> - a stream of nonsense code (like POP B POP B POP B POP C XTHL XTHL XTHL)
>
> That there is probably data in there that renders some portion of your
> disassembly unusable. You might have to restart it an an offset if this happens
> to you.

## Thanks

- [emulator101.com] as an invaluable resource and guide.
- [gameboy-rom-parser] as a good working example of how to use the [nom] parsing library.

## Contributing

I'm intending for this to be a solo learning project without high quality
standards, this is not intended to be a serious product. If you do run into any
issues trying to use this, feel free to make a Github issue, but please don't
expect a rapid response.

At the moment I will not be accepting PRs, as I'm still in active development
and want to implement this system myself. You're welcome to fork the project
and work on your own repo!

## License
MIT

[emulator101.com]: https://web.archive.org/web/20241010195903/http://www.emulator101.com/
[emulator101 source]: https://github.com/kpmiller/emulator101
[disassembler warning]: https://web.archive.org/web/20240118230844/http://www.emulator101.com/disassembler-pt-2.html
[Rust]: https://www.rust-lang.org/tools/install
[Just]: https://github.com/casey/just?tab=readme-ov-file#installation
[Justfile]: ./Justfile
[gameboy-rom-parser]: https://github.com/MarkMcCaskey/gameboy-rom-parser
[nom]: https://github.com/rust-bakery/nom?tab=readme-ov-file
