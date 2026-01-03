[![qubit](media/banner.png)](https://ismaelxyz.github.io/qubit/)

# qubit

<div align="center">

**[💥 Visit Website To Use Calculator](https://ismaelxyz.github.io/qubit/)**

[![qubit](media/demo.gif)](https://ismaelxyz.github.io/qubit/)

</div>

## Example

<div align="center">

**[💥 Visit Website To Use Calculator](https://ismaelxyz.github.io/qubit/)**

</div>

```
2 + 2

// Variables (persist across lines)
x = 10
x * 3

// User-defined functions (1 argument)
f(x) = x^2 + 1
f(4)

// Functions can use variables
a = 3
g(x) = x + a
g(7)

sin( 90 ) + cos ( 120 )
sqrt(144) + 12
ceil ( 12.12 ) + 22
floor( 12.12) + 22

25 % of 100
25 % on 100

// Conversions
1024 kb to mb
22 kg to g
```

## Operations

### Basic Math

```
add         sub         multiply
divide      power       modulus
rightShift  leftShift
percentOf   percentOn
```

_Examples:_

```
2 + 2
2 plus 2
2 ^ 2

10 %of 100
10 %on 100

100 >> 2
100 << 2
```

### Constants

```
pi
e
tau
```

### Functions

All trigonometric functions expect input in degrees.

```
sin     cos     tan
asin    acos    atan
sinh    cosh    tanh

log     sqrt    cbrt
round   ceil    floor
```

_Examples:_

```
round ( 2.4 )
ceil ( 2.3 )

sin ( 90 )
cos ( 90 )

ceil ( 2.2 )
floor(3.3)
```

### Variables & User Functions

You can define variables and simple (unary) functions. Definitions persist across lines in the input editor.

**Variables**

```
x = 2
x + 3
y = x * 10
y
```

**User-defined functions (1 argument)**

```
f(x) = x * 2
f(5)

// can reference variables
a = 3
g(x) = x + a
g(4)
```

Notes:

- Function definitions currently support exactly 1 parameter.
- A user-defined function name overrides a built-in function with the same name.
- Definition lines don’t produce a numeric result (they show `-` in the results panel).

## Supported Conversions

- Angle
- Area
- Digital Information
- Length
- Mass
- Speed
- Time
- Temperature

# Development

### Stack qubit is using

- [Rust](https://www.rust-lang.org/) as programing language
- [Pest](https://pest.rs/) for parser + grammar
- [Iced](https://iced.rs/) for GUI applications
- [Tailwind](https://tailwindcss.com/) for CSS styles

### Local Development + Enhancement

- Clone the repo
- Pest Grammar is defined in `src/grammar.pest` file.
- Conversion chart is `src/convert_chart.rs`

To start the project locally on `:8080` _run_

```
trunk serve
```

Before creating pull request you can run sanity checks.

```
cargo fmt
cargo check
cargo test
```

Final build ( Optional )

```
trunk build --release --public-url=qubit
```

# Contribution

This project welcomes your PR and issues.
For example, refactoring, adding features, correcting English, etc.
If you need any help, you can contact me on [X](https://x.com/IsmaelBelisari3).

Thanks to all the people who already contributed!

<span>
  <img src="https://contributors-img.web.app/image?repo=ismaelxyz/qubit" />
</span>

# License

[MIT](./LICENSE)
