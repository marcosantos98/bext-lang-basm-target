#include <stdarg.h>
#include <stdio.h>
#include <bm.h>

// This function provides the support for `extrn printf`
// It requires the last value on the stack to be a ptr to a string in the BM memory;
//
// push "JKAsjdka" || push FMT_STR
Err bm_printf(Bm* bm) {

    // todo: make it format
    // - need to know the amount and the type (%d, %s, ...) of arguments to format (can count by parsing the fmt)
    // - since the arguments are pushed to the stack, the fmt string needs to be the last
    // argument instead of being the first, this is needed if I go with the 
    // "count arguments to format" path
    // 
    // push 1
    // push 2
    // push FMT_STR
    // native printf
    //
    if (bm->stack_size < 1) {
	return ERR_STACK_UNDERFLOW;
    }

    uint64_t fmt_off = bm->stack[bm->stack_size - 1].as_u64;
    void* fmt = (void*)bm->memory + fmt_off;

    printf("%s", fmt);

    bm->stack_size -= 1;

    return ERR_OK;
}

