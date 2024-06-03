enum StudentProfile{
    Name(String),
    Roll_no(Int),
    Total_marks(f64),
}

fn main(){
    let students = vec![
        StudentProfile::Name(String::from("Swata Swayam Dash")),
        StudentProfile::Roll_no(60),
        StudentProfile::Total_marks(435.234),
    ];
    {
        let v = vec![1,2,3];
        // do stuff with v
    }//// <- v goes out of scope and is freed here


}