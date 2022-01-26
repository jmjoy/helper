// Copyright (c) 2021 jmjoy
// Helper is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR
// FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

#[cfg(test)]
use proc_macro2::TokenStream;

#[cfg(test)]
pub(crate) fn pm_test_left(f: impl FnOnce(TokenStream) -> TokenStream, s: &str) -> String {
    f(s.parse().unwrap()).to_string()
}

#[cfg(test)]
pub(crate) fn pm_test_right(s: &str) -> String {
    s.parse::<TokenStream>().unwrap().to_string()
}
