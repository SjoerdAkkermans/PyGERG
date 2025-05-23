# Update 18-04-2025
Equinor has released a more complete python package doing the exact same thing, including a simple pip install. Can be found [here](https://github.com/equinor/pyaga8).



# Intro
A python wrapper of Roy Vegard Ovesen's [AGA8 crate](https://github.com/royvegard/aga8), which in turn is a port of NIST's 
[AGA8 code](https://github.com/usnistgov/AGA8).

Provides methods to calculate thermodynamic properties inlcuding compressibility factors and densities of natural gases using the GERG2008 equation of state described in AGA Report No. 8, Part 1, Third Edition, April 2017.

# How to build

One needs to follow following steps in the root directory.

```Python
pip install maturin #If maturin not yet installed
maturin build --release
```
if asked, select PyO3 bindings.
One will then find a wheel (.whl) file at .\target\wheels\. Navigate to this directory and install using
```Python
pip install .
```

