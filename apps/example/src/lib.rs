#![cfg_attr(not(test), no_std)]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
extern crate alloc;

use crate::types::call::Data;
use crate::types::{query, query_response, GetName, QueryResponse, ReservationInfo};
use primitive_types::Address;
use rune_framework::context::Context;
use rune_framework::io::{Blake2bHasher, StorageMap, StorageValue};
use rune_framework::*;
use rune_std::prelude::*;

#[allow(unused_imports)]
#[allow(dead_code)]
#[path = "./example.types.rs"]
pub mod types;

struct Nick;

struct NameMap;

struct ReservationFee;

impl StorageMap<Blake2bHasher, Address, ReservationInfo> for NameMap {
    fn storage_prefix() -> &'static [u8] {
        b"NameMap"
    }
}

impl StorageValue<Blake2bHasher, u64> for ReservationFee {
    fn storage_prefix() -> &'static [u8] {
        b"ReservationFee"
    }

    fn storage_key() -> &'static [u8] {
        b"Fee"
    }
}

impl RuntimeApplication for Nick {
    type Call = types::Call;
    type Query = types::Query;
    type QueryResponse = types::QueryResponse;

    fn genesis(_: Context) -> anyhow::Result<()> {
        ReservationFee::set(10 * 100_000);
        Ok(())
    }

    fn call(context: Context, call: Self::Call) -> anyhow::Result<()> {
        let sender = context.sender();
        let Some(data) = call.data else {
            return Ok(());
        };
        match data {
            Data::SetName(param) => {
                let fee = if let Some(info) = NameMap::get(sender)? {
                    info.fee
                } else {
                    ReservationFee::get()?
                };
                anyhow::ensure!(rune_framework::syscall::reserve(fee));
                NameMap::insert(
                    sender,
                    ReservationInfo {
                        name: param.name,
                        fee,
                    },
                )
            }
            Data::ClearName(_) => {
                NameMap::remove(sender)?;
            }
        }
        return Ok(());
    }

    fn query(query: Self::Query) -> Self::QueryResponse {
        let Some(data) = query.data else {
            return QueryResponse::default();
        };
        match data {
            query::Data::GetName(GetName { owner }) => {
                let Ok(Some(data)) = Address::from_slice(&owner).and_then(|owner| {
                    NameMap::get(owner).map_err(|_| ())
                })else {
                    return QueryResponse::default()
                };

                QueryResponse {
                    data: Some(query_response::Data::Info(data)),
                }
            }
        }
    }
}

export_app!(Nick);
