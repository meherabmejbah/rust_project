use std::io;                // ইউজারের কাছ থেকে ইনপুট নেওয়ার জন্য
use rand::Rng;              // random সংখ্যা জেনারেট করার জন্য
use std::cmp::Ordering;     // তুলনা করার জন্য enum: Less, Greater, Equal

fn main() {
    println!("Number Guessing Game 🎯");
    println!("একটা সংখ্যা অনুমান করো 1 থেকে 100 এর মধ্যে!");

    // কম্পিউটার একটা random সংখ্যা বেছে নিচ্ছে 1 থেকে 100 এর মধ্যে
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // এটা শুধু চেক করার জন্য, চাইলে দেখতে পারো
    // println!("(Debug: Secret number is: {})", secret_number);

    loop {
        println!("\nআপনার সংখ্যা লিখুন:");

        // নতুন একটা খালি স্ট্রিং বানানো হলো ইনপুট রাখার জন্য
        let mut guess = String::new();

        // ইনপুট নেওয়া হচ্ছে
        io::stdin()
            .read_line(&mut guess)
            .expect("ইনপুট নিতে সমস্যা হয়েছে!");

        // স্ট্রিংকে সংখ্যা (u32) তে রূপান্তর করছি
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,             // যদি সফলভাবে পার্স হয়
            Err(_) => {
                println!("দয়া করে একটা বৈধ সংখ্যা দিন!");
                continue;               // আবার চেষ্টা করতে বলছে
            }
        };

        println!("আপনি অনুমান করেছেন: {}", guess);

        // এবার তুলনা করা হচ্ছে
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("খুব ছোট! 📉"),
            Ordering::Greater => println!("খুব বড়! 📈"),
            Ordering::Equal => {
                println!("🎉 আপনি জিতেছেন! 🎉");
                break; // লুপ থামিয়ে দিচ্ছে কারণ আপনি ঠিক ধরেছেন
            }
        }
    }
}
