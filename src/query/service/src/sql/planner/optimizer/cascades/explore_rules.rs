// Copyright 2021 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::sql::optimizer::RuleID;
use crate::sql::optimizer::RuleSet;

pub fn get_explore_rule_set() -> RuleSet {
    RuleSet::create_with_ids(vec![
        RuleID::CommuteJoin,
        RuleID::LeftAssociateJoin,
        RuleID::RightAssociateJoin,
    ])
    .unwrap()
}

#[cfg(test)]
mod test {
    use crate::sql::optimizer::cascades::explore_rules::get_explore_rule_set;

    // Pass if don't panic
    #[test]
    fn test_get_explore_rule_set() {
        get_explore_rule_set();
    }
}