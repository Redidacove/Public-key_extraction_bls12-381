use lambdaworks_math::{
    cyclic_group::IsGroup,
    elliptic_curve::traits::IsEllipticCurve,field::element::FieldElement
};
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::field_extension::BLS12381PrimeField;

// use sha3::{Digest, Sha3_256};
type FEE=FieldElement<BLS12381PrimeField>;

fn main(){
    let g = bls12_381::curve::BLS12381Curve::generator();
    // let sk="0x6C616D6264617370";
    let sk=7809643498195481456u64;
    // let without_prefix = sk.trim_start_matches("0x");
    // let z = u64::from_str_radix(without_prefix, 16);
    // let z :u64= z.unwrap();
    g.operate_with_self(sk);
    let g_affine=g.to_affine();
    let x=g_affine.x();
    let y=g_affine.y();
    println!("pub_key_x = {}",x);
    println!("pub_key_y = {}",y);

    let res_x = FEE::new_base("67F9FFC5EAF6C19292112EADF50C11E7460E7568493141676F6BA1374BADD9F6AB1F2F5E155B0E3D2F4C1DE79554F80");
    let res_y = FEE::new_base("18509D22F2107B667A8F75DE737A4FB967F6C3E745A7C2361868515402318F006BD360B8A8763D7844381C6E510799CC");
    let expected_pk = bls12_381::curve::BLS12381Curve::create_point_from_affine(res_x, res_y).unwrap();
    // assert_eq!(g, expected_pk);
}

