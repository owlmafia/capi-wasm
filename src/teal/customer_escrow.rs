pub const SRC: &str = r#"
#pragma version 5
global GroupSize
int 10
==
bnz main_l6
global GroupSize
int 4
==
bnz main_l3
err
main_l3:
global GroupSize
int 4
==
bnz main_l5
err
main_l5:
gtxn 0 TypeEnum
int appl
==
assert
gtxn 0 OnCompletion
int NoOp
==
assert
gtxn 0 Sender
gtxn 1 Sender
==
assert
gtxn 1 TypeEnum
int appl
==
assert
gtxn 1 OnCompletion
int NoOp
==
assert
gtxn 2 TypeEnum
int axfer
==
assert
gtxn 2 AssetAmount
int 0
>
assert
gtxn 2 AssetReceiver
addr TMPL_CENTRAL_ESCROW_ADDRESS
==
assert
gtxn 2 Fee
int 0
==
assert
gtxn 2 AssetCloseTo
global ZeroAddress
==
assert
gtxn 2 RekeyTo
global ZeroAddress
==
assert
gtxn 3 TypeEnum
int axfer
==
assert
gtxn 3 AssetReceiver
addr TMPL_CAPI_ESCROW_ADDRESS
==
assert
gtxn 3 Fee
int 0
==
assert
gtxn 3 AssetCloseTo
global ZeroAddress
==
assert
gtxn 3 RekeyTo
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
int 4
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
gtxn 8 TypeEnum
int axfer
==
assert
gtxn 8 AssetAmount
int 0
==
assert
gtxn 8 Fee
int 0
==
assert
gtxn 8 AssetCloseTo
global ZeroAddress
==
assert
gtxn 8 RekeyTo
global ZeroAddress
==
assert
gtxn 9 TypeEnum
int axfer
==
assert
gtxn 9 XferAsset
gtxna 0 ApplicationArgs 2
btoi
==
assert
int 1
return
"#;