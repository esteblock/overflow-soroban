# overflow-soroban
Testing Overflow Restrictions in Soroban

In this repo we test different techniques in order to avoid overflow, underflow and div by zero.

Here we test:

`overflow-checks=true`

```
[profile.release]
overflow-checks = true
```

And `.checked_add()`, `+`, `-`, `checked_sub`, `/`, `.checked_div`

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

4.- Test the contract without overflow_checks
```
cd increment_without_overflow_checks
make test
```

5.- Take your conclusions