@ Apple clang version 16.0.0 (clang-1600.0.26.6)
@ clang -S -O1 -fomit-frame-pointer

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_main
	.p2align	2
_main:
	mov	w0, #42
	ret
