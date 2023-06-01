#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod poll {

    use ink::storage::Mapping;
    use ink::prelude::{string::String, vec::Vec};
    use ink::primitives::Hash;

    #[ink(storage)]
    pub struct Poll {
        owner: AccoundId,
        /// Title of job (keep it short)
        title: String,
        /// Description of poll (include multiple choice answers + format or other information
        /// that could assist with formating and compiling answers at the end of poll
        description: String,
        /// Reward and reward schemes
        reward: u32,
        reward_dist: bool,
        answers: Mapping<AccoundId, String>,
        // n_responses: u32,
        // filters: str
    }

    // pub struct Answer {
    //
    // }

    impl Job {
        #[ink(constructor)]
        pub fn new(
            _title: String, _description: String, _reward: u32, _reward_dist: bool
        ) -> Self {
            let caller = Self::env().caller();

            Self {
                owner: caller,
                title: _title,
                description: _description,
                reward: _reward,
                reward_dist: _reward_dist,
                answers: Mapping::default(),
            }
        }

        // #[ink(message)]
        // pub fn answer(&mut self, _answer: String) {
        //     let caller = self.env().caller();
        //
        //     // assert_eq!(self.answers.get(caller), );
        //
        //     self.answers.insert(caller, _answer);
        //
        //     // Send event of loss submission after submission of loss
        //     // self.env().emit_event(SentLoss {});
        // }
    }
}