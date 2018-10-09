use std::io;

#[derive(Clone, Debug)]
struct Term {
    binary: [i8; 16],
    drop_term: i8,
}


impl Term {
    fn new(d: i32) -> Term {
        let two: i32 = 2;
        let mut dec = d;
        let mut sign: i8 = 0;
        let mut n = Term {
            binary: [0; 16],
            drop_term: 0,
        };

        if dec < 0 {
            sign = 1;
            dec = (dec * -1);
        }

        for x in 0..7 { 
            n.binary[15 - x] = ((dec / two.pow(x as u32)) % 2) as i8;
        }

        if sign == 1 {
            let mut one = [0; 16];
            one[15] = 1;

            for i in 8..16 {
                 n.binary[i] = (n.binary[i] + 1) % 2;
            } 
        
            Term::add_back(&mut n.binary, one);
        
        }

        (n)
    }

    fn asr(&mut self) {

        self.drop_term = self.binary[15];

        for x in 0..15 {
            self.binary[15 - x] = self.binary[(15 - x) - 1];
        }

    }

    fn add(&mut self, other: &Term ) {
        Term::add_front(&mut self.binary, other.binary);
    }

    fn add_front(a: &mut [i8; 16], b: [i8; 16]) {
        for i in 0..8 { // 8 isn't included
            a[i] = a[i] + b[i + 8];
        }

        for i in 1..8 {
            if a[8-i] > 1 {
                 a[8 - i] = a[8 - i] % 2;
                 a[8 - 1 - i] = a[8 - 1 - i] + 1; 
            }
        }

        a[0] = a[0] % 2;
    }

    
    fn add_back(a: &mut [i8; 16], b: [i8; 16]) {

        for i in 8..16 { // 8 isn't included
            a[i] = a[i] + b[i];
        }

        for i in 1..8 {
            if a[16-i] > 1 {
                 a[16 - i] = a[16 - i] % 2;
                 a[16 - 1 - i] = a[16 - 1 - i] + 1;
            }
        }

        a[8] = a[8] % 2;
    }

    fn sub(&mut self, other: &Term) {
        let mut neg = other.binary;
        let mut one = [0; 16];
        one[15] = 1;

        for i in 8..16 {
            neg[i] = (other.binary[i] + 1) % 2;
        }

        Term::add_back(&mut neg, one); 
        Term::add_front(&mut self.binary, neg);

    }

    fn as_string(&self) -> String {
        let b = self.binary;
        let a = format!(
            "{}{}{}{}{}{}{}{} {}{}{}{}{}{}{}{}",
            b[0],
            b[1],
            b[2],
            b[3],
            b[4],
            b[5],
            b[6],
            b[7],
            b[8],
            b[9],
            b[10],
            b[11],
            b[12],
            b[13],
            b[14],
            b[15]
        );
        (a)
    }

    fn next_op(&self) -> i8 {
        let a = self.binary[15] - self.drop_term;

        match a {
            0 => (1),
            1 => (2),
            -1 => (3),
            _ => (4),
        }

    }
}


fn pretty_print(op: i8, iter: i32, mult: &Term, prod: &mut Term) {
    for a in 0..62 {
        print!("-");
    }

    println!("");

    // 0 -> Title
    // 1 -> 00 && 11 no op
    // 2 -> 10 prod - mcont
    // 3 -> 01 prod + mcont
    // 4 -> init
    match op {
        0 => {
            println!("| Itr |    Ops    |       Mult        |        Prod         |");
        }
        1 => {
            println!(
                "|  {}  |{}| {} | {} {} |",
                iter,
                "   No Op   ",
                mult.as_string(),
                prod.as_string(),
                prod.drop_term
            );
            prod.asr();
            println!(
                "|  {}  |{}| {} | {} {} |",
                iter,
                "   Shift   ",
                mult.as_string(),
                prod.as_string(),
                prod.drop_term
            );
        }
        2 => {
            prod.sub(mult);
            println!(
                "|  {}  |{}| {} | {} {} |",
                iter,
                " Prod-mcon ",
                mult.as_string(),
                prod.as_string(),
                prod.drop_term
            );
            prod.asr();
            println!(
                "|  {}  |{}| {} | {} {} |",
                iter,
                "   Shift   ",
                mult.as_string(),
                prod.as_string(),
                prod.drop_term
            );
        }
        3 => {
            prod.add(mult);
            println!(
                "|  {}  |{}| {} | {} {} |",
                iter,
                " Prod+mcon ",
                mult.as_string(),
                prod.as_string(),
                prod.drop_term
            );
            prod.asr();
            println!(
                "|  {}  |{}| {} | {} {} |",
                iter,
                "   Shift   ",
                mult.as_string(),
                prod.as_string(),
                prod.drop_term
            );
        }
        4 => {
            // iter number | operation | mult | prod |
            println!(
                "|  {}  |{}| {} | {} {} |",
                0,
                "    Init   ",
                mult.as_string(),
                prod.as_string(),
                prod.drop_term
            );
        }
        _ => {
            println!("This should be impossible. Not sure how you got here...");
        }

    }

}


fn get_numb() -> i32 {

    let mut inp = "".to_string();
    println!("Please enter a number");
    io::stdin().read_line(&mut inp);
    let r = inp.trim().parse::<i32>();

    if r.is_ok() {
        let z = r.unwrap();
        if z >= -64 && z <= 63 {
            (z)
        } else {
            println!("Number too big");
            (get_numb())
        }

    } else {
        (get_numb())
    }



}


fn main() {

    let mut mult = Term::new(get_numb());
    let mut prod = Term::new(get_numb());

    pretty_print(0, 0, &mult, &mut prod);
    pretty_print(4, 0, &mult, &mut prod);

    for a in 1..9 {
        let o = prod.next_op();
        pretty_print(o, a, &mult, &mut prod);
    }

    println!("The answer is \"{}\"", prod.as_string());

}
