.global _start
.text

_start:
    mov x0, #1                         /* file descriptor of stdout */
    adr x1, msg                        /* get current address and add offset to `msg` label */
    ldr x2, =msg_len                   /* length of the `msg` string */
    mov x8, #64                        /* write system call number for aarch64 */
    svc #0                             /* invoke system call */
    ret

msg:
    .ascii "Hello from assembly!\n"    /* string is placed directly in the text section */
msg_len = . - msg                      /* msg_len = (current address) - (address of msg) */
