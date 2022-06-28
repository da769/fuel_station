use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen};
use std::collections::HashMap;

mod info;

#[near_bindgen]
#[derive(Default, BorshSerialize, BorshDeserialize)]
pub struct Client{
    pump: u8,
    fuel : String,
    cost : f32,
}
#[near_bindgen]
#[derive(Default, BorshSerialize, BorshDeserialize)]
pub struct Pay{
    tag_name : String,
    amount : f32,
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract{
    prizetag: HashMap<String, f32>,
    pump_allocation: HashMap<u8, Client>,
    payments: HashMap<String, Pay>,
}
#[near_bindgen]
impl Default for Contract{
    fn default() -> Self{
        Self {
            pump_allocation : HashMap::new(), 
            prizetag: info::prizetag_items(),
            payments : HashMap::new(), 
        }
    }
}
#[near_bindgen]
impl Contract {
    pub fn prizetag(){
        env::log_str("\n
        diesel 2.09
        petrol 3.10,
        kerosine 1.06,
        gas 2.08,")
    }
    pub fn fuel( &mut self, pump_number : u8,fuel_choice : String){
        let fuel_choice = fuel_choice.to_lowercase();

        if self.prizetag.contains_key(&fuel_choice){
            let cost : f32 =self.prizetag[&fuel_choice];
            let client_new = Client {
                pump : pump_number,
                fuel : fuel_choice,
                cost
            };
            self.pump_allocation.insert(pump_number, client_new);
            log!
            ("Your fuel should cost: {}tokens",
        self.pump_allocation[&pump_number].cost);
            }
        else{
            env::log_str("Your choice is not served here, please try another option");
        }
    }
    pub fn pays(& mut self , tag : String , cash : f32){
        let new_pay = Pay {
            tag_name : tag.to_string(), 
            amount : cash,
        };
        self.payments.insert(tag, new_pay);

    }

    pub fn pay(&mut self, pump_number : u8 , _price :f64) -> String{
        let tokens = env::attached_deposit();
        let charge = self.pump_allocation[&pump_number].cost;

        let token_near = Contract::to_near(tokens);
        if token_near <= 0.00002{
            return "unsuccessful".to_string()
        } else {log!("You have paid more by:{}", (token_near - charge));
        return "paid more".to_string()}
    }

    fn to_near(yocto : u128) -> f32{
        (yocto as f32) / 100000000000000000000.0
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    // use near_sdk::test_utils::{get_logs, VMContextBuilder};
    // use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    // fn get_context(predecessor:AccountId) -> VMContextBuilder {
    //     let mut builder = VMContextBuilder::new();
    //     builder.predecessor_account_id(predecessor);
    //     builder
    // }

    // TESTS HERE

#[test]
    fn choose_fuel(){
        let mut contract: Contract = Contract::default();
        contract.fuel(2, "diesel".to_string());
        contract.fuel(4,"petrol".to_string());
        assert_eq!(2, contract.pump_allocation.len());
        assert_eq!("diesel".to_string(), contract.pump_allocation[&2].fuel);
        let diesel_cost= 2.09;
        assert_eq!(diesel_cost, contract.pump_allocation[&2].cost);
    }
    #[test]
    fn pay_test(){
        let mut contract: Contract = Contract::default();
        contract.pays("diesel".to_string(), 6.27); 
        contract.pays("petrol".to_string(), 6.20); 

        let totals = contract.payments.len();
        assert_eq!(2, totals);
    }
}




