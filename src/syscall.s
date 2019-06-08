.global kaku, owaru
kaku:
    movq $0x0000001, %rax
    syscall
    movq $0x0000000, %rdi
    movq $0x000003C, %rax
    syscall
    retq

owaru:
    movq $0x000003C, %rax
    syscall
