use super::*; 
fn absolute_diff(x:f32, y:f32) -> f32{
    (x - y).abs()
}                                                                                                                                                                                                                                                                                                                                                  
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