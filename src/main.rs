fn main() {
    // let mut val = "reciprocal";
    // val = "asd";


    // let another = &val;

    // let other = &&another;
    
    // let value = &val;
    // println!("{value}")

    //Expression block
    let sum = {
        let number_1 = 11;
        let number_2 = 31;
        number_1 +  number_2
    };
    

    // println!("{sum}")


    let a_number = 27;

    let a_number = a_number +  5; // this is a shadowing variable

     
    // let suma = &sum;
    // println!("{suma}");
    // println!(" Soy {} , y mi apellido es {}"  , "Matu" ,"Ledesma");

    println!(" Tengo {} ", a_number);


    let tuple = (true, 67, "matu", 4.5);


    println!("{}", tuple.0)


}
