use std::collections::HashMap;


#[allow(dead_code)]
pub struct Chmod{

}


impl Chmod{
    
    pub fn convert_octal_to_symbolic(&self,octal:u16){
        let mut octal_to_symbolic:HashMap<u16,String> = HashMap::new(); 
        octal_to_symbolic.insert(0, "---".to_string());
        octal_to_symbolic.insert(1, "--x".to_string());
        octal_to_symbolic.insert(2, "-w-".to_string());
        octal_to_symbolic.insert(3, "-wx".to_string());
        octal_to_symbolic.insert(4, "r--".to_string());
        octal_to_symbolic.insert(5, "r-x".to_string());
        octal_to_symbolic.insert(6, "rw-".to_string());
        octal_to_symbolic.insert(7, "rwx".to_string());
        let owner:u16 = octal % 10;
        let group:u16 = (octal/10)%10;
        let user:u16  = (octal/100)%10;

        let binding = "<Incorrect Value>".to_string();
        let user_string = octal_to_symbolic.get(&user).unwrap_or(&binding);
        let group_string = octal_to_symbolic.get(&group).unwrap_or(&binding);
        let owner_string = octal_to_symbolic.get(&owner).unwrap_or(&binding);
        print!("{}{}{}",user_string,group_string,owner_string);

    }   
}