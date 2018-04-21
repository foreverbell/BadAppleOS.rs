[bits 32]
[global gdt_flush]
[global idt_flush]
[section .text]

KRNL_CODE_SEL equ 0x8
KRNL_DATA_SEL equ 0x10

gdt_flush:
  push ebp
  mov ebp, esp

  mov eax, [ebp+8]
  lgdt [eax]

  jmp KRNL_CODE_SEL:gdt_flush.label

.label:
  mov ax, KRNL_DATA_SEL
  mov ds, ax
  mov es, ax
  mov fs, ax
  mov gs, ax
  mov ss, ax

  mov esp, ebp
  pop ebp
  ret

idt_flush:
  push ebp
  mov ebp, esp

  mov eax, [ebp+8]
  lidt [eax]

  mov esp, ebp
  pop ebp
  ret
