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

pub struct ReadSettings {
    pub storage_io_min_bytes_for_seek: u64,
    pub storage_io_max_page_bytes_for_read: u64,
}

impl Default for ReadSettings {
    fn default() -> Self {
        ReadSettings {
            storage_io_min_bytes_for_seek: 1024,
            storage_io_max_page_bytes_for_read: 1024 * 1024,
        }
    }
}