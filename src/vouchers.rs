use serde::{Deserialize, Serialize};
use const_default::{ConstDefault as ConstDef, ConstDefault};

#[derive(Default, ConstDefault, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vouchers {
    pub overstock: VoucherLevel,
    pub crystal_ball: VoucherLevel,
    pub seed_money: VoucherLevel,
    pub wasteful: VoucherLevel,
    pub telescope: VoucherLevel,
    pub grabber: VoucherLevel,
    pub tarot_merch: VoucherLevel,
    pub planet_merch: VoucherLevel,
    pub antimatter: VoucherLevel,
    pub magic_trick: VoucherLevel,
    pub heiroglyph: VoucherLevel,
    pub directors_cut: VoucherLevel,
    pub paint_brush: VoucherLevel,
    pub reroll_surplus: VoucherLevel,
    pub hone: VoucherLevel,
    pub discount: VoucherLevel,
}
impl Vouchers {
    pub const fn with_overstock(&'static self, level: VoucherLevel) -> Vouchers {
        Vouchers {
            overstock: level,
            ..*self
        }
    }

    pub const fn with_crystal_ball(&'static self, level: VoucherLevel) -> Vouchers {
        Vouchers {
            crystal_ball: level,
            ..*self
        }
    }

    pub const fn with_seed_money(&'static self, level: VoucherLevel) -> Vouchers {
        Vouchers {
            seed_money: level,
            ..*self
        }
    }

    pub const fn with_wasteful(&'static self, level: VoucherLevel) -> Vouchers {
        Vouchers {
            wasteful: level,
            ..*self
        }
    }

    pub const fn with_telescope(&'static self, level: VoucherLevel) -> Vouchers {
        Vouchers {
            telescope: level,
            ..*self
        }
    }

    pub const fn with_grabber(&'static self, level: VoucherLevel) -> Vouchers {
        Vouchers {
            grabber: level,
            ..*self
        }
    }

    pub const fn with_tarot_merch(&'static self, level: VoucherLevel) -> Vouchers {
        Vouchers {
            tarot_merch: level,
            ..*self
        }
    }

    pub const fn with_planet_merch(&'static self, level: VoucherLevel) -> Vouchers {
        Vouchers {
            planet_merch: level,
            ..*self
        }
    }

    pub const fn with_antimatter(&'static self, level: VoucherLevel) -> Vouchers {
        Vouchers {
            antimatter: level,
            ..*self
        }
    }

    pub const fn with_magic_trick(&'static self, level: VoucherLevel) -> Vouchers {
        Vouchers {
            magic_trick: level,
            ..*self
        }
    }

    pub const fn with_heiroglyph(&'static self, level: VoucherLevel) -> Vouchers {
        Vouchers {
            heiroglyph: level,
            ..*self
        }
    }

    pub const fn with_directors_cut(&'static self, level: VoucherLevel) -> Vouchers {
        Vouchers {
            directors_cut: level,
            ..*self
        }
    }

    pub const fn with_paint_brush(&'static self, level: VoucherLevel) -> Vouchers {
        Vouchers {
            paint_brush: level,
            ..*self
        }
    }

    pub const fn with_reroll_surplus(&'static self, level: VoucherLevel) -> Vouchers {
        Vouchers {
            reroll_surplus: level,
            ..*self
        }
    }

    pub const fn with_hone(&'static self, level: VoucherLevel) -> Vouchers {
        Vouchers {
            hone: level,
            ..*self
        }
    }

    pub const fn with_discount(&'static self, level: VoucherLevel) -> Vouchers {
        Vouchers {
            discount: level,
            ..*self
        }
    }
}

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum VoucherLevel {
    #[default] Unbought,
    Base,
    Upgrade
}
impl ConstDef for VoucherLevel {
    const DEFAULT: Self = VoucherLevel::Unbought;
}