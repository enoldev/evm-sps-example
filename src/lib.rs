mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

const USDC_TRACKED_CONTRACT: [u8; 20] = hex!("a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48");

fn map_usdc_events(blk: &eth::Block, events: &mut contract::Events) {
    events.usdc_admin_changeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDC_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdc_contract::events::AdminChanged::match_and_decode(log) {
                        return Some(contract::UsdcAdminChanged {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_admin: event.new_admin,
                            previous_admin: event.previous_admin,
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdc_approvals.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDC_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdc_contract::events::Approval::match_and_decode(log) {
                        return Some(contract::UsdcApproval {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            owner: event.owner,
                            spender: event.spender,
                            value: event.value.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdc_authorization_canceleds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDC_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdc_contract::events::AuthorizationCanceled::match_and_decode(log) {
                        return Some(contract::UsdcAuthorizationCanceled {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            authorizer: event.authorizer,
                            nonce: Vec::from(event.nonce),
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdc_authorization_useds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDC_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdc_contract::events::AuthorizationUsed::match_and_decode(log) {
                        return Some(contract::UsdcAuthorizationUsed {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            authorizer: event.authorizer,
                            nonce: Vec::from(event.nonce),
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdc_blacklisteds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDC_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdc_contract::events::Blacklisted::match_and_decode(log) {
                        return Some(contract::UsdcBlacklisted {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            u_account: event.u_account,
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdc_blacklister_changeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDC_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdc_contract::events::BlacklisterChanged::match_and_decode(log) {
                        return Some(contract::UsdcBlacklisterChanged {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_blacklister: event.new_blacklister,
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdc_burns.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDC_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdc_contract::events::Burn::match_and_decode(log) {
                        return Some(contract::UsdcBurn {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            burner: event.burner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdc_master_minter_changeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDC_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdc_contract::events::MasterMinterChanged::match_and_decode(log) {
                        return Some(contract::UsdcMasterMinterChanged {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_master_minter: event.new_master_minter,
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdc_mints.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDC_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdc_contract::events::Mint::match_and_decode(log) {
                        return Some(contract::UsdcMint {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            minter: event.minter,
                            to: event.to,
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdc_minter_configureds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDC_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdc_contract::events::MinterConfigured::match_and_decode(log) {
                        return Some(contract::UsdcMinterConfigured {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            minter: event.minter,
                            minter_allowed_amount: event.minter_allowed_amount.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdc_minter_removeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDC_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdc_contract::events::MinterRemoved::match_and_decode(log) {
                        return Some(contract::UsdcMinterRemoved {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            old_minter: event.old_minter,
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdc_ownership_transferreds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDC_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdc_contract::events::OwnershipTransferred::match_and_decode(log) {
                        return Some(contract::UsdcOwnershipTransferred {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_owner: event.new_owner,
                            previous_owner: event.previous_owner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdc_pauses.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDC_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdc_contract::events::Pause::match_and_decode(log) {
                        return Some(contract::UsdcPause {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdc_pauser_changeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDC_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdc_contract::events::PauserChanged::match_and_decode(log) {
                        return Some(contract::UsdcPauserChanged {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_address: event.new_address,
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdc_rescuer_changeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDC_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdc_contract::events::RescuerChanged::match_and_decode(log) {
                        return Some(contract::UsdcRescuerChanged {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_rescuer: event.new_rescuer,
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdc_transfers.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDC_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdc_contract::events::Transfer::match_and_decode(log) {
                        return Some(contract::UsdcTransfer {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            from: event.from,
                            to: event.to,
                            value: event.value.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdc_un_blacklisteds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDC_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdc_contract::events::UnBlacklisted::match_and_decode(log) {
                        return Some(contract::UsdcUnBlacklisted {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            u_account: event.u_account,
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdc_unpauses.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDC_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdc_contract::events::Unpause::match_and_decode(log) {
                        return Some(contract::UsdcUnpause {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                        });
                    }

                    None
                })
        })
        .collect());
    events.usdc_upgradeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == USDC_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::usdc_contract::events::Upgraded::match_and_decode(log) {
                        return Some(contract::UsdcUpgraded {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            implementation: event.implementation,
                        });
                    }

                    None
                })
        })
        .collect());
}
fn map_usdc_calls(blk: &eth::Block, calls: &mut contract::Calls) {
    calls.usdc_call_approves.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::Approve::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::Approve::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::usdc_contract::functions::Approve::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::UsdcApproveCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                output_param0: output_param0,
                                spender: decoded_call.spender,
                                value: decoded_call.value.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_blacklists.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::Blacklist::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::Blacklist::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdcBlacklistCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_account: decoded_call.u_account,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_burns.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::Burn::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::Burn::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdcBurnCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_amount: decoded_call.u_amount.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_cancel_authorization_1s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::CancelAuthorization1::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::CancelAuthorization1::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdcCancelAuthorization1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                authorizer: decoded_call.authorizer,
                                nonce: Vec::from(decoded_call.nonce),
                                r: Vec::from(decoded_call.r),
                                s: Vec::from(decoded_call.s),
                                v: decoded_call.v.to_u64(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_cancel_authorization_2s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::CancelAuthorization2::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::CancelAuthorization2::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdcCancelAuthorization2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                authorizer: decoded_call.authorizer,
                                nonce: Vec::from(decoded_call.nonce),
                                signature: decoded_call.signature,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_change_admins.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::ChangeAdmin::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::ChangeAdmin::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdcChangeAdminCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_admin: decoded_call.new_admin,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_configure_minters.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::ConfigureMinter::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::ConfigureMinter::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::usdc_contract::functions::ConfigureMinter::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::UsdcConfigureMinterCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                minter: decoded_call.minter,
                                minter_allowed_amount: decoded_call.minter_allowed_amount.to_string(),
                                output_param0: output_param0,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_decrease_allowances.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::DecreaseAllowance::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::DecreaseAllowance::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::usdc_contract::functions::DecreaseAllowance::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::UsdcDecreaseAllowanceCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                decrement: decoded_call.decrement.to_string(),
                                output_param0: output_param0,
                                spender: decoded_call.spender,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_increase_allowances.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::IncreaseAllowance::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::IncreaseAllowance::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::usdc_contract::functions::IncreaseAllowance::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::UsdcIncreaseAllowanceCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                increment: decoded_call.increment.to_string(),
                                output_param0: output_param0,
                                spender: decoded_call.spender,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_initializes.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::Initialize::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::Initialize::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdcInitializeCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_blacklister: decoded_call.new_blacklister,
                                new_master_minter: decoded_call.new_master_minter,
                                new_owner: decoded_call.new_owner,
                                new_pauser: decoded_call.new_pauser,
                                token_currency: decoded_call.token_currency,
                                token_decimals: decoded_call.token_decimals.to_u64(),
                                token_name: decoded_call.token_name,
                                token_symbol: decoded_call.token_symbol,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_initialize_v_2s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::InitializeV2::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::InitializeV2::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdcInitializeV2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_name: decoded_call.new_name,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_initialize_v2_1s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::InitializeV21::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::InitializeV21::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdcInitializeV21call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                lost_and_found: decoded_call.lost_and_found,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_initialize_v2_2s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::InitializeV22::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::InitializeV22::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdcInitializeV22call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                accounts_to_blacklist: decoded_call.accounts_to_blacklist.into_iter().map(|x| x).collect::<Vec<_>>(),
                                new_symbol: decoded_call.new_symbol,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_mints.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::Mint::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::Mint::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::usdc_contract::functions::Mint::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::UsdcMintCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                output_param0: output_param0,
                                u_amount: decoded_call.u_amount.to_string(),
                                u_to: decoded_call.u_to,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_pauses.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::Pause::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::Pause::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdcPauseCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_permit_1s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::Permit1::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::Permit1::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdcPermit1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                deadline: decoded_call.deadline.to_string(),
                                owner: decoded_call.owner,
                                signature: decoded_call.signature,
                                spender: decoded_call.spender,
                                value: decoded_call.value.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_permit_2s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::Permit2::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::Permit2::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdcPermit2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                deadline: decoded_call.deadline.to_string(),
                                owner: decoded_call.owner,
                                r: Vec::from(decoded_call.r),
                                s: Vec::from(decoded_call.s),
                                spender: decoded_call.spender,
                                v: decoded_call.v.to_u64(),
                                value: decoded_call.value.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_receive_with_authorization_1s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::ReceiveWithAuthorization1::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::ReceiveWithAuthorization1::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdcReceiveWithAuthorization1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                from: decoded_call.from,
                                nonce: Vec::from(decoded_call.nonce),
                                signature: decoded_call.signature,
                                to: decoded_call.to,
                                valid_after: decoded_call.valid_after.to_string(),
                                valid_before: decoded_call.valid_before.to_string(),
                                value: decoded_call.value.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_receive_with_authorization_2s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::ReceiveWithAuthorization2::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::ReceiveWithAuthorization2::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdcReceiveWithAuthorization2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                from: decoded_call.from,
                                nonce: Vec::from(decoded_call.nonce),
                                r: Vec::from(decoded_call.r),
                                s: Vec::from(decoded_call.s),
                                to: decoded_call.to,
                                v: decoded_call.v.to_u64(),
                                valid_after: decoded_call.valid_after.to_string(),
                                valid_before: decoded_call.valid_before.to_string(),
                                value: decoded_call.value.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_remove_minters.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::RemoveMinter::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::RemoveMinter::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::usdc_contract::functions::RemoveMinter::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::UsdcRemoveMinterCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                minter: decoded_call.minter,
                                output_param0: output_param0,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_rescue_erc_20s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::RescueErc20::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::RescueErc20::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdcRescueErc20call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                to: decoded_call.to,
                                token_contract: decoded_call.token_contract,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_transfers.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::Transfer::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::Transfer::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::usdc_contract::functions::Transfer::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::UsdcTransferCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                output_param0: output_param0,
                                to: decoded_call.to,
                                value: decoded_call.value.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_transfer_froms.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::TransferFrom::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::TransferFrom::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::usdc_contract::functions::TransferFrom::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::UsdcTransferFromCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                from: decoded_call.from,
                                output_param0: output_param0,
                                to: decoded_call.to,
                                value: decoded_call.value.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_transfer_ownerships.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::TransferOwnership::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::TransferOwnership::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdcTransferOwnershipCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_owner: decoded_call.new_owner,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_transfer_with_authorization_1s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::TransferWithAuthorization1::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::TransferWithAuthorization1::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdcTransferWithAuthorization1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                from: decoded_call.from,
                                nonce: Vec::from(decoded_call.nonce),
                                signature: decoded_call.signature,
                                to: decoded_call.to,
                                valid_after: decoded_call.valid_after.to_string(),
                                valid_before: decoded_call.valid_before.to_string(),
                                value: decoded_call.value.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_transfer_with_authorization_2s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::TransferWithAuthorization2::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::TransferWithAuthorization2::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdcTransferWithAuthorization2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                from: decoded_call.from,
                                nonce: Vec::from(decoded_call.nonce),
                                r: Vec::from(decoded_call.r),
                                s: Vec::from(decoded_call.s),
                                to: decoded_call.to,
                                v: decoded_call.v.to_u64(),
                                valid_after: decoded_call.valid_after.to_string(),
                                valid_before: decoded_call.valid_before.to_string(),
                                value: decoded_call.value.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_un_blacklists.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::UnBlacklist::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::UnBlacklist::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdcUnBlacklistCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_account: decoded_call.u_account,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_unpauses.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::Unpause::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::Unpause::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdcUnpauseCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_update_blacklisters.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::UpdateBlacklister::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::UpdateBlacklister::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdcUpdateBlacklisterCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_new_blacklister: decoded_call.u_new_blacklister,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_update_master_minters.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::UpdateMasterMinter::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::UpdateMasterMinter::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdcUpdateMasterMinterCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_new_master_minter: decoded_call.u_new_master_minter,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_update_pausers.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::UpdatePauser::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::UpdatePauser::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdcUpdatePauserCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_new_pauser: decoded_call.u_new_pauser,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_update_rescuers.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::UpdateRescuer::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::UpdateRescuer::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdcUpdateRescuerCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_rescuer: decoded_call.new_rescuer,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_upgrade_tos.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::UpgradeTo::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::UpgradeTo::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdcUpgradeToCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_implementation: decoded_call.new_implementation,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.usdc_call_upgrade_to_and_calls.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == USDC_TRACKED_CONTRACT && abi::usdc_contract::functions::UpgradeToAndCall::match_call(call))
                .filter_map(|call| {
                    match abi::usdc_contract::functions::UpgradeToAndCall::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::UsdcUpgradeToAndCallCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                data: decoded_call.data,
                                new_implementation: decoded_call.new_implementation,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
}

#[substreams::handlers::map]
fn map_events_calls(
    events: contract::Events,
    calls: contract::Calls,
) -> Result<contract::EventsCalls, substreams::errors::Error> {
    Ok(contract::EventsCalls {
        events: Some(events),
        calls: Some(calls),
    })
}
#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_usdc_events(&blk, &mut events);
    substreams::skip_empty_output();
    Ok(events)
}
#[substreams::handlers::map]
fn map_calls(blk: eth::Block) -> Result<contract::Calls, substreams::errors::Error> {
let mut calls = contract::Calls::default();
    map_usdc_calls(&blk, &mut calls);
    substreams::skip_empty_output();
    Ok(calls)
}

