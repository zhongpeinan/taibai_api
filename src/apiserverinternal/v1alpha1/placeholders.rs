use crate::common::UnimplementedConversion;
use crate::impl_unimplemented_prost_message;

use super::{StorageVersion, StorageVersionList};

impl UnimplementedConversion for StorageVersion {}

impl_unimplemented_prost_message!(StorageVersion);
impl_unimplemented_prost_message!(StorageVersionList);
