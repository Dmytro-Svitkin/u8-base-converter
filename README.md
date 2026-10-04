# u8-base-converter
`u8-base-converter` is a minimalistic stack-allocated u8 base coverter, capable of converting arbitrary-length <sub>(up to 1024)</sub> numerals of `u8` bases to other `u8` bases.
`u8-base-converter` features *zero* dependencies and is suitable for `no_std` environments.

## Functionality
The crate revolves around two core types:
* `Base` i.e., the number system containing its alphabet,
* `Numeral` i.e., the numeral of a certain base containging its value and base.

### `base` constants
`u8-base-converter::base` includes some standard `Base` constants.
| Name | Constant | Radix | Character set |
| --- | --- | --- | --- |
| Unary | `UNARY` | 1 | `\|` |
| Binary | `BINARY` | 2 | `01` |
| Morse Code | `MORSE` | 3 | `/.-` |
| Octal | `OCTAL` | 8 | `0`-`7` |
| Decimal | `DECIMAL` | 10 | `0`-`9` |
| Hexadecimal | `HEXADECIMAL` | 16 | `0`-`9`, `A`-`F` |
| Alphanumeric Base62 | `ALPHANUMERIC` | 62 | `0`-`9`, `A`-`Z`, `a`-`z` |
| Base64 | `BASE64` | 64 | `0`-`9`, `A`-`Z`, `a`-`z`, `+/` |
| Printable ASCII | `PRINTABLE_ASCII` | 95 | ASCII consecutively from ` ` to `~` |
| Base256 | `BASE256` | 256 | All bytes |

```rust
// Example Usage

const morse_symbols: Base = MORSE;
const base62: Base = ALPHANUMERIC;
```

### `Base` methods
| Name | Method | Description | Input | Output |
| --- | --- | --- | --- | --- |
| New base | `new` | Creates a new base from byte slice | `&[u8]` | `Base` |
| New base from another base | `from_base` | Creates a new base from another base, its start and end index | `Base`, `usize` start, `usize` end | `Base` |
| New base from radix | `from_radix` | Creates a new base from the given radix (e.g., 8 returns octal base) | `usize` radix | `Base` |
| New base from string slice | `from_str` | Creates a new base from string slice | `&str` base | `Base` |
| Base length | `len` | Returns the number of digits held by the base | ╱ | `usize` length |
| Base radix | `radix` | Returns the base's radix (e.g., 10 for decimal) | ╱ | `usize` radix |
| Digit zero | `zero` | Returns the first digit (i.e., with index `0`), which corresponds to digit *zero* | ╱ | `u8` digit byte |
| Base as string slice | `as_str` | Transforms the given base into a string slice, returns an empty string slice if the given base contains any illegal character | ╱ | `&str` base |
| Base as unchecked string slice | `as_str_unchecked` | Transforms the given base into a string slice (regardless) | ╱ | `&str` base |
| Base as printable ASCII string slice | `as_printable_ascii` | Transforms the given base into a string slice all the way to the first non-printable-ASCII character | ╱ | `&str` base |

```rust
// Example Usage

const custom_base5: Base = Base::new(b"abcde");
const custom_base4: Base = Base::from_base(custom_base5,0,4);
const standard_base36: Base = Base::from_radix(36);
const str_hex: Base = Base::from_str("0123456789ABCDEF");

println!("{}", custom_base5.len()); // 5
println!("{}", custom_base4.radix()); // 4
println!("{}", standard_base36.zero()); // 48 (which is the byte of 0)
println!("{}", str_hex.as_printable_ascii()); // "0123456789ABCDEF"
```

### `Numeral` methods
| Name | Method | Description | Input | Output |
| --- | --- | --- | --- | --- |
| New numeral | `new` | Creates a new numeral from a value byte slice and a base | `&[u8]` value, `Base` | `Numeral` |
| New binary numeral | `new_bin` | Creates a new *binary* numeral from a value byte slice | `&[u8]` value | `Numeral` |
| New octal numeral | `new_oct` | Creates a new *octal* numeral from a value byte slice | `&[u8]` value | `Numeral` |
| New decimal numeral | `new_dec` | Creates a new *decimal* numeral from a value byte slice | `&[u8]` value | `Numeral` |
| New decimal numeral from value in `u128` | `new_dec_from_u128` | Creates a new *decimal* numeral from a value *`u128`* | `u128` value | `Numeral` |
| New hexadecimal numeral | `new_hex` | Creates a new *hexadecimal* numeral from a value byte slice | `&[u8]` value | `Numeral` |
| New numeral from string slice | `from_str` | Creates a new numeral from a value and base string slices | `&str` value, `&str` base | `Numeral` |
| Numeral value byte slice | `value` | Returns the numeral in the form of a byte slice | ╱ | `&[u8]` value |
| Numeral length | `len` | Returns the number of digits held by the numeral | ╱ | `usize` length |
| Numeral base | `base` | Returns the numeral's base | ╱ | `Base` |
| Numeral radix | `radix` | Returns the numeral's base's radix (e.g., 10 for decimal) | ╱ | `usize` radix |
| Empty numeral check | `is_empty` | Returns `true` if the numeral is empty, returns `false` otherwise | ╱ | `bool` |
| Numeral value as string slice | `value_as_str` | Transforms the given numeral into a string slice, returns an empty string slice if the given base contains any illegal character | ╱ | `&str` numeral value |
| Numeral value as unchecked string slice | `value_as_str_unchecked` | Transforms the given numeral into a string slice (regardless) | ╱ | `&str` numeral value |
| Numeral value as printable ASCII string slice | `value_as_printable_ascii` | Transforms the given numeral into a string slice all the way to the first non-printable-ASCII character | ╱ | `&str` numeral value |
| Converted numeral | `converted_to` | Returns a converted numeral based on the given numeral value and base, returns an empty numeral in case of an overflow | `Base` | `Numeral` |
| Converted numeral as `u128` | `as_u128` | Returns an `u128` value based on appropriately converted numeral | ╱ | `u128` value |

```rust
// Example Usage

const base12_value: Numeral = Numeral::new(b"A1239", Base::from_radix(12));
const bin_key: Numeral = Numeral::new_bin(b"011010010101010010101010");
const hex_key: Numeral = bin_key.converted_to(HEXADECIMAL);
const morse_message: Numeral = Numeral::from_str("-...../--..-..-....../...../--..-...-.", MORSE.as_str());
const id: Numeral = Numeral::new_dec_from(129718946141235950);
const readable_id: Numeral = id.converted_to(ALPHANUMERIC)

println!("{:?}",base12_value.as_u128());
println!("{:?}",hex_key.as_u128());
println!("{:?}",morse_message.as_printable_ascii());
println!("{:?}",readable_id.as_str());
```

## Installation
Add `u8-base-converter` to your `Cargo.toml` dependencies:

```toml
[dependencies]
u8-base-converter = "0.1.1"
```

<sub>This project is licensed under the BSD 3-Clause License.</sub>