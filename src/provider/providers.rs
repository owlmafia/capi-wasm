use super::{
    add_roadmap_item_provider::AddRoadmapItemProvider,
    app_updates_provider::AppUpdatesProvider,
    balance_provider::BalanceProvider,
    buy_shares::BuySharesProvider,
    claim_provider::ClaimProvider,
    create_assets_provider::CreateAssetsProvider,
    create_dao_provider::CreateDaoProvider,
    dao_user_view_provider::DaoUserViewProvider,
    def::{
        add_roadmap_item_provider_def::AddRoadmapItemProviderDef,
        app_updates_provider_def::AppUpdatesProviderDef, balance_provider_def::BalanceProviderDef,
        buy_shares_provider_def::BuySharesProviderDef, claim_provider_def::ClaimProviderDef,
        create_assets_provider_def::CreateAssetsProviderDef,
        create_dao_provider_def::CreateDaoProviderDef,
        dao_user_view_provider_def::DaoUserViewProviderDef, drain_provider_def::DrainProviderDef,
        funds_activity_provider_def::FundsActivityProviderDef,
        holders_count_provider_def::HoldersCountProviderDef,
        income_vs_spending_provider_def::IncomeVsSpendingProviderDef,
        investment_provider_def::InvestmentProviderDef, lock_provider_def::LockProviderDef,
        my_daos_provider_def::MyDaosProviderDef, my_shares_provider_def::MySharesProviderDef,
        optin_to_app_provider_def::OptinToAppProviderDef, pay_dao_provider_def::PayDaoProviderDef,
        roadmap_provider_def::RoadmapProviderDef,
        shares_count_provider_def::SharesCountProviderDef,
        shares_distribution_provider::SharesDistributionProviderDef,
        unlock_provider_def::UnlockProviderDef, update_app_provider_def::UpdateAppProviderDef,
        update_data_provider_def::UpdateDataProviderDef, view_dao_provider_def::ViewDaoProviderDef,
        withdraw_provider_def::WithdrawProviderDef,
        withdrawal_history_provider_def::WithdrawalHistoryProviderDef,
    },
    drain_provider::DrainProvider,
    funds_activity_provider::FundsActivityProvider,
    holders_count_provider::HoldersCountProvider,
    income_vs_spending_provider::IncomeVsSpendingProvider,
    investment_provider::InvestmentProvider,
    lock_provider::LockProvider,
    my_daos_provider::MyDaosProvider,
    my_shares_provider::MySharesProvider,
    optin_to_app_provider::OptinToAppProvider,
    pay_dao_provider::PayDaoProvider,
    roadmap_provider::RoadmapProvider,
    shares_count_provider::SharesCountProvider,
    shares_distribution_provider::SharesDistributionProvider,
    unlock_provider::UnlockProvider,
    update_app_provider::UpdateAppProvider,
    update_data_provider::UpdateDataProvider,
    view_dao_provider::ViewDaoProvider,
    withdraw_provider::WithdrawProvider,
    withdrawal_history_provider::WithdrawalHistoryProvider,
};

pub struct Providers<'a> {
    pub funds_activity: &'a dyn FundsActivityProvider,
    pub balance: &'a dyn BalanceProvider,
    pub buy_shares: &'a dyn BuySharesProvider,
    pub shares_count: &'a dyn SharesCountProvider,
    pub dao_user_view: &'a dyn DaoUserViewProvider,
    pub app_optin: &'a dyn OptinToAppProvider,
    pub claim: &'a dyn ClaimProvider,
    pub investment: &'a dyn InvestmentProvider,
    pub lock: &'a dyn LockProvider,
    pub pay_dao: &'a dyn PayDaoProvider,
    pub holders_count: &'a dyn HoldersCountProvider,
    pub income_vs_spending: &'a dyn IncomeVsSpendingProvider,
    pub my_daos: &'a dyn MyDaosProvider,
    pub my_shares: &'a dyn MySharesProvider,
    pub shares_distribution: &'a dyn SharesDistributionProvider,
    pub add_roadmap_item: &'a dyn AddRoadmapItemProvider,
    pub roadmap: &'a dyn RoadmapProvider,
    pub unlock: &'a dyn UnlockProvider,
    pub app_updates: &'a dyn AppUpdatesProvider,
    pub update_app: &'a dyn UpdateAppProvider,
    pub update_data: &'a dyn UpdateDataProvider,
    pub view_dao: &'a dyn ViewDaoProvider,
    pub drain: &'a dyn DrainProvider,
    pub withdraw: &'a dyn WithdrawProvider,
    pub withdrawals_history: &'a dyn WithdrawalHistoryProvider, // remove ? seems not to be used anymore (route/comp in react, but not used)
    pub create_dao: &'a dyn CreateDaoProvider,
    pub create_assets: &'a dyn CreateAssetsProvider,
}

pub fn providers<'a>() -> Providers<'a> {
    Providers {
        funds_activity: &FundsActivityProviderDef {},
        balance: &BalanceProviderDef {},
        buy_shares: &BuySharesProviderDef {},
        shares_count: &SharesCountProviderDef {},
        dao_user_view: &DaoUserViewProviderDef {},
        app_optin: &OptinToAppProviderDef {},
        claim: &ClaimProviderDef {},
        investment: &InvestmentProviderDef {},
        lock: &LockProviderDef {},
        pay_dao: &PayDaoProviderDef {},
        holders_count: &HoldersCountProviderDef {},
        income_vs_spending: &IncomeVsSpendingProviderDef {},
        my_daos: &MyDaosProviderDef {},
        my_shares: &MySharesProviderDef {},
        shares_distribution: &SharesDistributionProviderDef {},
        add_roadmap_item: &AddRoadmapItemProviderDef {},
        roadmap: &RoadmapProviderDef {},
        unlock: &UnlockProviderDef {},
        app_updates: &AppUpdatesProviderDef {},
        update_app: &UpdateAppProviderDef {},
        update_data: &UpdateDataProviderDef {},
        view_dao: &ViewDaoProviderDef {},
        drain: &DrainProviderDef {},
        withdraw: &WithdrawProviderDef {},
        withdrawals_history: &WithdrawalHistoryProviderDef {},
        create_dao: &CreateDaoProviderDef {},
        create_assets: &CreateAssetsProviderDef {},
    }
}
