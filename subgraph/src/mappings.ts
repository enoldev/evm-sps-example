import { Protobuf } from "as-proto/assembly";
import { EventsCalls as protoEventsCalls } from "./pb/contract/v1/EventsCalls";
import { Events } from "./pb/contract/v1/Events";
import { Calls } from "./pb/contract/v1/Calls";
import { TransferEvent, ApproveCall } from "../generated/schema";
import { BigInt, log, crypto, Bytes } from "@graphprotocol/graph-ts";

export function handleTriggers(bytes: Uint8Array): void {
  const input = Protobuf.decode<protoEventsCalls>(bytes, protoEventsCalls.decode);
  let keccak256 = crypto.keccak256(Bytes.fromUint8Array(bytes)).toHexString();

  if (input.events !== null) {
    handleEvents(keccak256, input.events!!);
  }

  if (input.calls !== null) {
    handleCalls(keccak256, input.calls!!);
  }
}

function handleCalls(keccak256: String, calls: Calls): void {
  for (let i = 0; i < calls.usdcCallApproves.length; i++) {
    let approval = calls.usdcCallApproves[i];

    let entityId = `${keccak256}-call-${i}`;
    let entity = new ApproveCall(entityId);
    entity.blockNumber = approval.callBlockNumber as i32;
    entity.timestamp = approval.callBlockTime!!.nanos;
    entity.spender = approval.spender.toString();
    entity.value = approval.value;

    entity.save();
  }
}

function handleEvents(keccak256: String, events: Events): void {
  for (let i = 0; i < events.usdcTransfers.length; i++) {
    let transfer = events.usdcTransfers[i];

    let entityId = `${keccak256}-transfer-${i}`;
    let entity = new TransferEvent(entityId);
    entity.blockNumber = transfer.evtBlockNumber as i32;
    entity.timestamp = transfer.evtBlockTime!!.nanos;
    entity.from = transfer.from.toString();
    entity.to = transfer.to.toString();
    entity.value = transfer.value;

    entity.save();
  }
}
