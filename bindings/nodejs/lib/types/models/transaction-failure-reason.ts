// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * Reason for transaction failure.
 */
export enum TransactionFailureReason {
    /**
     * The referenced UTXO was already spent.
     */
    InputUTXOAlreadySpent = 1,

    /**
     * The transaction is conflicting with another transaction.
     * Conflicting specifically means a double spend situation that both transactions pass all validation rules,
     * eventually losing one(s) should have this reason.
     */
    ConflictingWithAnotherTx = 2,

    /**
     * The referenced UTXO is invalid.
     */
    InvalidReferencedUtxo = 3,

    /**
     * The transaction is invalid.
     */
    InvalidTransaction = 4,

    /**
     * The sum of the inputs and output base token amount does not match.
     */
    SumInputsOutputsAmountMismatch = 5,

    /**
     * The unlock block signature is invalid.
     */
    InvalidUnlockBlockSignature = 6,

    /**
     * The configured timelock is not yet expired.
     */
    TimelockNotExpired = 7,

    /**
     * The given native tokens are invalid.
     */
    InvalidNativeTokens = 8,

    /**
     * The return amount in a transaction is not fulfilled by the output side.
     */
    StorageDepositReturnUnfulfilled = 9,

    /**
     * An input unlock was invalid.
     */
    InvalidInputUnlock = 10,

    /**
     * The output contains a Sender with an ident (address) which is not unlocked.
     */
    SenderNotUnlocked = 11,

    /**
     * The chain state transition is invalid.
     */
    InvalidChainStateTransition = 12,

    /**
     * The referenced input is created after transaction issuing time.
     */
    InvalidTransactionIssuingTime = 13,

    /**
     * The mana amount is invalid.
     */
    InvalidManaAmount = 14,

    /**
     * The Block Issuance Credits amount is invalid.
     */
    InvalidBlockIssuanceCreditsAmount = 15,

    /**
     * Reward Context Input is invalid.
     */
    InvalidRewardContextInput = 16,

    /**
     * Commitment Context Input is invalid.
     */
    InvalidCommitmentContextInput = 17,

    /**
     * Staking Feature is not provided in account output when claiming rewards.
     */
    MissingStakingFeature = 18,

    /**
     * Failed to claim staking reward.
     */
    FailedToClaimStakingReward = 19,

    /**
     * Failed to claim delegation reward.
     */
    FailedToClaimDelegationReward = 20,

    /**
     * Burning of native tokens is not allowed in the transaction capabilities.
     */
    TransactionCapabilityNativeTokenBurningNotAllowed = 21,

    /**
     * Burning of mana is not allowed in the transaction capabilities.
     */
    TransactionCapabilityManaBurningNotAllowed = 22,

    /**
     * Destruction of accounts is not allowed in the transaction capabilities.
     */
    TransactionCapabilityAccountDestructionNotAllowed = 23,

    /**
     * Destruction of anchors is not allowed in the transaction capabilities.
     */
    TransactionCapabilityAnchorDestructionNotAllowed = 24,

    /**
     * Destruction of foundries is not allowed in the transaction capabilities.
     */
    TransactionCapabilityFoundryDestructionNotAllowed = 25,

    /**
     * Destruction of nfts is not allowed in the transaction capabilities.
     */
    TransactionCapabilityNftDestructionNotAllowed = 26,

    /**
     * The semantic validation failed for a reason not covered by the previous variants.
     */
    SemanticValidationFailed = 255,
}

/**
 * Transaction failure reason strings.
 */
export const TRANSACTION_FAILURE_REASON_STRINGS: {
    [key in TransactionFailureReason]: string;
} = {
    [TransactionFailureReason.InputUTXOAlreadySpent]:
        'The referenced UTXO was already spent.',
    [TransactionFailureReason.ConflictingWithAnotherTx]:
        'The transaction is conflicting with another transaction. Conflicting specifically means a double spend situation that both transactions pass all validation rules, eventually losing one(s) should have this reason.',
    [TransactionFailureReason.InvalidReferencedUtxo]:
        'The referenced UTXO is invalid.',
    [TransactionFailureReason.InvalidTransaction]:
        'The transaction is invalid.',
    [TransactionFailureReason.SumInputsOutputsAmountMismatch]:
        'The sum of the inputs and output base token amount does not match.',
    [TransactionFailureReason.InvalidUnlockBlockSignature]:
        'The unlock block signature is invalid.',
    [TransactionFailureReason.TimelockNotExpired]:
        'The configured timelock is not yet expired.',
    [TransactionFailureReason.InvalidNativeTokens]:
        'The given native tokens are invalid.',
    [TransactionFailureReason.StorageDepositReturnUnfulfilled]:
        'The return amount in a transaction is not fulfilled by the output side.',
    [TransactionFailureReason.InvalidInputUnlock]:
        'An input unlock was invalid.',
    [TransactionFailureReason.SenderNotUnlocked]:
        'The output contains a Sender with an ident (address) which is not unlocked.',
    [TransactionFailureReason.InvalidChainStateTransition]:
        'The chain state transition is invalid.',
    [TransactionFailureReason.InvalidTransactionIssuingTime]:
        'The referenced input is created after transaction issuing time.',
    [TransactionFailureReason.InvalidManaAmount]: 'The mana amount is invalid.',
    [TransactionFailureReason.InvalidBlockIssuanceCreditsAmount]:
        'The Block Issuance Credits amount is invalid.',
    [TransactionFailureReason.InvalidRewardContextInput]:
        'Reward Context Input is invalid.',
    [TransactionFailureReason.InvalidCommitmentContextInput]:
        'Commitment Context Input is invalid.',
    [TransactionFailureReason.MissingStakingFeature]:
        'Staking Feature is not provided in account output when claiming rewards.',
    [TransactionFailureReason.FailedToClaimStakingReward]:
        'Failed to claim staking reward.',
    [TransactionFailureReason.FailedToClaimDelegationReward]:
        'Failed to claim delegation reward.',
    [TransactionFailureReason.TransactionCapabilityNativeTokenBurningNotAllowed]:
        'Burning of native tokens is not allowed in the transaction capabilities.',
    [TransactionFailureReason.TransactionCapabilityManaBurningNotAllowed]:
        'Burning of mana is not allowed in the transaction capabilities.',
    [TransactionFailureReason.TransactionCapabilityAccountDestructionNotAllowed]:
        'Destruction of accounts is not allowed in the transaction capabilities.',
    [TransactionFailureReason.TransactionCapabilityAnchorDestructionNotAllowed]:
        'Destruction of anchors is not allowed in the transaction capabilities.',
    [TransactionFailureReason.TransactionCapabilityFoundryDestructionNotAllowed]:
        'Destruction of foundries is not allowed in the transaction capabilities.',
    [TransactionFailureReason.TransactionCapabilityNftDestructionNotAllowed]:
        'Destruction of nfts is not allowed in the transaction capabilities.',
    [TransactionFailureReason.SemanticValidationFailed]:
        'The semantic validation failed for a reason not covered by the previous variants.',
};
