/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use core::marker::PhantomData;
use core::sync::atomic::{self, AtomicU64};

use enum_iterator::{cardinality, Sequence};

pub struct TypedCounters<Type: Sequence + Into<usize>> {
    counters: Vec<AtomicU64>,
    _type: PhantomData<Type>,
}
impl<Type: Sequence + Into<usize>> TypedCounters<Type> {
    pub fn new() -> Self {
        let num_types = cardinality::<Type>();
        let mut counters = Vec::with_capacity(num_types);
        counters.resize_with(num_types, AtomicU64::default);
        Self {
            counters,
            _type: PhantomData::default(),
        }
    }
    pub fn add(&self, t: Type, n: u64) {
        self.counters[t.into()].fetch_add(n, atomic::Ordering::Relaxed);
    }
    pub fn counter(&self, t: Type) -> u64 {
        self.counters[t.into()].load(atomic::Ordering::Relaxed)
    }
    pub fn counters(&self) -> &[AtomicU64] {
        self.counters.as_slice()
    }
}
