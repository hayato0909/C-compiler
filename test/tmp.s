.intel_syntax noprefix
.globl main
main:
  push rbp
  mov rbp, rsp
  sub rsp, 208
  push 3
  pop rdi
  push 4
  push 4
  pop rdi
  pop rax
  add rax, rdi
  push rax
  push 2
  pop rdi
  pop rax
  sub rax, rdi
  push rax
  pop rsi
  call foo
  pop rax
  pop rax
  mov rsp, rbp
  pop rbp
  ret
