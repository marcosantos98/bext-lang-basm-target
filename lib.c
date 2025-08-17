#include <stdarg.h>
#include <stdio.h>
#include <bm.h>

typedef struct LenStr {
    int len;
    char* data;
} LenStr;

LenStr pop_str(Bm* bm) {
    int len = bm->stack[bm->stack_size - 1].as_i64;
    bm->stack_size -= 1;
    uint64_t data_off = bm->stack[bm->stack_size - 1].as_u64;
    char* data = (char*)bm->memory + data_off;
    bm->stack_size -= 1;
    return (LenStr){
	.len = len,
	.data = data,
    };
}

// note(marco): hackish printf implemented with probably some backdoors as well;
Err bm_printf(Bm* bm) {

    if (bm->stack_size < 1) {
	return ERR_STACK_UNDERFLOW;
    }

    LenStr fmt_str = pop_str(bm);
    char* fmt = fmt_str.data;

    char buf[1024];
    memset(buf, 0, 1024);
    size_t buf_len = 0;

    int i = 0;
    while (i < fmt_str.len) {
	if (fmt[i] == '%') {
	    i++;
	    if (fmt[i] == 's') {
		LenStr arg = pop_str(bm);
		buf_len += snprintf(buf + buf_len, sizeof(buf) - buf_len, "%.*s", (int)arg.len, arg.data);
		i++;
	    } else if(fmt[i] == 'd') {
		uint64_t arg = bm->stack[bm->stack_size - 1].as_u64;
		buf_len += snprintf(buf + buf_len, sizeof(buf) - buf_len, "%zu", arg);
		bm->stack_size -= 1;
		i++;
	    } else {
		printf("printf %c not implemented\n", fmt[i]);
		return ERR_STACK_UNDERFLOW;
	    }
	} else {
	    snprintf(buf + buf_len, sizeof(buf) - buf_len, "%c", fmt[i]);
	    buf_len += 1;
	    i++;
	}
    }

    printf("%.*s", buf_len, buf);

    return ERR_OK;
}

Err bm_putchar(Bm* bm) {
    if (bm->stack_size < 1) return ERR_STACK_UNDERFLOW;

    char c = (char)bm->stack[bm->stack_size - 1].as_u64;
    putchar(c);

    bm->stack_size -= 1;

    return ERR_OK;
}
