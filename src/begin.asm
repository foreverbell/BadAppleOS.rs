[extern kinitialize]
[bits 32]

[section .mbheader]
mbheader:
  dd 0x1badb002
  dd 0x0
  dd 0xe4524ffe

TMP_PGDIR  equ 0x6000

PAGE_SIZE  equ 4096
PAGE_P     equ 0x1
PAGE_W     equ 0x2
PAGE_S     equ 0x80

KERNEL_LMA equ 0x10000
KERNEL_VMA equ 0xc0010000

CR0_PE     equ 0x1
CR0_PG     equ 0x80000000
CR4_PSE    equ 0x10

[section .text]
begin:
  ; initialize kernel stack (reuse memory below LMA)
  mov ebp, KERNEL_LMA
  mov esp, ebp

  ; turn on page size extension (PSE) for 4Mbytes pages
  mov eax, cr4
  or  eax, CR4_PSE
  mov cr4, eax

  ; initialize tempororay page directory table, with two huge pages mapped
  xor eax, eax
  mov edi, TMP_PGDIR
  mov ecx, PAGE_SIZE
  cld
  rep stosb

  mov edi, PAGE_P | PAGE_W | PAGE_S

  mov ebx, TMP_PGDIR
  xor ecx, ecx
  lea eax, [ebx + 4*ecx]
  mov [eax], edi

  mov ecx, KERNEL_VMA >> 22
  lea eax, [ebx + 4*ecx]
  mov [eax], edi

  ; initialize paging, notice cr3 requires physical address
  mov eax, TMP_PGDIR
  mov cr3, eax

  ; turn on paging
  mov eax, cr0
  or  eax, CR0_PE | CR0_PG
  mov cr0, eax

  ; directly jump to Rust main, use "jmp eax" to avoid relative jump
  mov eax, kinitialize
  jmp eax
