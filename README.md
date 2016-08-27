# node-brainfuck-native

This is node binding of the Rust implemented Brainfuck interpreter
project [bfck](https://github.com/zippunov/bfck)


## Prerequisites
- Install [Rust tools](https://www.rust-lang.org/en-US/) 
- Add to **package.json** dependency to **node-brainfuck-native**
```
    "dependencies": {
        ...
        "node-brainfuck-native": "zippunov/node-brainfuck-native",
    },
```
- Run `npm install`

## Usage

```
'use strict';

const bfck = require('node-brainfuck-native');

// ROT13 brainfuck programm
const code = `
,+[-->++++[>++++++++<-]<[->+>-[>+>>]>[+[-<+>]>+>>]<<<<<<]>>[-]>>--[-[>
-<[-]]]>+[-<+++++++++++++<[->-[>+>>]>[+[-<+>]>+>>]<<<<<]>[-]>[-]+>[<--
>-[<+>-]]<[<<<<+++++++++++++>>>>-]]<<[-]<<+.[-]<,+]
`;
const input = 'Native BF interpreter';
const result = bfck(code, input);

console.log(result)

```
gives output
```
Angvir OS vagrecergre
```
