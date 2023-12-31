use ethnum::U256;

/// There are 10^6 micro-lamports in one lamport
const MICRO_LAMPORTS_PER_LAMPORT: u128 = 1_000_000;

// type MicroLamports = u128;

pub enum PrioritizationFeeType {
    ComputeUnitPrice(u128),
    // TODO: remove 'Deprecated' after feature remove_deprecated_request_unit_ix::id() is activated
    Deprecated(u128),
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct PrioritizationFeeDetails {
    fee: u128,
    priority: u128,
}

impl PrioritizationFeeDetails {
    pub fn new(fee_type: PrioritizationFeeType, compute_unit_limit: u128) -> Self {
        match fee_type {
            // TODO: remove support of 'Deprecated' after feature remove_deprecated_request_unit_ix::id() is activated
            PrioritizationFeeType::Deprecated(fee) => {
                let priority = if compute_unit_limit == 0 {
                    0
                } else {
                    // let micro_lamport_fee: MicroLamports =
                    //     (fee as u128).saturating_mul(MICRO_LAMPORTS_PER_LAMPORT as u128);
                    // let priority = micro_lamport_fee.saturating_div(compute_unit_limit as u128);
                    // u64::try_from(priority).unwrap_or(u64::MAX)

                    let micro_lamport_fee: U256 =
                    U256::new(fee).saturating_mul(U256::new(MICRO_LAMPORTS_PER_LAMPORT));
                    let priority = micro_lamport_fee.saturating_div(U256::new(compute_unit_limit as u128));
                    u128::try_from(priority).unwrap_or(u128::MAX)
                };

                Self { fee, priority }
            }
            PrioritizationFeeType::ComputeUnitPrice(cu_price) => {
                let fee = {
                    // let micro_lamport_fee: MicroLamports =
                    //     (cu_price as u128).saturating_mul(compute_unit_limit as u128);
                    // let fee = micro_lamport_fee
                    //     .saturating_add(MICRO_LAMPORTS_PER_LAMPORT.saturating_sub(1) as u128)
                    //     .saturating_div(MICRO_LAMPORTS_PER_LAMPORT as u128);
                    // u64::try_from(fee).unwrap_or(u64::MAX)

                    let micro_lamport_fee: U256 =
                        U256::new(cu_price).saturating_mul(U256::new(compute_unit_limit as u128));
                    let fee = micro_lamport_fee
                        .saturating_add(U256::new(MICRO_LAMPORTS_PER_LAMPORT.saturating_sub(1)))
                        .saturating_div(U256::new(MICRO_LAMPORTS_PER_LAMPORT));
                    u128::try_from(fee).unwrap_or(u128::MAX)
                };

                Self {
                    fee,
                    priority: cu_price,
                }
            }
        }
    }

    pub fn get_fee(&self) -> u128 {
        self.fee
    }

    pub fn get_priority(&self) -> u128 {
        self.priority
    }
}

#[cfg(test)]
mod test {
    use super::{PrioritizationFeeDetails as FeeDetails, PrioritizationFeeType as FeeType, *};

    #[test]
    fn test_new_with_no_fee() {
        for compute_units in [0, 1, MICRO_LAMPORTS_PER_LAMPORT, u128::MAX] {
            assert_eq!(
                FeeDetails::new(FeeType::ComputeUnitPrice(0), compute_units),
                FeeDetails::default(),
            );
            assert_eq!(
                FeeDetails::new(FeeType::Deprecated(0), compute_units),
                FeeDetails::default(),
            );
        }
    }

    #[test]
    fn test_new_with_compute_unit_price() {
        assert_eq!(
            FeeDetails::new(FeeType::ComputeUnitPrice(MICRO_LAMPORTS_PER_LAMPORT - 1), 1),
            FeeDetails {
                fee: 1,
                priority: MICRO_LAMPORTS_PER_LAMPORT - 1,
            },
            "should round up (<1.0) lamport fee to 1 lamport"
        );

        assert_eq!(
            FeeDetails::new(FeeType::ComputeUnitPrice(MICRO_LAMPORTS_PER_LAMPORT), 1),
            FeeDetails {
                fee: 1,
                priority: MICRO_LAMPORTS_PER_LAMPORT,
            },
        );

        assert_eq!(
            FeeDetails::new(FeeType::ComputeUnitPrice(MICRO_LAMPORTS_PER_LAMPORT + 1), 1),
            FeeDetails {
                fee: 2,
                priority: MICRO_LAMPORTS_PER_LAMPORT + 1,
            },
            "should round up (>1.0) lamport fee to 2 lamports"
        );

        assert_eq!(
            FeeDetails::new(FeeType::ComputeUnitPrice(200), 100_000),
            FeeDetails {
                fee: 20,
                priority: 200,
            },
        );

        assert_eq!(
            FeeDetails::new(
                FeeType::ComputeUnitPrice(MICRO_LAMPORTS_PER_LAMPORT),
                u128::MAX
            ),
            FeeDetails {
                fee: u128::MAX,
                priority: MICRO_LAMPORTS_PER_LAMPORT,
            },
        );

        assert_eq!(
            FeeDetails::new(FeeType::ComputeUnitPrice(u128::MAX), u128::MAX),
            FeeDetails {
                fee: u128::MAX,
                priority: u128::MAX,
            },
        );
    }

    #[test]
    fn test_new_with_deprecated_fee() {
        assert_eq!(
            FeeDetails::new(FeeType::Deprecated(1), MICRO_LAMPORTS_PER_LAMPORT / 2 - 1),
            FeeDetails {
                fee: 1,
                priority: 2,
            },
            "should round down fee rate of (>2.0) to priority value 1"
        );

        assert_eq!(
            FeeDetails::new(FeeType::Deprecated(1), MICRO_LAMPORTS_PER_LAMPORT / 2),
            FeeDetails {
                fee: 1,
                priority: 2,
            },
        );

        assert_eq!(
            FeeDetails::new(FeeType::Deprecated(1), MICRO_LAMPORTS_PER_LAMPORT / 2 + 1),
            FeeDetails {
                fee: 1,
                priority: 1,
            },
            "should round down fee rate of (<2.0) to priority value 1"
        );

        assert_eq!(
            FeeDetails::new(FeeType::Deprecated(1), MICRO_LAMPORTS_PER_LAMPORT),
            FeeDetails {
                fee: 1,
                priority: 1,
            },
        );

        assert_eq!(
            FeeDetails::new(FeeType::Deprecated(42), 42 * MICRO_LAMPORTS_PER_LAMPORT),
            FeeDetails {
                fee: 42,
                priority: 1,
            },
        );

        assert_eq!(
            FeeDetails::new(FeeType::Deprecated(420), 42 * MICRO_LAMPORTS_PER_LAMPORT),
            FeeDetails {
                fee: 420,
                priority: 10,
            },
        );

        assert_eq!(
            FeeDetails::new(
                FeeType::Deprecated(u128::MAX),
                2 * MICRO_LAMPORTS_PER_LAMPORT
            ),
            FeeDetails {
                fee: u128::MAX,
                priority: u128::MAX / 2,
            },
        );

        assert_eq!(
            FeeDetails::new(FeeType::Deprecated(u128::MAX), u128::MAX),
            FeeDetails {
                fee: u128::MAX,
                priority: MICRO_LAMPORTS_PER_LAMPORT,
            },
        );
    }
}
