#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait Factorial {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn factorial(&self, value : u32) -> () {
        let result = self.calculate_factorial(value);
        require!(
            self.blockchain().get_sc_balance(&EgldOrEsdtTokenIdentifier::egld(), 0) > result,
            "Not enough money in the SC"
        );
        self.send().direct_egld(&self.blockchain().get_caller(), &result);
    }
    
    #[payable("EGLD")]
    #[only_owner]
    #[endpoint]
    fn fill(&self) -> SCResult<()> {
        Ok(())
    }

    #[view]
    fn calculate_factorial(&self, value : u32) -> BigUint {
        let one = 1u32;
        let mut result= 1u32;
        if value != 0 {
            let mut x = 1u32;
            while x <= value {
                result *= &x;
                x += &one;
            }
        }
        BigUint::from(result)
    }

}
