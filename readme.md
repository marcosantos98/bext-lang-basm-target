# BASM target from bm to bext-lang

Test target using **bext-lang/b** target API for **tsoding/bm** assembly.

## Notes:

It doesn't work with all tests, but it works with **hello.b** and **hello_world.b** which uses some hacks;

- **extrn**'s are implemented in `lib.c` that need to be compiled. Check [Usage](#usage);

## Usage:

- **basm** and **bme** are required in your PATH and you need the headers to compile **lib.c** since it depends on **bm.h**;
    - https://github.com/tsoding/bm
- The codegen requires you to provide the compiled dynamic library path of **lib.c**
    - you can obtain it with `cc -shared -fPIC -o lib.so lib.c -I<bm/bm/src/> -I<bm/common/>`
    - the **bme** will try to load the compiled **lib.so** from **./build/lib.so** you may need to copy it or move it there;

```cmd
b -t basm ./tests/hello.b -run
```

## Links:

- [tsoding/bm](https://github.com/tsoding/bm)
- [bext-lang/b](https://github.com/bext-lang/b)
