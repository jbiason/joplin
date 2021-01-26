/*
   A library for accessing Joplin
   Copyright (C) 2021  Julio Biason <julio.biason@pm.me>

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use serde_derive::Deserialize;

/// Joplin information
pub struct Joplin {
    port: u16,
    token: String,
}

// A paged result
struct Paged<T> {
    items: Vec<T>,
    has_more: bool,
}

/// Folder information
#[derive(Deserialize)]
struct Folder {
    id: String,
    title: String,
    parent_id: String,
}

impl Joplin {
    /// Create a Joplin connection with the WebClipper token.
    pub fn with_token(token: &str) -> Self {
        Self::with_token_and_port(token, 41128)
    }

    /// Create a Joplin connection using the WebClipper token, but the
    /// webclipper is not running on the default port.
    pub fn with_token_and_port(token: &str, port: u16) -> Self {
        Self {
            port,
            token: token.to_string(),
        }
    }
}
