export const idlFactory = ({ IDL }) => {
  const UserStatus = IDL.Variant({
    'Premium' : IDL.Null,
    'Basic' : IDL.Null,
    'Ultimate' : IDL.Null,
  });
  const GuestBookEntry = IDL.Record({
    'status' : UserStatus,
    'text' : IDL.Text,
    'author' : IDL.Principal,
  });
  const UserDetails = IDL.Record({
    'status' : UserStatus,
    'principal' : IDL.Principal,
  });
  return IDL.Service({
    'add' : IDL.Func([IDL.Text], [IDL.Bool], []),
    'getAll' : IDL.Func([], [IDL.Vec(GuestBookEntry)], ['query']),
    'getUserDetails' : IDL.Func([], [UserDetails], ['query']),
    'greet' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
    'upgradePremium' : IDL.Func([], [IDL.Text], []),
    'upgradeUltimate' : IDL.Func([], [IDL.Bool], []),
    'verifyPremium' : IDL.Func([], [IDL.Text], []),
  });
};
export const init = ({ IDL }) => { return []; };
