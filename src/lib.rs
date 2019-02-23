use rand::Rng;

pub struct Generation {
    individuals : Vec<Vec<u8>>,
    block_size : usize,
}

impl Generation {
    pub fn new(gene_length: usize, num_of_inds: usize, block_size: usize) -> Generation {
        let inds = initialize_individuals(gene_length, num_of_inds);
        Generation { individuals : inds, block_size : block_size }
    }
}

fn initialize_individuals(gene_length: usize, num_of_inds: usize) -> Vec<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let mut gene = vec![0u8; ((gene_length-1)/8)+1];
    let mut individuals: Vec<Vec<u8>> = Vec::with_capacity(num_of_inds);
    for _ in 0..num_of_inds {
        rng.fill(&mut gene[..]);
        individuals.push(gene.clone());
    }
    individuals
}

pub fn binary_ops() {
    let a = 0b11110000u8;
    let b = 0b00110011u8;
    println!("a  : {:08b}", a  );
    println!("b  : {:08b}", b  );
    println!("AND: {:08b}", a&b);
    println!("OR : {:08b}", a|b);
    println!("XOR: {:08b}", a^b);
}
