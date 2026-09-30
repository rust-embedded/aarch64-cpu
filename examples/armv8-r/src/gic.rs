use core::ptr::NonNull;

use aarch64_cpu::registers::{self, ReadWriteable as _, Readable as _};
use arm_fvp_base_pac::PhysicalInstance;
pub use arm_gic::IntId;
use arm_gic::{
    UniqueMmioPointer,
    gicv3::{
        GicCpuInterface, GicV3, InterruptGroup, SgiTarget, SgiTargetGroup,
        registers::{Gicd, GicrSgi},
    },
};

pub struct Gic {
    inner: GicV3<'static>,
    priority_bits: u8,
}

const BOOT_CORE: usize = 0;
const NUM_CORES: usize = 1;

impl Gic {
    pub(super) fn setup(gicd: PhysicalInstance<Gicd>, gicr: PhysicalInstance<GicrSgi>) -> Self {
        let mut gic = {
            let gicd =
                unsafe { UniqueMmioPointer::new(NonNull::new(gicd.pa() as *mut _).unwrap()) };
            let gicr = NonNull::new(gicr.pa() as *mut _).unwrap();

            let is_gicv4 = false; // only has GICv3
            unsafe { GicV3::new(gicd, gicr, NUM_CORES, is_gicv4) }
        };
        // this also enables "interrupt group 1"
        gic.setup(BOOT_CORE);

        let in_el2 = registers::CurrentEL.read_as_enum(registers::CurrentEL::EL)
            == Some(registers::CurrentEL::EL::Value::EL2);
        if in_el2 {
            // set 'Trap General Exceptions' bit
            // "All exceptions that would be routed to EL1 are routed to EL2"
            // without this (off by default) EL2 won't see the IRQ trigger
            registers::HCR_EL2.modify(registers::HCR_EL2::TGE::SET);
        }

        // sanity check that we installed the exception handlers
        let vbar = if in_el2 {
            registers::VBAR_EL2.get()
        } else {
            registers::VBAR_EL1.get()
        };
        assert_ne!(
            0, vbar,
            "exception handlers have not been installed (check aarch64-rt.features)"
        );

        GicCpuInterface::set_priority_mask(!0);
        let mask = GicCpuInterface::get_priority_mask();
        let priority_bits = mask.count_ones() as u8;
        Self {
            inner: gic,
            priority_bits,
        }
    }

    pub fn get_and_acknowledge_interrupt() -> Option<IntId> {
        GicCpuInterface::get_and_acknowledge_interrupt(InterruptGroup::Group1)
    }

    pub fn end_interrupt(int_id: IntId) {
        GicCpuInterface::end_interrupt(int_id, InterruptGroup::Group1);
    }

    pub fn send_sgi(int_id: IntId) {
        let dont_care = 0;
        let target_self = SgiTarget::List {
            affinity3: dont_care,
            affinity2: dont_care,
            affinity1: dont_care,
            target_list: 1 << BOOT_CORE,
        };

        GicCpuInterface::send_sgi(int_id, target_self, SgiTargetGroup::CurrentGroup1)
            .expect("send_sgi");
    }

    pub fn set_priority_mask(&self, logical_prio: u8) {
        GicCpuInterface::set_priority_mask(self.logical2hw(logical_prio));
    }

    pub fn set_interrupt_priority(&mut self, int_id: IntId, logical_priority: u8) {
        self.inner
            .set_interrupt_priority(int_id, Some(BOOT_CORE), self.logical2hw(logical_priority))
            .expect("set_interrupt_priority");
    }

    pub fn enable_interrupt(&mut self, int_id: IntId, enabled: bool) {
        self.inner
            .enable_interrupt(int_id, Some(BOOT_CORE), enabled)
            .expect("enable_interrupt");
    }

    fn logical2hw(&self, logical_priority: u8) -> u8 {
        let max_logical = (1 << self.priority_bits) - 1;
        assert!(
            logical_priority <= max_logical,
            "this logical priority is not supported ({} bits)",
            self.priority_bits
        );

        (max_logical - logical_priority) << (8 - self.priority_bits)
    }
}
