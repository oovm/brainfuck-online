Brainfuck Encoder/Decoder
=========================

## Encoder

Search the brainfuck representation of a piece of text, algorithm complexity is O(n).

It’s a pity that it is not real-time, sometimes feels stuck, it takes about 100ms

It's faster and shorter than the following implementations:

- https://www.dcode.fr/brainfuck-language

- https://www.splitbrain.org/services/ook

It's shorter than the following implementations:

- https://copy.sh/brainfuck/text.html

## Decoder

A brainfuck virtual machine without jit, allow overflow and negative index.

Even if this is not very fast, it is faster than many js implementations.

Notice that the output must be a valid utf8 string.

## TODO

- [ ] Short string length optimize
- [ ] Long string speed optimize