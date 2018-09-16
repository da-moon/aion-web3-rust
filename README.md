# aion-web3-rust

Aion Web3 functions in rust

# general todo

- [ ] use traits to rewrite methods with overloading whenever it is needed.

# utlis

## sha3.js

- [x] sha3 => GO over again and streamline

## config.js 

<!-- Should I add anything? -->

 ## browser-bn.js 

<!-- Figurte out how to write bignumber in rust -->

 ## bloom.js

- [ ] codePointToInt
- [ ] testBytes
- [ ] testAddress
- [ ] testTopic

## utlis.js

- [ ] toHex => big number wip - 'object' equivalant in rust? - isFinite condition?
- [x] fromDecimal => big number
- [x] ~~padLeft~~
- [x] ~~padRight~~
- [x] ~~toUtf8~~
- [x] ~~toAscii~~
- [x] ~~fromUtf8~~
- [x] ~~fromAscii~~
- [ ] transformToFullName => Need AION Web3 library
- [ ] extractDisplayName => Need AION Web3 library
- [ ] extractTypeName => Need AION Web3 library
- [x] toDecimal => Use big number - add more tests

  - [ ] to_hex_string

- [ ] getValueOfUnit => depends on big number

- [ ] fromWei => Add consts

- [ ] toWei => Add consts

- [x] toBigNumber => depends on big number -- ~~check value for string input~~ --

- [ ] toTwosComplement

- [x] isStrictAddress
- [ ] isAddress
- [ ] isChecksumAddress
- [ ] toChecksumAddress
- [ ] toAddress
- [ ] isBigNumber
- [ ] isString
- [ ] isFunction
- [ ] isObject
- [ ] isBoolean
- [ ] isArray
- [ ] isJson
- [ ] isBloom
- [ ] isTopic

## config.js

- [x] export => deal with bignumber.

# web3

## requestmanager.js

- [x] Struct

## formatters.js

- [x] inputAddressFormatter => Add more tests
- [x] ~~isPredefinedBlockNumber~~
- [ ] inputBlockNumberFormatter
- [ ] inputDefaultBlockNumberFormatter
