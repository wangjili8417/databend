// Copyright 2023 Datafuse Labs.
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
//

/// This is an ongoing refactor of table mutations:
/// which will eventually unify the mutation operation, mutation log, commit action
pub mod mutation_meta;
mod mutator;
mod processors;

pub use mutator::mutation_accumulator::MutationAccumulator;
pub use processors::AppendTransform;
pub use processors::BroadcastProcessor;
pub use processors::CommitSink;
pub use processors::MergeIntoOperationAggregator;
pub use processors::OnConflictField;
pub use processors::TableMutationAggregator;