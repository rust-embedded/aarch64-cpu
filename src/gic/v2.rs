mod registers;
use paste::paste;
pub use registers::*;
use tock_registers::{
    interfaces::{ReadWriteable, Readable, Writeable},
    registers::*,
    *,
};

pub const GICC_CTLR_EN_BIT: usize = 0x1;
pub const GICC_CTLR_EOIMODENS_BIT: usize = 1 << 9;
pub const GIC_SGIS_NUM: usize = 16;
pub const GIC_INTS_MAX: usize = 1024;
pub const GIC_PRIVINT_NUM: usize = GIC_SGIS_NUM + GIC_PPIS_NUM;
pub const GIC_SPI_MAX: usize = 1024 - GIC_PRIVINT_NUM;
pub const GIC_PRIO_BITS: usize = 8;
pub const GIC_TARGET_BITS: usize = 8;
pub const GIC_TARGETS_MAX: usize = GIC_TARGET_BITS;
pub const GIC_CONFIG_BITS: usize = 2;

const GICD_CTLR_EN_BIT: usize = 0x1;
const GIC_PPIS_NUM: usize = 16;
const GICH_HCR_LRENPIE_BIT: usize = 1 << 2;
const GIC_INT_REGS_NUM: usize = GIC_INTS_MAX / 32;
const GIC_PRIO_REGS_NUM: usize = GIC_INTS_MAX * 8 / 32;
const GIC_TARGET_REGS_NUM: usize = GIC_INTS_MAX * 8 / 32;
const GIC_CONFIG_REGS_NUM: usize = GIC_INTS_MAX * 2 / 32;
const GIC_SEC_REGS_NUM: usize = GIC_INTS_MAX * 2 / 32;

pub const GIC_SGI_REGS_NUM: usize = GIC_SGIS_NUM * 8 / 32;
pub const GIC_LIST_REGS_NUM: usize = 64;
pub const GICD_TYPER_CPUNUM_OFF: usize = 5;
pub const GICD_TYPER_CPUNUM_MSK: usize = 0b11111;

register_structs! {
    #[allow(non_snake_case)]
    #[repr(C)]
    pub GicDistributorInner {
        (0x0000 => pub CTLR: ReadWrite<u32>),
        (0x0004 => pub TYPER: ReadOnly<u32>),
        (0x0008 => pub IIDR: ReadOnly<u32>),
        (0x000c => reserve0),
        (0x0080 => pub IGROUPR: [ReadWrite<u32>; GIC_INT_REGS_NUM]),
        (0x0100 => pub ISENABLER: [ReadWrite<u32>; GIC_INT_REGS_NUM]),
        (0x0180 => pub ICENABLER: [ReadWrite<u32>; GIC_INT_REGS_NUM]),
        (0x0200 => pub ISPENDR: [ReadWrite<u32>; GIC_INT_REGS_NUM]),
        (0x0280 => pub ICPENDR: [ReadWrite<u32>; GIC_INT_REGS_NUM]),
        (0x0300 => pub ISACTIVER: [ReadWrite<u32>; GIC_INT_REGS_NUM]),
        (0x0380 => pub ICACTIVER: [ReadWrite<u32>; GIC_INT_REGS_NUM]),
        (0x0400 => pub IPRIORITYR: [ReadWrite<u32>; GIC_PRIO_REGS_NUM]),
        (0x0800 => pub ITARGETSR: [ReadWrite<u32>; GIC_TARGET_REGS_NUM]),
        (0x0c00 => pub ICFGR: [ReadWrite<u32>; GIC_CONFIG_REGS_NUM]),
        (0x0d00 => reserve1),
        (0x0e00 => pub NSACR: [ReadWrite<u32>; GIC_SEC_REGS_NUM]),
        (0x0f00 => pub SGIR: WriteOnly<u32>),
        (0x0f04 => reserve2),
        (0x0f10 => pub CPENDSGIR: [ReadWrite<u32>; GIC_SGI_REGS_NUM]),
        (0x0f20 => pub SPENDSGIR: [ReadWrite<u32>; GIC_SGI_REGS_NUM]),
        (0x0f30 => _reserved_3),
        (0x1000 => @END),
    }
}

register_structs! {
  #[allow(non_snake_case)]
  #[repr(C)]
  pub GicCpuInterfaceInner {
    (0x0000 => CTLR: ReadWrite<u32>),   // CPU Interface Control Register
    (0x0004 => PMR: ReadWrite<u32>),    // Interrupt Priority Mask Register
    (0x0008 => BPR: ReadWrite<u32>),    // Binary Point Register
    (0x000c => IAR: ReadOnly<u32>),     // Interrupt Acknowledge Register
    (0x0010 => EOIR: WriteOnly<u32>),   // End of Interrupt Register
    (0x0014 => RPR: ReadOnly<u32>),     // Running Priority Register
    (0x0018 => HPPIR: ReadOnly<u32>),   // Highest Priority Pending Interrupt Register
    (0x001c => ABPR: ReadWrite<u32>),   // Aliased Binary Point Register
    (0x0020 => AIAR: ReadOnly<u32>),    // Aliased Interrupt Acknowledge Register
    (0x0024 => AEOIR: WriteOnly<u32>),  // Aliased End of Interrupt Register
    (0x0028 => AHPPIR: ReadOnly<u32>),  // Aliased Highest Priority Pending Interrupt Register
    (0x002c => reserved_0),
    (0x00d0 => APR: [ReadWrite<u32>; 4]),    // Active Priorities Register
    (0x00e0 => NSAPR: [ReadWrite<u32>; 4]),  // Non-secure Active Priorities Register
    (0x00f0 => reserved_1),
    (0x00fc => IIDR: ReadOnly<u32>),    // CPU Interface Identification Register
    (0x0100 => reserved_2),
    (0x1000 => DIR: WriteOnly<u32>),    // Deactivate Interrupt Register
    (0x1004 => reserved_3),
    (0x2000 => @END),
    }
}

register_structs! {
    #[allow(non_snake_case)]
    #[repr(C)]
    pub GicHypervisorInterfaceInner {
        (0x0000 => HCR: ReadWrite<u32>),
        (0x0004 => VTR: ReadOnly<u32>),
        (0x0008 => VMCR: ReadWrite<u32>),
        (0x000c => reserve0),
        (0x0010 => MISR: ReadOnly<u32>),
        (0x0014 => reserve1),
        (0x0020 => EISR: [ReadOnly<u32>; GIC_LIST_REGS_NUM / 32]),
        (0x0028 => reserve2),
        (0x0030 => ELRSR: [ReadOnly<u32>; GIC_LIST_REGS_NUM / 32]),
        (0x0038 => reserve3),
        (0x00f0 => APR: ReadWrite<u32>),
        (0x00f4 => reserve4),
        (0x0100 => LR: [ReadWrite<u32>; GIC_LIST_REGS_NUM]),
        (0x0200 => reserve5),
        (0x1000 => @END),
    }
}

unsafe impl Sync for GicDistributorInner {}
unsafe impl Sync for GicCpuInterfaceInner {}
unsafe impl Sync for GicHypervisorInterfaceInner {}

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
            pub fn [<group_ $method>](&mut self, gid: u32, value: u32){
                self.$field[gid as usize].set(value);
            }
        }
    };
}

macro_rules! field_imp {
    ($method: ident, $field: ident, $width: expr, $interface: ident) => {
        pub fn $method(
            &mut self,
            id: u32,
            field: tock_registers::fields::FieldValue<u32, $interface::Register>,
        ) {
            let field_num = 32 / $width;
            let reg_id = id / field_num;
            let field_id = id & (field_num - 1);
            let mask = (1 << $width) - 1;
            self.$field[reg_id as usize].set(
                (self.$field[reg_id as usize].get() & (!(mask << (field_id * $width))))
                    | (field.value << (field_id * $width)),
            );
        }
    };
}

impl GicDistributorInner {
    bit_imp!(set_pend, ISPENDR, 1);
    bit_imp!(clear_pend, ICPENDR, 1);
    bit_imp!(activate, ISACTIVER, 1);
    bit_imp!(deactivate, ICACTIVER, 1);
    bit_imp!(set_priority, IPRIORITYR, 8);
    bit_imp!(enable_intr, ISENABLER, 1);
    bit_imp!(disable_intr, ICENABLER, 1);

    field_imp!(set_nsacr, NSACR, 2, GICD_NSCAR);
    field_imp!(set_icfgr, ICFGR, 2, GICD_ICFGR);

    pub fn forward(&mut self, id: u32, cpu: u32) {
        let i = (id / 4) as usize;
        let j = id % 4;
        let value = 1 << cpu;
        let mask = (1 << 8) - 1;
        self.ITARGETSR[i].set((self.ITARGETSR[i].get() & !(mask << (j << 3))) | value << (j << 3))
    }
    pub fn enable_gic(&mut self) {
        self.CTLR
            .set((GICD_CTLR::ENGrp0::Enable + GICD_CTLR::ENGrp1::Enable).into())
    }

    pub fn disable_gic(&mut self) {
        self.CTLR
            .set((GICD_CTLR::ENGrp0::Disable + GICD_CTLR::ENGrp1::Disable).into())
    }
}

impl GicCpuInterfaceInner {
    pub fn enable(&mut self) {
        self.CTLR
            .set((GICC_CTLR::EN::Enable + GICC_CTLR::EOI::EOIR_DIR).into());
    }
    pub fn disable(&mut self) {
        self.CTLR.set(GICC_CTLR::EN::Disable.into())
    }
    pub fn set_priority_mask(&mut self, mask: u8) {
        self.PMR.set(GICC_PMR::PRIORITY.val(mask as u32).into())
    }
    pub fn set_bpr(&mut self, bpr: u8) {
        self.BPR.set(GICC_BPR::BP.val(bpr as u32).into())
    }
}
