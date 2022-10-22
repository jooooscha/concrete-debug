use concrete_integer::gen_keys;
use concrete_integer::wopbs::*;
use concrete_shortint::parameters::*;
use concrete_shortint::parameters::parameters_wopbs_message_carry::*;

fn main () {
    println!("Start");
    let basis : Vec<u64> = vec![5,7];
    let nb_block = basis.len();

    //Generate the client key and the server key:
    println!("Gen keys");
    let (cks, sks) = gen_keys(&PARAM_MESSAGE_1_CARRY_0);
    println!("Gen wopbs");
    let wopbs_key =  WopbsKey::new_wopbs_key(&cks, &sks, &WOPBS_PARAM_MESSAGE_1_CARRY_0);
    println!("Ready");

    let mut msg_space = 1;
    for modulus in basis.iter() {
        msg_space *= modulus;
    }
    let clear = 42 % msg_space;
    let ct = cks.encrypt_crt(clear, basis.clone());
    let ct = wopbs_key.keyswitch_to_wopbs_params(&sks,&ct);
    let lut = wopbs_key.generate_lut_crt(&ct, |x| x);
    let ct_res = wopbs_key.wopbs(&ct, &lut);
    let ct_res = wopbs_key.keyswitch_to_pbs_params(&ct_res);
    let res = cks.decrypt_crt(&ct_res);
    assert_eq!(res, clear);
}
