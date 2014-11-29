use utils::{outb};

static PIC1		:u16 = 0x20;		/* IO base address for master PIC */
static PIC2		:u16 = 0xA0;		/* IO base address for slave PIC */
static PIC1_COMMAND	:u16 = 0x20;//PIC1;
static PIC1_DATA	:u16 = 0x21;//PIC1 + 1;
static PIC2_COMMAND	:u16 = 0xA0;//PIC2;
static PIC2_DATA	:u16 = 0xA1;//PIC2 + 1;

static ICW1_ICW4	:u8 = 0x01;		/* ICW4 (not) needed */
static ICW1_INIT	:u8 = 0x10;		/* Initialization - required! */

static ICW4_8086	:u8 = 0x01;		/* 8086/88 (MCS-80/85) mode */
static ICW4_SFNM	:u8 = 0x10;		/* Special fully nested (not) */

static REMAP_BASE       :u8 = 0x20;


/*
arguments:
	offset1 - vector offset for master PIC
		vectors on the master become offset1..offset1+7
	offset2 - same for slave PIC: offset2..offset2+7
*/


pub unsafe fn PIC_remap()
{
    outb(PIC1_COMMAND, ICW1_INIT+ICW1_ICW4);  // starts the initialization sequence (in cascade mode)
    outb(PIC2_COMMAND, ICW1_INIT+ICW1_ICW4);
    outb(PIC1_DATA, REMAP_BASE);                 // ICW2: Master PIC vector offset
    outb(PIC2_DATA, REMAP_BASE + 8);                 // ICW2: Slave PIC vector offset
    outb(PIC1_DATA, 4);                       // ICW3: tell Master PIC that there is a slave PIC at IRQ2 (0000 0100)
    outb(PIC2_DATA, 2);                       // ICW3: tell Slave PIC its cascade identity (0000 0010)
    outb(PIC1_DATA, ICW4_8086);
    outb(PIC2_DATA, ICW4_8086);
    outb(PIC1_DATA, 0x0);
    outb(PIC2_DATA, 0x0);
}
