use ethers::types::Address;
use std::str::FromStr;

trait EtheriumAddress {
    fn convert_adress(&self) -> Result<Address, &'static str>;
}

impl EtheriumAddress for &str {
    fn convert_adress(&self) -> Result<Address, &'static str> {
        match Address::from_str(self) {
            Ok(address) => Ok(address),
            Err(_) => Err("Invalid adress"),
        }
    }
}

impl EtheriumAddress for Address {
    fn convert_adress(&self) -> Result<Address, &'static str> {
        Ok(*self)
        // match Address::from_str(self) {

        //     // Ok(address) => Ok(address),
        //     // Err(_) => Err("Invalid adress")
        // }
    }
}

fn get_etherium_data<T: EtheriumAddress>(address: T) -> Address {
    let converted_address: Address = address.convert_adress().unwrap();
    converted_address
    //assert_eq!(converted_address, Address)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_poly() {
        //dbg!("Hello tests_structs!");
        let addr: Address =
            Address::from_str("0x45D74e7c05092BeA1446006Fb5Cb2479041E1dC0").unwrap();

            assert_eq!(addr, Address::from_str("0x45D74e7c05092BeA1446006Fb5Cb2479041E1dC0").unwrap());    
    }
}
