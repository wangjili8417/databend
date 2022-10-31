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

use std::sync::Arc;

use common_datablocks::DataBlock;
use common_datavalues::prelude::*;
use common_datavalues::Series;
use common_exception::Result;
use common_planner::plans::ShowRolesPlan;
use common_users::UserApiProvider;

use crate::interpreters::Interpreter;
use crate::pipelines::PipelineBuildResult;
use crate::sessions::QueryContext;
use crate::sessions::TableContext;

#[derive(Debug)]
pub struct ShowRolesInterpreter {
    ctx: Arc<QueryContext>,
    plan: ShowRolesPlan,
}

impl ShowRolesInterpreter {
    pub fn try_create(ctx: Arc<QueryContext>, plan: ShowRolesPlan) -> Result<Self> {
        Ok(ShowRolesInterpreter { ctx, plan })
    }
}

#[async_trait::async_trait]
impl Interpreter for ShowRolesInterpreter {
    fn name(&self) -> &str {
        "ShowRolesInterpreter"
    }

    #[tracing::instrument(level = "debug", skip(self), fields(ctx.id = self.ctx.get_id().as_str()))]
    async fn execute2(&self) -> Result<PipelineBuildResult> {
        let session = self.ctx.get_current_session();
        let roles = session.get_all_available_roles().await?;

        let names: Vec<&str> = roles.iter().map(|x| x.name.as_str()).collect();
        let inherited_roles: Vec<u64> = roles
            .iter()
            .map(|x| x.grants.roles().len() as u64)
            .collect();

        // TODO
        PipelineBuildResult::from_blocks(vec![DataBlock::create(self.plan.schema(), vec![
            Series::from_data(names),
            Series::from_data(inherited_roles),
        ])])
    }
}
