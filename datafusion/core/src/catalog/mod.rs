// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! This module contains interfaces and default implementations
//! of table namespacing concepts, including catalogs and schemas.

// TODO(clippy): Having a `catalog::catalog` module path is unclear and ambiguous.
// The parent module should probably be renamed to something that more accurately
// describes its content. Something along the lines of `database_meta`, `metadata`
// or `meta`, perhaps?
#![allow(clippy::module_inception)]
pub mod catalog;
pub mod information_schema;
pub mod listing_schema;
pub mod schema;

pub use datafusion_sql::{ResolvedTableReference, TableReference};
