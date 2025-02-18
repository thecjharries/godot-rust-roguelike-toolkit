// Copyright 2025 CJ Harries
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

pub mod bsp;
pub mod grid;
pub mod point;
pub mod rectangle;

type Result<T> = std::result::Result<T, MapError>;

#[derive(Debug)]
pub enum MapError {
    MinimumRoomSizeMustBeSmallerThanGridSize,
}

impl MapError {
    fn description(&self) -> &str {
        match self {
            MapError::MinimumRoomSizeMustBeSmallerThanGridSize => {
                "Minimum room size must be smaller than grid size"
            }
            _ => "An unknown error occurred",
        }
    }
}

impl std::fmt::Display for MapError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl std::error::Error for MapError {
    fn description(&self) -> &str {
        self.description()
    }
}
