putchar(c) {
    // needs `swap` since the top value is the return addr
    // push 69
    // call putchar
    //
    // 0: 69
    // 1: 2 <- return addr
    // swap 1
    // 0: 2
    // 1: 69
    __asm__(
	"swap 1",
    	"push WRITE_BUFFER",
    	"swap 1",
    	"write64", // <- write the `c` to WRITE_BUFFER
    	"push WRITE_BUFFER",
    	"push 1",
    	"%native write",
    	"native write"
    );
}
