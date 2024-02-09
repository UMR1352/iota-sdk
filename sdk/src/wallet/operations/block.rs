// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::secret::{BlockSignExt, SecretManage},
    types::block::{output::AccountId, payload::Payload, BlockId},
    wallet::{core::SecretData, Error, Result, Wallet},
};

impl<S: SecretManage> Wallet<SecretData<S>> {
    pub(crate) async fn submit_basic_block(
        &self,
        payload: impl Into<Option<Payload>> + Send,
        issuer_id: impl Into<Option<AccountId>> + Send,
        allow_negative_bic: bool,
    ) -> Result<BlockId> {
        log::debug!("submit_basic_block");
        // If an issuer ID is provided, use it; otherwise, use the first available account or implicit account.
        let issuer_id = match issuer_id.into() {
            Some(id) => id,
            None => self.data().await.first_account_id().ok_or(Error::AccountNotFound)?,
        };

        let unsigned_block = self.client().build_basic_block(issuer_id, payload).await?;

        if !allow_negative_bic {
            let protocol_parameters = self.client().get_protocol_parameters().await?;
            let work_score = protocol_parameters.work_score(unsigned_block.body.as_basic());
            let congestion = self.client().get_account_congestion(&issuer_id, work_score).await?;
            if (congestion.reference_mana_cost * work_score as u64) as i128 > congestion.block_issuance_credits {
                return Err(crate::wallet::Error::InsufficientBic {
                    available: congestion.block_issuance_credits,
                    required: work_score as u64 * congestion.reference_mana_cost,
                });
            }
        }

        let block = unsigned_block
            .sign_ed25519(&*self.secret_manager().read().await, &self.signing_options())
            .await?;

        let block_id = self.client().post_block(&block).await?;

        log::debug!("submitted block {}", block_id);

        Ok(block_id)
    }
}
