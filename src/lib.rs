/// एंट्री और स्टॉप-लॉस के आधार पर सुरक्षित शेयर मात्रा (Quantity) की गणना करता है।
pub fn calculate_position_size(
    capital: f64,
    risk_percent: f64,
    entry_price: f64,
    stop_loss: f64,
) -> Result<u64, &'static str> {
    if capital <= 0.0 || risk_percent <= 0.0 || risk_percent > 5.0 {
        return Err("अमान्य पूंजी या रिस्क प्रतिशत (अधिकतम 5% अनुमत है)");
    }
    if entry_price <= stop_loss {
        return Err("लॉन्ग ट्रेड के लिए Stop-Loss, Entry Price से कम होना चाहिए");
    }

    let max_risk_amount = capital * (risk_percent / 100.0);
    let risk_per_share = entry_price - stop_loss;
    let quantity = (max_risk_amount / risk_per_share).floor() as u64;

    Ok(quantity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_nifty_stock_position_size() {
        // पूंजी: ₹1,00,000 | रिस्क: 1% (₹1,000) | एंट्री: ₹500 | SL: ₹480 (रिस्क/शेयर: ₹20)
        // अपेक्षित क्वांटिटी: 1000 / 20 = 50 शेयर
        let qty = calculate_position_size(100_000.0, 1.0, 500.0, 480.0).unwrap();
        assert_eq!(qty, 50);
    }

    #[test]
    fn test_reject_invalid_stop_loss() {
        // अगर गलती से SL, Entry से ऊपर डाल दिया जाए तो एरर आना चाहिए
        let result = calculate_position_size(100_000.0, 1.0, 500.0, 520.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_reject_excessive_risk() {
        // 5% से ज्यादा रिस्क पर सिस्टम को ट्रेड रिजेक्ट करना चाहिए
        let result = calculate_position_size(100_000.0, 10.0, 500.0, 480.0);
        assert!(result.is_err());
    }
}
