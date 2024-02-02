// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::{
    client::secret::types::InputSigningData,
    types::{
        block::{
            address::Address,
            output::Output,
            payload::{
                signed_transaction::{
                    dto::{SignedTransactionPayloadDto, TransactionDto},
                    Transaction,
                },
                SignedTransactionPayload,
            },
            protocol::ProtocolParameters,
            Error,
        },
        TryFromDto,
    },
};

/// Helper struct for offline signing
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedTransactionData {
    /// Transaction
    pub transaction: Transaction,
    /// Required input information for signing. Inputs need to be ordered by address type
    pub inputs_data: Vec<InputSigningData>,
    /// Remainder outputs information
    pub remainders: Vec<RemainderData>,
}

/// PreparedTransactionData Dto
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparedTransactionDataDto {
    /// Transaction
    pub transaction: TransactionDto,
    /// Required address information for signing
    pub inputs_data: Vec<InputSigningData>,
    /// Remainder outputs information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remainders: Vec<RemainderData>,
}

impl From<&PreparedTransactionData> for PreparedTransactionDataDto {
    fn from(value: &PreparedTransactionData) -> Self {
        Self {
            transaction: TransactionDto::from(&value.transaction),
            inputs_data: value.inputs_data.clone(),
            remainders: value.remainders.clone(),
        }
    }
}

impl TryFromDto<PreparedTransactionDataDto> for PreparedTransactionData {
    type Error = Error;

    fn try_from_dto_with_params_inner(
        dto: PreparedTransactionDataDto,
        params: Option<&ProtocolParameters>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction: Transaction::try_from_dto_with_params_inner(dto.transaction, params)
                .map_err(|_| Error::InvalidField("transaction"))?,
            inputs_data: dto.inputs_data,
            remainders: dto.remainders,
        })
    }
}

impl Serialize for PreparedTransactionData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        PreparedTransactionDataDto::from(self).serialize(serializer)
    }
}

/// Helper struct for offline signing
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignedTransactionData {
    /// Signed transaction payload
    pub payload: SignedTransactionPayload,
    /// Required address information for signing
    pub inputs_data: Vec<InputSigningData>,
}

/// SignedTransactionData Dto
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedTransactionDataDto {
    /// Signed transaction payload
    pub payload: SignedTransactionPayloadDto,
    /// Required address information for signing
    pub inputs_data: Vec<InputSigningData>,
}

impl From<&SignedTransactionData> for SignedTransactionDataDto {
    fn from(value: &SignedTransactionData) -> Self {
        Self {
            payload: SignedTransactionPayloadDto::from(&value.payload),
            inputs_data: value.inputs_data.clone(),
        }
    }
}

impl TryFromDto<SignedTransactionDataDto> for SignedTransactionData {
    type Error = Error;

    fn try_from_dto_with_params_inner(
        dto: SignedTransactionDataDto,
        params: Option<&ProtocolParameters>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            payload: SignedTransactionPayload::try_from_dto_with_params_inner(dto.payload, params)
                .map_err(|_| Error::InvalidField("transaction_payload"))?,
            inputs_data: dto.inputs_data,
        })
    }
}

/// Data for a remainder output, used for ledger nano
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RemainderData {
    /// The remainder output
    pub output: Output,
    /// The remainder address
    pub address: Address,
}
