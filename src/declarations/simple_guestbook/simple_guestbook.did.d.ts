import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface GuestBookEntry {
  'status' : UserStatus,
  'text' : string,
  'author' : Principal,
}
export interface UserDetails { 'status' : UserStatus, 'principal' : Principal }
export type UserStatus = { 'Premium' : null } |
  { 'Basic' : null } |
  { 'Ultimate' : null };
export interface _SERVICE {
  'add' : ActorMethod<[string], boolean>,
  'getAll' : ActorMethod<[], Array<GuestBookEntry>>,
  'getUserDetails' : ActorMethod<[], UserDetails>,
  'greet' : ActorMethod<[string], string>,
  'upgradePremium' : ActorMethod<[], string>,
  'upgradeUltimate' : ActorMethod<[], boolean>,
  'verifyPremium' : ActorMethod<[], string>,
}
