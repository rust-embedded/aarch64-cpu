/*
Basic AArch64 linker script for PMSA (MPU based) applications.

You must supply a file called `memory.x` in your linker search path. It must
define Region Aliases 'CODE', 'DATA', 'STACKS'.

Here is an example `memory.x` file:

-------------
MEMORY {
  DRAM0 : ORIGIN = 0x00000000, LENGTH = 1 << 31
}

REGION_ALIAS("CODE", DRAM0);
REGION_ALIAS("DATA", DRAM0);
REGION_ALIAS("STACKS", DRAM0);
-------------

The AArch64 platform uses one stack per exception level. This crate sets up a "boot" stack
for the initial exception level. You can set up and use different stacks when entering
lower exception levels. The size of the boot stack is controlled using the `entry!` macro,
instead of being controlled here.

Based upon the linker script from https://github.com/rust-embedded/aarch32

NOTE this is NOT placed in the root of the repository to avoid it being picked by the linker by
default. In other words, we want to make sure the build script is placing it where the linker will
search.

*/


INCLUDE memory.x

/* just using `(NOLOAD)` still produces an ELF that maps sections like */
/* `.bss` into segments marked LOAD (as reported by `readelf -l`). that */
/* causes the FVP to zero-initialize the section at load time instead of */
/* writing the fill-pattern to it */
/* to work around the issue, we manually map `(NOLOAD)` sections to a */
/* `noload` program header which is marked as 'unused' (PT_NULL) and all */
/* other sections that must be loaded to memory to a `load` program header */
PHDRS {
  load PT_LOAD;
  noload PT_NULL;
}

SECTIONS {
  /* align all sections to 64-byte boundaries for MPU compatibility */
  .text : ALIGN(64) {
    _image_lower = .;
    __stext = .;
    KEEP(*(.text._start));
    *(.text .text.*);
    . = ALIGN(64);
    __etext = .;
  } >CODE :load

  .rodata : ALIGN(64) {
    __srodata = .;
    *(.rodata .rodata.*);
    . = ALIGN(64);
    __erodata = .;
  } >CODE :load

  .data : ALIGN(64) {
    __sdata = .;
    *(.data. .data.*);
    . = ALIGN(64);
    __edata = .;
  } >DATA :load

  /* alignment must be at least 8 bytes because zero_bss routine assumes */
  /* 8-byte alignment [src/start.s] */
  .bss (NOLOAD) : ALIGN(64) {
    __sbss = .;
    *(.bss .bss.*);
    . = ALIGN(64);
    __ebss = .;
  } >DATA :noload

  .uninit (NOLOAD) : ALIGN(64) {
    __suninit = .;
    *(.uninit .uninit.*);
    . = ALIGN(64);
    __euninit = .;
  } >DATA :noload

  /* the ISA requires that the stack pointer is 16-byte aligned [ARM-R64/D1.3.10.2] */
  .stack (NOLOAD) : ALIGN(64) {
    __sstack = .;

    __sboot_stack = .;
    KEEP(*(.stack.boot_stack));
    . = ALIGN(64);
    __eboot_stack = .;

    /* for other stacks, the Stack API provides boundary information */
    *(.stack .stack.*);
    . = ALIGN(64);

    __estack = .;
    _image_higher = .;
  } >STACKS :noload

  /* discard non-debug sections unless there's a clear reason to */
  /* add them to the image */
  /DISCARD/ : {
    *(.eh_frame);
    *(.eh_frame_hdr);
    /* TODO: will need to something with this section to support */
    /* static-pie binaries */
    *(.got);
  }
}

ENTRY(_start);
EXTERN(_start);
PROVIDE(_start = _default_start);

ASSERT(
    __stext % 64 == 0 && __etext % 64 == 0,
    ".text section boundaries are not 64 byte aligned"
);
ASSERT(
    __srodata % 64 == 0 && __erodata % 64 == 0,
    ".rodata section boundaries are not 64 byte aligned"
);
ASSERT(
    __sdata % 64 == 0 && __edata % 64 == 0,
    ".data section boundaries are not 64 byte aligned"
);
ASSERT(
    __sbss % 64 == 0 && __ebss % 64 == 0,
    ".bss section boundaries are not 64 byte aligned"
);
ASSERT(
    __suninit % 64 == 0 && __euninit % 64 == 0,
    ".uninit section boundaries are not 64 byte aligned"
);
ASSERT(
    __sstack % 64 == 0 && __estack % 64 == 0,
    ".stack section boundaries are not 64 byte aligned"
);
ASSERT(
    __sboot_stack % 64 == 0 && __eboot_stack % 64 == 0,
    "boot stack boundaries are not 64 byte aligned"
);
ASSERT(
    _image_lower % 64 == 0 && _image_higher % 64 == 0,
    "image boundaries are not 64 byte aligned"
);
