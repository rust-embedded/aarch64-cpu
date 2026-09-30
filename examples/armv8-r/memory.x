MEMORY {
  /*
  NOTE as per the Fast Models Reference 11.28, the BaseR model has the same
  memory layout as the Base model but swaps the first and second 2 GiB blocks at
  the start of the memory space. Thus DRAM0 starts at address 0x0000_0000 on BaseR, rather
  than 0x8000_0000 it appears at on other Base FVPs. see
  https://developer.arm.com/documentation/100964/1128/Base-Platform/Base-Platform-memory/BaseR-Platform-memory-map/
  */
  DRAM0 : ORIGIN = 0x00000000, LENGTH = 1 << 31
}

REGION_ALIAS("CODE", DRAM0);
REGION_ALIAS("DATA", DRAM0);
REGION_ALIAS("STACKS", DRAM0);
