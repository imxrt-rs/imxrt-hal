/* A cortex-m-rt compatible linker script.
 *
 * Supports LMA and VMA separation for text, read-only data, and even the vector table.
 * We're using a fork of cortex-m-rt to achieve this.
 */

/* Emitted from build.rs based on the board selection. */
INCLUDE board.x
/* Emitted from the imxrt-ral shim library */
INCLUDE interrupt_shims.x

/* Runtime moves VTABLE from FLASH to DTCM. */
REGION_ALIAS("REGION_LOAD_VTABLE", FLASH);
REGION_ALIAS("REGION_VTABLE", DTCM);

/* Runtime moves text from FLASH to ITCM. */
REGION_ALIAS("REGION_LOAD_TEXT", FLASH);
REGION_ALIAS("REGION_TEXT", ITCM);

/* Runtime moves read-only data from FLASH to DTCM. */
REGION_ALIAS("REGION_LOAD_RODATA", FLASH);
REGION_ALIAS("REGION_RODATA", DTCM);

/* Runtime moves data from FLASH to DTCM. */
REGION_ALIAS("REGION_LOAD_DATA", FLASH);
REGION_ALIAS("REGION_DATA", DTCM);

/* Runtime zeros bss, does nothing with uninit. */
REGION_ALIAS("REGION_BSS", DTCM);
REGION_ALIAS("REGION_UNINIT", DTCM);

EXTERN(FLEXSPI_CONFIGURATION_BLOCK);

_stack_start = ORIGIN(DTCM) + LENGTH(DTCM);

SECTIONS
{
    /* Boot header for serial NOR FlexSPI XIP.
     *
     * It's 'XIP' in that it starts executing instructions
     * from flash immediately out of reset. The runtime then
     * manually copies instructions (data, etc.), and we jump
     * to that. After that jump, we're no longer XIP.
     *
     * The i.MX RT boot ROM also supports a way to copy the
     * application image by changing the boot data configuration.
     * Specifically, point the 'start of image' to somewhere other
     * than the start of flash, and specify how many bytes to copy.
     * The boot ROM copies the image, then jumps to the vector table.
     * There's a catch: the boot ROM copies the first 8K from the
     * start of flash too. This represents the entire boot header,
     * including the FCB, IVT, and boot data. (NXP docs say that the
     * initial load region is 4K; my testing shows that it's 8K, and
     * this aligns with observations of others.) If you ever want to
     * try this, make sure you're specifing the VMA and LMA of the
     * boot head section to represent this 8K relocation.
     */
    .boot :
    {
        . += __fcb_offset;          /* Changes based on the chip */
        KEEP(*(.fcb));
        . = ADDR(.boot) + 0x1000;
        /* ------------------
         * Image vector table
         * ------------------
         *
         * Not to be confused with the ARM vector table. This tells the boot ROM
         * where to find the boot data and (eventual) first vector table.
         * The IVT needs to reside right here.
         */
        __ivt = .;
        LONG(0x402000D1);           /* Header, magic number */
        LONG(__sivector_table);     /* Address of the vectors table */
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
        LONG(ORIGIN(FLASH));        /* Start of image */
        LONG(__image_size);         /* Length of image */
        LONG(0x00000000);           /* Plugin flag (unused) */
        LONG(0xDEADBEEF);           /* Dummy to align boot data to 16 bytes */
        *(.Reset);                  /* Jam the cortex-m-rt reset handler into flash. */
        . = ADDR(.boot) + 0x2000;   /* Reserve the remaining 8K as a convenience for a non-XIP boot. */
    } > FLASH
}

/* If you're ever playing with the boot ROM copy, this is your image size.
 *
 * Note that it depends on the section layout! Need to represent contiguous
 * sections starting from the boot header.
 */
__image_size = SIZEOF(.boot) + SIZEOF(.vector_table) + SIZEOF(.text) + SIZEOF(.rodata);
