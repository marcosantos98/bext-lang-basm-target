# BASM target from bm to bext-lang

Test target using **bext-lang/b** target API for **tsoding/bm** assembly.

## Notes:

It doesn't work with all tests, but it works with **hello.b** and **hello_world.b** which uses some hacks;

- **printf** kinda works. It can load the format string and print it using the dynamic loader in **bm**, but can't actually format the string since I don't have a way to know I much arguments I need to pop from the stack and print it as the correct type, unless I write my own **printf**;
- **lib.c** contains the implemention of **extrn** functions, since **bm** can't do something like **call puts**
    - **lib.so** is the library loaded by **bm** when **-run** is provided;
    - **printf** is actually implemented in **libc**, loaded from **lib.so** and called with **native printf**;
- **putchar**:
    - **bm** contains only one stack for both data and return that**s why in its implemention it has some **swap** instructions;
    - it uses the builtin **write** native which makes the final assembly contain a write buffer;
        - **write** needs two values on the stack: **ptr**, **count**
    - it was fun to mess arround with it, but it can be simplified with a call to libc **putchar** using the **lib.so** approach;

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
