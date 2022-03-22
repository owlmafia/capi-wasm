pub const SRC: &str = r#"
#pragma version 5
global GroupSize
int 10
==
bnz main_l6
gtxn 0 NumAppArgs
int 0
==
global GroupSize
int 2
==
&&
bnz main_l5
gtxna 0 ApplicationArgs 0
byte "claim"
==
bnz main_l4
err
main_l4:
global GroupSize
int 2
==
assert
gtxn 0 TypeEnum
int appl
==
assert
gtxn 0 ApplicationID
int TMPL_CENTRAL_APP_ID
==
assert
gtxn 0 OnCompletion
int NoOp
==
assert
gtxn 0 Sender
gtxn 1 AssetReceiver
==
assert
gtxn 1 TypeEnum
int axfer
==
assert
gtxn 1 XferAsset
int TMPL_FUNDS_ASSET_ID
==
assert
gtxn 1 AssetAmount
int 0
>
assert
gtxn 1 Fee
int 0
==
assert
gtxn 1 AssetCloseTo
global ZeroAddress
==
assert
gtxn 1 RekeyTo
global ZeroAddress
==
assert
int 1
return
main_l5:
gtxn 0 TypeEnum
int pay
==
assert
gtxn 0 Sender
addr TMPL_OWNER
==
assert
gtxn 0 Amount
int 0
==
assert
gtxn 1 TypeEnum
int axfer
==
assert
gtxn 1 AssetAmount
int 0
>
assert
gtxn 1 XferAsset
int TMPL_FUNDS_ASSET_ID
==
assert
gtxn 1 AssetReceiver
addr TMPL_OWNER
==
assert
gtxn 1 Fee
int 0
==
assert
gtxn 1 AssetCloseTo
global ZeroAddress
==
assert
gtxn 1 RekeyTo
global ZeroAddress
==
assert
int 1
return
main_l6:
gtxn 0 TypeEnum
int appl
==
assert
gtxn 0 OnCompletion
int NoOp
==
assert
gtxn 0 ApplicationID
int TMPL_CENTRAL_APP_ID
==
assert
gtxn 0 NumAppArgs
int 13
==
assert
gtxn 1 TypeEnum
int pay
==
assert
gtxn 1 Receiver
gtxna 0 ApplicationArgs 0
==
assert
gtxn 2 TypeEnum
int pay
==
assert
gtxn 2 Receiver
gtxna 0 ApplicationArgs 1
==
assert
gtxn 3 TypeEnum
int pay
==
assert
gtxn 4 TypeEnum
int pay
==
assert
gtxn 5 TypeEnum
int axfer
==
assert
gtxn 5 AssetAmount
int 0
==
assert
gtxn 6 TypeEnum
int axfer
==
assert
gtxn 6 AssetAmount
int 0
==
assert
gtxn 7 TypeEnum
int axfer
==
assert
gtxn 7 AssetAmount
int 0
==
assert
gtxn 7 Fee
int 0
==
assert
gtxn 7 AssetCloseTo
global ZeroAddress
==
assert
gtxn 7 RekeyTo
global ZeroAddress
==
assert
gtxn 8 TypeEnum
int axfer
==
assert
gtxn 8 AssetAmount
int 0
==
assert
gtxn 9 TypeEnum
int axfer
==
assert
gtxn 9 XferAsset
gtxna 0 ApplicationArgs 4
btoi
==
assert
int 1
return
"#;
