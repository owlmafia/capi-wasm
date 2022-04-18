pub const SRC: &str = r#"
#pragma version 5
global GroupSize
int 7
==
bnz main_l22
gtxn 0 ApplicationID
int 0
==
bnz main_l21
gtxn 0 OnCompletion
int 4
==
bnz main_l20
gtxna 0 ApplicationArgs 0
byte "optin"
==
bnz main_l19
gtxna 0 ApplicationArgs 0
byte "unlock"
==
bnz main_l18
gtxna 0 ApplicationArgs 0
byte "claim"
==
bnz main_l17
gtxna 0 ApplicationArgs 0
byte "lock"
==
bnz main_l16
gtxna 0 ApplicationArgs 0
byte "drain"
==
bnz main_l15
gtxna 0 ApplicationArgs 0
byte "invest"
==
bnz main_l14
gtxna 0 ApplicationArgs 0
byte "update_data"
==
bnz main_l13
gtxna 0 ApplicationArgs 0
byte "withdraw"
==
bnz main_l12
err
main_l12:
gtxn 0 Sender
addr TMPL_OWNER
==
assert
itxn_begin
int axfer
itxn_field TypeEnum
gtxna 0 ApplicationArgs 1
btoi
itxn_field AssetAmount
txn Sender
itxn_field AssetReceiver
byte "FundsAssetId"
app_global_get
itxn_field XferAsset
itxn_submit
int 1
return
main_l13:
gtxn 0 TypeEnum
int appl
==
assert
gtxn 0 ApplicationID
global CurrentApplicationID
==
assert
gtxn 0 OnCompletion
int NoOp
==
assert
gtxn 0 NumAppArgs
int 10
==
assert
byte "CustomerEscrowAddress"
gtxna 0 ApplicationArgs 1
app_global_put
byte "InvestingEscrowAddress"
gtxna 0 ApplicationArgs 2
app_global_put
byte "DaoName"
gtxna 0 ApplicationArgs 3
app_global_put
byte "DaoDesc"
gtxna 0 ApplicationArgs 4
app_global_put
byte "SharePrice"
gtxna 0 ApplicationArgs 5
btoi
app_global_put
byte "LogoUrl"
gtxna 0 ApplicationArgs 6
app_global_put
byte "SocialMediaUrl"
gtxna 0 ApplicationArgs 7
app_global_put
byte "Owner"
gtxna 0 ApplicationArgs 8
app_global_put
byte "Versions"
gtxna 0 ApplicationArgs 9
app_global_put
int 1
return
main_l14:
global GroupSize
int 4
==
assert
gtxn 0 TypeEnum
int appl
==
assert
gtxn 0 ApplicationID
global CurrentApplicationID
==
assert
gtxn 0 OnCompletion
int NoOp
==
assert
gtxn 0 NumAppArgs
int 2
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
byte "SharesAssetId"
app_global_get
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
gtxn 2 XferAsset
byte "FundsAssetId"
app_global_get
==
assert
gtxn 2 AssetReceiver
global CurrentApplicationAddress
==
assert
gtxn 3 TypeEnum
int axfer
==
assert
gtxn 3 XferAsset
byte "SharesAssetId"
app_global_get
==
assert
gtxn 3 AssetAmount
int 0
==
assert
gtxn 3 AssetReceiver
gtxn 3 Sender
==
assert
gtxn 0 Sender
gtxn 2 Sender
==
assert
gtxn 2 Sender
gtxn 3 Sender
==
assert
gtxn 2 AssetAmount
gtxn 1 AssetAmount
int TMPL_SHARE_PRICE
*
==
assert
gtxn 1 AssetAmount
int 0
>
assert
gtxn 0 Sender
byte "Shares"
gtxn 0 Sender
byte "Shares"
app_local_get
gtxn 1 AssetAmount
+
app_local_put
gtxn 0 Sender
byte "ClaimedTotal"
gtxn 0 Sender
byte "Shares"
app_local_get
int TMPL_PRECISION__
*
int TMPL_INVESTORS_SHARE
*
int TMPL_SHARE_SUPPLY
/
byte "CentralReceivedTotal"
app_global_get
*
int TMPL_PRECISION_SQUARE
/
gtxn 0 Sender
byte "ClaimedTotal"
app_local_get
-
app_local_put
gtxn 0 Sender
byte "ClaimedInit"
gtxn 0 Sender
byte "ClaimedTotal"
app_local_get
app_local_put
gtxn 0 Sender
byte "Dao"
gtxna 0 ApplicationArgs 1
app_local_put
int 1
return
main_l15:
global GroupSize
int 4
==
assert
gtxn 0 TypeEnum
int appl
==
assert
gtxn 0 ApplicationID
global CurrentApplicationID
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
gtxn 1 ApplicationID
int TMPL_CAPI_APP_ID
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
gtxn 2 Sender
byte "CustomerEscrowAddress"
app_global_get
==
assert
gtxn 2 XferAsset
byte "FundsAssetId"
app_global_get
==
assert
gtxn 2 AssetReceiver
global CurrentApplicationAddress
==
assert
gtxn 3 TypeEnum
int axfer
==
assert
gtxn 3 XferAsset
byte "FundsAssetId"
app_global_get
==
assert
gtxn 3 AssetReceiver
addr TMPL_CAPI_ESCROW_ADDRESS
==
assert
gtxn 2 Sender
gtxn 2 XferAsset
asset_holding_get AssetBalance
store 1
store 0
gtxn 3 AssetAmount
load 0
int TMPL_PRECISION__
*
int TMPL_CAPI_SHARE
*
int TMPL_PRECISION_SQUARE
/
==
assert
byte "CentralReceivedTotal"
byte "CentralReceivedTotal"
app_global_get
gtxn 2 AssetAmount
+
app_global_put
int 1
return
main_l16:
global GroupSize
int 2
==
assert
gtxn 0 TypeEnum
int appl
==
assert
gtxn 0 ApplicationID
global CurrentApplicationID
==
assert
gtxn 0 OnCompletion
int NoOp
==
assert
gtxn 0 NumAppArgs
int 2
==
assert
gtxn 0 Sender
gtxn 1 Sender
==
assert
gtxn 1 TypeEnum
int axfer
==
assert
gtxn 1 XferAsset
byte "SharesAssetId"
app_global_get
==
assert
gtxn 1 AssetReceiver
global CurrentApplicationAddress
==
assert
gtxn 1 AssetAmount
int 0
>
assert
gtxn 1 AssetAmount
int 0
>
assert
gtxn 0 Sender
byte "Shares"
gtxn 0 Sender
byte "Shares"
app_local_get
gtxn 1 AssetAmount
+
app_local_put
gtxn 0 Sender
byte "ClaimedTotal"
gtxn 0 Sender
byte "Shares"
app_local_get
int TMPL_PRECISION__
*
int TMPL_INVESTORS_SHARE
*
int TMPL_SHARE_SUPPLY
/
byte "CentralReceivedTotal"
app_global_get
*
int TMPL_PRECISION_SQUARE
/
gtxn 0 Sender
byte "ClaimedTotal"
app_local_get
-
app_local_put
gtxn 0 Sender
byte "ClaimedInit"
gtxn 0 Sender
byte "ClaimedTotal"
app_local_get
app_local_put
gtxn 0 Sender
byte "Dao"
gtxna 0 ApplicationArgs 1
app_local_put
int 1
return
main_l17:
global GroupSize
int 1
==
assert
gtxn 0 TypeEnum
int appl
==
assert
gtxn 0 ApplicationID
global CurrentApplicationID
==
assert
gtxn 0 OnCompletion
int NoOp
==
assert
itxn_begin
int axfer
itxn_field TypeEnum
gtxn 0 Sender
byte "Shares"
app_local_get
int TMPL_PRECISION__
*
int TMPL_INVESTORS_SHARE
*
int TMPL_SHARE_SUPPLY
/
byte "CentralReceivedTotal"
app_global_get
*
int TMPL_PRECISION_SQUARE
/
gtxn 0 Sender
byte "ClaimedTotal"
app_local_get
-
itxn_field AssetAmount
txn Sender
itxn_field AssetReceiver
byte "FundsAssetId"
app_global_get
itxn_field XferAsset
itxn_submit
gtxn 0 Sender
byte "ClaimedTotal"
gtxn 0 Sender
byte "ClaimedTotal"
app_local_get
gtxn 0 Sender
byte "Shares"
app_local_get
int TMPL_PRECISION__
*
int TMPL_INVESTORS_SHARE
*
int TMPL_SHARE_SUPPLY
/
byte "CentralReceivedTotal"
app_global_get
*
int TMPL_PRECISION_SQUARE
/
gtxn 0 Sender
byte "ClaimedTotal"
app_local_get
-
+
app_local_put
int 1
return
main_l18:
global GroupSize
int 1
==
assert
gtxn 0 TypeEnum
int appl
==
assert
gtxn 0 OnCompletion
int CloseOut
==
assert
gtxn 0 ApplicationID
global CurrentApplicationID
==
assert
itxn_begin
int axfer
itxn_field TypeEnum
gtxn 0 Sender
itxn_field AssetReceiver
gtxn 0 Sender
byte "Shares"
app_local_get
itxn_field AssetAmount
byte "SharesAssetId"
app_global_get
itxn_field XferAsset
int 0
itxn_field Fee
itxn_submit
int 1
return
main_l19:
gtxn 0 TypeEnum
int appl
==
assert
gtxn 0 ApplicationID
global CurrentApplicationID
==
assert
gtxn 0 OnCompletion
int OptIn
==
assert
int 1
return
main_l20:
global GroupSize
int 1
==
assert
gtxn 0 TypeEnum
int appl
==
assert
gtxn 0 Sender
addr TMPL_OWNER
==
assert
int 1
return
main_l21:
gtxn 0 TypeEnum
int appl
==
assert
int 1
return
main_l22:
gtxn 0 TypeEnum
int pay
==
assert
gtxn 0 Receiver
global CurrentApplicationAddress
==
assert
gtxn 1 TypeEnum
int appl
==
assert
gtxn 1 ApplicationID
global CurrentApplicationID
==
assert
gtxn 1 OnCompletion
int NoOp
==
assert
gtxn 1 NumAppArgs
int 12
==
assert
gtxn 2 TypeEnum
int pay
==
assert
gtxn 2 Receiver
gtxna 1 ApplicationArgs 0
==
assert
gtxn 3 TypeEnum
int pay
==
assert
gtxn 4 TypeEnum
int axfer
==
assert
gtxn 4 AssetAmount
int 0
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
gtxn 6 XferAsset
gtxna 1 ApplicationArgs 2
btoi
==
assert
byte "CentralReceivedTotal"
int 0
app_global_put
byte "CustomerEscrowAddress"
gtxna 1 ApplicationArgs 0
app_global_put
byte "InvestingEscrowAddress"
gtxna 1 ApplicationArgs 1
app_global_put
byte "SharesAssetId"
gtxna 1 ApplicationArgs 2
btoi
app_global_put
byte "FundsAssetId"
gtxna 1 ApplicationArgs 3
btoi
app_global_put
byte "DaoName"
gtxna 1 ApplicationArgs 4
app_global_put
byte "DaoDesc"
gtxna 1 ApplicationArgs 5
app_global_put
byte "SharePrice"
gtxna 1 ApplicationArgs 6
btoi
app_global_put
byte "InvestorsPart"
gtxna 1 ApplicationArgs 7
btoi
app_global_put
byte "LogoUrl"
gtxna 1 ApplicationArgs 8
app_global_put
byte "SocialMediaUrl"
gtxna 1 ApplicationArgs 9
app_global_put
byte "Owner"
gtxna 1 ApplicationArgs 10
app_global_put
byte "Versions"
gtxna 1 ApplicationArgs 11
app_global_put
itxn_begin
int axfer
itxn_field TypeEnum
global CurrentApplicationAddress
itxn_field AssetReceiver
int 0
itxn_field AssetAmount
gtxna 1 Assets 0
itxn_field XferAsset
int 0
itxn_field Fee
itxn_submit
itxn_begin
int axfer
itxn_field TypeEnum
global CurrentApplicationAddress
itxn_field AssetReceiver
int 0
itxn_field AssetAmount
gtxna 1 Assets 1
itxn_field XferAsset
int 0
itxn_field Fee
itxn_submit
int 1
return
"#;