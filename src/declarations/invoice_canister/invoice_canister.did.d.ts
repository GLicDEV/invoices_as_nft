import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type RequestStatus = { 'Empty' : null } |
  { 'Paid' : null } |
  { 'Pending' : null };
export interface _SERVICE {
  'checkPayment' : ActorMethod<[string], boolean>,
  'greet' : ActorMethod<[string], string>,
  'upgrade_premium' : ActorMethod<[Principal], [] | [Array<number>]>,
  'upgrade_ultimate' : ActorMethod<[], [] | [Array<number>]>,
  'verifyPayment' : ActorMethod<[string], [] | [RequestStatus]>,
}
