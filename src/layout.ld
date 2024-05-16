SECTIONS
{
  . = 0x80000;
  .text :
  {
    *(.text._start)
    *(.text*)
  }
  .rodata :
  {
      *(.rodata*)
  }
  .data : AT (ADDR(.rodata) + SIZEOF(.rodata))
  {
    *(.data*)
  }
  .bss : AT (ADDR(.data) + SIZEOF(.data))
  {
    *(.bss*)
  } 
}
