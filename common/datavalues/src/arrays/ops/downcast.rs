// Copyright 2020 Datafuse Labs.
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

use common_arrow::arrow::array::*;
use common_arrow::arrow::bitmap::utils::BitmapIter;
use common_arrow::arrow::bitmap::utils::ZipValidity;

use crate::prelude::*;
use crate::series::Series;

impl<T> DFPrimitiveArray<T>
where T: DFPrimitiveType
{
    #[inline]
    pub fn downcast_ref(&self) -> &PrimitiveArray<T> {
        &self.array
    }

    pub fn downcast_iter(&self) -> ZipValidity<'_, &'_ T, std::slice::Iter<'_, T>> {
        self.array.iter()
    }

    pub fn collect_values(&self) -> Vec<Option<T>> {
        let e = self.downcast_iter().map(|c| c.copied());
        e.collect()
    }

    pub fn from_arrow_array(array: PrimitiveArray<T>) -> Self {
        let array_ref = Arc::new(array) as ArrayRef;
        array_ref.into()
    }
}

impl AsRef<BooleanArray> for DFBooleanArray {
    fn as_ref(&self) -> &BooleanArray {
        self.downcast_ref()
    }
}

impl DFBooleanArray {
    #[inline]
    pub fn downcast_ref(&self) -> &BooleanArray {
        &self.array
    }

    pub fn downcast_iter(&self) -> ZipValidity<'_, bool, BitmapIter<'_>> {
        &self.array.iter()
    }

    pub fn collect_values(&self) -> Vec<Option<bool>> {
        self.downcast_iter().collect()
    }

    pub fn from_arrow_array(array: BooleanArray) -> Self {
        let array_ref = Arc::new(array) as ArrayRef;
        array_ref.into()
    }
}

impl AsRef<LargeUtf8Array> for DFUtf8Array {
    fn as_ref(&self) -> &LargeUtf8Array {
        self.downcast_ref()
    }
}

impl DFUtf8Array {
    #[inline]
    pub fn downcast_ref(&self) -> &LargeUtf8Array {
        &self.array
    }

    pub fn downcast_iter(&self) -> ZipValidity<'_, &'_ str, Utf8ValuesIter<'_, i64>> {
        let arr = &*self.array;
        let arr = unsafe { &*(arr as *const dyn Array as *const LargeUtf8Array) };
        arr.iter()
    }

    pub fn collect_values(&self) -> Vec<Option<&'_ str>> {
        self.downcast_iter().collect()
    }

    pub fn from_arrow_array(array: LargeUtf8Array) -> Self {
        let array_ref = Arc::new(array) as ArrayRef;
        array_ref.into()
    }
}

impl DFListArray {
    pub fn downcast_ref(&self) -> &LargeListArray {
        &self.array
    }

    pub fn downcast_iter(&self) -> impl Iterator<Item = Option<Series>> + DoubleEndedIterator {
        self.array.iter().map(|a| {
            a.map(|b| {
                let c: ArrayRef = Arc::from(b);
                c.into_series()
            })
        })
    }

    pub fn from_arrow_array(array: LargeListArray) -> Self {
        let array_ref = Arc::new(array) as ArrayRef;
        array_ref.into()
    }
}

impl DFBinaryArray {
    pub fn downcast_ref(&self) -> &LargeBinaryArray {
        &self.array
    }

    pub fn collect_values(&self) -> Vec<Option<Vec<u8>>> {
        let e = self.downcast_ref().iter().map(|c| c.map(|d| d.to_owned()));
        e.collect()
    }

    pub fn from_arrow_array(array: LargeBinaryArray) -> Self {
        let array_ref = Arc::new(array) as ArrayRef;
        array_ref.into()
    }
}

impl DFStructArray {
    pub fn downcast_ref(&self) -> &StructArray {
        &self.array
    }

    pub fn from_arrow_array(array: StructArray) -> Self {
        let array_ref = Arc::new(array) as ArrayRef;
        array_ref.into()
    }
}
