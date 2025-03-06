const CASHES: [u32; 8] = [1, 2, 5, 10, 20, 30, 50, 100];
pub fn dp_rec_mc(amount: u32) -> u32 {
    let mut num: u32 = 0;
    let mut amount = amount;
    while amount > 0 {
        for cash in CASHES.iter().rev() {
            if amount >= *cash {
                amount -= cash;
                num += 1;
                break;
            }
        }
    }
    num
}
