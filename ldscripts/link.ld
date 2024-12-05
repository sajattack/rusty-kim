/* Commodore KIM-1 linker script.*/

MEMORY {
    zp (rw): ORIGIN = 0x0020, LENGTH = 0x80
    stack(rw): ORIGIN = 0x100, LENGTH = 0xff
    ram (rw) : ORIGIN = 0x0200, LENGTH = 1024 - 0x200
}

__rc0 = 0x0;
__zp_data_start = 0x20;
__zp_data_size = 0xd0;

INCLUDE ldscripts/imag-regs.ld

SECTIONS {
    INCLUDE ldscripts/c.ld
}

REGION_ALIAS("c_readonly", ram)
REGION_ALIAS("c_writeable", ram)


/*OUTPUT_FORMAT {
    TRIM(ram)
}*/
