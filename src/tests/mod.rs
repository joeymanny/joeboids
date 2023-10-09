use crate::angle::Angle; 
use std::f32::consts::PI;                                                                                                                                                                                                                                                                                                                                               
#[test]
fn face_test_lesser(){
    assert_eq!(
        PI / 2.0, 
        Angle::new(0.0).face(Angle::new(PI / 2.0)));
}
#[test]
fn face_test_greater(){
    assert_eq!(
        format!("{:.6}",-PI / 2.0),
        format!("{:.6}", Angle::new(0.0).face(Angle::new(PI * 1.5)))
    );

}
#[test]
fn face_test_equal(){
    assert_eq!(
        PI,
        Angle::new(0.0).face(Angle::new(PI))
    );
    
}
#[test]
fn face_test_greater_offset_1_pi(){
    assert_eq!(
        PI/2.0
        ,
        Angle::new(PI / 2.0).face(Angle::new(PI)));

}
#[test]
fn face_test_identity_1(){
    let subject = Angle::new(2.6);
    let target = Angle::new(3.9);
    assert_eq!(
        target,
        *subject + subject.face(target)
    )
}
#[test]
fn face_test_identity_2(){
    let subject = Angle::new(3.9);
    let target = Angle::new(2.6);
    assert_eq!(
        target,
        *subject + subject.face(target)
    )
}
#[test]
fn face_test_identity_3(){
    let subject = Angle::new((2.0 * PI) - 0.3 * PI);
    let target = Angle::new(0.3 * PI);
    assert_eq!(
        target,
        subject + Angle(subject.face(target))
    )
}
#[test] 
fn div_pos(){
    assert_eq!(
        Angle::new(PI/4.0),
        Angle::new(PI/2.0) / 2.0
    )
}
#[test]
fn div_neg(){
    assert_eq!(
        Angle::new(PI/-4.0),
        Angle::new(PI/-2.0) / 2.0
    )
}
#[test]
fn div_third(){
    assert_eq!(
        Angle::new(PI/3.0),
        Angle::new(PI) / 3.0
    )
}
#[test]
fn div_other_side(){
    assert_eq!(
        Angle::new(1.75 * PI),
        Angle::new(1.5 * PI) / 2.0
    )
}