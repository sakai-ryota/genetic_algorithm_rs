use rand::Rng;

fn initialize() {
    let mut rng = rand::thread_rng();
    let mut gene = vec![0u8; 5000];
    let mut genes: Vec<Vec<u8>> = Vec::with_capacity(1000);
    for _ in 0..100 {
        rng.fill(&mut gene[..]);
        genes.push(gene.clone());
    }
}

fn binary_ops() {
    let a = 0b11110000u8;
    let b = 0b00110011u8;
    println!("a  : {:08b}", a  );
    println!("b  : {:08b}", b  );
    println!("AND: {:08b}", a&b);
    println!("OR : {:08b}", a|b);
    println!("XOR: {:08b}", a^b);
}

fn main() {
    //initialize();
    binary_ops();
}
