/* A cortex-m-rt compatible linker script.
 *
 * Supports LMA and VMA differentiation for instructions, which isn't currently available
 * in cortex-m-rt.
 *
 * This approach has only been tested with serial NOR FlexSPI boot. It will likely also work
 * with XIP by updating some REGION_ALIAS specifiers, but this hasn't been tested either.
 */

/* Defined in the example's build.rs */
INCLUDE board.x

/* Boot ROM moves VTABLE from FLASH to ITCM. */
REGION_ALIAS("REGION_LOAD_VTABLE", FLASH);
REGION_ALIAS("REGION_VTABLE", ITCM);

/* Boot ROM moves text from FLASH to ITCM. */
REGION_ALIAS("REGION_LOAD_TEXT", FLASH);
REGION_ALIAS("REGION_TEXT", ITCM);

/* Boot ROM moves read-only data from FLASH to ITCM. */
REGION_ALIAS("REGION_LOAD_RODATA", FLASH);
REGION_ALIAS("REGION_RODATA", ITCM);

/* Runtime moves data from FLASH to DTCM. */
REGION_ALIAS("REGION_LOAD_DATA", FLASH);
REGION_ALIAS("REGION_DATA", DTCM);

/* Runtime zeros bss, does nothing with uninit. */
REGION_ALIAS("REGION_BSS", DTCM);
REGION_ALIAS("REGION_UNINIT", DTCM);

EXTERN(FLEXSPI_CONFIGURATION_BLOCK);

__svectors = ADDR(.vector_table);
_stack_start = ORIGIN(DTCM) + LENGTH(DTCM);

__image_size = SIZEOF(.boot) + SIZEOF(.vector_table) + SIZEOF(.text) + SIZEOF(.rodata);

SECTIONS
{
    .boot :
    {
        . += __fcb_offset;
        KEEP(*(.fcb));
        . = ADDR(.boot) + 0x1000;
        __ivt = .;
        LONG(0x402000D1);           /* Header, magic number */
        LONG(__svectors);           /* Address of the vectors table */
        LONG(0x00000000);           /* RESERVED */
        LONG(0x00000000);           /* Device Configuration Data (unused) */
        LONG(__boot_data);          /* Address to boot data */
        LONG(__ivt);                /* Self reference */
        LONG(0x00000000);           /* Command Sequence File (unused) */
        LONG(0x00000000);           /* RESERVED */
        /* ---------
         * Boot data
         * ---------
         */
        __boot_data = .;
        LONG(ORIGIN(ITCM));        /* Start of image */
        LONG(__image_size);         /* Length of image */
        LONG(0x00000000);           /* Plugin flag (unused) */
        LONG(0xDEADBEEF);           /* Dummy to align boot data to 16 bytes */
        . = ADDR(.boot) + 0x2000;   /* TODO make this not board dependent? */
    } > ITCM AT> FLASH
}
