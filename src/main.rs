use std::collections::HashMap; //HasMmap
use std::env; //Arguments

//Prototype struct pour stocker l'instruction et encodage
struct OpcodeSig{
    pub instruction : String,
    pub encoding : String
}

fn gen_decod_tabl() -> HashMap<u8, OpcodeSig>{
    //hashmap pour sauvegarder chaque signification en fonction de l'opcode (binaire)
    let mut decode_tabl: HashMap<u8, OpcodeSig> = HashMap::new();

    decode_tabl.insert(0b1100011, OpcodeSig{ instruction: String::from("BRANCH"), encoding: String::from("Sb")});
    decode_tabl.insert(0b1100111, OpcodeSig{ instruction: String::from("JALR"), encoding: String::from("I") });
    decode_tabl.insert(0b0000011, OpcodeSig{ instruction: String::from("LOAD"), encoding: String::from("I") });
    decode_tabl.insert(0b0001111, OpcodeSig{ instruction: String::from("MISC-MEM"), encoding: String::from("I") });
    decode_tabl.insert(0b0010011, OpcodeSig{ instruction: String::from("OP-IMM"), encoding: String::from("I") });
    decode_tabl.insert(0b1110011, OpcodeSig{ instruction: String::from("SYSTEM"), encoding: String::from("I") });
    decode_tabl.insert(0b1101111, OpcodeSig{ instruction: String::from("JAL"), encoding: String::from("Uj") });
    decode_tabl.insert(0b0110011, OpcodeSig{ instruction: String::from("OP"), encoding: String::from("R") });
    decode_tabl.insert(0b0100011, OpcodeSig{ instruction: String::from("STORE"), encoding: String::from("S") });
    decode_tabl.insert(0b0010111, OpcodeSig{ instruction: String::from("AUIPC"), encoding: String::from("U") });
    decode_tabl.insert(0b0110111, OpcodeSig{ instruction: String::from("LUI"), encoding: String::from("U") });

    return decode_tabl;
}

fn file_reader(file_path: &String) -> Vec<u8>{
    return std::fs::read(file_path).unwrap();
}

fn little_endian_word_opcode(byte1: u8, byte2: u8, byte3: u8, byte4: u8) -> (u32, u8){
    let word = u32::from_le_bytes([byte1, byte2, byte3, byte4]);
    let word_opcode = (word & 0x7F) as u8;
    return (word, word_opcode);
}

fn main() {
    //Récupère les arguments au lancement du script
    let args: Vec<String> = env::args().collect();

    if  args.len() < 2 || args[1] == "-h" {
        println!("Usage : {} <binary_file>", args[0]);
        return;
    }

    let decode_tabl = gen_decod_tabl();

    let bytes = file_reader(&args[1]);

    //println!("Ce que j'ai sauvegardé : {} {}", decode_tabl.get(&0b1100011).unwrap().instruction, decode_tabl.get(&0b1100011).unwrap().encoding);
    //println!("{:08b} {:08b} {:08b} {:08b}", bytes[0], bytes[1], bytes[2], bytes[3]);
    //println!("{}", decode_tabl.get(&bytes[43]).unwrap().instruction);

    println!("offset, value, opcode, encoding");
    let mut offset: u8 = 0;
    for word_bytes in bytes.chunks_exact(4) {
        let instru: String;
        let encod: String;

        let (word, word_opcode) = little_endian_word_opcode(word_bytes[0], word_bytes[1], word_bytes[2], word_bytes[3]);

        if decode_tabl.contains_key(&word_opcode){
            instru = decode_tabl.get(&word_opcode).unwrap().instruction.to_string();
            encod = decode_tabl.get(&word_opcode).unwrap().encoding.to_string();
        }else {
            instru = String::from("NOT_FOUND");
            encod = String::from("NOT_FOUND");
        }
        //println!("{:08b} {:08b} {:08b} {:08b} {}",word_bytes[0], word_bytes[1], word_bytes[2], word_bytes[3], instru);
        println!("{:08x}, {:08x}, {}, {}", offset, word, instru, encod);
        offset += 4;
    }
}
