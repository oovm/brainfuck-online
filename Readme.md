Brainfuck Encoder/Decoder
=========================

## Encoder

Search the brainfuck representation of a piece of text, algorithm complexity is O(n).

It’s a pity that it’s not real-time, sometimes feels stuck, it takes about 50ms

It is faster and shorter than the following implementations:

- https://www.dcode.fr/brainfuck-language

- https://www.splitbrain.org/services/ook

## Decoder

A brainfuck virtual machine without jit, allow overflow and negative index.

Even if this is not very fast, it is faster than many js implementations.

Notice that the output must be a valid utf8 string.