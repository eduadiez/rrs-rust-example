/* Copyright 2021 Gregory Chadwick <mail@gregchadwick.co.uk>
   Licensed under the Apache License Version 2.0, with LLVM Exceptions, see LICENSE for details.
   SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception */
/* SRC: https://github.com/GregAC/rrs/blob/main/riscv-sw/build-flow/link.ld */

OUTPUT_ARCH(riscv)

MEMORY {
  ram   (wxa) : ORIGIN = 0x100000, LENGTH = 1M
}

SECTIONS
{
  . = ORIGIN(ram);
  .text.start : { *(.text.start) } > ram
  .text : { *(.text) } > ram
  .data : { *(.data) *(.sdata) } > ram
  . = ALIGN(4);
  _bss_start = .;
  .bss : { *(.bss) *(.sbss) } > ram
  . = ALIGN(4);
  _bss_end = .;

   _stack_start = _bss_end;
}