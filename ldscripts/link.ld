/* Commodore KIM-1 linker script.*/

__ram_size = 1;
__STACKSIZE__ = 0x0080;
STARTADDRESS  = 0x0200;

/*__basic_zp_start = 0x0002;
__basic_zp_end = 0xe0;*/

MEMORY {
    zp (rw): ORIGIN = 0x0000, LENGTH = 0xe0
    ram (rw) : ORIGIN = 0x0200, LENGTH = 0x1000 - 0x200
}

__rc0 = 0xe0;
INCLUDE ldscripts/imag-regs.ld
ASSERT(__rc31 == 0xff, "Inconsistent zero page map.")



SECTIONS {
    INCLUDE ldscripts/c.ld
}


REGION_ALIAS("c_readonly", ram)
REGION_ALIAS("c_writeable", ram)



/*INCLUDE commodore.ld*/

/* Grow stack downwards from end of RAM */
__stack = (__ram_size * 1024);

OUTPUT_FORMAT {
    TRIM(ram)
}
