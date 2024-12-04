/* Commodore KIM-1 linker script.*/

__STACKSIZE__ = 0x40;

/*__basic_zp_start = 0x0002;
__basic_zp_end = 0xe0;*/

MEMORY {
    zp (rw): ORIGIN = 0x0020, LENGTH = 0x80
    ram (rw) : ORIGIN = 0x0100, LENGTH = 0x1000 - 0x100
}

__rc0 = 0x0;
INCLUDE ldscripts/imag-regs.ld


SECTIONS {
    INCLUDE ldscripts/c.ld
}


REGION_ALIAS("c_readonly", ram)
REGION_ALIAS("c_writeable", ram)



/*INCLUDE commodore.ld*/

__stack = __ram_size;

OUTPUT_FORMAT {
    TRIM(ram)
}
