mod registers;
use core::mem::size_of;
use paste::paste;
pub use registers::*;
use tock_registers::{
    interfaces::{Readable, Writeable},
    registers::*,
    *,
};

use crate::{asm::barrier::*, registers::*};

pub const GIC_SGIS_NUM: usize = 16;
pub const GIC_INTS_MAX: usize = 1024;
pub const GIC_PRIVINT_NUM: usize = GIC_SGIS_NUM + GIC_PPIS_NUM;
pub const GIC_SPI_MAX: usize = GIC_INTS_MAX - GIC_PRIVINT_NUM;
pub const GIC_PRIO_BITS: usize = 8;
pub const GIC_TARGET_BITS: usize = 8;
pub const GIC_TARGETS_MAX: usize = GIC_TARGET_BITS;
pub const GIC_CONFIG_BITS: usize = 2;
pub const GIC_SGI_REGS_NUM: usize = GIC_SGIS_NUM * 8 / 32;
pub const GIC_LIST_REGS_NUM: usize = 64;

const GIC_PPIS_NUM: usize = 16;
const GIC_INT_RT_NUM: usize = 1019 - 32 + 1;
const GIC_INT_REGS_NUM: usize = GIC_INTS_MAX / 32;
const GIC_PRIO_REGS_NUM: usize = GIC_INTS_MAX * 8 / 32;
const GIC_TARGET_REGS_NUM: usize = GIC_INTS_MAX * 8 / 32;
const GIC_CONFIG_REGS_NUM: usize = GIC_INTS_MAX * 2 / 32;
const GIC_SEC_REGS_NUM: usize = GIC_INTS_MAX * 2 / 32;

macro_rules! bit_imp {
    ($method: ident, $field: ident, $width: expr) => {
        pub fn $method(&mut self, id: u32, pos: u32) {
            let field_num = 32 / $width;
            let reg_id = id / field_num;
            let field_id = id & (field_num - 1);
            self.$field[reg_id as usize]
                .set(self.$field[reg_id as usize].get() | (1 << pos) << (field_id * $width));
        }
        paste! {
            pub fn [<$method _val>](&mut self, id: u32, val: u32){
            let field_num = 32 / $width;
            let reg_id = id / field_num;
            let field_id = id & (field_num - 1);
            self.$field[reg_id as usize]
                .set(self.$field[reg_id as usize].get() | (val << (field_id * $width)));
            }
        }
        paste! {
            pub fn [<group_ $method>](&mut self, gid: u32, value: u32){
                self.$field[gid as usize].set(value);
            }
        }
    };
}

register_structs! {
    #[allow(non_snake_case)]
    pub GicDistributorInner {
        (0x0000 => CTLR: ReadWrite<u32>), //Distributor Control Register
        (0x0004 => pub TYPER: ReadOnly<u32>), //Interrupt Controller Type Register
        (0x0008 => pub IIDR: ReadOnly<u32>),  //Distributor Implementer Identification Register
        (0x000c => TYPER2: ReadOnly<u32>), //Interrupt controller Type Register 2
        (0x0010 => STATUSR: ReadWrite<u32>), //Error Reporting Status Register, optional
        (0x0014 => reserved0),
        (0x0040 => SETSPI_NSR: WriteOnly<u32>), //Set SPI Register
        (0x0044 => reserved1),
        (0x0048 => CLRSPI_NSR: WriteOnly<u32>), //Clear SPI Register
        (0x004c => reserved2),
        (0x0050 => SETSPI_SR: WriteOnly<u32>), //Set SPI, Secure Register
        (0x0054 => reserved3),
        (0x0058 => CLRSPI_SR: WriteOnly<u32>), //Clear SPI, Secure Register
        (0x005c => reserved4),
        (0x0080 => IGROUPR: [ReadWrite<u32>; GIC_INT_REGS_NUM]), //Interrupt Group Registers
        (0x0100 => pub ISENABLER: [ReadWrite<u32>; GIC_INT_REGS_NUM]), //Interrupt Set-Enable Registers
        (0x0180 => pub ICENABLER: [ReadWrite<u32>; GIC_INT_REGS_NUM]), //Interrupt Clear-Enable Registers
        (0x0200 => pub ISPENDR: [ReadWrite<u32>; GIC_INT_REGS_NUM]), //Interrupt Set-Pending Registers
        (0x0280 => pub ICPENDR: [ReadWrite<u32>; GIC_INT_REGS_NUM]), //Interrupt Clear-Pending Registers
        (0x0300 => pub ISACTIVER: [ReadWrite<u32>; GIC_INT_REGS_NUM]), //Interrupt Set-Active Registers
        (0x0380 => pub ICACTIVER: [ReadWrite<u32>; GIC_INT_REGS_NUM]), //Interrupt Clear-Active Registers
        (0x0400 => pub IPRIORITYR: [ReadWrite<u32>; GIC_PRIO_REGS_NUM]), //Interrupt Priority Registers
        (0x0800 => pub ITARGETSR: [ReadWrite<u32>; GIC_TARGET_REGS_NUM]), //Interrupt Processor Targets Registers
        (0x0c00 => pub ICFGR: [ReadWrite<u32>; GIC_CONFIG_REGS_NUM]), //Interrupt Configuration Registers
        (0x0d00 => pub IGRPMODR: [ReadWrite<u32>; GIC_CONFIG_REGS_NUM]), //Interrupt Group Modifier Registers
        (0x0e00 => pub NSACR: [ReadWrite<u32>; GIC_SEC_REGS_NUM]), //Non-secure Access Control Registers
        (0x0f00 => pub SGIR: WriteOnly<u32>),  //Software Generated Interrupt Register
        (0x0f04 => reserved6),
        (0x0f10 => CPENDSGIR: [ReadWrite<u32>; GIC_SGI_REGS_NUM]), //SGI Clear-Pending Registers
        (0x0f20 => SPENDSGIR: [ReadWrite<u32>; GIC_SGI_REGS_NUM]), //SGI Set-Pending Registers
        (0x0f30 => reserved7),
        (0x6000 => IROUTER: [ReadWrite<u64>; (0x8000 - 0x6000) / size_of::<u64>()]), //Interrupt Routing Registers for extended SPI range
        (0x8000 => reserved21),
        (0xffd0 => ID_res1: [ReadOnly<u32>; (0xffe8 - 0xffd0) / size_of::<u32>()]), //Reserved for IMPLEMENTATION registers
        (0xffe8 => pub PIDR2: ReadOnly<u32>),  //Distributor Peripheral ID2 Register
        (0xffec => ID_res2: [ReadOnly<u32>; (0x10000 - 0xffec) / size_of::<u32>()]), //Reserved for IMPLEMENTATION registers
        (0x10000 => @END),
    }
}

register_structs! {
    #[allow(non_snake_case)]
    pub GicRedistributorInner {
        (0x0000 => pub CTLR: ReadWrite<u32>),   // Redistributor Control Register
        (0x0004 => pub IIDR: ReadOnly<u32>),    // Implementer Identification Register
        (0x0008 => pub TYPER: ReadOnly<u64>),   // Redistributor Type Register
        (0x0010 => STATUSR: ReadWrite<u32>),  // Error Reporting Status Register, optional
        (0x0014 => pub WAKER: ReadWrite<u32>),     // Redistributor Wake Register
        (0x0018 => MPAMIDR: ReadOnly<u32>),   // Report maximum PARTID and PMG Register
        (0x001c => PARTIDR: ReadWrite<u32>),   // Set PARTID and PMG Register
        (0x0020 => reserved18),
        (0x0040 => SETLPIR: WriteOnly<u64>),    // Set LPI Pending Register
        (0x0048 => CLRLPIR: WriteOnly<u64>),  // Clear LPI Pending Register
        (0x0050 => reserved17),
        (0x0070 => PROPBASER: ReadWrite<u64>),  //Redistributor Properties Base Address Register
        (0x0078 => PEDNBASER: ReadWrite<u64>),    //Redistributor LPI Pending Table Base Address Register
        (0x0080 => reserved16),
        (0x00a0 => INVLPIR: WriteOnly<u64>),  // Redistributor Invalidate LPI Register
        (0x00a8 => reserved15),
        (0x00b0 => INVALLR: WriteOnly<u64>),    // Redistributor Invalidate All Register
        (0x00b8 => reserved14),
        (0x00c0 => SYNCR: ReadOnly<u64>),    // Redistributor Synchronize Register
        (0x00c8 => reserved13),
        (0xffd0 => ID_res1: [ReadOnly<u32>; (0xffe8 - 0xffd0) / size_of::<u32>()]), //Reserved for IMPLEMENTATION registers
        (0xffe8 => pub PIDR2: ReadOnly<u32>),  //Distributor Peripheral ID2 Register
        (0xffec => ID_res2: [ReadOnly<u32>; (0x10000 - 0xffec) / size_of::<u32>()]), //Reserved for IMPLEMENTATION registers
        (0x10000 => reserved12),
        (0x10080 => pub IGROUPR0: ReadWrite<u32>), //SGI_base frame, all below
        (0x10084 => reserved11),
        (0x10100 => pub ISENABLER0: ReadWrite<u32>),
        (0x10104 => reserved10),
        (0x10180 => pub ICENABLER0: ReadWrite<u32>),
        (0x10184 => reserved9),
        (0x10200 => pub ISPENDR0: ReadWrite<u32>),
        (0x10204 => reserved8),
        (0x10280 => pub ICPENDR0: ReadWrite<u32>),
        (0x10284 => reserved7),
        (0x10300 => pub ISACTIVER0: ReadWrite<u32>),
        (0x10304 => reserved6),
        (0x10380 => pub ICACTIVER0: ReadWrite<u32>),
        (0x10384 => reserved5),
        (0x10400 => IPRIORITYR: [ReadWrite<u32>;8]),
        (0x10420 => reserved4),
        (0x10c00 => pub ICFGR0: ReadWrite<u32>),
        (0x10c04 => pub ICFGR1: ReadWrite<u32>),
        (0x10c08 => reserved3),
        (0x10d00 => IGRPMODR0: ReadWrite<u32>),
        (0x10d04 => reserved2),
        (0x10e00 => NSACR: ReadWrite<u32>),
        (0x10e04 => reserved1),
        (0x20000 => @END),
    }
}

#[repr(C)]
pub struct GICCpuInterface;

unsafe impl Sync for GicDistributorInner {}
unsafe impl Sync for GicRedistributorInner {}
unsafe impl Sync for GICCpuInterface {}

impl GicDistributorInner {
    bit_imp!(set_priority, IPRIORITYR, 8);
    bit_imp!(deactivate, ICACTIVER, 1);
    bit_imp!(enable_intr, ISENABLER, 1);
    bit_imp!(disable_intr, ICENABLER, 1);
    bit_imp!(clear_pend, ICPENDR, 1);
    bit_imp!(set_group, IGROUPR, 1);
    bit_imp!(set_group_mod, IGRPMODR, 1);
    bit_imp!(configure_intr, ICFGR, 2);

    pub fn forward(&mut self, id: u32, cpu: u32) {
        // remove this hardcode
        let cpumask = (cpu & 0x1) + ((cpu >> 1) << 7);
        self.IROUTER[id as usize].set(cpumask as u64);
    }
    pub fn enable_gic(&mut self) {
        self.CTLR
            .set((GICD_CTLR::ENGrp1NS::Enable + GICD_CTLR::ARE_NS::Enable).into())
    }

    pub fn disable_gic(&mut self) {
        self.CTLR
            .set((GICD_CTLR::ENGrp1NS::Disable + GICD_CTLR::ARE_NS::Disable).into())
    }

    pub fn get_nr_lines(&self) -> u32 {
        32 * (1 + GICD_TYPER::ITLines.read(self.TYPER.get()))
    }
}

impl GicRedistributorInner {
    bit_imp!(set_priority, IPRIORITYR, 8);

    pub fn wake_up_redis(&mut self) {
        self.WAKER
            .set(GICR_WAKER::ProcessorSleep::NotinLowState.modify(self.WAKER.get()));
        while GICR_WAKER::ChildrenAsleep.is_set(self.WAKER.get()) {}
    }
}

impl GICCpuInterface {
    pub fn enable_sre_bit(&self) {
        if !ICC_SRE_EL1.is_set(ICC_SRE_EL1::SRE) {
            ICC_SRE_EL1.modify(ICC_SRE_EL1::SRE::SR);
            isb(NONE);
            assert!(ICC_SRE_EL1.is_set(ICC_SRE_EL1::SRE));
        }
    }
}
