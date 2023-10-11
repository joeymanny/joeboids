use crate::angle::rad;
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
fn face_test_greater_offset_1_pi_inverse(){
    assert_eq!(
        Angle::new(PI).face(Angle::new(PI/2.0))
        ,
        PI/-2.0
    );

}
#[test]
fn face_test_identity_1(){
    let subject = Angle::new(2.6);
    let target = Angle::new(3.9);
    assert_eq!(
        target,
        subject + subject.face(target)
    )
}
#[test]
fn face_test_identity_2(){
    let subject = Angle::new(3.9);
    let target = Angle::new(2.6);
    assert_eq!(
        target,
        subject + subject.face(target)
    )
}
#[test]
fn face_test_identity_3(){
    let subject = Angle::new(rad(360.0) - rad(40.0));
    let target = Angle::new(rad(40.0));
    assert_eq!(
        format!("{:.4}", target.0),
        format!("{:.4}", (subject + Angle(subject.face(target))).0)
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
#[test]
fn rad_180(){
    assert_eq!(
        PI,
        rad(180.0)
    );
}
#[test]
fn angle_add_f32(){
    assert_eq!(
        format!("{:.4}", Angle(rad(10.0)).0),
        format!("{:.4}", (Angle(rad(350.0)) + rad(20.0)).0)
    );
}
#[test]
fn face_no_op_360(){
    assert_eq!(
        0.0,
        Angle::new(rad(360.0)).face(Angle::new(rad(360.0)))
    );
}
#[test]
fn face_no_op_0(){
    assert_eq!(
        0.0,
        Angle::new(rad(0.0)).face(Angle::new(rad(0.0)))
    );
}
// #[test]
// fn undo_atan2_1(){
//        todo!();
       
// }