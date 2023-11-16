extern crate wasm_bindgen;
extern crate serde_wasm_bindgen;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct FinancialMetrics {
    gross_margin: f64,
    sg_and_a_margin: f64,
    r_and_d_margin: f64,
    depreciation_margin: f64,
    interest_margin: f64,
    tax_margin: f64,
    net_income_margin: f64,
    eps_growth: f64,
    cash_to_debt: bool,
    adjusted_debt_to_equity: f64,
    retained_earnings_growth: f64,
    treasury_stock_exists: bool,
    capex_margin: f64,
}



fn calculate_metrics(data: &serde_json::Value) -> FinancialMetrics {
    let gross_profit = data["gross_profit"].as_str().unwrap().parse::<f64>().unwrap();
    let net_sales = data["net_sales"].as_str().unwrap().parse::<f64>().unwrap();
    let sg_and_a = data["sg_and_a"].as_str().unwrap().parse::<f64>().unwrap();
    let rd = data["rd"].as_str().unwrap().parse::<f64>().unwrap();
    let depreciation = data["depreciation"].as_str().unwrap().parse::<f64>().unwrap();
    let interest = data["interest"].as_str().unwrap().parse::<f64>().unwrap();
    let operating_income = data["operating_income"].as_str().unwrap().parse::<f64>().unwrap();
    let taxes = data["taxes"].as_str().unwrap().parse::<f64>().unwrap();
    let pre_tax_income = data["pre_tax_income"].as_str().unwrap().parse::<f64>().unwrap();
    let net_income = data["net_income"].as_str().unwrap().parse::<f64>().unwrap();
    let eps_year_2 = data["eps_year_2"].as_str().unwrap().parse::<f64>().unwrap();
    let eps_year_1 = data["eps_year_1"].as_str().unwrap().parse::<f64>().unwrap();
    let cash = data["cash"].as_str().unwrap().parse::<f64>().unwrap();
    let debt = data["debt"].as_str().unwrap().parse::<f64>().unwrap();
    let total_liabilities = data["total_liabilities"].as_str().unwrap().parse::<f64>().unwrap();
    let shareholder_equity = data["shareholder_equity"].as_str().unwrap().parse::<f64>().unwrap();
    let treasury_stocks = data["treasury_stocks"].as_str().unwrap().parse::<f64>().unwrap();
    let retained_earnings_year_2 = data["retained_earnings_year_2"].as_str().unwrap().parse::<f64>().unwrap();
    let retained_earnings_year_1 = data["retained_earnings_year_1"].as_str().unwrap().parse::<f64>().unwrap();
    let increase_in_fixed_assets = data["increase_in_fixed_assets"]
    .as_str()
    .unwrap()
    .parse::<f64>()
    .unwrap();
    let depreciation_and_amortization_ope_cf = data["depreciation_and_amortization_ope_cf"]
    .as_str()
    .unwrap()
    .parse::<f64>()
    .unwrap();

    FinancialMetrics {
        gross_margin: gross_profit / net_sales,
        sg_and_a_margin: sg_and_a / gross_profit,
        r_and_d_margin: rd / gross_profit,
        depreciation_margin: depreciation / gross_profit,
        interest_margin: interest / operating_income,
        tax_margin: taxes / pre_tax_income,
        net_income_margin: net_income / net_sales,
        eps_growth: (eps_year_2) / eps_year_1,
        cash_to_debt: cash > debt,
        adjusted_debt_to_equity: (total_liabilities) / (shareholder_equity + treasury_stocks),
        retained_earnings_growth: retained_earnings_year_2 / retained_earnings_year_1,
        treasury_stock_exists: treasury_stocks > 0.0,
        capex_margin: (increase_in_fixed_assets + depreciation_and_amortization_ope_cf) / net_income,
    }
}

#[wasm_bindgen]
pub fn calculate_financial_metrics(data: &JsValue) -> JsValue {
    // `serde_wasm_bindgen::from_value` を使ってデータをデシリアライズする
    let input_data: serde_json::Value = serde_wasm_bindgen::from_value(data.clone()).unwrap();
    
    // 計算する
    let metrics = calculate_metrics(&input_data);
    
    // `serde_wasm_bindgen::to_value` を使って結果をシリアライズする
    serde_wasm_bindgen::to_value(&metrics).unwrap()
}
