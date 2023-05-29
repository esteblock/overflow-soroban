# overflow-soroban
Testing Overflow Restrictions in Soroban

In this repo we test different techniques in order to avoid overflow, underflow and div by zero.

Here we test:

```
overflow-checks = true
```

`.checked_add()`, `+`, `-`, `checked_sub`, `/`, `.checked_div`, `*` and `.checked_mul`

## The code
The code is divided in two:
In `increment_overflow_checks` you will find a `Cargo.toml` with


```
[profile.dev]
overflow-checks = true

...

[profile.release]
overflow-checks = true
```

Oh the other hand, in `increment_without_overflow_checks`, you will find a `Cargo.toml` with

```
[profile.dev]
overflow-checks = false

...

[profile.release]
overflow-checks = false
```

## Test with cargo:

1.- Read the code

2.- open de soroban-preview:8 docker container
```
bash run.sh 
```

3.- Test the contract with overflow_checks
```
cd increment_overflow_checks
make test
```
As overflow_checks is activated, all functions that can overflow ("+", "*") should panic. Also those with checked_fn

4.- Test the contract without overflow_checks
```
cd increment_without_overflow_checks
make test
```

As overflow_checks it's not activated, it is expected to happen overflow

5.- Take your conclusions

## Test with soroban-cli
To do...
