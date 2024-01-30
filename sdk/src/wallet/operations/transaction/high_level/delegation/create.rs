// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::{
    client::{api::PreparedTransactionData, secret::SecretManage},
    types::block::{
        address::{AccountAddress, Bech32Address},
        context_input::{CommitmentContextInput, ContextInput},
        output::{unlock_condition::AddressUnlockCondition, DelegationId, DelegationOutputBuilder},
    },
    wallet::{operations::transaction::TransactionOptions, types::TransactionWithMetadata, Wallet},
};

/// Params for `create_delegation_output()`
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDelegationParams {
    /// Bech32 encoded address which will control the delegation.
    /// By default, the ed25519 wallet address will be used.
    // TODO: https://github.com/iotaledger/iota-sdk/issues/1888
    pub address: Option<Bech32Address>,
    /// The amount to delegate.
    pub delegated_amount: u64,
    /// The Account Address of the validator to which this output will delegate.
    pub validator_address: AccountAddress,
}

/// The result of a transaction to create a delegation
#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDelegationTransaction {
    pub delegation_id: DelegationId,
    pub transaction: TransactionWithMetadata,
}

/// The result of preparing a transaction to create a delegation
#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparedCreateDelegationTransaction {
    pub delegation_id: DelegationId,
    pub transaction: PreparedTransactionData,
}

impl<S: 'static + SecretManage> Wallet<S>
where
    crate::wallet::Error: From<S::Error>,
    crate::client::Error: From<S::Error>,
{
    /// Creates a delegation output.
    /// ```ignore
    /// let params = CreateDelegationParams {
    ///     address: None,
    ///     delegated_amount: 200,
    ///     validator_address: AccountAddress::from_str("0xe1d4bad757d5180811ab81f6c014bb2d66c152efe56cf7a467047625b0016868")?,
    ///     start_epoch: EpochIndex(20),
    ///     end_epoch: EpochIndex(30),
    /// };
    ///
    /// let transaction = delegation.create_delegation_output(params, None).await?;
    /// println!(
    ///     "Transaction sent: {}/transaction/{}",
    ///     std::env::var("EXPLORER_URL").unwrap(),
    ///     transaction.transaction_id
    /// );
    /// ```
    pub async fn create_delegation_output(
        &self,
        params: CreateDelegationParams,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<CreateDelegationTransaction> {
        let options = options.into();
        let prepared = self.prepare_create_delegation_output(params, options.clone()).await?;

        self.sign_and_submit_transaction(prepared.transaction, None, options)
            .await
            .map(|transaction| CreateDelegationTransaction {
                delegation_id: prepared.delegation_id,
                transaction,
            })
    }

    /// Prepares the transaction for [Wallet::create_delegation_output()].
    pub async fn prepare_create_delegation_output(
        &self,
        params: CreateDelegationParams,
        options: impl Into<Option<TransactionOptions>> + Send,
    ) -> crate::wallet::Result<PreparedCreateDelegationTransaction> {
        log::debug!("[TRANSACTION] prepare_create_delegation_output");
        let protocol_parameters = self.client().get_protocol_parameters().await?;

        let address = match params.address.as_ref() {
            Some(bech32_address) => {
                self.client().bech32_hrp_matches(bech32_address.hrp()).await?;
                bech32_address.inner().clone()
            }
            None => self.address().await.inner().clone(),
        };

        let mut options = options.into();
        let slot_commitment_id = if let Some(commitment_context) = options
            .iter()
            .flat_map(|opts| opts.context_inputs.iter())
            .flat_map(|c| c.iter())
            .find_map(|c| c.as_commitment_opt())
        {
            commitment_context.slot_commitment_id()
        } else {
            // Add a commitment context input with the latest slot commitment
            let latest_id = self.client().get_info().await?.node_info.status.latest_commitment_id;
            let context_input = ContextInput::from(CommitmentContextInput::new(latest_id));
            if let Some(options) = &mut options {
                if let Some(context_inputs) = &mut options.context_inputs {
                    context_inputs.push(context_input);
                } else {
                    options.context_inputs.replace(vec![context_input]);
                }
            } else {
                options.replace(TransactionOptions {
                    context_inputs: Some(vec![context_input]),
                    ..Default::default()
                });
            }
            latest_id
        };

        let delegation_output_builder = DelegationOutputBuilder::new_with_amount(
            params.delegated_amount,
            DelegationId::null(),
            params.validator_address,
        )
        .with_start_epoch(protocol_parameters.delegation_start_epoch(slot_commitment_id))
        .add_unlock_condition(AddressUnlockCondition::new(address.clone()));

        let output = delegation_output_builder.finish_output()?;

        let transaction = self.prepare_transaction([output], options).await?;

        Ok(PreparedCreateDelegationTransaction {
            delegation_id: DelegationId::from(&transaction.transaction.id().into_output_id(0)),
            transaction,
        })
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{
        client::constants::IOTA_BECH32_HRP,
        types::block::{
            address::ToBech32Ext,
            rand::address::{rand_account_address, rand_address},
        },
    };

    #[test]
    fn create_delegation_params_serde() {
        let params_none_1 = CreateDelegationParams {
            address: None,
            delegated_amount: 100,
            validator_address: rand_account_address(),
        };
        let json_none = serde_json::to_string(&params_none_1).unwrap();
        let params_none_2 = serde_json::from_str(&json_none).unwrap();

        assert_eq!(params_none_1, params_none_2);

        let params_some_1 = CreateDelegationParams {
            address: Some(rand_address().to_bech32(IOTA_BECH32_HRP)),
            delegated_amount: 200,
            validator_address: rand_account_address(),
        };
        let json_some = serde_json::to_string(&params_some_1).unwrap();
        let params_some_2 = serde_json::from_str(&json_some).unwrap();

        assert_eq!(params_some_1, params_some_2);
    }
}
