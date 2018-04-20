phys = 0x10000;    /* physical address of entry point. */

STARTUP(begin.o)

SECTIONS
{
  . = 0xc0010000;  /* virtual address of kernel. */

  /* AT(phys) for load memory address of this section. */
  .mbheader ALIGN(4) : AT(phys) {
    mbheader = .;
    *(.mbheader)
  }

  .text ALIGN(4) : AT(phys + (code - mbheader)) {
    code = .;
    *(.text .text.*)
    *(.rodata .rodata.*)
  }

  .data ALIGN(4) : AT(phys + (data - mbheader)) {
    data = .;
    *(.data)
    *(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*)
  }

  .bss ALIGN(4) : AT(phys + (bss - mbheader)) {
    bss = .;
    *(.bss)
  }

  /DISCARD/ : {
    *(.debug_*)
  }

  end = .;
}
