MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 1024K
  RAM : ORIGIN = 0x20000000, LENGTH = 128K
}

_stack_start = ORIGIN(RAM) + LENGTH(RAM);
_heap_size = 1024;

PROVIDE(_vector_table = ORIGIN(FLASH));
PROVIDE(_stext = _vector_table + 0x200);