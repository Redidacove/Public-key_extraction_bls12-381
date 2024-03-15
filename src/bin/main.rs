use lambdaworks_math::{
    cyclic_group::IsGroup,
};
use lambdaworks_math::elliptic_curve::{
    short_weierstrass::curves::bls12_381::curve::BLS12381Curve,
    traits::IsEllipticCurve,
};
// use lambdaworks_math::elliptic_curve::traits::IsEllipticCurve;

fn main(){
    let g = BLS12381Curve::generator();
    let sk:u64=0x6C616D6264617370;
    g.operate_with_self(sk);
    let g2=g.to_affine();
    let x=g2.x();
    let y=g2.y();

    println!("pub_key_x = {}",x);
    println!("pub_key_y = {}",y);
    println!("pubkey concatenated = {}{}",x,y);
}
