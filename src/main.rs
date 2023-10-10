use std::cmp;

fn main() {
    println!("How many games {}", howManyGames(20, 3, 6, 70));
}

fn howManyGames(p: i32, d: i32, m: i32, s: i32) -> i32 {
    // Return the number of games you can buy
    let mut games = 0;
    let mut money = s;
    let mut discount = d;
    let mut price = p;
    let min_price = m;
    while money >= price {
        games += 1;
        money = money - price;
        price = if price > min_price {cmp::max(min_price, price - discount)} else { min_price }
    }
    games
}