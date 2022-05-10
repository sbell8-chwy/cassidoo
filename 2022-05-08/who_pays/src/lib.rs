use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PaymentPair {
    name: String,
    paid: u64,
}

pub fn who_owes(receipts: &Vec<PaymentPair>) -> String {
    let mut total = 0u64;
    let mut sorted_pay_totals: Vec<PaymentPair> = receipts.iter()
        .fold(HashMap::new(), |mut acc, receipt| {
            *acc.entry(receipt.name.clone())
                .or_insert(0) += receipt.paid;
            total += receipt.paid;
            acc
        }).into_iter()
        .map(|(k, v)| PaymentPair { name: k, paid: v })
        .collect();
    sorted_pay_totals.sort_by(|a, b| b.paid.cmp(&a.paid));

    let amount_per = total / (sorted_pay_totals.len() as u64);
    let mut recieve = 0usize;
    let mut send = sorted_pay_totals.len() - 1;
    let mut payment_message = String::new();
    while recieve < send {
        let excess = sorted_pay_totals[recieve].paid - amount_per;
        let shortfall = amount_per - sorted_pay_totals[send].paid;
        if excess == shortfall {
            payment_message.push_str(
                &format!("{} owes {} ${},",
                    sorted_pay_totals[send].name,
                    sorted_pay_totals[recieve].name,
                    excess));
            send -= 1;
            recieve += 1;
        } else if excess > shortfall {
            payment_message.push_str(
                &format!("{} owes {} ${},",
                    sorted_pay_totals[send].name,
                    sorted_pay_totals[recieve].name,
                    shortfall));
            sorted_pay_totals[recieve].paid = sorted_pay_totals[recieve].paid - shortfall;
            send -= 1;
        } else { // shortfall > excess
            payment_message.push_str(
                &format!("{} owes {} ${},",
                    sorted_pay_totals[send].name,
                    sorted_pay_totals[recieve].name,
                    shortfall));
            sorted_pay_totals[send].paid = sorted_pay_totals[send].paid + excess;
            recieve += 1;
        }
    }
    payment_message.pop();
    payment_message
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut receipts = vec![];
        receipts.push(PaymentPair{ name: "Ximena".to_string(), paid: 45});
        receipts.push(PaymentPair{ name: "Clara".to_string(), paid: 130});
        receipts.push(PaymentPair{ name: "Ximena".to_string(), paid: 100});
        receipts.push(PaymentPair{ name: "Cassidy".to_string(), paid: 140});
        receipts.push(PaymentPair{ name: "Cassidy".to_string(), paid: 76});
        receipts.push(PaymentPair{ name: "Clara".to_string(), paid: 29});
        receipts.push(PaymentPair{ name: "Ximena".to_string(), paid: 20});
        assert_eq!(who_owes(&receipts), "Clara owes Cassidy $21,Ximena owes Cassidy $15");
    }
}
