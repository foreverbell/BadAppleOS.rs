[bits 32]
[global gdt_flush]
[global idt_flush]
[section .text]

gdt_flush:
  push ebp
  mov ebp, esp

  mov eax, [ebp+8]
  lgdt [eax]

  jmp 0x8:gdt_flush.label

.label:
  mov ax, 0x10
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
