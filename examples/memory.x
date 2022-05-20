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

/* Boot ROM moves VTABLE from APP_FLASH to OCRAM. */
REGION_ALIAS("REGION_LOAD_VTABLE", APP_FLASH);
REGION_ALIAS("REGION_VTABLE", OCRAM);

/* Boot ROM moves text from APP_FLASH to OCRAM. */
REGION_ALIAS("REGION_LOAD_TEXT", APP_FLASH);
REGION_ALIAS("REGION_TEXT", OCRAM);

/* Boot ROM moves read-only data from APP_FLASH to OCRAM. */
REGION_ALIAS("REGION_LOAD_RODATA", APP_FLASH);
REGION_ALIAS("REGION_RODATA", OCRAM);

/* Runtime moves data from APP_FLASH to DTCM. */
REGION_ALIAS("REGION_LOAD_DATA", APP_FLASH);
REGION_ALIAS("REGION_DATA", DTCM);

/* Runtime zeros bss, does nothing with uninit. */
REGION_ALIAS("REGION_BSS", DTCM);
REGION_ALIAS("REGION_UNINIT", DTCM);

EXTERN(FLEXSPI_CONFIGURATION_BLOCK);

__svectors = ADDR(.vector_table);
_stack_start = ORIGIN(DTCM) + LENGTH(DTCM);

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
        LONG(ORIGIN(OCRAM));        /* Start of image (origin of flash) */
        LONG(LENGTH(OCRAM));        /* Length of flash */
        LONG(0x00000000);           /* Plugin flag (unused) */
        LONG(0xDEADBEEF);           /* Dummy to align boot data to 16 bytes */
        . = ADDR(.boot) + 0x2000;   /* TODO make this not board dependent? */
    } > OCRAM AT> BOOT_FLASH
}
