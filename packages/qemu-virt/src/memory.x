/* memory layout of QEMU's 'virt' machine model */
/* reference: https://github.com/qemu/qemu/blob/master/hw/arm/virt.c */
MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 128M
  /* manpage indicates "128 MiB" is the default RAM size, see `-m` flag */
  RAM   : ORIGIN = 0x40000000, LENGTH = 128M
}

/* place image in RAM */
REGION_ALIAS("CODE", RAM);
REGION_ALIAS("DATA", RAM);
REGION_ALIAS("STACKS", RAM);
