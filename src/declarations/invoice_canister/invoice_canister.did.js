export const idlFactory = ({ IDL }) => {
  const RequestStatus = IDL.Variant({
    'Empty' : IDL.Null,
    'Paid' : IDL.Null,
    'Pending' : IDL.Null,
  });
  return IDL.Service({
    'checkPayment' : IDL.Func([IDL.Text], [IDL.Bool], []),
    'greet' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
    'upgrade_premium' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(IDL.Vec(IDL.Nat8))],
        [],
      ),
    'upgrade_ultimate' : IDL.Func([], [IDL.Opt(IDL.Vec(IDL.Nat8))], []),
    'verifyPayment' : IDL.Func([IDL.Text], [IDL.Opt(RequestStatus)], []),
  });
};
export const init = ({ IDL }) => { return []; };
