/* These scripts place C read-only sections in "c_readonly" and C writeable sections
 * in "c_writeable", with initializers LMA in "c_readonly". ZP VMAs are in "zp"; ZP LMAs are
 * like regular sections. */
INCLUDE ldscripts/zp.ld
INCLUDE ldscripts/text.ld
INCLUDE ldscripts/rodata.ld
INCLUDE ldscripts/data.ld
INCLUDE ldscripts/bss.ld
INCLUDE ldscripts/noinit.ld
